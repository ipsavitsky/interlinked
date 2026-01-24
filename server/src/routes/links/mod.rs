use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

pub mod new_link;
pub mod redirect;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(new_link::handler))
        .route("/{id}", get(redirect::handler))
}
