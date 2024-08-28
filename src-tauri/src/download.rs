use std::collections::HashMap;
use std::ffi::{c_char, CString};
use std::fs::{OpenOptions, remove_file};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, RANGE, REFERER, USER_AGENT};
use rusqlite::{Connection, Error, params, Result};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tauri::path::BaseDirectory;
use tauri_plugin_shell::ShellExt;
use tokio::sync::{mpsc, Semaphore};
use tokio::sync::Mutex;

use crate::Agent;
use crate::config::CONFIG;
use crate::path::{get_path_absolute, get_path_str, get_unique_file_path};

// 定义一个结构体来表示数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Download {
    pub(crate) id: i32,
    pub(crate) video_url: String,
    pub(crate) audio_url: String,
    pub(crate) file_name: String,
    pub(crate) file_path: String,
    pub(crate) referer: String,
    pub(crate) total_size: i64,
    pub(crate) video_size: i64,
    pub(crate) audio_size: i64,
    pub(crate) downloaded_size: i64,
    pub(crate) status: String,
    pub(crate) added_date: String,
    pub(crate) last_updated_date: String,
}

// 定义下载进度用于发布事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadProgress {
    pub(crate) id: i32,
    pub(crate) chunk_length: i64,
}

impl Download {
    fn to_params(&self) -> (String, String, String, String, String, i64, i64, i64, i64, String, String) {
        (
            self.video_url.clone(),
            self.audio_url.clone(),
            self.file_name.clone(),
            self.file_path.clone(),
            self.referer.clone(),
            self.video_size.clone(),
            self.audio_size.clone(),
            self.total_size,
            self.downloaded_size,
            self.status.clone(),
            self.added_date.clone(),
        )
    }
}

lazy_static! {
    static ref TASK_MAP: Arc<Mutex<HashMap<i32, mpsc::Sender<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref CONN: Mutex<Connection> = Mutex::new(create_table("downloads.db").unwrap());
    static ref SEMAPHORE: Arc<Semaphore> = Arc::new(Semaphore::new(3));
}

pub async fn add_download_file(app: AppHandle, mut download: Download) -> Result<()> {
    {
        let mut config = CONFIG.lock().unwrap();
        download.file_path = get_path_absolute(&config.save_path, &[(format!("{}{}", download.file_name, ".mp4")).as_str()]);
        download.file_path = get_unique_file_path(&download.file_path);
    }

    if !download.video_url.is_empty() && download.video_size == 0 {
        match get_file_size(&download.video_url, &download.referer).await {
            Ok(size) => download.video_size = size,
            Err(err) => {
                println!("{}", err);
                return Err(Error::SqliteSingleThreadedMode);
            }
        }
    }
    if !download.audio_url.is_empty() && download.audio_size == 0 {
        match get_file_size(&download.audio_url, &download.referer).await {
            Ok(size) => download.audio_size = size,
            Err(err) => {
                println!("{}", err);
                return Err(Error::SqliteSingleThreadedMode);
            }
        }
    }
    download.total_size = download.audio_size + download.video_size;

    let id;
    // 插入数据
    {
        let conn = &*CONN.lock().await;
        if let Err(e) = conn.execute(
            "INSERT INTO downloads (video_url, audio_url, file_name, file_path, referer, video_size, audio_size, total_size, downloaded_size, status, added_date, last_updated_date)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![download.video_url, download.audio_url, download.file_name, download.file_path, download.referer, download.video_size, download.audio_size, download.total_size, download.downloaded_size, download.status, download.added_date, download.last_updated_date],
        ) {
            eprintln!("Error inserting data: {}", e);
        }
        id = conn.last_insert_rowid() as i32;
    }

    tokio::spawn(start_downloading(app, id));

    Ok(())
}

