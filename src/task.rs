mod id;
pub(crate) use id::Id;

mod title;
pub(crate) use title::{Title, TitleNewError};

mod description;
pub(crate) use description::{Description, DescriptionNewError};

mod status;
pub(crate) use status::{Status, StatusNewError};

mod deadline;
pub(crate) use deadline::{Deadline, DeadlineNewError};

use crate::user;

#[derive(serde::Serialize)]
pub(crate) struct Task {
    id: Id,
    user_id: user::Id,
    title: Title,
    description: Description,
    status: Status,
    deadline: Deadline,
}

impl Task {
    pub(crate) fn new(
        id: Id,
        user_id: user::Id,
        title: Title,
        description: Description,
        status: Status,
        deadline: Deadline,
    ) -> Self {
        Self {
            id,
            user_id,
            title,
            description,
            status,
            deadline,
        }
    }
}
