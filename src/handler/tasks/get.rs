use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    handler::{InternalServerError, NotFoundError},
    sql,
    task::Task,
    usecase::AppContext,
    user::User,
};

pub(crate) async fn get(
    Path(task_id): Path<String>,
    State(context): State<AppContext>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
    match sql::get_task_by_id::query(user.id().value(), &task_id, &context.db_pool).await {
        Ok(task) => Ok((StatusCode::OK, Json(Response::Ok(task)))),
        Err(sql::get_task_by_id::Error::NotFound) => Err(Error::NotFound),
        Err(sql::get_task_by_id::Error::Unknown) => Err(Error::DatabaseError),
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum Response {
    Ok(Task),
    NotFound(NotFoundError),
    InternalServerError(InternalServerError),
}

#[derive(serde::Serialize, Debug)]
pub(crate) enum Error {
    NotFound,
    DatabaseError,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::InternalServerError(InternalServerError::default())),
            ),
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                Json(Response::NotFound(NotFoundError::default())),
            ),
        }
        .into_response()
    }
}
