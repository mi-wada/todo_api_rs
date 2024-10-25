use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    handler::{InternalServerError, NotFoundError},
    task::Task,
    usecase::AppContext,
    user::User,
};

use sqlx::Row;

pub(crate) async fn get(
    Path(task_id): Path<String>,
    State(context): State<AppContext>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
    let task = sqlx::query(
        r#"
SELECT id, user_id, title, description, status, deadline
FROM tasks
WHERE user_id = $1::uuid AND id = $2::uuid
        "#,
    )
    .bind(user.id().value())
    .bind(task_id)
    .map(|row: sqlx::postgres::PgRow| {
        Task::restore(
            row.get::<sqlx::types::Uuid, _>(0).into(),
            row.get::<sqlx::types::Uuid, _>(1).into(),
            row.get(2),
            row.get(3),
            row.get(4),
            row.get(5),
        )
    })
    .fetch_optional(&context.db_pool)
    .await
    .map_err(|_| Error::DatabaseError)?;

    match task {
        Some(task) => Ok((StatusCode::OK, Json(Response::Ok(task)))),
        None => Err(Error::NotFound),
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
