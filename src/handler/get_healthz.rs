use axum::{extract::State, http::StatusCode, response::IntoResponse};

use super::AppState;

pub(crate) async fn get_healthz(State(state): State<AppState>) -> impl IntoResponse {
    if sqlx::query("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .is_err()
    {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    }
}
