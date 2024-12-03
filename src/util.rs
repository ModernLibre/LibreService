// get a absolute path from a relative path
pub fn abs_path(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let absolute_path = std::env::current_dir()?.join(path);
    Ok(absolute_path.to_str().unwrap().to_string())
}

// wrap a blocking function in a actix-web thread
pub async fn run_blocking<F, T>(f: F) -> Result<T, crate::error::ServiceError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    actix_web::web::block(move || f())
        .await
        .map_err(|_| crate::error::ServiceError::InternalServerError)
}
