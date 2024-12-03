use super::casdoor::validator;
use crate::controller::book::book_config;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let middleware = actix_web_httpauth::middleware::HttpAuthentication::bearer(validator);

    cfg.service(web::scope("/").wrap(middleware).configure(v1));
}

pub fn v1(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/libre/v1").configure(book_config));
}
