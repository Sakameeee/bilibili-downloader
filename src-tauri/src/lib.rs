use std::error::Error;
use std::fs::OpenOptions;
use std::future::Future;
use std::io::Write;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::ShellExt;

use crate::anime::{Anime, check_ep_id, get_anime_info};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use crate::config::{BiliConfig, CONFIG, create_default_config, read_config, save_config};
use crate::download::{add_download_file, check_download_init, delete_download_file, Download, get_all_downloaded_files, get_all_downloading_files, search_downloads, start_downloading, stop_downloading};
use crate::path::{get_path_absolute, get_unique_file_path};
use crate::utils::{create_res, create_res_err, create_res_ok, Response};
use crate::video::{get_video_info, Video};

mod config;
mod download;
mod path;
mod utils;
mod anime;
mod video;

const DANMU_URL: &str = "https://api.bilibili.com/x/v1/dm/list.so?oid=";
const Agent: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36 Edg/126.0.0.0";

#[tauri::command]
fn get_config() -> Response<BiliConfig> {
    match read_config() {
        Ok(data) => create_res_ok(data),
        Err(err) => create_res(
            create_default_config(),
            format!("load config failed: [{:?}].", err),
        ),
    }
}

#[tauri::command]
fn update_config(config: BiliConfig) -> Response<String> {
    match save_config(config) {
        Ok(data) => create_res_ok("ok".to_string()),
        Err(err) => create_res_err(format!("update config failed: [{:?}].", err)),
    }
}

#[tauri::command]
async fn get_downloading_files() -> Response<Vec<Download>> {
    match get_all_downloading_files().await {
        Ok(data) => create_res_ok(data),
        Err(err) => create_res(
            Vec::new(),
            format!("list downloading files failed: [{:?}].", err),
        ),
    }
}

#[tauri::command]
async fn get_downloaded_files() -> Response<Vec<Download>> {
    match get_all_downloaded_files().await {
        Ok(data) => create_res_ok(data),
        Err(err) => create_res(
            Vec::new(),
            format!("list downloaded files failed: [{:?}].", err),
        ),
    }
}

#[tauri::command]
async fn search_downloaded(text: String) -> Response<Vec<Download>> {
    match search_downloads(text).await {
        Ok(data) => create_res_ok(data),
        Err(err) => create_res(
            Vec::new(),
            format!("search download files failed: [{:?}].", err),
        ),
    }
}

#[tauri::command]
async fn add_download(app: AppHandle, download: Download) -> Response<String> {
    match add_download_file(app, download).await {
        Ok(_) => create_res_ok("ok".to_string()),
        Err(err) => create_res_err(format!("add download file failed: [{:?}].", err)),
    }
}

#[tauri::command]
async fn delete_download(id: i32) -> Response<String> {
    match delete_download_file(id).await {
        Ok(_) => create_res_ok("ok".to_string()),
        Err(err) => create_res_err(format!("delete download file failed: [{:?}].", err)),
    }
}

#[tauri::command]
async fn start_downloading_file(app: AppHandle, id: i32) -> Response<String> {
    match start_downloading(app, id).await {
        Ok(data) => create_res_ok("ok".to_string()),
        Err(err) => create_res_err(format!("start downloading failed: [{:?}].", err)),
    }
}

#[tauri::command]
async fn stop_downloading_file(id: i32) -> Response<String> {
    match stop_downloading(id).await {
        Ok(_) => create_res_ok("ok".to_string()),
        Err(err) => create_res_err(format!("stop downloading failed: [{:?}].", err)),
    }
}

#[tauri::command]
async fn open_file_directory(app: AppHandle, path: String) -> Result<(), String> {
    let command = match std::env::consts::OS {
        "windows" => "explorer",
        "macos" => "open",
        "linux" => "xdg-open",
        _ => return Err("Unsupported OS".to_string()),
    };

    let args = match std::env::consts::OS {
        "windows" => vec!["/select,".to_string(), path],
        "macos" => vec!["-R".to_string(), path],
        "linux" => vec![path],
        _ => return Err("Unsupported OS".to_string()),
    };

    let shell = app.shell();
    shell
        .command(command)
        .args(args)
        .output()
        .await
        .unwrap();

    Ok(())
}

#[tauri::command]
fn get_animates(ep_id: &str) -> Response<Anime> {
    match tokio::runtime::Runtime::new().unwrap().block_on(get_anime_info(ep_id)) {
        Ok(anime) => create_res_ok(anime),
        Err(err) => create_res(Anime::default(), err)
    }
}

#[tauri::command]
fn get_videos(bv_id: &str) -> Response<Video> {
    let mut id = bv_id;
    if bv_id.starts_with("http") {
        // 定义正则表达式，匹配以 "BV" 开头后面跟随一串字母和数字的字符串
        let re = Regex::new(r"/(BV[a-zA-Z0-9]+)/").unwrap();

        // 查找并提取匹配的内容
        if let Some(caps) = re.captures(bv_id) {
            if let Some(bv) = caps.get(1) {
                id = bv.as_str();
            }
        } else {
            create_res(Video::default(), "链接匹配 BV 号失败".to_string());
        }
    }
    match tokio::runtime::Runtime::new().unwrap().block_on(get_video_info(id)) {
        Ok(video) => create_res_ok(video),
        Err(err) => create_res(Video::default(), err)
    }
}

#[tauri::command]
async fn download_cover(url: String) -> Response<String> {
    let client = Client::new();

    // 发送异步 GET 请求
    let response = client.get(&url).send().await.unwrap();

    // 检查请求是否成功
    if !response.status().is_success() {
        create_res_err("request url failed".to_string());
    }

    let file_name;
    let mut save_path;

    {
        let config = CONFIG.lock().unwrap();
        file_name = url.split('/').last().unwrap().to_string();
        save_path = get_path_absolute(&config.save_path, &[&file_name]);
        save_path = get_unique_file_path(&save_path);
    }

    // 创建文件
    let mut out = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(save_path)
        .unwrap();

    // 将响应内容写入文件
    let content = response.bytes().await.unwrap();
    out.write_all(&content).unwrap();

    create_res_ok("ok".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                check_download_init(app_handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            update_config,
            get_downloading_files,
            get_downloaded_files,
            search_downloaded,
            add_download,
            delete_download,
            start_downloading_file,
            stop_downloading_file,
            open_file_directory,
            get_animates,
            get_videos,
            download_cover
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}