pub async fn get_all_downloading_files() -> Result<Vec<Download>> {
    // 查询数据
    let conn = &*CONN.lock().await;
    let mut stmt = conn.prepare("SELECT id, video_url, audio_url, file_name, file_path, referer, video_size, audio_size, total_size, downloaded_size, status, added_date, last_updated_date FROM downloads WHERE status == 'downloading' OR status == 'paused'")?;
    let download_iter = stmt.query_map([], |row| {
        Ok(Download {
            id: row.get(0)?,
            video_url: row.get(1)?,
            audio_url: row.get(2)?,
            file_name: row.get(3)?,
            file_path: row.get(4)?,
            referer: row.get(5)?,
            video_size: row.get(6)?,
            audio_size: row.get(7)?,
            total_size: row.get(8)?,
            downloaded_size: row.get(9)?,
            status: row.get(10)?,
            added_date: row.get(11)?,
            last_updated_date: row.get(12)?,
        })
    })?;

    let mut downloads = Vec::new();
    for download in download_iter {
        downloads.push(download?);
    }

    Ok(downloads)
}

pub async fn get_all_downloaded_files() -> Result<Vec<Download>> {
    // 查询数据
    let conn = &*CONN.lock().await;
    let mut stmt = conn.prepare("SELECT id, video_url, audio_url, file_name, file_path, referer, video_size, audio_size, total_size, downloaded_size, status, added_date, last_updated_date FROM downloads WHERE status == 'completed'")?;
    let download_iter = stmt.query_map([], |row| {
        Ok(Download {
            id: row.get(0)?,
            video_url: row.get(1)?,
            audio_url: row.get(2)?,
            file_name: row.get(3)?,
            file_path: row.get(4)?,
            referer: row.get(5)?,
            video_size: row.get(6)?,
            audio_size: row.get(7)?,
            total_size: row.get(8)?,
            downloaded_size: row.get(9)?,
            status: row.get(10)?,
            added_date: row.get(11)?,
            last_updated_date: row.get(12)?,
        })
    })?;

    let mut downloads = Vec::new();
    for download in download_iter {
        downloads.push(download?);
    }

    Ok(downloads)
}

pub async fn search_downloads(text: String) -> Result<Vec<Download>> {
    let search_pattern = format!("%{}%", text);
    // 查询数据
    let conn = &*CONN.lock().await;
    let mut stmt = conn.prepare("SELECT id, video_url, audio_url, file_name, file_path, referer, video_size, audio_size, total_size, downloaded_size, status, added_date, last_updated_date FROM downloads WHERE file_name like ?1")?;
    let download_iter = stmt.query_map([search_pattern], |row| {
        Ok(Download {
            id: row.get(0)?,
            video_url: row.get(1)?,
            audio_url: row.get(2)?,
            file_name: row.get(3)?,
            file_path: row.get(4)?,
            referer: row.get(5)?,
            video_size: row.get(6)?,
            audio_size: row.get(7)?,
            total_size: row.get(8)?,
            downloaded_size: row.get(9)?,
            status: row.get(10)?,
            added_date: row.get(11)?,
            last_updated_date: row.get(12)?,
        })
    })?;

    let mut downloads = Vec::new();
    for download in download_iter {
        downloads.push(download?);
    }

    Ok(downloads)
}

pub async fn delete_download_file(id: i32) -> Result<()> {
    // 先暂停下下载，如果正在下载的话
    let _ = stop_downloading(id).await;

    // 删除数据
    let conn = &*CONN.lock().await;
    if let Err(e) = conn.execute("DELETE FROM downloads WHERE id = ?1", params![id]) {
        eprintln!("Error inserting data: {}", e);
    }

    Ok(())
}

pub async fn get_download_file(id: i32) -> Result<(Download)> {
    // 查询数据
    let conn = &*CONN.lock().await;
    let mut stmt = conn.prepare("SELECT id, video_url, audio_url, file_name, file_path, referer, video_size, audio_size, total_size, downloaded_size, status, added_date, last_updated_date FROM downloads WHERE id = ?1")?;
    let download = stmt.query_row([id], |row| {
        Ok(Download {
            id: row.get(0)?,
            video_url: row.get(1)?,
            audio_url: row.get(2)?,
            file_name: row.get(3)?,
            file_path: row.get(4)?,
            referer: row.get(5)?,
            video_size: row.get(6)?,
            audio_size: row.get(7)?,
            total_size: row.get(8)?,
            downloaded_size: row.get(9)?,
            status: row.get(10)?,
            added_date: row.get(11)?,
            last_updated_date: row.get(12)?,
        })
    })?;

    Ok(download)
}

