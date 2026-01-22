use axum::{
    Router,
    routing::{get, post},
};
use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use std::env;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

pub mod models;
pub mod routes;
pub mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn establish_connection(db_url: &str) -> SqliteConnection {
    let mut conn = SqliteConnection::establish(db_url).expect("cannot connect to db");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("failed to run migrations");
    conn
}

#[derive(Clone)]
pub struct AppState {
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
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap_or("../db/main.db".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let difficulty = env::var("DIFFICULTY")
        .unwrap_or("1".to_string())
        .parse::<usize>()
        .unwrap();
    let state = AppState::new(&db_url, difficulty);
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);
    let backend_routes = Router::new()
        .route("/difficulty", get(routes::api::difficulty::handler))
        .route("/{id}", get(routes::api::redirect::handler))
        .route("/", post(routes::api::new_link::handler));

    let all_routes = Router::new()
        .nest("/api", backend_routes)
        .route("/", get(routes::index::handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, all_routes).await.unwrap();
}
