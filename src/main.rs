use actix_web::{web::Data, App, HttpServer};
use diesel::{r2d2, PgConnection};
use dotenv::dotenv;
use libre_service::routes::init_routes;
use std::env;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if dotenv().is_err() {
        println!("Failed to read .env file");
    } else {
        println!(".env file loaded successfully");
    }
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::debug!("Starting server");
    // create db connection pool
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    // 打印所有环境变量
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone())) // 将连接池传递给App
            .configure(init_routes)
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
