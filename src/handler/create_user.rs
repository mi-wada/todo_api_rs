use axum::{extract::State, http::StatusCode, Json};

use crate::{
    handler::{AppState, InternalServerError, InternalServerErrorCode},
    password,
    user::{self, User},
};

pub(crate) async fn create_user(
    State(state): State<AppState>,
    payload: axum::Json<CreateUserPayload>,
) -> (StatusCode, Json<CreateUserResponse>) {
    if payload.email.is_none() {
        return bad_request(BadRequestErrorCode::EmailEmpty, "Email is missing");
    }
    let email = match user::Email::new(payload.email.clone().unwrap()) {
        Ok(email) => email,
        Err(user::EmailNewError::Empty) => {
            return bad_request(BadRequestErrorCode::EmailEmpty, "Email is empty");
        }
        Err(user::EmailNewError::TooLong) => {
            return bad_request(BadRequestErrorCode::EmailTooLong, "Email is too long");
        }
        Err(user::EmailNewError::WrongFormat) => {
            return bad_request(
                BadRequestErrorCode::EmailWrongFormat,
                "Email is in wrong format",
            );
        }
    };

    if payload.password.is_none() {
        return bad_request(BadRequestErrorCode::PasswordEmpty, "Password is missing");
    }
    let password = match user::Password::new(payload.password.clone().unwrap()) {
        Ok(password) => password,
        Err(user::PasswordNewError::TooShort) => {
            return bad_request(
                BadRequestErrorCode::PasswordTooShort,
                "Password is too short",
            );
        }
        Err(user::PasswordNewError::TooLong) => {
            return bad_request(BadRequestErrorCode::PasswordTooLong, "Password is too long");
        }
    };
    let hashed_password = password::hash_password(password.value());

    let id = user::Id::new();

    // TODO: Move this to other module.
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
            return bad_request(BadRequestErrorCode::EmailTaken, "Email is already taken");
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateUserResponse::InternalServerError(
                    InternalServerError {
                        code: InternalServerErrorCode::InternalServerError,
                        message: "Internal server error".into(),
                    },
                )),
            );
        }
    }

    (
        StatusCode::CREATED,
        Json(CreateUserResponse::Created(User::new(id, email))),
    )
}

#[derive(serde::Deserialize)]
pub(crate) struct CreateUserPayload {
    email: Option<String>,
    password: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum CreateUserResponse {
    Created(User),
    BadRequest(BadRequestError),
    InternalServerError(InternalServerError),
}

fn bad_request(
    error: BadRequestErrorCode,
    message: &str,
) -> (StatusCode, Json<CreateUserResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(CreateUserResponse::BadRequest(BadRequestError {
            code: error,
            message: message.into(),
        })),
    )
}

#[derive(serde::Serialize)]
pub(crate) struct BadRequestError {
    code: BadRequestErrorCode,
    message: String,
}

#[derive(serde::Serialize)]
pub(crate) enum BadRequestErrorCode {
    EmailEmpty,
    EmailTooLong,
    EmailWrongFormat,
    EmailTaken,
    PasswordEmpty,
    PasswordTooShort,
    PasswordTooLong,
}
