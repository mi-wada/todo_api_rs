use crate::user::{self, PasswordNewError, User, UserNewError};

use super::AppContext;

pub(crate) async fn create_user(
    payload: CreateUserPayload,
    context: AppContext,
) -> Result<User, CreateUserError> {
    let user = user::User::new(payload.email.ok_or(CreateUserError::EmailEmpty)?)?;

    let hashed_password =
        user::Password::new(payload.password.ok_or(CreateUserError::PasswordEmpty)?)?.hashed();

    match sqlx::query(
        r#"
INSERT INTO users (id, email, password)
VALUES ($1::uuid, $2, $3)
        "#,
    )
    .bind(user.id().value())
    .bind(user.email().value())
    .bind(hashed_password)
    .execute(&context.db_pool)
    .await
    {
        Ok(_) => {}
        Err(sqlx::Error::Database(db_error)) if db_error.is_unique_violation() => {
            return Err(CreateUserError::EmailTaken);
        }
        Err(_) => {
            return Err(CreateUserError::DatabaseError);
        }
    }

    Ok(user)
}

#[derive(serde::Deserialize)]
pub(crate) struct CreateUserPayload {
    email: Option<String>,
    password: Option<String>,
}

#[derive(serde::Serialize, Debug)]
pub(crate) enum CreateUserError {
    EmailEmpty,
    EmailTooLong,
    EmailWrongFormat,
    EmailTaken,
    PasswordEmpty,
    PasswordTooShort,
    PasswordTooLong,
    DatabaseError,
}

impl From<UserNewError> for CreateUserError {
    fn from(err: UserNewError) -> Self {
        match err {
            UserNewError::EmailEmpty => Self::EmailEmpty,
            UserNewError::EmailTooLong => Self::EmailTooLong,
            UserNewError::EmailWrongFormat => Self::EmailWrongFormat,
        }
    }
}

impl From<PasswordNewError> for CreateUserError {
    fn from(err: PasswordNewError) -> Self {
        match err {
            PasswordNewError::TooShort => Self::PasswordTooShort,
            PasswordNewError::TooLong => Self::PasswordTooLong,
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

    #[tokio::test]
    async fn test_create_user_ok() {
        let email = format!("{}@example.com", uuid::Uuid::now_v7());
        let payload = CreateUserPayload {
            email: Some(email.clone()),
            password: Some("password".into()),
        };

        let user = create_user(payload, context().await).await.unwrap();
        assert!(!user.id().value().is_empty());
        assert_eq!(user.email().value(), email);
        // TODO: need to rollback
        // * Fix AppContext.db_pool type. Accepts Pool and Tx.
    }
}
