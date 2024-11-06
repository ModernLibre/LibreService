use serde::Serialize;
#[derive(Serialize)]
pub struct BaseResponse<T> {
    pub data: T,
    pub message: String,
    pub status: u16,
}