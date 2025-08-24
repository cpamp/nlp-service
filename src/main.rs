use axum::body::Body;
use axum::routing::post;
use axum::{
    Router,
    http::StatusCode,
    middleware::Next,
    response::Response,
    http::Request,
};
use llama_cpp_rs::options::ModelOptions;
use std::sync::Arc;
use llama_cpp_rs::LLama;
use tokenizers::Tokenizer;

mod routes;
mod config;

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

pub struct AppState {
    llm: LLama,
    tokenizer: Tokenizer,
}

#[tokio::main]
async fn main() {
    let llm_opts = ModelOptions::default();
    let llm = LLama::new("models/gemma-3-270m-it/model.gguf".into(), &llm_opts)
        .expect("Failed to load model");
    let tokenizer = Tokenizer::from_file("models/gemma-3-270m-it/tokenizer.json")
        .expect("Failed to load tokenizer");

    let ctx = Arc::new(AppState {
        llm: llm,
        tokenizer: tokenizer,
    });

    let router = Router::new()
        .route("/generate", post(routes::generate::handler))
        .route("/tokenize", routes::tokenize::get_handler(ctx))
        .with_state(ctx);

    let port = config::get_config().port;
    println!("Running on http://127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
