use llama_cpp_rs::{options::ModelOptions, LLama};
use tokenizers::{Encoding, Tokenizer};
use tokio::sync::{mpsc, oneshot};

pub struct AppState {
    pub llm: mpsc::Sender<(String, oneshot::Sender<String>)>,
    pub tokenizer: mpsc::Sender<(String, oneshot::Sender<Encoding>)>,
}

pub fn get_app_state() -> AppState {
    let (llm_tx, mut llm_rx) = mpsc::channel::<(String, oneshot::Sender<String>)>(32);

    std::thread::spawn(move || {
        let llm_opts = ModelOptions::default();
        let llm = LLama::new("models/gemma-3-270m-it/model.gguf".into(), &llm_opts)
            .expect("Failed to load model");

        // Worker loop
        while let Some((prompt, resp_tx)) = llm_rx.blocking_recv() {
            let result = llm.predict(prompt, Default::default())
                            .unwrap_or_else(|_| "Error".to_string());
            let _ = resp_tx.send(result);
        }
    });

    let (tok_tx, mut tok_rx) = mpsc::channel::<(String, oneshot::Sender<Encoding>)>(32);
    std::thread::spawn(move || {
        let tokenizer = Tokenizer::from_file("models/gemma-3-270m-it/tokenizer.json")
            .expect("Failed to load tokenizer");

        while let Some((text, resp_tx)) = tok_rx.blocking_recv() {
            let result = tokenizer.encode(text, false).unwrap();
            let _ = resp_tx.send(result);
        }
    });

    AppState { llm: llm_tx, tokenizer: tok_tx }
}