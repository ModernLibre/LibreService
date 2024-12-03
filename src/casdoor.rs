use std::env;

use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use casdoor_rust_sdk::{AuthService, CasdoorConfig};
use tokio::task;

use crate::error::ServiceError;

/// 本地环境直接读取 casdoor 配置文件
pub fn create_casdoor_client() -> CasdoorConfig {
    let config_path = "casdoorConf.toml";

    CasdoorConfig::from_toml(config_path)
        .expect("Failed to load Casdoor configuration from conf.toml")
}

/// 集群环境从环境变量中读取配置
pub fn create_casdoor_client_from_env() -> Result<CasdoorConfig, env::VarError> {
    let endpoint = env::var("CASDOOR_ENDPOINT")?;
    let client_id = env::var("CASDOOR_CLIENT_ID")?;
    let client_secret = env::var("CASDOOR_CLIENT_SECRET")?;
    let certificate = env::var("CASDOOR_CERTIFICATE")?;
    let org_name = env::var("CASDOOR_ORG_NAME")?;
    let app_name = env::var("CASDOOR_APP_NAME").ok();

    Ok(CasdoorConfig::new(endpoint, client_id, client_secret, certificate, org_name, app_name))
}

lazy_static! {
    static ref config: CasdoorConfig = create_casdoor_client_from_env().unwrap();
    static ref auth: casdoor_rust_sdk::AuthService<'static> =
    casdoor_rust_sdk::AuthService::new(&config);
}

pub fn casdoor_auth() -> casdoor_rust_sdk::AuthService<'static> {
    casdoor_rust_sdk::AuthService::new(&config)
}

pub(crate) async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let jwt = auth
        .get_auth_token(credentials.token().to_string())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid jwt"))?;
    let user = casdoor_auth()
        .parse_jwt_token(jwt)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid jwt"))?;

    req.extensions_mut().insert(user);

    Ok(req)
}

pub(crate) async fn parse_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let user = auth
        .parse_jwt_token(credentials.token().to_string())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid jwt"))?;

    req.extensions_mut().insert(user);

    Ok(req)
}

pub async fn load_casdoor() {
    let authed_user = task::spawn_blocking(|| {
        // 创建 conf 和 auth_src 实例

        // 检查环境变量中的 KUBERNETES_SERVICE 标志位
        // 如果是集群环境，从环境变量中读取配置
        let conf;
        let is_kubernetes = env::var("KUBERNETES_SERVICE").unwrap_or_else(|_| "false".to_string()) == "true";
        if is_kubernetes {
            conf = create_casdoor_client_from_env().unwrap();
        }else{
            // 本地环境直接读取 casdoor 配置文件
            conf = create_casdoor_client();
        }
        
        let auth_src = AuthService::new(&conf);

        // 获取认证 token 并解析用户信息
        let token = auth_src
            .get_auth_token("any_code".to_owned())
            .map_err(ServiceError::from)?;
        auth_src.parse_jwt_token(token).map_err(ServiceError::from)
    })
    .await
    .expect("Failed to execute blocking task"); // 处理错误

    log::debug!("Authed User: {:?}", authed_user);
}