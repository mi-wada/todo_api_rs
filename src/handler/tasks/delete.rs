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

pub(crate) async fn delete(
    Path(task_id): Path<String>,
    State(context): State<AppContext>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, usecase::delete_task::Error> {
    usecase::delete_task::delete_task(
        usecase::delete_task::Payload::new(user.id().value().into(), task_id),
        context,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

impl IntoResponse for usecase::delete_task::Error {
    fn into_response(self) -> axum::response::Response {
        match self {
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
    NoContent,
    InternalServerError(InternalServerError),
}
