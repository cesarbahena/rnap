use rnap_genome::GenomeId;

/// Acceptance criteria scoped per Gene per Ribosome.
///
/// rRNA parameterizes a reusable Ribosome for a specific Gene's evaluation context.
/// Example: "coverage >= 90%"
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Rrna {
    id: uuid::Uuid,
    ribosome_id: uuid::Uuid,
    gene_id: uuid::Uuid,
    criteria: String,
    genome_id: GenomeId,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RrnaError {
    #[error("rRNA criteria must not be empty")]
    EmptyCriteria,
}

impl Rrna {
    pub fn new(
        id: uuid::Uuid,
        ribosome_id: uuid::Uuid,
        gene_id: uuid::Uuid,
        criteria: String,
        genome_id: GenomeId,
    ) -> Result<Self, RrnaError> {
        if criteria.trim().is_empty() {
            return Err(RrnaError::EmptyCriteria);
        }
        Ok(Self {
            id,
            ribosome_id,
            gene_id,
            criteria,
            genome_id,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn ribosome_id(&self) -> &uuid::Uuid {
        &self.ribosome_id
    }

    pub fn gene_id(&self) -> &uuid::Uuid {
        &self.gene_id
    }

    pub fn criteria(&self) -> &str {
        &self.criteria
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }
}

pub trait RrnaRepository {
    fn save(&mut self, rrna: Rrna);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Rrna>;
}

pub struct InMemoryRrnaRepository {
    entries: std::collections::HashMap<uuid::Uuid, Rrna>,
}

impl InMemoryRrnaRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryRrnaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl RrnaRepository for InMemoryRrnaRepository {
    fn save(&mut self, rrna: Rrna) {
        self.entries.insert(*rrna.id(), rrna);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Rrna> {
        self.entries.get(id)
    }
}