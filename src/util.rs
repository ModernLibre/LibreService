use std::env;
use dotenv::dotenv;

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
    actix_web::web::block(f)
        .await
        .map_err(|_| crate::error::ServiceError::InternalServerError)
}

/// 根据本地/集群环境加载不同的环境变量
pub fn load_env() {
    // 检查环境变量中的 KUBERNETES_SERVICE 标志位
    let is_kubernetes = env::var("KUBERNETES_SERVICE").unwrap_or_else(|_| "false".to_string()) == "true";
    
    // 如果不在 Kubernetes 集群中，则加载 .env 文件，否则默认使用 ConfigMap 和 Secret 注入的环境变量
    if !is_kubernetes {
        if dotenv().is_err() {
            println!("Failed to read .env file");
        } else {
            println!(".env file loaded successfully");
        }
    }
    
    // 设置日志级别
    env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
}