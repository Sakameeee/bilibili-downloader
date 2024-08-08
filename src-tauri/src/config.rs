use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use serde_json::Result;
use std::path::Path;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::path::get_path_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct BiliConfig {
    pub(crate) cookie: String,
    pub(crate) agent: String,
    pub(crate) save_path: String,
}

lazy_static! {
    pub static ref CONFIG: Mutex<BiliConfig> = Mutex::new(read_config().unwrap());
}

pub fn create_default_config() -> BiliConfig {
    BiliConfig {
        cookie: "".to_string(),
        agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36 Edg/126.0.0.0".to_string(),
        save_path: "D:\\download".to_string(),
    }
}

pub fn check_and_create_config_file() -> Result<()> {
    let path = &get_path_str("config.json");
    if !Path::new(path).exists() {
        let default_config = create_default_config();
        let mut file = File::create(path).unwrap();
        save_config(default_config);
    }
    Ok(())
}

pub fn read_config() -> Result<BiliConfig> {
    check_and_create_config_file()?;
    let path = &get_path_str("config.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: BiliConfig = serde_json::from_reader(reader)?;
    println!("{:?}", config);
    Ok(config)
}

pub fn save_config(config: BiliConfig) -> std::io::Result<()> {
    let path = &get_path_str("config.json");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    let config_data = serde_json::to_string_pretty(&config)?;
    file.set_len(0)?; // 清空文件
    file.write_all(config_data.as_bytes())?;
    let mut old_config = CONFIG.lock().unwrap();
    old_config.save_path = config.save_path;
    old_config.cookie = config.cookie;
    old_config.agent = config.agent;
    Ok(())
}
