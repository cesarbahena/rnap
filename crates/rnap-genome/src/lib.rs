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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Genome {
    id: GenomeId,
    name: String,
}

impl Genome {
    pub fn new(id: GenomeId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &GenomeId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
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

    #[test]
    fn genome_has_id_and_name() {
        let id = GenomeId::new();
        let genome = Genome::new(id, "acme-corp".to_string());
        assert_eq!(genome.name(), "acme-corp");
        assert_eq!(genome.id(), &id);
    }
}
