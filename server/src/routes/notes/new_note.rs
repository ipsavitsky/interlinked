use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError},
};
use shared::{NewNoteScheme, get_hash};

use crate::{
    AppState,
    models::{NewRecord, Record},
};

pub async fn handler(
    State(state): State<AppState>,
    body: Json<NewNoteScheme>,
) -> impl IntoResponse {
    use crate::schema::records;
    let hash = get_hash(&body.challenge);
    let hash_prefix = "0".repeat(state.current_difficulty);
    if !hash.starts_with(&hash_prefix) {
        (
            StatusCode::BAD_REQUEST,
            "Hash does not compute!".to_string(),
        )
    } else {
        let values = NewRecord {
            payload: body.payload.as_ref(),
            challenge_proof: &body.challenge,
            record_type: "note",
        };
        match diesel::insert_into(records::table)
            .values(values)
            .returning(Record::as_returning())
            .get_result(&mut *state.db.lock().unwrap())
        {
            Ok(r) => (StatusCode::OK, r.id.to_string()),
            Err(DatabaseError(UniqueViolation, _)) => (
                StatusCode::CONFLICT,
                "Proof already used, try again!".to_string(),
            ),
            Err(e) => panic!("{}", e),
        }
    }
}
