use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};

use crate::models::Record;
use crate::routes::common::{RecordHandler, fetch_record};

pub struct NoteHandler;

impl RecordHandler for NoteHandler {
    fn record_type() -> &'static str {
        "note"
    }

    fn not_found_message(id: &str) -> String {
        format!("record with id {id} not found")
    }

    fn wrong_type_message(id: &str) -> String {
        format!("record with id {id} is a link and not a note")
    }

    fn handle_record(
        rec: &Record,
        _id: &str,
        _headers: Option<&axum::http::HeaderMap>,
    ) -> Response {
        Response::builder()
            .status(200)
            .body(axum::body::Body::from(rec.payload.clone()))
            .unwrap()
    }
}

pub async fn handler(id: Path<String>, state: State<crate::AppState>) -> impl IntoResponse {
    fetch_record::<NoteHandler>(id, state, None).await
}
