use actix_web::{web::Data, App, HttpServer};
use casdoor_rust_sdk::AuthService;
use diesel::{r2d2, PgConnection};
use dotenv::dotenv;
use libre_service::{casdoor::create_casdoor_client, error::ServiceError, routes::init_routes};
use tokio::task;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if dotenv().is_err() {
        println!("Failed to read .env file");
    } else {
        println!(".env file loaded successfully");
    }
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::debug!("Starting server");

    // 创建数据库连接池
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let authed_user = task::spawn_blocking(|| {
        // 创建 conf 和 auth_src 实例
        let conf = create_casdoor_client();
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

    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::permissive()) // TODO: 使用环境变量配置DisableCors
            .app_data(Data::new(pool.clone())) // 将连接池传递给App
            .configure(init_routes)
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
