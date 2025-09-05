use std::net::SocketAddr;
use std::time::Instant;

use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tower::{Layer, Service};
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct RateLimiter {
    pub burst: f64,
    pub rate_per_sec: f64,
}

struct Bucket {
    tokens: f64,
    last: Instant,
}

static BUCKETS: Lazy<DashMap<SocketAddr, Bucket>> = Lazy::new(DashMap::new);

pub async fn ip_rate_limit(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let (mut burst, mut rate_per_sec) = (50.0, 25.0);

    if let Some(cfg) = req.extensions().get::<RateLimiter>() {
        burst = cfg.burst;
        rate_per_sec = cfg.rate_per_sec;
    }

    let now = Instant::now();
    let mut allow = false;

    {
        let mut entry = BUCKETS
            .entry(addr)
            .or_insert(Bucket { tokens: burst, last: now });

        let elapsed = now.duration_since(entry.last).as_secs_f64();
        entry.tokens = (entry.tokens + elapsed * rate_per_sec).min(burst);
        entry.last = now;

        if entry.tokens >= 1.0 {
            entry.tokens -= 1.0;
            allow = true;
        }
    }

    if allow {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}

#[derive(Clone)]
pub struct RateLimitLayer {
    cfg: RateLimiter,
}

impl RateLimitLayer {
    pub fn new(burst: f64, rate_per_sec: f64) -> Self {
        Self {
            cfg: RateLimiter { burst, rate_per_sec },
        }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    cfg: RateLimiter,
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            cfg: self.cfg.clone(),
        }
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for RateLimitService<S>
where
    S: Service<Request<ReqBody>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        req.extensions_mut().insert(self.cfg.clone());
        self.inner.call(req)
    }
}
