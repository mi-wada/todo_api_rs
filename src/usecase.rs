mod create_user;
pub(crate) use create_user::{create_user, CreateUserError, CreateUserPayload};

mod delete_task;
pub(crate) use delete_task::{delete_task, DeleteTaskError, DeleteTaskPayload};

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
