use crate::casdoor::create_casdoor_client;
use crate::error::ServiceError;
use actix_web::{dev::ServiceRequest, Error};
use actix_web::{get, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use casdoor_rust_sdk::AuthService;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tokio::task;
#[get("/login")]
pub async fn login() -> Result<HttpResponse, ServiceError> {
    let conf = create_casdoor_client();
    let auth_service = AuthService::new(&conf);
    let redirect_url = auth_service.get_signin_url("http://localhost:8082/callback".to_string());
    Ok(HttpResponse::Ok().json(redirect_url))
}

#[get("/signup")]
pub async fn signup() -> Result<HttpResponse, ServiceError> {
    let conf = create_casdoor_client();
    let auth_service = AuthService::new(&conf);
    let redirect_url = auth_service.get_signup_url_enable_password();
    Ok(HttpResponse::Ok().json(redirect_url))
}

#[get("/auth/{code}")]
pub async fn callback(code: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let code = code.into_inner();

    let authed_user = task::spawn_blocking(move || {
        // 创建 conf 和 auth_src 实例
        let conf = create_casdoor_client();
        let auth_src = AuthService::new(&conf);

        // 获取认证 token 并解析用户信息
        let token = auth_src.get_auth_token(code).map_err(|e| {
            log::error!("Failed to get auth token: {:?}", e);
            ServiceError::from(e)
        })?;

        auth_src.parse_jwt_token(token).map_err(|e| {
            log::error!("Failed to parse JWT token: {:?}", e);
            ServiceError::from(e)
        })
    })
    .await
    .expect("Failed to execute blocking task")?;

    log::debug!("Authed User: {:?}", authed_user);

    Ok(HttpResponse::Ok().json(authed_user))
}

#[get("/logout")]
pub async fn logout() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub(crate) async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    let token = credentials.token();
    let decoding_key = DecodingKey::from_secret("secret".as_ref());

    match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)) {
        Ok(_) => Ok(req),
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
