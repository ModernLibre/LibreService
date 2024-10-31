use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::models::{book::Book, common::{BaseResponse, BookPageReq}};


pub async fn list_books(_req: web::Json<BookPageReq>) -> HttpResponse {
    // 模拟数据
    let books = vec![
        Book { id: 1, title: "Book 1".to_string(), author: "Author 1".to_string(), rating: 4.5 },
        Book { id: 2, title: "Book 2".to_string(), author: "Author 2".to_string(), rating: 4.0 },
    ];

    HttpResponse::Ok().json(BaseResponse {
        data: books,
        message: "Success".to_string(),
    })
}

pub async fn get_book_details(book_id: web::Path<u32>) -> HttpResponse {
    // 模拟数据
    let book = Book { id: *book_id, title: "Book 1".to_string(), author: "Author 1".to_string(), rating: 4.5 };

    HttpResponse::Ok().json(BaseResponse {
        data: book,
        message: "Success".to_string(),
    })
}

#[derive(Deserialize)]
pub struct RecentBooksQuery {
    pub limit: Option<u32>,
}


pub async fn recent_books(query: web::Query<RecentBooksQuery>) -> HttpResponse {
    // 模拟数据
    let limit = query.limit.unwrap_or(10);
    let books: Vec<Book> = (1..=limit).map(|i| Book {
        id: i,
        title: format!("Recent Book {}", i),
        author: format!("Author {}", i),
        rating: 4.0,
    }).collect();

    HttpResponse::Ok().json(BaseResponse {
        data: books,
        message: "Success".to_string(),
    })
}

#[derive(Deserialize)]
pub struct TopRatedBooksQuery {
    pub limit: Option<u32>,
}


pub async fn top_rated_books(query: web::Query<TopRatedBooksQuery>) -> HttpResponse {
    // 模拟数据
    let limit = query.limit.unwrap_or(10);
    let books: Vec<Book> = (1..=limit).map(|i| Book {
        id: i,
        title: format!("Top Rated Book {}", i),
        author: format!("Author {}", i),
        rating: 5.0,
    }).collect();

    HttpResponse::Ok().json(BaseResponse {
        data: books,
        message: "Success".to_string(),
    })
}