pub async fn update_download_file(download: &Download) -> Result<()> {
    let conn = &*CONN.lock().await;
    if let Err(e) = conn.execute(
        "UPDATE downloads SET video_size = ?1, audio_size = ?2, total_size = ?3, downloaded_size = ?4, status = ?5, last_updated_date = ?6 WHERE id = ?7",
        params![download.video_size, download.audio_size, download.total_size, download.downloaded_size, download.status, download.last_updated_date, download.id],
    ) {
        eprintln!("Error inserting data: {}", e);
    }

    Ok(())
}

pub async fn start_downloading(app: AppHandle, id: i32) -> Result<(), String> {
    let (tx, mut rx) = mpsc::channel(1);
    {
        // 将任务句柄和取消信号发送者存储到 HashMap 中
        let mut map = TASK_MAP.lock().await;
        if map.contains_key(&id) {
            return Err("task already existed".to_string());
        }
        map.insert(id, tx);
    }

    let mut download = get_download_file(id).await.unwrap();
    if download.status == "paused" {
        download.status = "downloading".to_string();
        update_download_file(&download).await.unwrap();
    }

    // 启动下载任务
    loop {
        tokio::select! {
            // 下载任务
            result = download_file(&app, &mut download) => {
                match result {
                    Ok(_) => {
                        println!("Download completed successfully.");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Download failed with error: {}", e);
                        app.emit("progress", DownloadProgress {
                           id: download.id,
                           chunk_length: -1,
                        }).unwrap();
                        download.status = "failed".to_string();
                        update_download_file(&download).await.unwrap();
                        break;
                    }
                }
            }
            // 检查是否接收到取消信号
            Some(_) = rx.recv() => {
                println!("Download interrupted");
                break;
            }
        }
    }

    Ok(())
}

