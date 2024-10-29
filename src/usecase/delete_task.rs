use super::AppContext;

pub(crate) async fn delete_task(payload: Payload, context: AppContext) -> Result<(), Error> {
    match sqlx::query(
        r#"
DELETE tasks
WHERE
    id=$1::uuid AND
    user_id=$2::uuid
        "#,
    )
    .bind(payload.task_id)
    .bind(payload.user_id)
    .execute(&context.db_pool)
    .await
    {
        Ok(_) => {}
        Err(_) => {
            return Err(Error::DatabaseError);
        }
    }

    Ok(())
}

#[derive(serde::Deserialize)]
pub(crate) struct Payload {
    user_id: String,
    task_id: String,
}

impl Payload {
    pub(crate) fn new(user_id: String, task_id: String) -> Self {
        Self { user_id, task_id }
    }
}

#[derive(serde::Serialize, Debug)]
pub(crate) enum Error {
    DatabaseError,
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Impl test
}
