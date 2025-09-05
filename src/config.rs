use std::env;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub http_addr: String,
    pub grpc_addr: String,
    pub server_ip: String,
    pub server_port: u16,
    pub server_name: String,
    pub server_location: String,
    pub rate_burst: f64,
    pub rate_per_sec: f64,
    pub use_mock: bool,
    #[cfg(feature = "mysql")]
    pub mysql_url: Option<String>,
    #[cfg(feature = "redis")]
    pub redis_url: Option<String>,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let http_addr = env::var("LOGIN_HTTP_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
        let grpc_addr = env::var("LOGIN_GRPC_ADDR").unwrap_or_else(|_| "0.0.0.0:50051".into());
        let server_ip = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".into());
        let server_port = env::var("SERVER_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(7171);
        let server_name = env::var("SERVER_NAME").unwrap_or_else(|_| "Rust World".into());
        let server_location = env::var("SERVER_LOCATION").unwrap_or_else(|_| "BR".into());

        let rate_burst = env::var("RATE_LIMITER_BURST").ok().and_then(|s| s.parse().ok()).unwrap_or(50.0);
        let rate_per_sec = env::var("RATE_LIMITER_RATE").ok().and_then(|s| s.parse().ok()).unwrap_or(25.0);

        let use_mock = env::var("USE_MOCK").ok().map(|v| v == "1" || v.to_lowercase() == "true").unwrap_or(true);

        #[cfg(feature = "mysql")]
        let mysql_url = env::var("MYSQL_URL").ok();

        #[cfg(feature = "redis")]
        let redis_url = env::var("REDIS_URL").ok();

        Self {
            http_addr,
            grpc_addr,
            server_ip,
            server_port,
            server_name,
            server_location,
            rate_burst,
            rate_per_sec,
            use_mock,
            #[cfg(feature = "mysql")]
            mysql_url,
            #[cfg(feature = "redis")]
            redis_url,
        }
    }
}
