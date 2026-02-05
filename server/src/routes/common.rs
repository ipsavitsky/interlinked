use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError},
};
use shared::{RecordPayload, proof_of_work::hash_string};

use crate::{
    AppState,
    models::{NewRecord, Record},
};

pub trait Recordable {
    fn get_payload(&self, state: &AppState) -> impl Future<Output = String>;
}

pub async fn create_record<T: RecordPayload + Recordable>(
    State(state): State<AppState>,
    body: Json<T>,
) -> impl IntoResponse {
    use crate::schema::records;
    let hash = hash_string(body.challenge());
    let hash_prefix = "0".repeat(state.configuration.difficulty as usize);
    if !hash.starts_with(&hash_prefix) {
        (
            StatusCode::BAD_REQUEST,
            "Hash does not compute!".to_string(),
        )
    } else {
        let values = NewRecord {
            payload: &body.get_payload(&state).await,
            challenge_proof: body.challenge(),
            record_type: body.record_type(),
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

pub trait RecordHandler {
    fn record_type() -> &'static str;
    fn not_found_message(id: &str) -> String;
    fn wrong_type_message(id: &str) -> String;
    fn handle_record(
        rec: &Record,
        id: &str,
        state: &AppState,
        headers: Option<&HeaderMap>,
    ) -> impl Future<Output = Response>;
}

pub async fn process_record_request<T: RecordHandler>(
    Path(id): Path<String>,
    State(state): State<AppState>,
    headers: Option<HeaderMap>,
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
            if !rec.record_type.eq(T::record_type()) {
                return Response::builder()
                    .status(404)
                    .body(axum::body::Body::from(T::wrong_type_message(&id)))
                    .unwrap();
            }
            T::handle_record(&rec, &id, &state, headers.as_ref()).await
        }
        None => Response::builder()
            .status(404)
            .body(axum::body::Body::from(T::not_found_message(&id)))
            .unwrap(),
    }
}
