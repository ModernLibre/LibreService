use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection, RunQueryDsl,
};

use crate::{
    error::ServiceError,
    models::epub::epub::{epub_parse, Epub},
    schema,
};
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
}

pub async fn epub_upload(
    db_pool: web::Data<DbPool>,
    payload: MultipartForm<UploadForm>,
) -> Result<HttpResponse, ServiceError> {
    //TODO: 调用FileService upload接口上传epub文件至s3, 并获得url
    let mut epub_object = Epub::new(payload);
    //使用epub对象初始化章节内容表
    let chapters = epub_parse(&mut epub_object).await;
    // 尝试建立数据库连接
    let mut conn = db_pool.get().map_err(|e| {
        log::error!("获取数据库连接失败:{}", e);
        ServiceError::InternalServerError
    })?;

    for chapter in chapters {
        diesel::insert_into(schema::chapter::table)
            .values(&chapter)
            .execute(&mut conn)
            .map_err(|e| {
                log::error!("插入章节内容失败:{}", e);
                ServiceError::InternalServerError
            })?;
    }
    return Ok(HttpResponse::Ok().json("Epub Uploaded"));
}
