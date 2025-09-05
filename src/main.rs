mod config;
mod models;
mod state;
mod grpc;
mod http;
mod rate_limit;

use crate::config::ServerConfig;
use crate::state::AppState;
use grpc::LoginSvc;
use tracing_subscriber::EnvFilter;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let cfg = ServerConfig::from_env();
    let state = Arc::new(AppState::new(cfg.clone()).await?);

    use tokio::net::TcpListener;

    let http_app = http::router(state.clone(), cfg.rate_burst, cfg.rate_per_sec);
    let http_addr: SocketAddr = cfg.http_addr.parse()?;
    let listener = TcpListener::bind(http_addr).await?;
    let make_svc = http_app.into_make_service_with_connect_info::<SocketAddr>();
    tracing::info!("HTTP listening on http://{}", http_addr);

    let http = async move {
        axum::serve(listener, make_svc)
            .await
            .map_err(anyhow::Error::from)
    };

    let grpc_addr: SocketAddr = cfg.grpc_addr.parse()?;
    let login_svc = LoginSvc::new(state.clone()).into_server();
    let grpc = async move {
        tonic::transport::Server::builder()
            .add_service(login_svc)
            .serve(grpc_addr)
            .await
            .map_err(anyhow::Error::from)
    };

    tokio::try_join!(http, grpc)?;

    Ok(())
}
