use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::Serialize;
use std::error::Error;
use uuid::Error as ParseError;

#[derive(Debug, Display, Serialize)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {_0}")]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// 实现 `ResponseError` 以便将错误转换为 HTTP 响应
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

// 为 `ServiceError` 实现 `Send + Sync` 特性
impl Error for ServiceError {}
unsafe impl Send for ServiceError {}
unsafe impl Sync for ServiceError {}

// 将 ParseError 转换为 ServiceError
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

// 添加从 Box<dyn std::error::Error> 到 ServiceError 的转换
impl From<Box<dyn std::error::Error>> for ServiceError {
    fn from(_: Box<dyn std::error::Error>) -> ServiceError {
        ServiceError::InternalServerError
    }
}
