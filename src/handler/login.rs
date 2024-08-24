use axum::{extract::State, http::StatusCode, Json};
use sqlx::Row;

use crate::{usecase::AppState, user};

pub(crate) async fn login(
    State(state): State<AppState>,
    payload: axum::Json<LoginPayload>,
) -> (StatusCode, Json<LoginResponse>) {
    if payload.email.is_none() {
        return bad_request(BadRequestErrorCode::EmailEmpty, "Email is missing");
    }
    if payload.password.is_none() {
        return bad_request(BadRequestErrorCode::PasswordEmpty, "Password is missing");
    }

    match authenticate(
        &state.db_pool,
        &payload.email.clone().unwrap(),
        &payload.password.clone().unwrap(),
    )
    .await
    {
        Some(user_id) => (
            StatusCode::OK,
            Json(LoginResponse::Ok {
                token: user::access_token::encode(user_id, None, &state.env.access_token_secret),
            }),
        ),
        None => (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse::Unauthorized(UnauthorizedError {
                code: UnauthorizedErrorCode::AuthenticationFailed,
                message: "Authentication failed".into(),
            })),
        ),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct LoginPayload {
    email: Option<String>,
    password: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum LoginResponse {
    Ok { token: String },
    BadRequest(BadRequestError),
    Unauthorized(UnauthorizedError),
}

fn bad_request(error: BadRequestErrorCode, message: &str) -> (StatusCode, Json<LoginResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(LoginResponse::BadRequest(BadRequestError {
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
    PasswordEmpty,
}

#[derive(serde::Serialize)]
pub(crate) struct UnauthorizedError {
    code: UnauthorizedErrorCode,
    message: String,
}

#[derive(serde::Serialize)]
pub(crate) enum UnauthorizedErrorCode {
    AuthenticationFailed,
}

// TODO: Move it to user module.
async fn authenticate(db_pool: &sqlx::PgPool, email: &str, password: &str) -> Option<user::Id> {
    let (id, hashed_password) = sqlx::query(
        r#"
SELECT id, password
FROM users
WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_one(db_pool)
    .await
    .map(|row| {
        (
            row.get::<sqlx::types::Uuid, _>("id"),
            row.get::<String, _>("password"),
        )
    })
    .unwrap();

    if crate::password::verify_password(password, &hashed_password) {
        Some(user::Id::restore(id.into()))
    } else {
        None
    }
}
