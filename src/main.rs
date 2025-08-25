use axum::body::Body;
use axum::middleware;
use axum::routing::post;
use axum::{
    Router,
    http::StatusCode,
    middleware::Next,
    response::Response,
    http::Request,
};
use std::sync::{Arc};

mod routes;
mod config;
mod app_state;
//mod llm;

async fn require_api_key(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let api_key_header = req
        .headers()
        .get("x-api-key")
        .and_then(|h| h.to_str().ok());

    if let Some(key) = api_key_header {
        if key == config::get_config().api_key {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

#[tokio::main]
async fn main() {
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let ctx = Arc::new(app_state::get_app_state());

    let router = Router::new()
        .route("/generate", post(routes::generate::handler))
        .route("/tokenize", post(routes::tokenize::handler))
        .with_state(ctx)
        .layer(middleware::from_fn(require_api_key));

    let port = config::get_config().port;
    println!("Running on http://127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
