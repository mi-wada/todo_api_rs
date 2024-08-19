#[derive(Debug, PartialEq, Clone, serde::Serialize)]
#[serde(transparent)]
pub(crate) struct Deadline {
    value: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum DeadlineNewError {
    WrongFormat,
}

impl Deadline {
    pub(crate) fn new(value: String) -> Result<Self, DeadlineNewError> {
        let value = chrono::DateTime::parse_from_rfc3339(&value)
            .map_err(|_| DeadlineNewError::WrongFormat)?;

        Ok(Self {
            value: value.into(),
        })
    }
}

impl Deadline {
    pub(crate) fn value(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deadline_new_ok() {
        for deadline_str in &[
            // https://datatracker.ietf.org/doc/html/rfc3339#section-5.8
            "1985-04-12T23:20:50.52Z",
            "1996-12-19T16:39:57-08:00",
            "1990-12-31T23:59:60Z",
            "1990-12-31T15:59:60-08:00",
            "1937-01-01T12:00:27.87+00:20",
        ] {
            let deadline = Deadline::new((*deadline_str).into());

            assert!(deadline.is_ok());
            assert_eq!(
                deadline.unwrap().value,
                chrono::DateTime::parse_from_rfc3339(deadline_str).unwrap()
            );
        }
    }

    #[test]
    fn test_deadline_new_invalid_format() {
        for deadline_str in &[
            "1985-04-12T23:20:50.52", // missing timezone
            "2024-039",               // ISO8601-compatible format
            "invalid",                // invalid format
            "",                       // empty
        ] {
            let deadline = Deadline::new((*deadline_str).into());

            assert!(deadline.is_err());
            assert!(matches!(
                deadline.unwrap_err(),
                DeadlineNewError::WrongFormat
            ));
        }
    }
}
