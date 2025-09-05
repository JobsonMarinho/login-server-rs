use axum::{Router, routing::{post, get}, Json, extract::State, response::IntoResponse};
use axum::http::StatusCode;
use tracing::error;
use crate::state::SharedState;
use crate::models::{LoginHttpRequest, LoginHttpResponse, World};
use crate::rate_limit::{RateLimitLayer, ip_rate_limit};
use axum::middleware;

pub fn router(state: SharedState, burst: f64, rate_per_sec: f64) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route("/login", post(login))
        .with_state(state)
        .layer(middleware::from_fn(ip_rate_limit))
        .layer(RateLimitLayer::new(burst, rate_per_sec))
}

async fn health() -> &'static str { "ok" }

async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginHttpRequest>,
) -> impl IntoResponse {
    match state.login(&payload.account, &payload.password).await {
        Ok((chars, world)) => {
            let resp = LoginHttpResponse { ok: true, message: "".into(), characters: chars, world };
            (StatusCode::OK, Json(resp))
        }
        Err(e) => {
            error!("login error: {:?}", e);
            let world = World { name: state.cfg.server_name.clone(), ip: state.cfg.server_ip.clone(), port: state.cfg.server_port, location: state.cfg.server_location.clone() };
            let resp = LoginHttpResponse { ok: false, message: e.to_string(), characters: vec![], world };
            (StatusCode::UNAUTHORIZED, Json(resp))
        }
    }
}
