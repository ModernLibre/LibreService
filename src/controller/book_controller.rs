use crate::error::ServiceError;
use crate::models::book::Book;
use crate::models::common::BaseResponse;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use serde::Deserialize;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn list_books(db_pool: web::Data<DbPool>) -> Result<HttpResponse, ServiceError> {
    // 尝试从连接池获取数据库连接
    let mut conn = db_pool.get().map_err(|e| {
        eprintln!("获取数据库连接失败：{}", e);
        ServiceError::InternalServerError
    })?;

    // 在阻塞线程中执行数据库查询
    let books = web::block(move || {
        use crate::schema::book::dsl::*;
        book.select(Book::as_select())
            .order(id.desc())
            .limit(10)
            .load::<Book>(&mut conn)
            .map_err(|e| {
                // 记录详细的错误信息
                eprintln!("查询书籍列表时发生错误：{}", e);
                ServiceError::from(e)
            })
    })
    .await
    .map_err(|e| {
        // 处理阻塞操作可能产生的错误
        eprintln!("异步操作执行失败：{}", e);
        ServiceError::InternalServerError
    })?;

    // 记录成功获取的书籍数量
    println!("成功获取了 {} 本书籍。", books.as_ref().unwrap().len());

    // 返回成功的响应
    Ok(HttpResponse::Ok().json(BaseResponse {
        data: books,
        message: "Success".to_string(),
        status: 200,
    }))
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

    Ok(HttpResponse::Ok().json(BaseResponse {
        data: book,
        message: "Success".to_string(),
        status: 200,
    }))
}

#[derive(Deserialize)]
pub struct RecentBooksQuery {
    pub limit: Option<u32>,
}

pub async fn recent_books(
    db_pool: web::Data<DbPool>,
    query: web::Query<RecentBooksQuery>,
) -> Result<HttpResponse, ServiceError> {
    let mut conn = db_pool
        .get()
        .map_err(|_| ServiceError::InternalServerError)?;

    let limit = query.limit.unwrap_or(10);
    let books = web::block(move || {
        use crate::schema::book::dsl::*;
        book.select(Book::as_select())
            .order(added_date.desc())
            .limit(limit as i64)
            .load::<Book>(&mut conn)
            .map_err(ServiceError::from)
    })
    .await
    .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(BaseResponse {
        data: books,
        message: "Success".to_string(),
        status: 200,
    }))
}

#[derive(Deserialize)]
pub struct TopRatedBooksQuery {
    pub limit: Option<u32>,
}

pub async fn top_rated_books(
    db_pool: web::Data<DbPool>,
    query: web::Query<TopRatedBooksQuery>,
) -> Result<HttpResponse, ServiceError> {
    let mut conn = db_pool
        .get()
        .map_err(|_| ServiceError::InternalServerError)?;

    let limit = query.limit.unwrap_or(10);
    let books = web::block(move || {
        use crate::schema::book::dsl::*;
        book.select(Book::as_select())
            .order(rating.desc())
            .limit(limit as i64)
            .load::<Book>(&mut conn)
            .map_err(ServiceError::from)
    })
    .await
    .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(BaseResponse {
        data: books,
        message: "Success".to_string(),
        status: 200,
    }))
}

// 接收File Service 传回的书本信息
pub async fn upload_book_info(
    db_pool: web::Data<DbPool>,
    book_: Json<Book>,
) -> Result<HttpResponse, ServiceError> {
    //log::debug!("get book info: {:?}", book);
    let book = book_.into_inner();
    // 将book信息插入数据库
    let mut conn = db_pool
        .get()
        .map_err(|_| ServiceError::InternalServerError)?;

    let ans = web::block(move || {
        diesel::insert_into(crate::schema::book::table)
            .values(&book)
            .execute(&mut conn)
            .map_err(ServiceError::from)
    })
    .await
    .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(BaseResponse {
        data: ans,
        message: "Success".to_string(),
        status: 200,
    }))
}
