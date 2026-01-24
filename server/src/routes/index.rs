use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use url::Url;

use crate::AppState;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct MainPage {
    difficulty: usize,
    address: String,
}

pub async fn handler(State(state): State<AppState>) -> MainPage {
    MainPage {
        difficulty: state.current_difficulty,
        address: state.address,
    }
}
