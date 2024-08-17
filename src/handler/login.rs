use axum::{extract::State, http::StatusCode, Json};
use sqlx::Row;

use crate::handler::AppState;

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

    if authenticate(
        &state.db_pool,
        &payload.email.clone().unwrap(),
        &payload.password.clone().unwrap(),
    )
    .await
    {
        (
            StatusCode::OK,
            Json(LoginResponse::Ok {
                token: "TODO".to_string(),
            }),
        )
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse::Unauthorized(UnauthorizedError {
                code: UnauthorizedErrorCode::AuthenticationFailed,
                message: "Authentication failed".into(),
            })),
        )
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
async fn authenticate(db_pool: &sqlx::PgPool, email: &str, password: &str) -> bool {
    let hashed_password = sqlx::query(
        r#"
SELECT password
FROM users
WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_one(db_pool)
    .await
    .map(|row| row.get::<String, _>("password"))
    .unwrap();

    crate::password::verify_password(password, &hashed_password)
}
