use crate::task;

use sqlx::Row;

pub(crate) enum Error {
    NotFound,
    Unknown,
}

pub(crate) async fn query(
    user_id: &str,
    task_id: &str,
    db_pool: &sqlx::PgPool,
) -> Result<task::Task, Error> {
    match sqlx::query(
        r#"
SELECT
    id,
    user_id,
    title,
    description,
    status,
    deadline
FROM tasks
WHERE
    user_id = $1::uuid AND
    id = $2::uuid
"#,
    )
    .bind(user_id)
    .bind(task_id)
    .map(|row: sqlx::postgres::PgRow| {
        task::Task::restore(
            row.get::<sqlx::types::Uuid, _>(0).into(),
            row.get::<sqlx::types::Uuid, _>(1).into(),
            row.get(2),
            row.get(3),
            row.get(4),
            row.get(5),
        )
    })
    .fetch_one(db_pool)
    .await
    {
        Ok(task) => Ok(task),
        Err(sqlx::Error::RowNotFound) => Err(Error::NotFound),
        // TODO: Add log
        Err(_) => Err(Error::Unknown),
    }
}
