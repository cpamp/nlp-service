use std::{sync::{Arc}};

use axum::{
    extract::State, response::IntoResponse, Json
};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

#[derive(Deserialize)]
pub struct TokenizeRequest {
    text: String,
}

#[derive(Serialize)]
pub struct TokenizeResponse {
    output: Vec<u32>,
}


#[axum::debug_handler]
pub async fn handler(State(state): State<Arc<crate::app_state::AppState>>, Json(req): Json<TokenizeRequest>) -> impl IntoResponse {
    let (resp_tx, resp_rx) = oneshot::channel();

    state.tokenizer.send((req.text, resp_tx)).await.unwrap();

    let results = resp_rx.await.unwrap();
    
    Json(TokenizeResponse { output: results.get_ids().to_vec() })
}