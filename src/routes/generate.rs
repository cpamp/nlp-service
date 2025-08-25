use std::{convert::Infallible, sync::{Arc, Mutex}};

use axum::{
    extract::State, response::IntoResponse, routing::{post, MethodRouter}, Json, Router
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GenerateRequest {
    prompt: String,
    max_tokens: Option<usize>,
}

#[derive(Serialize)]
struct GenerateResponse {
    output: String,
}

#[axum::debug_handler]
pub async fn handler(State(state): State<Arc<crate::AppState>>, Json(req): Json<GenerateRequest>) -> impl IntoResponse {
    let state = state.clone();
    let result: String = state.llm.lock().unwrap()
        .predict(req.prompt.into(), Default::default())
        .unwrap_or_else(|_| "Error generating text".to_string());

    Json(GenerateResponse { output: result })
}