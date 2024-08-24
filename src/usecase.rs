mod create_user;
pub(crate) use create_user::{create_user, CreateUserError, CreateUserPayload};

use crate::env;

// TODO: rename to `Context`? `AppContext`?
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
