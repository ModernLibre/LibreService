use crate::controller::book_controller::{
    get_book_details, list_books, recent_books, top_rated_books,
};
use crate::controller::epub::epub_controller::epub_upload;
use actix_web::web;
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/libre/v1/books")
            .route("/list", web::post().to(list_books))
            .route("/details/{book_id}", web::get().to(get_book_details))
            .route("/recent", web::get().to(recent_books))
            .route("/top-rated", web::get().to(top_rated_books)),
    );

    cfg.service(
        web::scope("/api/libre/v1/epub")
        .route("/epub_upload", web::post().to(epub_upload))
    );
}
