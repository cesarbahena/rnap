#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Dna {
    id: uuid::Uuid,
    path: String,
    genome_id: rnap_genome::GenomeId,
}

impl Dna {
    pub fn new(id: uuid::Uuid, path: String, genome_id: rnap_genome::GenomeId) -> Result<Self, DnaError> {
        if path.trim().is_empty() {
            return Err(DnaError::EmptyPath);
        }
        Ok(Self {
            id,
            path,
            genome_id,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn genome_id(&self) -> &rnap_genome::GenomeId {
        &self.genome_id
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DnaError {
    #[error("DNA path must not be empty")]
    EmptyPath,
}

pub trait DnaRepository {
    fn save(&mut self, dna: Dna);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Dna>;
}

pub struct InMemoryDnaRepository {
    entries: std::collections::HashMap<uuid::Uuid, Dna>,
}

impl InMemoryDnaRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryDnaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl DnaRepository for InMemoryDnaRepository {
    fn save(&mut self, dna: Dna) {
        self.entries.insert(*dna.id(), dna);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Dna> {
        self.entries.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rnap_genome::GenomeId;

    #[test]
    fn dna_can_be_created_with_id_path_and_genome_id() {
        let genome_id = GenomeId::new();
        let dna = Dna::new(
            uuid::Uuid::new_v4(),
            "dna/2024/q1/req-001.dna".to_string(),
            genome_id,
        ).unwrap();

        assert_eq!(
            dna.path(),
            "dna/2024/q1/req-001.dna"
        );
        assert_eq!(dna.genome_id(), &genome_id);
    }

    #[test]
    fn dna_rejects_empty_path() {
        let genome_id = GenomeId::new();
        let result = Dna::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            genome_id,
        );
        assert!(matches!(result, Err(DnaError::EmptyPath)));
    }

    #[test]
    fn in_memory_dna_repo_saves_and_finds_dna() {
        let genome_id = GenomeId::new();
        let id = uuid::Uuid::new_v4();
        let dna = Dna::new(
            id,
            "dna/2024/q1/req-001.dna".to_string(),
            genome_id,
        ).unwrap();

        let mut repo = InMemoryDnaRepository::new();
        repo.save(dna);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.path(), "dna/2024/q1/req-001.dna");
    }
}