pub async fn download_file(app: &AppHandle, mut download: &mut Download) -> Result<(), String> {
    let permit = SEMAPHORE.acquire().await.unwrap();
    {
        let mut map = TASK_MAP.lock().await;
        if !map.contains_key(&download.id) {
            return Err(format!("downloading error, {:?} didn't exist", download));
        }
    }

    let client = Client::new();
    // 构造文件路径
    let mut video_file = download.file_path.clone();
    let mut audio_file = download.file_path.clone();
    if download.video_size != 0 && download.audio_size != 0 {
        video_file = download.file_path.replace(".mp4", "_video.mp4");
        audio_file = download.file_path.replace(".mp4", "_audio.mp4");
    }

    // 断点续传下载文件
    if download.downloaded_size < download.video_size && download.video_size != 0 {
        let mut headers1 = HeaderMap::new();
        headers1.insert(
            RANGE,
            format!(
                "bytes={}-{}",
                &download.downloaded_size, &download.video_size
            )
                .parse()
                .unwrap(),
        );
        headers1.insert(USER_AGENT, HeaderValue::from_static(Agent));
        headers1.insert(REFERER, HeaderValue::from_str(&download.referer).unwrap());

        let mut res = client
            .get(&download.video_url)
            .headers(headers1)
            .send()
            .await
            .unwrap();

        if !res.status().is_success() {
            return Err(format!("request video url failed, response: {:?}", res));
        }

        let mut downloaded_file_size = download.downloaded_size;

        let path = Path::new(&video_file);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap();
        let mut writer = BufWriter::new(&file);

        let mut last_update_time = tokio::time::Instant::now();
        let update_interval = Duration::from_secs(1); // 每 2 秒更新一次
        while let Some(chunk) = res.chunk().await.unwrap() {
            downloaded_file_size += chunk.len() as i64;
            writer.write_all(&chunk).unwrap();
            writer.flush().unwrap();

            // 计算进度
            let percent_completed = (downloaded_file_size * 100 / download.total_size) as u32;
            println!("Downloaded {} bytes", downloaded_file_size);
            println!("Download progress: {}%", percent_completed);

            download.downloaded_size = downloaded_file_size;

            // 检查是否需要更新
            if tokio::time::Instant::now().duration_since(last_update_time) >= update_interval {
                let download_clone = download.clone();
                app.emit("progress", DownloadProgress {
                    id: download.id,
                    chunk_length: download.downloaded_size,
                }).unwrap();
                tokio::spawn(async move {
                    update_download_file(&download_clone).await.unwrap();
                });
                last_update_time = tokio::time::Instant::now();
            }
        }
    }

    if download.total_size - download.downloaded_size <= download.audio_size && download.audio_size != 0 {
        let mut headers1 = HeaderMap::new();
        headers1.insert(
            RANGE,
            format!(
                "bytes={}-{}",
                &download.downloaded_size - download.video_size, &download.audio_size
            )
                .parse()
                .unwrap(),
        );
        headers1.insert(USER_AGENT, HeaderValue::from_static(Agent));
        headers1.insert(REFERER, HeaderValue::from_str(&download.referer).unwrap());

        let mut res = client
            .get(&download.audio_url)
            .headers(headers1)
            .send()
            .await
            .unwrap();

        if !res.status().is_success() {
            return Err(format!("request video url failed, response: {:?}", res));
        }

        let mut downloaded_file_size = download.downloaded_size;

        let path = Path::new(&audio_file);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap();
        let mut writer = BufWriter::new(&file);

        let mut last_update_time = tokio::time::Instant::now();
        let update_interval = Duration::from_secs(1); // 每 2 秒更新一次
        while let Some(chunk) = res.chunk().await.unwrap() {
            downloaded_file_size += chunk.len() as i64;
            writer.write_all(&chunk).unwrap();
            writer.flush().unwrap();

            // 计算进度
            let percent_completed = (downloaded_file_size * 100 / download.total_size) as u32;
            println!("Downloaded {} bytes", downloaded_file_size);
            println!("Download progress: {}%", percent_completed);

            download.downloaded_size = downloaded_file_size;

            // 检查是否需要更新
            if tokio::time::Instant::now().duration_since(last_update_time) >= update_interval {
                let download_clone = download.clone();
                app.emit("progress", DownloadProgress {
                    id: download.id,
                    chunk_length: download.downloaded_size,
                }).unwrap();
                tokio::spawn(async move {
                    update_download_file(&download_clone).await.unwrap();
                });
                last_update_time = tokio::time::Instant::now();
            }
        }
    }

    if download.video_size != 0 && download.audio_size != 0 {
        match merge_file(app, download).await {
            Ok(_) => {}
            Err(err) => { return Err(err) }
        }
    }

    download.downloaded_size = download.total_size;
    download.status = "completed".to_string();
    update_download_file(download).await.unwrap();
    app.emit("progress", DownloadProgress {
        id: download.id,
        chunk_length: download.total_size,
    }).unwrap();

    Ok(())
}

pub async fn stop_downloading(id: i32) -> Result<(), String> {
    let tx;
    {
        let mut map = TASK_MAP.lock().await;
        if !map.contains_key(&id) {
            return Err("task didn't exist".to_string());
        }
        tx = map.remove(&id).unwrap();
    }

    let mut download = get_download_file(id).await.unwrap();
    download.status = "paused".to_string();
    update_download_file(&download).await.unwrap();

    tx.send(()).await.unwrap();

    Ok(())
}

pub async fn check_download_init(app: AppHandle) {
    let downloadings = get_all_downloading_files().await.unwrap();
    for downloading in downloadings {
        start_downloading(app.clone(), downloading.id).await.unwrap();
    }
}

async fn get_file_size(url: &str, referer: &str) -> Result<i64, String> {
    let client = Client::new();
    let mut headers2 = HeaderMap::new();
    headers2.insert(RANGE, HeaderValue::from_str("bytes=0-100").unwrap());
    headers2.insert(USER_AGENT, HeaderValue::from_static(Agent));
    headers2.insert(REFERER, HeaderValue::from_str(&referer).unwrap());
    let response = client
        .get(url)
        .headers(headers2)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        return Err(format!("request download url failed, response:{:?}", response));
    }

    let content_range = response
        .headers()
        .get("Content-Range")
        .ok_or("No Content-Range header found");
    let content_range_str = content_range.unwrap().to_str().unwrap();
    let file_total_size: i64 = content_range_str
        .split('/')
        .nth(1)
        .ok_or("Invalid Content-Range format")
        .unwrap()
        .parse()
        .unwrap();

    Ok(file_total_size)
}

