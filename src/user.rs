mod email;
mod password;

pub(crate) use email::{Email, EmailNewError};

pub(crate) struct User {
    id: String,
    email: Email,
}
