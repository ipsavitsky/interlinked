use crate::{
    AppState,
    routes::common::{Recordable, create_record},
};
use axum::{Json, extract::State, response::IntoResponse};
use shared::new_object_schemes::NewLinkScheme;

impl Recordable for NewLinkScheme {
    async fn get_payload(&self, _state: &AppState) -> String {
        self.payload.to_string()
    }
}

pub async fn handler(
    state: State<crate::AppState>,
    body: Json<NewLinkScheme>,
) -> impl IntoResponse {
    create_record(state, body).await
}
