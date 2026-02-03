use axum::{Router, routing::get};
use config::Config;
use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use object_store::{ObjectStore, local::LocalFileSystem};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use tower_http::cors::{Any, CorsLayer};

pub mod config;
pub mod models;
pub mod routes;
pub mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn establish_connection(path: &Path) -> SqliteConnection {
    let mut conn = SqliteConnection::establish(
        path.join("interlinked.db")
            .to_str()
            .expect("cannot get path"),
    )
    .expect("cannot connect to db");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("failed to run migrations");
    conn
}

fn connect_to_store(path: &Path) -> LocalFileSystem {
    let objects_path = path.join("objects");
    if !objects_path.exists() {
        std::fs::create_dir(&objects_path).expect("Could not create objects dir");
    }
    if !objects_path.is_dir() {
        panic!("objects path is not a directory");
    }
    LocalFileSystem::new_with_prefix(objects_path).expect("Failed to initialize object storage")
}

#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<SqliteConnection>>,
    bucket: Arc<dyn ObjectStore>,
    configuration: Config,
}

impl AppState {
    fn new(conf: Config) -> AppState {
        AppState {
            db: Arc::new(Mutex::new(establish_connection(&conf.store_dir))),
            bucket: Arc::new(connect_to_store(&conf.store_dir)),
            configuration: conf,
        }
    }
}

#[tokio::main]
async fn main() {
    let conf = Config::parse().expect("Failed to parse config");

    tracing_subscriber::fmt::fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_max_level(conf.log_level)
        .init();

    let address = conf.address.clone(); // FIXME
    let state = AppState::new(conf);
    // fix this ahh cors policy
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let all_routes = Router::new()
        .nest("/api", routes::api::router())
        .nest("/link", routes::links::router())
        .nest("/note", routes::notes::router())
        .nest("/pkg", routes::assets::router())
        .route("/", get(routes::index::handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, all_routes).await.unwrap();
}
