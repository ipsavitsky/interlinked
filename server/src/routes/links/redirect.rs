use axum::{
    extract::{Path, State},
    http::{HeaderMap, header},
    response::{IntoResponse, Response},
};

use crate::models::Record;
use crate::routes::common::{RecordHandler, fetch_record};

pub struct LinkHandler;

impl RecordHandler for LinkHandler {
    fn record_type() -> &'static str {
        "link"
    }

    fn not_found_message(id: &str) -> String {
        format!("record with id {id} not found")
    }

    fn wrong_type_message(id: &str) -> String {
        format!("record with id {id} is a note, not a link")
    }

    fn handle_record(rec: &Record, _id: &str, headers: Option<&HeaderMap>) -> Response {
        if let Some(accept_header) = headers.and_then(|h| h.get(header::ACCEPT))
            && let Ok(accept_str) = accept_header.to_str()
            && accept_str.contains("text/html")
        {
            return Response::builder()
                .status(302)
                .header(header::LOCATION, rec.payload.clone())
                .body(axum::body::Body::empty())
                .unwrap();
        }
        Response::new(axum::body::Body::from(rec.payload.clone()))
    }
}

pub async fn handler(
    id: Path<String>,
    state: State<crate::AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    fetch_record::<LinkHandler>(id, state, Some(headers)).await
}
