use uuid::Uuid;

#[derive(serde::Serialize, Clone)]
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

    pub(crate) fn restore(id: String) -> Self {
        Self { id }
    }
}

impl Id {
    pub(crate) fn value(&self) -> &str {
        &self.id
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
