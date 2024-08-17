use uuid::Uuid;

#[derive(serde::Serialize)]
#[serde(transparent)]
pub(crate) struct Id {
    id: String,
}

impl Id {
    pub(crate) fn new() -> Self {
        Self {
            id: Uuid::now_v7().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_new() {
        let id = Id::new();
        assert!(!id.id.is_empty());
    }
}
