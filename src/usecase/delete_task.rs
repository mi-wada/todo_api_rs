use crate::sql;

use super::AppContext;

pub(crate) async fn delete_task(payload: Payload, context: AppContext) -> Result<(), Error> {
    match sql::delete_task_by_id::query(&payload.user_id, &payload.task_id, &context.db_pool).await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::Database),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct Payload {
    user_id: String,
    task_id: String,
}

impl Payload {
    pub(crate) fn new(user_id: String, task_id: String) -> Self {
        Self { user_id, task_id }
    }
}

#[derive(serde::Serialize, Debug)]
pub(crate) enum Error {
    Database,
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Impl test
}
