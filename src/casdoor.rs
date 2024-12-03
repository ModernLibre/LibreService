use casdoor_rust_sdk::CasdoorConfig;
use actix_web::{dev::ServiceRequest, Error};

use actix_web_httpauth::extractors::bearer::BearerAuth;

pub fn create_casdoor_client() -> CasdoorConfig {
    let config_path = "casdoorConf.toml"; // 请根据实际路径修改

    CasdoorConfig::from_toml(config_path)
        .expect("Failed to load Casdoor configuration from conf.toml")
}

lazy_static! {
    static ref config: CasdoorConfig = create_casdoor_client();
}

pub fn casdoor_auth() -> casdoor_rust_sdk::AuthService<'static> {
    casdoor_rust_sdk::AuthService::new(&config)
}

pub(crate) async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let _jwt = casdoor_auth()
        .get_auth_token(credentials.token().to_string())
        .map_err(|_| Error::from(crate::error::ServiceError::Unauthorized))?;

    Ok(req)
}
