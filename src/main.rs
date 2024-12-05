use actix_web::{web::Data, App, HttpServer};
use diesel::{r2d2, PgConnection};
use libre_service::{
    casdoor::load_casdoor, routes::init_routes, util,
};
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
    load_casdoor().await;
    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::permissive()) // TODO: 使用环境变量配置DisableCors
            .app_data(Data::new(pool.clone())) // 将连接池传递给App
            .configure(init_routes)
    })
    .bind("0.0.0.0:8083")?
    .run()
    .await
}
