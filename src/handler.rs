use crate::env;

mod create_user;
pub(crate) use create_user::create_user;

mod get_healthz;
pub(crate) use get_healthz::get_healthz;

mod login;
pub(crate) use login::login;

mod create_task;
pub(crate) use create_task::create_task;

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

#[derive(serde::Serialize)]
pub(crate) struct UnauthorizedError {
    code: UnauthorizedErrorCode,
    message: String,
}

#[derive(serde::Serialize)]
pub(crate) enum UnauthorizedErrorCode {
    AuthenticationFailed,
    TokenExpired,
}
