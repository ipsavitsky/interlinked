use axum::{
    Json, Router, debug_handler,
    extract::{Path, State},
    routing::{get, post},
};
use shared::{NewRecord, get_hash};
use std::{
    collections::HashMap, sync::{Arc, Mutex}
};

#[derive(Clone)]
struct AppState {
    cache: Arc<Mutex<HashMap<String, String>>>,
    current_difficulty: usize,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            cache: Arc::new(Mutex::new(HashMap::new())),
            current_difficulty: 5,
        }
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = Router::new()
        .route("/difficulty", get(difficulty_handler))
        .route("/{id}", get(handler))
        .route("/", post(new_link_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn difficulty_handler(State(state): State<AppState>) -> String {
    state.current_difficulty.to_string()
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
    let hash = get_hash(&body.challange);
    let hash_prefix = "0".repeat(state.current_difficulty);
    if !hash.starts_with(&hash_prefix) {
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
