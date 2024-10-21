use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{handler::InternalServerError, task::Task, usecase::AppContext, user::User};

use sqlx::Row;

pub(crate) async fn list(
    State(context): State<AppContext>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
    let tasks = sqlx::query(
        r#"
SELECT id, user_id, title, description, status, deadline
FROM tasks
WHERE user_id = $1::uuid
        "#,
    )
    .bind(user.id().value())
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
    .fetch_all(&context.db_pool)
    .await
    .map_err(|_| Error::DatabaseError)?;

    Ok((StatusCode::OK, Json(Response::Ok(tasks))))
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum Response {
    Ok(Vec<Task>),
    InternalServerError(InternalServerError),
}

#[derive(serde::Serialize, Debug)]
pub(crate) enum Error {
    DatabaseError,
}

impl IntoResponse for Error {
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
