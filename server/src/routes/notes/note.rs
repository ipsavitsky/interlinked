use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{AppState, models::Record};

use diesel::prelude::*;

pub async fn handler(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
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
            if !rec.record_type.eq("note") {
                return axum::response::Response::builder()
                    .status(404)
                    .body(axum::body::Body::from(format!(
                        "record with id {id} is a link and not a note"
                    )))
                    .unwrap();
            }
            axum::response::Response::builder()
                .status(200)
                .body(axum::body::Body::from(rec.payload))
                .unwrap()
        }
        None => axum::response::Response::builder()
            .status(404)
            .body(axum::body::Body::from(format!(
                "record with id {id} not found"
            )))
            .unwrap(),
    }
}
