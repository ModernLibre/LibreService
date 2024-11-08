use std::io::Read;

use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use actix_web::HttpResponse;
use epub::doc::EpubDoc;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct EpubMeta{
    filename: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    file: TempFile,
    json: MpJson<EpubMeta>,
}

pub async fn epub_upload(mut payload: MultipartForm<UploadForm>) -> HttpResponse {
    //TODO: 调用FileService upload接口上传epub文件至s3

    //创建ePub Doc对象
    let mut epub_buffer = Vec::new();
    if let Err(err) = payload.file.file.read_to_end(&mut epub_buffer) {
        return HttpResponse::InternalServerError().json(err.to_string());
    }

    let cursor = std::io::Cursor::new(epub_buffer);
    let epub_doc = EpubDoc::from_reader(cursor).unwrap();

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
