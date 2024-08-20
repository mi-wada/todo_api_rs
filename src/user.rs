mod email;
pub(crate) use email::{Email, EmailNewError};

mod password;
pub(crate) use password::{Password, PasswordNewError};

mod id;
pub(crate) use id::Id;

pub(crate) mod access_token;

#[derive(serde::Serialize, Clone)]
pub(crate) struct User {
    id: crate::user::Id,
    email: crate::user::Email,
}

impl User {
    pub(crate) fn new(id: crate::user::Id, email: crate::user::Email) -> Self {
        Self { id, email }
    }
}

impl User {
    pub(crate) fn id(&self) -> &crate::user::Id {
        &self.id
    }

    pub(crate) fn email(&self) -> &crate::user::Email {
        &self.email
    }
}
