const MAX_LENGTH: usize = 40;

#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub(crate) struct Title {
    value: String,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TitleNewError {
    Empty,
    TooLong,
}

impl Title {
    pub(crate) fn new(value: String) -> Result<Self, TitleNewError> {
        if value.is_empty() {
            return Err(TitleNewError::Empty);
        }
        if value.len() > MAX_LENGTH {
            return Err(TitleNewError::TooLong);
        }

        Ok(Self { value })
    }
}

impl Title {
    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_new_ok() {
        let title_str = "Title";
        let title = Title::new(title_str.into());
        assert!(title.is_ok());
        assert!(title.unwrap().value == title_str);
    }

    #[test]
    fn test_title_new_empty() {
        let title_str = "";
        let title = Title::new(title_str.into());
        assert!(title.is_err());
        assert!(matches!(title.unwrap_err(), TitleNewError::Empty));
    }

    #[test]
    fn test_title_new_too_long() {
        let title_str = "a".repeat(MAX_LENGTH + 1);
        let title = Title::new(title_str);
        assert!(title.is_err());
        assert!(matches!(title.unwrap_err(), TitleNewError::TooLong));
    }
}
