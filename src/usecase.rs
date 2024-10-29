pub(crate) mod create_user;

pub(crate) mod create_task;

pub(crate) mod delete_task;

use crate::env;

#[derive(Clone)]
pub(crate) struct AppContext {
    pub(crate) env: env::Env,
    pub(crate) db_pool: sqlx::PgPool,
}

impl AppContext {
    pub(crate) fn new(env: env::Env, db_pool: sqlx::PgPool) -> Self {
        Self { env, db_pool }
    }
}
