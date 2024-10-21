use super::AppContext;

pub(crate) async fn delete_task(
    payload: DeleteTaskPayload,
    context: AppContext,
) -> Result<(), DeleteTaskError> {
    match sqlx::query(
        r#"
DELETE tasks
WHERE id=$1::uuid AND user_id=$2::uuid
        "#,
    )
    .bind(payload.task_id)
    .bind(payload.user_id)
    .execute(&context.db_pool)
    .await
    {
        Ok(_) => {}
        Err(_) => {
            return Err(DeleteTaskError::DatabaseError);
        }
    }

    Ok(())
}

#[derive(serde::Deserialize)]
pub(crate) struct DeleteTaskPayload {
    user_id: String,
    task_id: String,
}

#[derive(serde::Serialize, Debug)]
pub(crate) enum DeleteTaskError {
    DatabaseError,
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

    // TODO: Impl test
}
