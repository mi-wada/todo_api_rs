use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use sqlx::Row;

use crate::{
    usecase::AppContext,
    user::{self, User},
};

use super::{InternalServerError, InternalServerErrorCode};

pub(crate) async fn auth(
    State(context): State<AppContext>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthMiddlewareResponse>)> {
    let token = match req.headers().get(header::AUTHORIZATION.as_str()) {
        Some(token) => token.to_str().unwrap().strip_prefix("Bearer ").unwrap(),
        None => {
            return Err(unauthorized_error(
                UnauthorizedErrorCode::AuthenticationFailed,
                "Token missing".into(),
            ))
        }
    };

    let user_id = match user::access_token::decode(token, &context.env.access_token_secret) {
        Ok(claims) => user::Id::restore(claims.sub().into()),
        Err(user::access_token::DecodeError::Expired) => {
            return Err(unauthorized_error(
                UnauthorizedErrorCode::TokenExpired,
                "Token expired".into(),
            ));
        }
        Err(user::access_token::DecodeError::Tempered) => {
            return Err(unauthorized_error(
                UnauthorizedErrorCode::AuthenticationFailed,
                "Authentication failed".into(),
            ));
        }
    };

    // TODO: Define at other module
    let user = sqlx::query(
        r#"
SELECT id, email
FROM users
WHERE id = $1::uuid
        "#,
    )
    .bind(user_id.value())
    .map(|row: sqlx::postgres::PgRow| {
        User::restore(row.get::<sqlx::types::Uuid, _>(0).into(), row.get(1))
    })
    .fetch_one(&context.db_pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => unauthorized_error(
            UnauthorizedErrorCode::AuthenticationFailed,
            "User not found".into(),
        ),
        _ => internal_server_error(
            InternalServerErrorCode::InternalServerError,
            "Database error".into(),
        ),
    })?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum AuthMiddlewareResponse {
    Unauthorized(UnauthorizedError),
    InternalServerError(InternalServerError),
}

fn internal_server_error(
    code: InternalServerErrorCode,
    message: String,
) -> (StatusCode, Json<AuthMiddlewareResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(AuthMiddlewareResponse::InternalServerError(
            InternalServerError { code, message },
        )),
    )
}

fn unauthorized_error(
    code: UnauthorizedErrorCode,
    message: String,
) -> (StatusCode, Json<AuthMiddlewareResponse>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(AuthMiddlewareResponse::Unauthorized(UnauthorizedError {
            code,
            message,
        })),
    )
}

#[derive(serde::Serialize)]
pub(crate) struct UnauthorizedError {
    code: UnauthorizedErrorCode,
    message: String,
}

#[derive(serde::Serialize)]
pub(crate) enum UnauthorizedErrorCode {
    AuthenticationFailed,
    TokenExpired,
}
