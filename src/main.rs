use actix_web::{App, HttpServer};
use libre_service::routes::init_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    log::debug!("Starting server");
    HttpServer::new(|| {
        App::new()
            .configure(init_routes)
    })
    .bind(("127.0.0.1", 8083))?
    .run()
    .await
}