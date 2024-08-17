static MIN_PASSWORD_LENGTH: usize = 8;
static MAX_PASSWORD_LENGTH: usize = 255;

#[derive(Debug, PartialEq)]
pub(crate) struct Password {
    password: String,
}

#[derive(Debug, PartialEq)]
pub(crate) enum PasswordNewError {
    TooShort,
    TooLong,
}

impl Password {
    pub(crate) fn new(password: String) -> Result<Self, PasswordNewError> {
        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(PasswordNewError::TooShort);
        }
        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(PasswordNewError::TooLong);
        }

        Ok(Self { password })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_new_ok() {
        let password_str = "password";
        let password = Password::new(password_str.into());
        assert!(password.is_ok());
        assert!(password.unwrap().password == password_str);
    }

    #[test]
    fn test_password_new_too_short() {
        let password_str = "a".repeat(MIN_PASSWORD_LENGTH - 1);
        let password = Password::new(password_str);
        assert!(password.is_err());
        assert!(matches!(password.unwrap_err(), PasswordNewError::TooShort));
    }

    #[test]
    fn test_password_new_too_long() {
        let password_str = "a".repeat(MAX_PASSWORD_LENGTH + 1);
        let password = Password::new(password_str);
        assert!(password.is_err());
        assert!(matches!(password.unwrap_err(), PasswordNewError::TooLong));
    }
}