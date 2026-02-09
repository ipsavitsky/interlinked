use crate::routes::common::{Recordable, create_record};
use axum::{Json, extract::State, response::IntoResponse};
use object_store::{ObjectStoreExt, PutPayload, path::Path};
use shared::new_object_schemes::NewNoteScheme;
use uuid::Uuid;

impl Recordable for NewNoteScheme {
    async fn get_payload(&self, state: &crate::AppState) -> String {
        let path = Uuid::new_v4().to_string();
        state
            .bucket
            .put(
                &Path::from(path.clone()),
                PutPayload::from(self.payload.clone()),
            )
            .await
            .unwrap();
        path
    }
}

pub async fn handler(
    state: State<crate::AppState>,
    body: Json<NewNoteScheme>,
) -> impl IntoResponse {
    create_record(state, body).await
}
