use actix_web::web;
use crate::controller::book_controller::{list_books, get_book_details, recent_books, top_rated_books};
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/books")
            .route("/list", web::post().to(list_books))
            .route("/details/{book_id}", web::get().to(get_book_details))
            .route("/recent", web::get().to(recent_books))
            .route("/top-rated", web::get().to(top_rated_books))
    );
}