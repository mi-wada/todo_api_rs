static MAX_EMAIL_LENGTH: usize = 255;
// https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
static EMAIL_REGEX: &str = r"^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$";

#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub(crate) struct Email {
    email: String,
}

#[derive(Debug, PartialEq)]
pub(crate) enum EmailNewError {
    Empty,
    TooLong,
    WrongFormat,
}

impl Email {
    pub(crate) fn new(email: String) -> Result<Self, EmailNewError> {
        if email.is_empty() {
            return Err(EmailNewError::Empty);
        }
        if email.len() > MAX_EMAIL_LENGTH {
            return Err(EmailNewError::TooLong);
        }
        if !regex::Regex::new(EMAIL_REGEX).unwrap().is_match(&email) {
            return Err(EmailNewError::WrongFormat);
        }

        Ok(Self { email })
    }
}

impl Email {
    pub(crate) fn value(&self) -> &str {
        &self.email
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_new_ok() {
        let email_str = "user@example.com";
        let email = Email::new(email_str.into());
        assert!(email.is_ok());
        assert!(email.unwrap().email == email_str);
    }

    #[test]
    fn test_email_new_empty() {
        let email_str = "";
        let email = Email::new(email_str.into());
        assert!(email.is_err());
        assert!(matches!(email.unwrap_err(), EmailNewError::Empty));
    }

    #[test]
    fn test_email_new_too_long() {
        let email_str = "a".repeat(MAX_EMAIL_LENGTH + 1);
        let email = Email::new(email_str);
        assert!(email.is_err());
        assert!(matches!(email.unwrap_err(), EmailNewError::TooLong));
    }

    #[test]
    fn test_email_new_wrong_format() {
        for email_str in ["user", "user@", "user@.", "user@.com", "user@exa mple.com"] {
            let email = Email::new(email_str.into());
            assert!(email.is_err());
            assert!(matches!(email.unwrap_err(), EmailNewError::WrongFormat));
        }
    }
}
