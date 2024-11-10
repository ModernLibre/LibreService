use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse};
use diesel::{r2d2::{self, ConnectionManager}, MysqlConnection, PgConnection, RunQueryDsl};

use crate::{error::ServiceError, models::epub::{epub::{epub_parse, Epub}, recourses::{Resource, Resources}}, schema};
// type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
}

pub async fn epub_upload(db_pool: web::Data<DbPool>, payload: MultipartForm<UploadForm>) -> Result<HttpResponse, ServiceError> {
    //TODO: 调用FileService upload接口上传epub文件至s3, 并获得url
    let mut epub_object = Epub::new(payload);
    //log::debug!("get epub file: {:?}", epub_object.get_title());
    //使用epub对象初始化章节内容表
    let chapters = epub_parse(&mut epub_object);
    log::debug!("chapter parse success!, len:{}", chapters.len());
    //使用epub对象初始化资源表
    let recources = Resources::init(epub_object.data.clone()).get_resources();
    log::debug!("recources parse success!, len:{}", recources.len());

    // 尝试建立数据库连接
    let mut conn = db_pool.get().map_err(|e| {
        log::error!("获取数据库连接失败:{}", e);
        ServiceError::InternalServerError
    })?;

    // 向数据库插入章节信息表
    for chapter in chapters {
        // log::debug!("chapter name:{}", chapter.title);
        diesel::insert_into(schema::chapter::table)
            .values(&chapter)
            .execute(&mut conn)
            .map_err(|e| {
                log::error!("插入章节内容失败:{}", e);
                ServiceError::InternalServerError
            })?;
    }

    //向数据库插入资源内容表
    for recource in recources {
        diesel::insert_into(schema::recources::table)
            .values(&recource)
            .execute(&mut conn)
            .map_err(|e| {
                log::error!("插入资源失败:{}", e);
                ServiceError::InternalServerError
            })?;
    }

    return Ok(HttpResponse::Ok().json("Epub Uploaded"));
}
