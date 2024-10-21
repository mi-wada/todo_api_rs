use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    handler::InternalServerError,
    usecase::{self, AppContext},
    user::User,
};

pub(crate) async fn delete_task(
    Path(task_id): Path<String>,
    State(context): State<AppContext>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, usecase::DeleteTaskError> {
    usecase::delete_task(
        usecase::DeleteTaskPayload::new(user.id().value().into(), task_id),
        context,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

impl IntoResponse for usecase::DeleteTaskError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteTaskResponse::InternalServerError(
                    InternalServerError::default(),
                )),
            ),
        }
        .into_response()
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum DeleteTaskResponse {
    NoContent,
    InternalServerError(InternalServerError),
}
