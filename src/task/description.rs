const MAX_LENGTH: usize = 1_000;

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
#[serde(transparent)]
pub(crate) struct Description {
    value: String,
}

#[derive(Debug, PartialEq)]
pub(crate) enum DescriptionNewError {
    TooLong,
}

impl Description {
    pub(crate) fn new(value: String) -> Result<Self, DescriptionNewError> {
        if value.len() > MAX_LENGTH {
            return Err(DescriptionNewError::TooLong);
        }

        Ok(Self { value })
    }
}

impl Description {
    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description_new_ok() {
        let description_str = "Description";
        let description = Description::new(description_str.into());
        assert!(description.is_ok());
        assert!(description.unwrap().value == description_str);
    }

    #[test]
    fn test_description_new_too_long() {
        let description_str = "a".repeat(MAX_LENGTH + 1);
        let description = Description::new(description_str);
        assert!(description.is_err());
        assert!(matches!(
            description.unwrap_err(),
            DescriptionNewError::TooLong
        ));
    }
}
