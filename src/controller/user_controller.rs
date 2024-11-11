use crate::casdoor::create_casdoor_client;
use crate::error::ServiceError;
use actix_web::{get, post, web, HttpResponse};
use casdoor_rust_sdk::{CasdoorUser, UserService};

#[get("/count/{is_online}")]
pub async fn user_count(is_online: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let conf = create_casdoor_client();
    let user_service = UserService::new(&conf);
    let count = user_service
        .get_user_count(is_online.into_inner())
        .await
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(HttpResponse::Ok().json(count))
}

#[get("/{name}")]
pub async fn get_user(name: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let conf = create_casdoor_client();
    let user_service = UserService::new(&conf);
    let user = user_service
        .get_user(name.into_inner())
        .await
        .map_err(|_| ServiceError::BadRequest("User not found".to_string()))?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/list")]
pub async fn get_user_list() -> Result<HttpResponse, ServiceError> {
    let conf = create_casdoor_client();
    let user_service = UserService::new(&conf);
    let users = user_service
        .get_users()
        .await
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/delete")]
pub async fn delete_user(user: web::Json<CasdoorUser>) -> Result<HttpResponse, ServiceError> {
    let conf = create_casdoor_client();
    let user_service = UserService::new(&conf);
    let code = user_service
        .delete_user(user.0)
        .await
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(HttpResponse::Ok().json(code.as_u16()))
}

#[post("/add")]
pub async fn add_user(user: web::Json<CasdoorUser>) -> Result<HttpResponse, ServiceError> {
    let conf = create_casdoor_client();
    let user_service = UserService::new(&conf);
    let code = user_service
        .add_user(user.0)
        .await
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(HttpResponse::Ok().json(code.as_u16()))
}
