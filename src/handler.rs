mod create_user;
pub(crate) use create_user::create_user;

mod get_healthz;
pub(crate) use get_healthz::get_healthz;

mod login;
pub(crate) use login::login;

mod create_task;
pub(crate) use create_task::create_task;

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
