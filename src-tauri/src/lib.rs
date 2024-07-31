// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use crate::config::{BiliConfig, create_default_config, read_config, save_config};
use crate::utils::{create_res, create_res_err, create_res_ok, Response};

mod utils;
mod config;
mod path;

#[tauri::command]
fn get_config() -> Response<BiliConfig> {
    match read_config() {
        Ok(data) => create_res_ok(data),
        Err(err) => create_res(create_default_config(), format!("load config failed: [{:?}].", err)),
    }
}

#[tauri::command]
fn update_config(config: BiliConfig) -> Response<String> {
    match save_config(config) {
        Ok(data) => create_res_ok("ok".to_string()),
        Err(err) => create_res_err(format!("update config failed: [{:?}].", err)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            update_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
