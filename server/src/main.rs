use axum::{Router, routing::get};
use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use std::env;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use url::Url;

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
    address: String,
}

impl AppState {
    fn new(db_url: &str, difficulty: usize, address: String) -> AppState {
        AppState {
            db: Arc::new(Mutex::new(establish_connection(db_url))),
            current_difficulty: difficulty,
            address,
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap_or("../db/main.db".to_string());
    let address = env::var("ADDRESS").unwrap_or("127.0.0.1:3000".to_string());
    let difficulty = env::var("DIFFICULTY")
        .unwrap_or("1".to_string())
        .parse::<usize>()
        .unwrap();

    let state = AppState::new(&db_url, difficulty, format!("http://{address}"));
    // fix this ahh cors policy
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let all_routes = Router::new()
        .nest("/api", routes::api::router())
        .nest("/links", routes::links::router())
        .nest("/pkg", routes::assets::router())
        .route("/", get(routes::index::handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, all_routes).await.unwrap();
}
