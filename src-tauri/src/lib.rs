use std::sync::{Mutex};
use lazy_static::lazy_static;
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::ShellExt;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use crate::config::{create_default_config, read_config, save_config, BiliConfig};
use crate::download::{add_download_file, get_all_downloaded_files, get_all_downloading_files, start_downloading, stop_downloading, Download, search_downloads, delete_download_file};
use crate::utils::{create_res, create_res_err, create_res_ok, Response};

mod config;
mod download;
mod path;
mod utils;

const DANMU_URL: &str = "https://api.bilibili.com/x/v1/dm/list.so?oid=";
const Cookie: &str = "buvid_fp_plain=undefined; buvid4=C5805786-4D94-E90A-594E-57A635A4772A95558-022120521-UK%2F6gr%2BzGhIJrqTUTPMI2w%3D%3D; header_theme_version=CLOSE; theme_style=light; DedeUserID=182771883; DedeUserID__ckMd5=359687335a028aa0; hit-new-style-dyn=1; LIVE_BUVID=AUTO7316922054510962; enable_web_push=DISABLE; buvid3=4CB663D6-6208-6C0E-4392-4C7DCEAB3E5E94322infoc; b_nut=1701866094; _uuid=23F35F18-E866-B9D7-7B4F-33B10A6C18C6994476infoc; home_feed_column=5; browser_resolution=1455-767; rpdid=|(u|Jl)~Yl~R0J'u~|m)|kukJ; hit-dyn-v2=1; CURRENT_BLACKGAP=0; CURRENT_FNVAL=4048; fingerprint=c95fbfb8fdb2168bff017ea7a215bb83; buvid_fp=7092100652d0dbb4d4cca8fd1c931d3d; PVID=1; bsource=search_bing; CURRENT_QUALITY=120; bp_t_offset_182771883=959901692889923584; SESSDATA=b329849a%2C1737902430%2Cf1736%2A72CjBn0ONB7IbwuafJ6dRizBh4qJZdYRUX80PHQ4q-kE0c2eWch3j2O2MnCO7ZwKimyz0SVlBueUNad2ZvdDhzcGdVa2tZWHc2NVlkMlhJYkQ3eXFJbzdYQ29TUGEzN205MWdFRTJEemg5d3RSbmk4Wi1yQnVOcTRTckF6QVhvaXBocXgxYmFUNXl3IIEC; bili_jct=c6aba40e58996f4eb38b86f5d6e6398b; sid=4khayo10; bili_ticket=eyJhbGciOiJIUzI1NiIsImtpZCI6InMwMyIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MjI3MzY2NTYsImlhdCI6MTcyMjQ3NzM5NiwicGx0IjotMX0.9Gz26uUFwCVKbrgTSdGjBSWsxZZRMRuo9R89Pt4dwXw; bili_ticket_expires=1722736596; b_lsid=1295D8A9_1910C55F52D";
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
fn get_downloading_files() -> Response<Vec<Download>> {
    match get_all_downloading_files() {
        Ok(data) => create_res_ok(data),
        Err(err) => create_res(
            Vec::new(),
            format!("list downloading files failed: [{:?}].", err),
        ),
    }
}

#[tauri::command]
fn get_downloaded_files() -> Response<Vec<Download>> {
    match get_all_downloaded_files() {
        Ok(data) => create_res_ok(data),
        Err(err) => create_res(
            Vec::new(),
            format!("list downloaded files failed: [{:?}].", err),
        ),
    }
}

#[tauri::command]
fn search_downloaded(text: String) -> Response<Vec<Download>> {
    match search_downloads(text) {
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
fn delete_download(id: i32) -> Response<String> {
    match delete_download_file(id) {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
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
            open_file_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}