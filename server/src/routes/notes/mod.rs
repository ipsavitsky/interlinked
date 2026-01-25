use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

pub mod new_note;
pub mod note;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(new_note::handler))
        .route("/{id}", get(note::handler))
}
