use crate::user::{self, PasswordNewError, User, UserNewError};

use super::AppContext;

pub(crate) async fn create_user(payload: Payload, context: AppContext) -> Result<User, Error> {
    let user = user::User::new(payload.email.ok_or(Error::EmailEmpty)?)?;

    let hashed_password =
        user::Password::new(payload.password.ok_or(Error::PasswordEmpty)?)?.hashed();

    match sqlx::query(
        r#"
INSERT INTO users (id, email, password)
VALUES ($1::uuid, $2, $3)
        "#,
    )
    .bind(user.id().value())
    .bind(user.email().value())
    .bind(hashed_password)
    .execute(&context.db_pool)
    .await
    {
        Ok(_) => {}
        Err(sqlx::Error::Database(db_error)) if db_error.is_unique_violation() => {
            return Err(Error::EmailTaken);
        }
        Err(_) => {
            return Err(Error::DatabaseError);
        }
    }

    Ok(user)
}

#[derive(serde::Deserialize)]
pub(crate) struct Payload {
    email: Option<String>,
    password: Option<String>,
}

#[derive(serde::Serialize, Debug, PartialEq)]
pub(crate) enum Error {
    EmailEmpty,
    EmailTooLong,
    EmailWrongFormat,
    EmailTaken,
    PasswordEmpty,
    PasswordTooShort,
    PasswordTooLong,
    DatabaseError,
}

impl From<UserNewError> for Error {
    fn from(err: UserNewError) -> Self {
        match err {
            UserNewError::EmailEmpty => Self::EmailEmpty,
            UserNewError::EmailTooLong => Self::EmailTooLong,
            UserNewError::EmailWrongFormat => Self::EmailWrongFormat,
        }
    }
}

impl From<PasswordNewError> for Error {
    fn from(err: PasswordNewError) -> Self {
        match err {
            PasswordNewError::TooShort => Self::PasswordTooShort,
            PasswordNewError::TooLong => Self::PasswordTooLong,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper;

    use super::*;

    #[tokio::test]
    async fn ok_create_user() {
        let email = test_helper::unique_email();
        let payload = Payload {
            email: Some(email.clone()),
            password: Some("password".into()),
        };

        let user = create_user(payload, test_helper::context().await)
            .await
            .unwrap();

        assert!(!user.id().value().is_empty());
        assert_eq!(user.email().value(), email);
        // TODO: Check that the user is saved to DB
    }

    #[tokio::test]
    async fn err_create_user() {
        struct Test {
            args: Payload,
            expected: Error,
        }

        let valid_email = Some(test_helper::unique_email());
        let valid_password = Some("password".into());

        let taken_email = {
            create_user(
                Payload {
                    email: Some(test_helper::unique_email()),
                    password: valid_password.clone(),
                },
                test_helper::context().await,
            )
            .await
            .unwrap()
            .email()
            .value()
            .to_string()
        };

        for test in [
            Test {
                args: Payload {
                    email: None,
                    password: None,
                },
                expected: Error::EmailEmpty,
            },
            Test {
                args: Payload {
                    email: None,
                    password: valid_password.clone(),
                },
                expected: Error::EmailEmpty,
            },
            Test {
                args: Payload {
                    email: Some("".into()),
                    password: valid_password.clone(),
                },
                expected: Error::EmailEmpty,
            },
            Test {
                args: Payload {
                    email: Some("a".repeat(user::email::MAX_LEN - 12 + 1) + "@example.com"),
                    password: valid_password.clone(),
                },
                expected: Error::EmailTooLong,
            },
            Test {
                args: Payload {
                    email: Some("invalid_email".into()),
                    password: valid_password.clone(),
                },
                expected: Error::EmailWrongFormat,
            },
            Test {
                args: Payload {
                    email: Some(taken_email.clone()),
                    password: valid_password.clone(),
                },
                expected: Error::EmailTaken,
            },
            Test {
                args: Payload {
                    email: valid_email.clone(),
                    password: None,
                },
                expected: Error::PasswordEmpty,
            },
            Test {
                args: Payload {
                    email: valid_email.clone(),
                    password: Some("".into()),
                },
                expected: Error::PasswordTooShort,
            },
            Test {
                args: Payload {
                    email: valid_email.clone(),
                    password: Some("a".repeat(user::password::MIN_LEN - 1)),
                },
                expected: Error::PasswordTooShort,
            },
            Test {
                args: Payload {
                    email: valid_email.clone(),
                    password: Some("a".repeat(user::password::MAX_LEN + 1)),
                },
                expected: Error::PasswordTooLong,
            },
            Test {
                args: Payload {
                    email: valid_email.clone(),
                    password: None,
                },
                expected: Error::PasswordEmpty,
            },
        ] {
            let err = create_user(test.args, test_helper::context().await)
                .await
                .unwrap_err();

            assert_eq!(err, test.expected);
            // TODO: Check that the user is not saved to DB
        }
    }
}
