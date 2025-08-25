use std::{sync::{Arc}};

use axum::{
    extract::State, response::IntoResponse, Json
};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

#[derive(Deserialize)]
pub struct GenerateRequest {
    prompt: String,
}

#[derive(Serialize)]
pub struct GenerateResponse {
    output: String,
}

#[axum::debug_handler]
pub async fn handler(State(state): State<Arc<crate::app_state::AppState>>, Json(req): Json<GenerateRequest>) -> impl IntoResponse {
    let (resp_tx, resp_rx) = oneshot::channel();

    state.llm.send((req.prompt, resp_tx)).await.unwrap();

    let results = resp_rx.await.unwrap();
    
    Json(GenerateResponse { output: results })
}