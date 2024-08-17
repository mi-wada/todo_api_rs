use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::env;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) env: env::Env,
    pub(crate) db_pool: sqlx::PgPool,
}

impl AppState {
    pub(crate) fn new(env: env::Env, db_pool: sqlx::PgPool) -> Self {
        Self { env, db_pool }
    }
}

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
