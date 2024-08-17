use axum::{extract::State, http::StatusCode, Json};

use crate::{
    handler::AppState,
    user::{self, User},
};

pub(crate) async fn create_user(
    State(state): State<AppState>,
    payload: axum::Json<CreateUserPayload>,
) -> (StatusCode, Json<CreateUserResponse>) {
    if payload.email.is_none() {
        return bad_request(ErrorCode::EmailEmpty, "Email is missing");
    }
    let email = match user::Email::new(payload.email.clone().unwrap()) {
        Ok(email) => email,
        Err(user::EmailNewError::Empty) => {
            return bad_request(ErrorCode::EmailEmpty, "Email is empty");
        }
        Err(user::EmailNewError::TooLong) => {
            return bad_request(ErrorCode::EmailTooLong, "Email is too long");
        }
        Err(user::EmailNewError::WrongFormat) => {
            return bad_request(ErrorCode::EmailWrongFormat, "Email is in wrong format");
        }
    };

    if payload.password.is_none() {
        return bad_request(ErrorCode::PasswordEmpty, "Password is missing");
    }
    let password = match user::Password::new(payload.password.clone().unwrap()) {
        Ok(password) => password,
        Err(user::PasswordNewError::TooShort) => {
            return bad_request(ErrorCode::PasswordTooShort, "Password is too short");
        }
        Err(user::PasswordNewError::TooLong) => {
            return bad_request(ErrorCode::PasswordTooLong, "Password is too long");
        }
    };
    let id = user::Id::new();

    // TODO: Save user to the database

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
    BadRequest(Error),
}

fn bad_request(error: ErrorCode, message: &str) -> (StatusCode, Json<CreateUserResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(CreateUserResponse::BadRequest(Error {
            code: error,
            message: message.into(),
        })),
    )
}

#[derive(serde::Serialize)]
pub(crate) struct Error {
    code: ErrorCode,
    message: String,
}

#[derive(serde::Serialize)]
pub(crate) enum ErrorCode {
    EmailEmpty,
    EmailTooLong,
    EmailWrongFormat,
    EmailTaken,
    PasswordEmpty,
    PasswordTooShort,
    PasswordTooLong,
}
