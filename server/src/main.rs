use axum::{
    Json, Router, debug_handler,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use shared::{NewRecordScheme, get_hash};
use std::env;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

use crate::models::{NewRecord, Record};

pub mod models;
pub mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn establish_connection(db_url: &str) -> SqliteConnection {
    let mut conn = SqliteConnection::establish(db_url).expect("cannot connect to db");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("failed to run migrations");
    conn
}

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<SqliteConnection>>,
    current_difficulty: usize,
}

impl AppState {
    fn new(db_url: &str, difficulty: usize) -> AppState {
        AppState {
            db: Arc::new(Mutex::new(establish_connection(db_url))),
            current_difficulty: difficulty,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap_or("../db/main.db".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let difficulty = env::var("DIFFICULTY")
        .unwrap_or("1".to_string())
        .parse::<usize>()
        .unwrap();
    let state = AppState::new(&db_url, difficulty);
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);
    let app = Router::new()
        .route("/difficulty", get(difficulty_handler))
        .route("/{id}", get(handler))
        .route("/", post(new_link_handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn difficulty_handler(State(state): State<AppState>) -> String {
    state.current_difficulty.to_string()
}

#[debug_handler]
async fn handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl axum::response::IntoResponse {
    use self::schema::records::dsl::{id as table_id, records};
    let id_num = id.parse::<i32>().unwrap();
    let selected_record = records
        .filter(table_id.eq(id_num))
        .select(Record::as_select())
        .first(&mut *state.db.lock().unwrap())
        .optional()
        .expect("failed to load record");

    match selected_record {
        Some(rec) => {
            if let Some(accept_header) = headers.get(header::ACCEPT) {
                if let Ok(accept_str) = accept_header.to_str() {
                    if accept_str.contains("text/html") {
                        let mut redirect_url = rec.redirect_url.clone();
                        if !redirect_url.starts_with("http://")
                            && !redirect_url.starts_with("https://")
                        {
                            redirect_url = format!("http://{}", redirect_url);
                        }
                        return axum::response::Response::builder()
                            .status(302)
                            .header(header::LOCATION, redirect_url)
                            .body(axum::body::Body::empty())
                            .unwrap();
                    }
                }
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

#[debug_handler]
async fn new_link_handler(
    State(state): State<AppState>,
    body: Json<NewRecordScheme>,
) -> impl IntoResponse {
    use self::schema::records;
    let hash = get_hash(&body.challenge);
    let hash_prefix = "0".repeat(state.current_difficulty);
    if !records::dsl::records
        .filter(records::dsl::challenge_proof.eq(body.challenge.clone()))
        .select(Record::as_select())
        .load(&mut *state.db.lock().unwrap())
        .expect("Could not query for existing proofs")
        .is_empty()
    {
        return (
            StatusCode::CONFLICT,
            "Proof already used! Try again".to_string(),
        );
    }
    if !hash.starts_with(&hash_prefix) {
        (
            StatusCode::BAD_REQUEST,
            "Hash does not compute!".to_string(),
        )
    } else {
        let values = NewRecord {
            redirect_url: body.payload.as_ref(),
            challenge_proof: &body.challenge,
        };
        let new_id = diesel::insert_into(records::table)
            .values(values)
            .returning(Record::as_returning())
            .get_result(&mut *state.db.lock().unwrap())
            .expect("failed writing record")
            .id
            .to_string();
        (StatusCode::OK, new_id)
    }
}
