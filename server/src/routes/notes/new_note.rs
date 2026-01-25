use crate::routes::common::create_record;
use axum::{Json, extract::State, response::IntoResponse};
use shared::NewNoteScheme;

pub async fn handler(
    state: State<crate::AppState>,
    body: Json<NewNoteScheme>,
) -> impl IntoResponse {
    create_record(state, body).await
}
