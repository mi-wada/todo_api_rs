use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    handler::InternalServerError,
    usecase::{self, AppContext},
    user::User,
};

pub(crate) async fn post(
    State(context): State<AppContext>,
    payload: axum::Json<usecase::CreateUserPayload>,
) -> Result<impl IntoResponse, usecase::CreateUserError> {
    let user = usecase::create_user(payload.0, context).await?;

    Ok((StatusCode::CREATED, Json(Response::Created(user))))
}

impl IntoResponse for usecase::CreateUserError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::EmailEmpty => bad_request(BadRequestErrorCode::EmailEmpty, "Email is empty"),
            Self::EmailTooLong => {
                bad_request(BadRequestErrorCode::EmailTooLong, "Email is too long")
            }
            Self::EmailWrongFormat => bad_request(
                BadRequestErrorCode::EmailWrongFormat,
                "Email has wrong format", // TODO: define message at Usecase level.
            ),
            Self::EmailTaken => bad_request(BadRequestErrorCode::EmailTaken, "Email is taken"),
            Self::PasswordEmpty => {
                bad_request(BadRequestErrorCode::PasswordEmpty, "Password is empty")
            }
            Self::PasswordTooShort => bad_request(
                BadRequestErrorCode::PasswordTooShort,
                "Password is too short",
            ),
            Self::PasswordTooLong => {
                bad_request(BadRequestErrorCode::PasswordTooLong, "Password is too long")
            }
            Self::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::InternalServerError(InternalServerError::default())),
            ),
        }
        .into_response()
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum Response {
    Created(User),
    BadRequest(BadRequestError),
    InternalServerError(InternalServerError),
}

fn bad_request(code: BadRequestErrorCode, message: &str) -> (StatusCode, Json<Response>) {
    (
        StatusCode::BAD_REQUEST,
        Json(Response::BadRequest(BadRequestError {
            code,
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
