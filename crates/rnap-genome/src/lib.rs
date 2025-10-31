#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct GenomeId(uuid::Uuid);

impl GenomeId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl std::fmt::Display for GenomeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<uuid::Uuid> for GenomeId {
    fn from(id: uuid::Uuid) -> Self {
        Self(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genome_id_displays_as_hyphenated_uuid() {
        let id = uuid::Uuid::new_v4();
        let genome_id = GenomeId::from(id);
        assert_eq!(genome_id.to_string(), id.to_string());
    }
}
