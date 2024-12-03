use crate::error::ServiceError;
use crate::schema::Book;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod list;

#[inline]
pub fn book_config(cfg: &mut web::ServiceConfig) {
    let middleware =
        actix_web_httpauth::middleware::HttpAuthentication::bearer(crate::casdoor::parse_jwt);

    cfg.service(
        web::scope("/books")
            .route("/{book_id}", web::get().to(get_book_details))
            .route("/list", web::get().to(list::list))
            .service(
                web::resource("/upload")
                    .wrap(middleware)
                    .route(web::post().to(upload_book_info)),
            ),
    );
}

pub async fn get_book_details(
    db_pool: web::Data<DbPool>,
    book_id: web::Path<u32>,
) -> Result<HttpResponse, ServiceError> {
    let mut conn = db_pool
        .get()
        .map_err(|_| ServiceError::InternalServerError)?;

    let book = web::block(move || {
        use crate::schema::book::dsl::*;
        book.select(Book::as_select())
            .find(*book_id as i32)
            .get_result::<Book>(&mut conn)
            .map_err(ServiceError::from)
    })
    .await
    .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(book))
}

// 接收File Service 传回的书本信息
pub async fn upload_book_info(
    db_pool: web::Data<DbPool>,
    book_: web::Json<Book>,
) -> Result<HttpResponse, ServiceError> {
    //log::debug!("get book info: {:?}", book);
    let book = book_.into_inner();
    // 将book信息插入数据库
    let mut conn = db_pool
        .get()
        .map_err(|_| ServiceError::InternalServerError)?;

    web::block(move || {
        diesel::insert_into(crate::schema::book::table)
            .values(&book)
            .execute(&mut conn)
            .map_err(ServiceError::from)
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}
