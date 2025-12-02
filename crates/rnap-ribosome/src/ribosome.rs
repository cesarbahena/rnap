use rnap_genome::GenomeId;
use serde_json::Value;

/// A reusable QA pipeline (linters, CI/CD, tests, LLM code review).
///
/// Named config like "CI", "Lint", "Security Scan".
/// Belongs to a Phenome and scoped to a Genome.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Ribosome {
    id: uuid::Uuid,
    name: String,
    config: Value,
    phenome_id: uuid::Uuid,
    genome_id: GenomeId,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RibosomeError {
    #[error("ribosome name must not be empty")]
    EmptyName,
}

impl Ribosome {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        config: Value,
        phenome_id: uuid::Uuid,
        genome_id: GenomeId,
    ) -> Result<Self, RibosomeError> {
        if name.trim().is_empty() {
            return Err(RibosomeError::EmptyName);
        }
        Ok(Self {
            id,
            name,
            config,
            phenome_id,
            genome_id,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn config(&self) -> &Value {
        &self.config
    }

    pub fn phenome_id(&self) -> &uuid::Uuid {
        &self.phenome_id
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }
}

pub trait RibosomeRepository {
    fn save(&mut self, ribosome: Ribosome);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Ribosome>;
}

pub struct InMemoryRibosomeRepository {
    entries: std::collections::HashMap<uuid::Uuid, Ribosome>,
}

impl InMemoryRibosomeRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryRibosomeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl RibosomeRepository for InMemoryRibosomeRepository {
    fn save(&mut self, ribosome: Ribosome) {
        self.entries.insert(*ribosome.id(), ribosome);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Ribosome> {
        self.entries.get(id)
    }
}