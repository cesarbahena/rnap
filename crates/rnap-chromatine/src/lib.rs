#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Chromatine {
    id: uuid::Uuid,
    path: String,
    genome_id: rnap_genome::GenomeId,
}

impl Chromatine {
    pub fn new(id: uuid::Uuid, path: String, genome_id: rnap_genome::GenomeId) -> Result<Self, ChromatineError> {
        if path.trim().is_empty() {
            return Err(ChromatineError::EmptyPath);
        }
        Ok(Self { id, path, genome_id })
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
pub enum ChromatineError {
    #[error("chromatine path must not be empty")]
    EmptyPath,
}

pub trait ChromatineRepository {
    fn save(&mut self, chromatine: Chromatine);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Chromatine>;
}

pub struct InMemoryChromatineRepository {
    entries: std::collections::HashMap<uuid::Uuid, Chromatine>,
}

impl InMemoryChromatineRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryChromatineRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromatineRepository for InMemoryChromatineRepository {
    fn save(&mut self, chromatine: Chromatine) {
        self.entries.insert(*chromatine.id(), chromatine);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Chromatine> {
        self.entries.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rnap_genome::GenomeId;

    #[test]
    fn chromatine_can_be_created_with_id_path_and_genome_id() {
        let genome_id = GenomeId::new();
        let chromatine = Chromatine::new(
            uuid::Uuid::new_v4(),
            "docs/research/user-interview-2024.md".to_string(),
            genome_id,
        ).unwrap();

        assert_eq!(chromatine.path(), "docs/research/user-interview-2024.md");
        assert_eq!(chromatine.genome_id(), &genome_id);
    }

    #[test]
    fn chromatine_rejects_empty_path() {
        let genome_id = GenomeId::new();
        let result = Chromatine::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            genome_id,
        );
        assert!(matches!(result, Err(ChromatineError::EmptyPath)));
    }

    #[test]
    fn in_memory_chromatine_repo_saves_and_finds() {
        let genome_id = GenomeId::new();
        let id = uuid::Uuid::new_v4();
        let chromatine = Chromatine::new(
            id,
            "docs/research/prd.pdf".to_string(),
            genome_id,
        ).unwrap();

        let mut repo = InMemoryChromatineRepository::new();
        repo.save(chromatine);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.path(), "docs/research/prd.pdf");
    }
}