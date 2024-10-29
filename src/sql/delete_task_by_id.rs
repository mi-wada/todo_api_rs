pub(crate) enum Error {
    Unknown,
}

pub(crate) async fn query(
    user_id: &str,
    task_id: &str,
    db_pool: &sqlx::PgPool,
) -> Result<(), Error> {
    match sqlx::query(
        r#"
DELETE tasks
WHERE
    user_id=$1::uuid AND
    id=$2::uuid
"#,
    )
    .bind(user_id)
    .bind(task_id)
    .execute(db_pool)
    .await
    {
        Ok(_) => Ok(()),
        // TODO: Add log
        Err(_) => Err(Error::Unknown),
    }
}
