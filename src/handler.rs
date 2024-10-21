pub(crate) mod healthz;
pub(crate) mod login;
pub(crate) mod tasks;
pub(crate) mod users;

pub mod auth_middleware;

#[derive(serde::Serialize)]
pub(crate) struct InternalServerError {
    code: InternalServerErrorCode,
    message: String,
}

impl Default for InternalServerError {
    fn default() -> Self {
        Self {
            code: InternalServerErrorCode::InternalServerError,
            message: "Internal server error".into(),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) enum InternalServerErrorCode {
    InternalServerError,
}
