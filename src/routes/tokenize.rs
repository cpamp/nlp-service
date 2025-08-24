use std::{any, convert::Infallible, sync::{Arc, Mutex}};

use axum::{
    routing::{post, MethodRouter},
    Json, Router,
};

pub fn get_handler(session: Arc<crate::AppState>) -> MethodRouter<X, Infallible>  {
    return post({
            let session = session.clone();
            move |Json(req): Json<GenerateRequest>| {
                let session = session.clone();
                async move {
                    let mut session = session.lock().await;
                    let max_tokens = req.max_tokens.unwrap_or(128);

                    // Run inference
                    let result: String = session
                        .infer::<String>(
                            req.prompt.as_str(),
                            None,
                            Default::default(),
                            max_tokens,
                        )
                        .unwrap_or_else(|_| "Error generating text".to_string());

                    Json(GenerateResponse { output: result })
                }
            }
        });
}