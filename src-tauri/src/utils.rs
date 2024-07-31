use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    status: String,
    data: T,
    err: String,
}

pub fn create_res<T>(data: T, err: String) -> Response<T> {
    let status = if err.is_empty() { "ok" } else { "err" };
    Response {
        status: status.to_string(),
        data,
        err,
    }
}

pub fn create_res_ok<T>(data: T) -> Response<T> {
    Response {
        status: "ok".to_string(),
        data,
        err: "".to_string(),
    }
}

pub fn create_res_err(err: String) -> Response<String> {
    Response {
        status: "err".to_string(),
        data: "".to_string(),
        err,
    }
}
