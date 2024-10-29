use crate::usecase::AppContext;
use sqlx::postgres::PgPoolOptions;

pub(crate) async fn db_pool(env: &crate::env::Env) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .idle_timeout(std::time::Duration::from_secs(5))
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&env.database_url)
        .await
        .expect("Failed to connect to DB")
}

pub(crate) async fn context() -> AppContext {
    crate::env::Env::init_test();
    let env = crate::env::Env::new();

    let db_pool = db_pool(&env).await;

    AppContext::new(env, db_pool)
}

pub(crate) fn unique_email() -> String {
    format!("user+{}@example.com", uuid::Uuid::now_v7())
}
