use crate::task;

use sqlx::Row;

pub(crate) enum Error {
    Unknown,
}

pub(crate) async fn query(user_id: &str, db_pool: &sqlx::PgPool) -> Result<Vec<task::Task>, Error> {
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
    user_id = $1::uuid
"#,
    )
    .bind(user_id)
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
    .fetch_all(db_pool)
    .await
    {
        Ok(tasks) => Ok(tasks),
        // TODO: Add log
        Err(_) => Err(Error::Unknown),
    }
}
