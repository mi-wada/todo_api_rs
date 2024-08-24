use crate::user::{self, EmailNewError, PasswordNewError, User};

use super::AppState;

pub(crate) async fn create_user(
    payload: CreateUserPayload,
    state: AppState,
) -> Result<User, CreateUserError> {
    let email = user::Email::new(payload.email.ok_or(CreateUserError::EmailEmpty)?)?;

    let hashed_password =
        user::Password::new(payload.password.ok_or(CreateUserError::PasswordEmpty)?)?.hashed();

    let id = user::Id::new();

    match sqlx::query(
        r#"
INSERT INTO users (id, email, password)
VALUES ($1::uuid, $2, $3)
        "#,
    )
    .bind(id.value())
    .bind(email.value())
    .bind(hashed_password)
    .execute(&state.db_pool)
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

    Ok(User::new(id, email))
}

#[derive(serde::Deserialize)]
pub(crate) struct CreateUserPayload {
    email: Option<String>,
    password: Option<String>,
}

#[derive(serde::Serialize)]
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

impl From<EmailNewError> for CreateUserError {
    fn from(err: EmailNewError) -> Self {
        match err {
            EmailNewError::Empty => Self::EmailEmpty,
            EmailNewError::TooLong => Self::EmailTooLong,
            EmailNewError::WrongFormat => Self::EmailWrongFormat,
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
