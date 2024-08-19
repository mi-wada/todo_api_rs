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

#[derive(serde::Serialize)]
pub(crate) struct Task {
    id: crate::task::Id,
    title: crate::task::Title,
    description: crate::task::Description,
    status: crate::task::Status,
    deadline: crate::task::Deadline,
}
