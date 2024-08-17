use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::env;

#[derive(Clone)]
pub(crate) struct AppState {
    env: env::Env,
}

impl AppState {
    pub(crate) fn new(env: env::Env) -> Self {
        Self { env }
    }
}

pub(crate) async fn get_healthz(State(state): State<AppState>) -> impl IntoResponse {
    StatusCode::OK
}
