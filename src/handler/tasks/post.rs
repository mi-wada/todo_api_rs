use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{
    handler::InternalServerError,
    task::Task,
    usecase::{create_task, AppContext},
    user::User,
};

pub(crate) async fn post(
    State(context): State<AppContext>,
    Extension(user): Extension<User>,
    payload: axum::Json<Payload>,
) -> Result<impl IntoResponse, create_task::Error> {
    let task = create_task::create_task(
        create_task::Payload {
            user_id: user.id().clone(),
            title: payload.title.clone(),
            description: payload.description.clone(),
            status: payload.status.clone(),
            deadline: payload.deadline.clone(),
        },
        context,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(Response::Created(task))))
}

#[derive(serde::Deserialize)]
pub(crate) struct Payload {
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    deadline: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum Response {
    Created(Task),
    BadRequest(BadRequestError),
    InternalServerError(InternalServerError),
}

fn bad_request_error(code: BadRequestErrorCode, message: String) -> (StatusCode, Json<Response>) {
    (
        StatusCode::BAD_REQUEST,
        Json(Response::BadRequest(BadRequestError { code, message })),
    )
}

#[derive(serde::Serialize)]
pub(crate) struct BadRequestError {
    code: BadRequestErrorCode,
    message: String,
}

#[derive(serde::Serialize)]
pub(crate) enum BadRequestErrorCode {
    TitleEmpty,
    TitleTooLong,
    DescriptionTooLong,
    StatusUnknown,
    DeadlineWrongFormat,
}

impl IntoResponse for create_task::Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::TitleEmpty => {
                bad_request_error(BadRequestErrorCode::TitleEmpty, "Title is empty".into())
            }
            Self::TitleTooLong => bad_request_error(
                BadRequestErrorCode::TitleTooLong,
                "Title is too long".into(),
            ),
            Self::DescriptionTooLong => bad_request_error(
                BadRequestErrorCode::DescriptionTooLong,
                "Description is too long".into(),
            ),
            Self::StatusUnknown => bad_request_error(
                BadRequestErrorCode::StatusUnknown,
                "Status is unknown".into(),
            ),
            Self::DeadlineWrongFormat => bad_request_error(
                BadRequestErrorCode::DeadlineWrongFormat,
                "Deadline has wrong format".into(),
            ),
            Self::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::InternalServerError(InternalServerError::default())),
            ),
        }
        .into_response()
    }
}
