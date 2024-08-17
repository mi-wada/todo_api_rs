use crate::env;

mod create_user;
pub(crate) use create_user::create_user;

mod get_healthz;
pub(crate) use get_healthz::get_healthz;

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
