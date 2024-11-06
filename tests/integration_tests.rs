use actix_web::{test, App};
use libre_service::routes::init_routes;

#[actix_rt::test]
async fn test_list_books() {
    let mut app = test::init_service(App::new().configure(init_routes)).await;
    let req = test::TestRequest::post()
        .uri("/books/list")
        .set_json(&serde_json::json!({
            "page": 1,
            "size": 10
        }))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_book_details() {
    let mut app = test::init_service(App::new().configure(init_routes)).await;
    let req = test::TestRequest::get()
        .uri("/books/details/1")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_recent_books() {
    let mut app = test::init_service(App::new().configure(init_routes)).await;
    let req = test::TestRequest::get()
        .uri("/books/recent?limit=5")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_top_rated_books() {
    let mut app = test::init_service(App::new().configure(init_routes)).await;
    let req = test::TestRequest::get()
        .uri("/books/top-rated?limit=5")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
