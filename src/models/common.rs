use serde::{Serialize, Deserialize};
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BaseResponse<T> {
    pub data: T,
    pub message: String,
    pub status: u16,
}
impl<T> BaseResponse<T>{
    pub fn new(data: T, message: String, status: u16) -> Self {
        BaseResponse {
            data,
            message,
            status,
        }
    }
    pub fn status(self)->u16{
        self.status
    }
    pub fn is_success(&self) -> bool {
        // 检查状态码是否在 200-299 范围内
        self.status >= 200 && self.status < 300
    }
}