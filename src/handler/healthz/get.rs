use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::usecase::AppContext;

pub(crate) async fn get(State(context): State<AppContext>) -> impl IntoResponse {
    if db_healthy(&context.db_pool).await {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    }
}

async fn db_healthy(db_pool: &sqlx::PgPool) -> bool {
    sqlx::query("SELECT 1").fetch_one(db_pool).await.is_ok()
}
