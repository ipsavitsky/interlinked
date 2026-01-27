use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;

use crate::AppState;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct MainPage {
    difficulty: u32,
    url: String,
}

pub async fn handler(State(state): State<AppState>) -> MainPage {
    MainPage {
        difficulty: state.configuration.difficulty,
        url: state
            .configuration
            .url
            .to_string()
            .trim_end_matches("/")
            .to_string(),
    }
}
