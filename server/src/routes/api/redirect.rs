use axum::{
    extract::{Path, State},
    http::{HeaderMap, header},
    response::IntoResponse,
};

use crate::{AppState, models::Record};

use diesel::prelude::*;

pub async fn handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    use crate::schema::records::dsl::{id as table_id, records};
    let id_num = id.parse::<i32>().unwrap();
    let selected_record = records
        .filter(table_id.eq(id_num))
        .select(Record::as_select())
        .first(&mut *state.db.lock().unwrap())
        .optional()
        .expect("failed to load record");

    match selected_record {
        Some(rec) => {
            if let Some(accept_header) = headers.get(header::ACCEPT)
                && let Ok(accept_str) = accept_header.to_str()
                && accept_str.contains("text/html")
            {
                return axum::response::Response::builder()
                    .status(302)
                    .header(header::LOCATION, rec.redirect_url)
                    .body(axum::body::Body::empty())
                    .unwrap();
            }
            axum::response::Response::new(axum::body::Body::from(rec.redirect_url))
        }
        None => axum::response::Response::builder()
            .status(404)
            .body(axum::body::Body::from(format!(
                "record with id {id} not found"
            )))
            .unwrap(),
    }
}
