use crate::{task, user};

use super::AppContext;

pub(crate) async fn create_task(
    payload: Payload,
    context: AppContext,
) -> Result<task::Task, Error> {
    let task = task::Task::_new(
        payload.user_id.value().into(),
        payload.title.ok_or(Error::TitleEmpty)?,
        payload.description,
        payload.status.ok_or(Error::StatusUnknown)?,
        payload.deadline,
    )?;

    if sqlx::query(
        r#"
INSERT INTO tasks (
    id,
    user_id,
    title,
    description,
    status,
    deadline
)
VALUES (
    $1::uuid,
    $2::uuid,
    $3,
    $4,
    $5,
    $6
)"#,
    )
    .bind(task.id().value())
    .bind(task.user_id().value())
    .bind(task.title().value())
    .bind(task.description().map(|d| d.value()))
    .bind::<&str>((*task.status()).into())
    .bind(task.deadline().map(|d| d.value()))
    .execute(&context.db_pool)
    .await
    .is_err()
    {
        // TODO: Add log
        return Err(Error::Database);
    }

    Ok(task)
}

pub(crate) struct Payload {
    pub user_id: user::Id,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub deadline: Option<String>,
}

#[derive(serde::Serialize, Debug)]
pub(crate) enum Error {
    TitleEmpty,
    TitleTooLong,
    DescriptionTooLong,
    StatusUnknown,
    DeadlineWrongFormat,
    Database,
}

impl From<task::TaskNewError> for Error {
    fn from(err: task::TaskNewError) -> Self {
        match err {
            task::TaskNewError::TitleEmpty => Self::TitleEmpty,
            task::TaskNewError::TitleTooLong => Self::TitleTooLong,
            task::TaskNewError::DescriptionTooLong => Self::DescriptionTooLong,
            task::TaskNewError::StatusUnknown => Self::StatusUnknown,
            task::TaskNewError::DeadlineWrongFormat => Self::DeadlineWrongFormat,
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlx::postgres::PgPoolOptions;

    use super::*;

    async fn db_pool(env: &crate::env::Env) -> sqlx::PgPool {
        PgPoolOptions::new()
            .max_connections(5)
            .idle_timeout(std::time::Duration::from_secs(5))
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&env.database_url)
            .await
            .expect("Failed to connect to DB")
    }

    async fn context() -> AppContext {
        crate::env::Env::init_test();
        let env = crate::env::Env::new();

        let db_pool = db_pool(&env).await;

        AppContext::new(env, db_pool)
    }

    // TODO: Add tests
}
