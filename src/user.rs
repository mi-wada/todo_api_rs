mod email;
pub(crate) use email::{Email, EmailNewError};

mod password;
pub(crate) use password::{Password, PasswordNewError};

mod id;

pub(crate) struct User {
    id: String,
    email: Email,
}
