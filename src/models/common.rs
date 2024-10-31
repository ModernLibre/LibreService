use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BookPageReq {
    pub page: u32,
    pub size: u32,
}

#[derive(Serialize)]
pub struct BaseResponse<T> {
    pub data: T,
    pub message: String,
}