// async fn merge_file(app: &AppHandle, download: &mut Download) -> Result<(), String> {
//     // 构造文件路径
//     let video_file = download.file_path.replace(".mp4", "_video.mp4");
//     let audio_file = download.file_path.replace(".mp4", "_audio.mp4");
//
//     // 使用 ffmpeg
//     let sidecar_command = app.shell().sidecar("ffmpeg").unwrap();
//
//     // 设置参数
//     let args = vec![
//         "-i", &video_file,
//         "-i", &audio_file,
//         "-vcodec", "copy",
//         "-acodec", "copy",
//         &download.file_path,
//     ];
//
//     // 执行命令
//     let output = sidecar_command
//         .args(args)
//         .output()
//         .await
//         .unwrap();
//
//     // 处理输出
//     if !output.status.success() {
//         eprintln!("Error executing ffmpeg: {:?}", output.stderr);
//         return Err("ffmpeg error".to_string());
//     }
//
//     if Path::new(&video_file).exists() {
//         match remove_file(video_file) {
//             Ok(_) => {}
//             Err(e) => eprintln!("Failed to delete video file: {}", e),
//         }
//     } else {
//         println!("video file does not exist.");
//     }
//
//     if Path::new(&audio_file).exists() {
//         match remove_file(audio_file) {
//             Ok(_) => {}
//             Err(e) => eprintln!("Failed to delete audio file: {}", e),
//         }
//     } else {
//         println!("audio file does not exist.");
//     }
//
//     Ok(())
// }
async fn merge_file(app: &AppHandle, download: &mut Download) -> Result<(), String> {
    unsafe {
        let resource_path;
        if cfg!(debug_assertions) {
            // 开发环境
            resource_path = app.path().resolve("bin/libffmpeg.dll", BaseDirectory::Resource).unwrap();
        } else {
            // 打包后的环境
            resource_path = app.path().resolve("libffmpeg.dll", BaseDirectory::Resource).unwrap();
        }
        let libffmpeg = Library::new(resource_path).unwrap();
        let merge_video_audio: Symbol<unsafe extern "C" fn(*const c_char, *const c_char, *const c_char) -> bool> = libffmpeg.get(b"merge_video_audio").unwrap();
        let video_path = CString::new(download.file_path.replace(".mp4", "_video.mp4")).unwrap();
        let audio_path = CString::new(download.file_path.replace(".mp4", "_audio.mp4")).unwrap();
        let output_path = CString::new(download.file_path.clone()).unwrap();
        let ret = merge_video_audio(video_path.as_ptr(), audio_path.as_ptr(), output_path.as_ptr());

        if Path::new(&download.file_path.replace(".mp4", "_video.mp4")).exists() {
            match remove_file(download.file_path.replace(".mp4", "_video.mp4")) {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to delete video file: {}", e),
            }
        } else {
            println!("video file does not exist.");
        }

        if Path::new(&download.file_path.replace(".mp4", "_audio.mp4")).exists() {
            match remove_file(download.file_path.replace(".mp4", "_audio.mp4")) {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to delete audio file: {}", e),
            }
        } else {
            println!("audio file does not exist.");
        }

        if !ret {
            Err("failed to merge video and audio file".parse().unwrap())
        } else {
            Ok(())
        }
    }
}

pub fn create_table(db_name: &str) -> Result<(Connection)> {
    let mut config = CONFIG.lock().unwrap();
    // 创建表格
    let conn = Connection::open(get_path_str("download.db"))?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS downloads (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            video_url       TEXT NULL,
            audio_url       TEXT NULL,
            file_name       TEXT NOT NULL,
            file_path       TEXT NOT NULL,
            referer         TEXT NOT NULL,
            video_size      INTEGER,
            audio_size      INTEGER,
            total_size      INTEGER,
            downloaded_size INTEGER NOT NULL,
            status          TEXT NOT NULL CHECK(status IN ('downloading', 'completed', 'paused', 'failed')),
            added_date      TEXT NOT NULL,
            last_updated_date TEXT
        )",
        [],
    )?;

    Ok(conn)
}
