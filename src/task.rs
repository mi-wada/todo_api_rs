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

#[derive(Debug, PartialEq)]
pub(crate) enum TaskNewError {
    TitleEmpty,
    TitleTooLong,
    DescriptionTooLong,
    StatusUnknown,
    DeadlineWrongFormat,
}

impl Task {
    pub(crate) fn _new(
        user_id: String,
        title: String,
        description: Option<String>,
        status: String,
        deadline: Option<String>,
    ) -> Result<Self, TaskNewError> {
        Ok(Self {
            id: crate::task::Id::new(),
            user_id: user::Id::restore(user_id),
            title: Title::new(title)?,
            description: description.map(Description::new).transpose()?,
            status: Status::try_from(status.as_str())?,
            deadline: deadline.map(Deadline::new).transpose()?,
        })
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

impl From<crate::task::TitleNewError> for TaskNewError {
    fn from(err: crate::task::TitleNewError) -> Self {
        match err {
            crate::task::TitleNewError::Empty => Self::TitleEmpty,
            crate::task::TitleNewError::TooLong => Self::TitleTooLong,
        }
    }
}

impl From<crate::task::DescriptionNewError> for TaskNewError {
    fn from(err: crate::task::DescriptionNewError) -> Self {
        match err {
            crate::task::DescriptionNewError::TooLong => Self::DescriptionTooLong,
        }
    }
}

impl From<crate::task::StatusNewError> for TaskNewError {
    fn from(err: crate::task::StatusNewError) -> Self {
        match err {
            crate::task::StatusNewError::Unknown => Self::StatusUnknown,
        }
    }
}

impl From<crate::task::DeadlineNewError> for TaskNewError {
    fn from(err: crate::task::DeadlineNewError) -> Self {
        match err {
            crate::task::DeadlineNewError::WrongFormat => Self::DeadlineWrongFormat,
        }
    }
}

impl Task {
    pub(crate) fn id(&self) -> &Id {
        &self.id
    }

    pub(crate) fn user_id(&self) -> &user::Id {
        &self.user_id
    }

    pub(crate) fn title(&self) -> &Title {
        &self.title
    }

    pub(crate) fn description(&self) -> Option<&Description> {
        self.description.as_ref()
    }

    pub(crate) fn status(&self) -> &Status {
        &self.status
    }

    pub(crate) fn deadline(&self) -> Option<&Deadline> {
        self.deadline.as_ref()
    }
}
