#[derive(Debug, PartialEq, serde::Serialize, Clone, Copy)]
pub(crate) enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq)]
pub(crate) enum StatusNewError {
    Unknown,
}

impl TryFrom<&str> for Status {
    type Error = StatusNewError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ToDo" => Ok(Self::ToDo),
            "InProgress" => Ok(Self::InProgress),
            "Done" => Ok(Self::Done),
            _ => Err(Self::Error::Unknown),
        }
    }
}

impl From<Status> for &str {
    fn from(status: Status) -> &'static str {
        match status {
            Status::ToDo => "ToDo",
            Status::InProgress => "InProgress",
            Status::Done => "Done",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_try_from_ok() {
        let status_str = "ToDo";
        let status = Status::try_from(status_str);
        assert!(status.is_ok());
        assert!(status.unwrap() == Status::ToDo);
    }

    #[test]
    fn test_status_try_from_unknown() {
        let status_str = "Unknown";
        let status = Status::try_from(status_str);
        assert!(status.is_err());
        assert!(matches!(status.unwrap_err(), StatusNewError::Unknown));
    }
}
