use crate::routes::common::create_record;
use axum::{Json, extract::State, response::IntoResponse};
use shared::NewRecordScheme;

pub async fn handler(
    state: State<crate::AppState>,
    body: Json<NewRecordScheme>,
) -> impl IntoResponse {
    create_record(state, body).await
}
