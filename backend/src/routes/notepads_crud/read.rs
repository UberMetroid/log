use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::cookie::CookieJar;

use crate::state::AppState;

pub const PAGE_HISTORY_COOKIE: &str = "log_page_history";

pub async fn get_notepads(jar: CookieJar, State(state): State<AppState>) -> impl IntoResponse {
    let list = state.notepads.read().await.clone();
    let note_history = jar
        .get(PAGE_HISTORY_COOKIE)
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| "default".to_string());

    axum::Json(serde_json::json!({
        "notepads_list": list,
        "note_history": note_history
    }))
}
