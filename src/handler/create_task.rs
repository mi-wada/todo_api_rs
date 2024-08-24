use axum::{extract::State, http::StatusCode, Extension, Json};

use crate::{
    handler::{InternalServerError, InternalServerErrorCode},
    task::{self, Task},
    usecase::AppState,
    user::User,
};

pub(crate) async fn create_task(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    payload: axum::Json<CreateTaskPayload>,
) -> (StatusCode, Json<CreateIssueResponse>) {
    let id = task::Id::new();

    if payload.title.is_none() {
        return bad_request_error(BadRequestErrorCode::TitleEmpty, "Title is missing".into());
    }
    let title = match task::Title::new(payload.title.clone().unwrap()) {
        Ok(title) => title,
        Err(task::TitleNewError::Empty) => {
            return bad_request_error(BadRequestErrorCode::TitleEmpty, "Title is empty".into());
        }
        Err(task::TitleNewError::TooLong) => {
            return bad_request_error(
                BadRequestErrorCode::TitleTooLong,
                "Title is too long".into(),
            );
        }
    };

    let description = match &payload.description {
        Some(description) => match task::Description::new(description.into()) {
            Ok(description) => Some(description),
            Err(task::DescriptionNewError::TooLong) => {
                return bad_request_error(
                    BadRequestErrorCode::DescriptionTooLong,
                    "Description is too long".into(),
                );
            }
        },
        None => None,
    };

    let status = match &payload.status {
        Some(status) => match task::Status::try_from(status.as_str()) {
            Ok(status) => status,
            Err(task::StatusNewError::Unknown) => {
                return bad_request_error(
                    BadRequestErrorCode::StatusUnknown,
                    "Status is unknown".into(),
                );
            }
        },
        None => {
            return bad_request_error(
                BadRequestErrorCode::StatusUnknown,
                "Status is unknown".into(),
            );
        }
    };

    let deadline = match &payload.deadline {
        Some(deadline) => match task::Deadline::new(deadline.into()) {
            Ok(deadline) => Some(deadline),
            Err(task::DeadlineNewError::WrongFormat) => {
                return bad_request_error(
                    BadRequestErrorCode::DeadlineWrongFormat,
                    "Deadline is in wrong format".into(),
                );
            }
        },
        None => None,
    };

    // TODO: Move this to other module.
    if sqlx::query(
        r#"
INSERT INTO tasks (id, user_id, title, description, status, deadline)
VALUES ($1::uuid, $2::uuid, $3, $4, $5, $6)
        "#,
    )
    .bind(id.value())
    .bind(user.id().value())
    .bind(title.value())
    .bind(description.clone().map(|d| d.value().to_string()))
    .bind::<&str>(status.into())
    .bind(deadline.clone().map(|d| *d.value()))
    .execute(&state.db_pool)
    .await
    .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CreateIssueResponse::InternalServerError(
                InternalServerError {
                    code: InternalServerErrorCode::InternalServerError,
                    message: "Internal server error".into(),
                },
            )),
        );
    }

    let task = Task::new(id, user.id().clone(), title, description, status, deadline);

    (
        StatusCode::CREATED,
        Json(CreateIssueResponse::Created(task)),
    )
}

#[derive(serde::Deserialize)]
pub(crate) struct CreateTaskPayload {
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    deadline: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum CreateIssueResponse {
    Created(Task),
    BadRequest(BadRequestError),
    InternalServerError(InternalServerError),
}

fn bad_request_error(
    code: BadRequestErrorCode,
    message: String,
) -> (StatusCode, Json<CreateIssueResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(CreateIssueResponse::BadRequest(BadRequestError {
            code,
            message,
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
    TitleEmpty,
    TitleTooLong,
    DescriptionTooLong,
    StatusUnknown,
    DeadlineWrongFormat,
}
