use casdoor_rust_sdk::CasdoorConfig;
use actix_web::{dev::ServiceRequest, Error, HttpMessage};

use actix_web_httpauth::extractors::bearer::BearerAuth;

pub fn create_casdoor_client() -> CasdoorConfig {
    let config_path = "casdoorConf.toml"; // 请根据实际路径修改

    CasdoorConfig::from_toml(config_path)
        .expect("Failed to load Casdoor configuration from conf.toml")
}

lazy_static! {
    static ref config: CasdoorConfig = create_casdoor_client();
    static ref auth: casdoor_rust_sdk::AuthService<'static> = casdoor_rust_sdk::AuthService::new(&config);
}

pub fn casdoor_auth() -> casdoor_rust_sdk::AuthService<'static> {
    casdoor_rust_sdk::AuthService::new(&config)
}

pub(crate) async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let jwt = auth.get_auth_token(credentials.token().to_string())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid jwt"))?;
    let user = casdoor_auth().parse_jwt_token(jwt)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid jwt"))?;

    req.extensions_mut().insert(user);

    Ok(req)
}

pub(crate) async fn parse_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let user = auth.parse_jwt_token(credentials.token().to_string())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid jwt"))?;

    req.extensions_mut().insert(user);

    Ok(req)
}
