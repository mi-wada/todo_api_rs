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

#[derive(Debug, PartialEq)]
pub(crate) enum UserNewError {
    EmailEmpty,
    EmailTooLong,
    EmailWrongFormat,
}

impl From<EmailNewError> for UserNewError {
    fn from(err: EmailNewError) -> Self {
        match err {
            EmailNewError::Empty => Self::EmailEmpty,
            EmailNewError::TooLong => Self::EmailTooLong,
            EmailNewError::WrongFormat => Self::EmailWrongFormat,
        }
    }
}

impl User {
    pub(crate) fn new(email: String) -> Result<Self, UserNewError> {
        let id = crate::user::Id::new();
        let email = crate::user::Email::new(email)?;

        Ok(Self { id, email })
    }

    pub(crate) fn restore(id: String, email: String) -> User {
        Self {
            id: crate::user::Id::restore(id),
            email: crate::user::Email::restore(email),
        }
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
