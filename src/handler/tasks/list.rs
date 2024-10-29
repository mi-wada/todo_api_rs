use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{handler::InternalServerError, sql, task::Task, usecase::AppContext, user::User};

pub(crate) async fn list(
    State(context): State<AppContext>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
    match sql::list_tasks::query(user.id().value(), &context.db_pool).await {
        Ok(tasks) => Ok((StatusCode::OK, Json(Response::Ok(tasks)))),
        Err(sql::list_tasks::Error::Unknown) => Err(Error::Database),
    }
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum Response {
    Ok(Vec<Task>),
    InternalServerError(InternalServerError),
}

#[derive(serde::Serialize, Debug)]
pub(crate) enum Error {
    Database,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::InternalServerError(InternalServerError::default())),
            ),
        }
        .into_response()
    }
}
