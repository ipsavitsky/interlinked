use axum::{
    Json, Router, debug_handler,
    extract::{Path, State},
    routing::{get, post},
};
use shared::NewRecord;
use sha256::digest;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct AppState {
    cache: Arc<Mutex<HashMap<String, String>>>,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = Router::new()
        .route("/{id}", get(handler))
        .route("/", post(new_link_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn handler(Path(id): Path<String>, State(state): State<AppState>) -> String {
    match state.cache.lock().unwrap().get(&id) {
        Some(p) => p.to_string(),
        None => format!("record with id {id} not found"),
    }
}

#[debug_handler]
async fn new_link_handler(State(state): State<AppState>, body: Json<NewRecord>) -> String {
    let hash = digest(body.challange.clone());
    if !hash.starts_with("000") {
        "Hash does not compute!".to_string()
    } else {
        state
            .cache
            .lock()
            .unwrap()
            .insert("1".to_string(), body.payload.clone());
        "1".to_string()
    }
}
