use axum::{extract::State, http::StatusCode, response::IntoResponse};

use super::AppState;

pub(crate) async fn get_healthz(State(state): State<AppState>) -> impl IntoResponse {
    if db_healthy(&state.db_pool).await {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    }
}

async fn db_healthy(db_pool: &sqlx::PgPool) -> bool {
    sqlx::query("SELECT 1").fetch_one(db_pool).await.is_ok()
}
