use actix_web::{web::Data, App, HttpServer};
use casdoor_rust_sdk::AuthService;
use diesel::{r2d2, PgConnection};
use libre_service::{
    casdoor::create_casdoor_client_from_env, error::ServiceError, routes::init_routes, util,
};
use tokio::task;
use util::load_env;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_env();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::debug!("Starting server");

    // 创建数据库连接池
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

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
