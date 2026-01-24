use axum::{Router, http::header, routing::get};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/frontend.js",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/javascript")],
                    include_str!("../../pkg/frontend.js"),
                )
            }),
        )
        .route(
            "/frontend_bg.wasm",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "application/wasm")],
                    include_bytes!("../../pkg/frontend_bg.wasm"),
                )
            }),
        )
}
