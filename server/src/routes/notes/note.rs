use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use object_store::ObjectStoreExt;

use crate::routes::common::{RecordHandler, process_record_request};
use crate::{AppState, models::Record};

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

    async fn handle_record(
        rec: &Record,
        _id: &str,
        state: &AppState,
        _headers: Option<&axum::http::HeaderMap>,
    ) -> Response {
        let data = state
            .bucket
            .get(&object_store::path::Path::from(rec.payload.clone()))
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        Response::builder()
            .status(200)
            .body(axum::body::Body::from(data))
            .unwrap()
    }
}

pub async fn handler(
    id: axum::extract::Path<String>,
    state: State<crate::AppState>,
) -> impl IntoResponse {
    process_record_request::<NoteHandler>(id, state, None).await
}
