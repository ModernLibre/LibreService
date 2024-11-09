use std::io::Read;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::HttpResponse;
use epub::doc::EpubDoc;

use crate::models::epub::epub::Epub;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
}

pub async fn epub_upload(mut payload: MultipartForm<UploadForm>) -> HttpResponse {
    //TODO: 调用FileService upload接口上传epub文件至s3, 并获得url
    let epub_object = Epub::new(payload);
    //持久化储存章节和资源

    return HttpResponse::Ok().json("Epub Uploaded");

}

// epub 文件解析
pub async fn epub_parse() {

}

pub async fn save_cover() {

}

pub async fn save_book_info() {

}

pub async fn save_recourse() {

}
