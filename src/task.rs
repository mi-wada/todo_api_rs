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
    description: Option<Description>,
    status: Status,
    deadline: Option<Deadline>,
}

impl Task {
    pub(crate) fn new(
        id: Id,
        user_id: user::Id,
        title: Title,
        description: Option<Description>,
        status: Status,
        deadline: Option<Deadline>,
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

    pub(crate) fn restore(
        id: String,
        user_id: String,
        title: String,
        description: Option<String>,
        status: String,
        deadline: Option<String>,
    ) -> Self {
        Self {
            id: Id::restore(id),
            user_id: user::Id::restore(user_id),
            title: Title::restore(title),
            description: description.map(Description::restore),
            status: Status::try_from(status.as_str()).unwrap(),
            deadline: deadline.map(Deadline::restore),
        }
    }
}
