use std::{env, path::PathBuf};
use std::path::Path;

pub fn get_path_str(path: &str) -> String {
    let path_buf = get_path(path);

    let path_ref = &path_buf;
    if let Some(path_str) = path_ref.to_str() {
        return path_str.to_string();
    } else {
        return String::new();
    }
}

pub fn get_path(path: &str) -> PathBuf {
    let base_dir: std::path::PathBuf = env::current_dir().unwrap();
    let dir_path: PathBuf = base_dir.join(path);

    return dir_path;
    // return dir_path.canonicalize().unwrap();
}

pub fn get_path_absolute(dir: &str, args: &[&str]) -> String {
    let mut path = PathBuf::from(dir);
    for &arg in args {
        path.push(arg);
    }
    return path.to_str().unwrap().to_string();
}

pub fn get_unique_file_path(original_path: &str) -> String {
    let original_path = Path::new(original_path);

    // 如果文件不存在，直接返回原始路径
    if !original_path.exists() {
        return original_path.to_str().unwrap().to_string();
    }

    // 获取文件名和扩展名
    let file_name = original_path.file_name().unwrap().to_string_lossy();
    let mut base_name = file_name.to_string();
    let mut extension = String::new();

    if let Some(ext) = original_path.extension() {
        extension = format!(".{}", ext.to_string_lossy());
        base_name = file_name.strip_suffix(&extension).unwrap_or(&base_name).to_string();
    }

    // 循环检查是否存在同名文件，若存在则递增数字后缀
    let mut counter = 1;
    loop {
        let new_file_name = format!("{}({}){}", base_name, counter, extension);
        let new_path = original_path.with_file_name(new_file_name);

        if !new_path.exists() {
            return new_path.to_str().unwrap().to_string();
        }

        counter += 1;
    }
}