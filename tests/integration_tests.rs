#[cfg(test)]
mod tests {
    use actix_web::{
        http, test,
        web::Data,
        App,
    };
    use diesel::{
        r2d2::{self, ConnectionManager, Pool},
        PgConnection,
    };
    use libre_service::routes::init_routes;
    use std::sync::Once;

    static INIT: Once = Once::new();

    async fn setup_pg() -> Pool<ConnectionManager<PgConnection>> {
        INIT.call_once(|| {
            if dotenv::dotenv().is_err() {
                println!("Failed to read .env file");
            } else {
                println!(".env file loaded successfully");
            }
            std::env::set_var("RUST_LOG", "debug");
            env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
        });

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    #[actix_web::test]
    async fn test_list_books() {
        let mut app = test::init_service(
            App::new()
                .configure(init_routes)
                .app_data(Data::new(setup_pg().await)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/libre/v1/books/list")
            .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        // // 响应状态码
        // println!("Response Status: {:?}", resp.status());
        // // 响应体
        // let body = test::read_body(resp).await;
        // println!("Response Body: {:?}", body);
    }

    #[actix_web::test]
    async fn test_get_book_details() {
        let mut app = test::init_service(
            App::new()
                .configure(init_routes)
                .app_data(Data::new(setup_pg().await)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/libre/v1/books/details/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_recent_books() {
        let mut app = test::init_service(
            App::new()
                .configure(init_routes)
                .app_data(Data::new(setup_pg().await)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/api/libre/v1/books/recent?limit=5")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_top_rated_books() {
        let mut app = test::init_service(
            App::new()
                .configure(init_routes)
                .app_data(Data::new(setup_pg().await)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/api/libre/v1/books/top-rated?limit=5")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
