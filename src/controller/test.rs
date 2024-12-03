use actix_web::{web, HttpMessage, HttpResponse};
use casdoor_rust_sdk::CasdoorUser;
use crate::{database, error::ServiceError};

pub (crate) fn service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/test")
            .configure(casdoor_config)
    );
}

pub(crate) fn casdoor_config(cfg: &mut web::ServiceConfig) {
    let middleware = actix_web_httpauth::middleware::HttpAuthentication::bearer(
        crate::casdoor::validator);
    cfg.service(
        web::resource("/user_info")
            .wrap(middleware)
            .route(web::get().to(user_info))
    );
}

async fn user_info (
    req: actix_web::HttpRequest
) -> Result<HttpResponse, ServiceError> {
    let user = req.extensions().get::<CasdoorUser>().ok_or(ServiceError::Unauthorized)?.clone();
    Ok(HttpResponse::Ok().json(user))
}