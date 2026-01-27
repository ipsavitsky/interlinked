use axum::extract::State;

use crate::AppState;

pub async fn handler(State(state): State<AppState>) -> String {
    state.configuration.difficulty.to_string()
}
