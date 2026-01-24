use axum::{Router, routing::get};

use crate::AppState;

pub mod difficulty;

pub fn router() -> Router<AppState> {
    Router::new().route("/difficulty", get(difficulty::handler))
}
