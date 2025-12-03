use chrono::{DateTime, Utc};
use rnap_genome::GenomeId;

/// The evaluation result of a Phenotype against a Phenome's Ribosomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProteinResult {
    Pass,
    Fail,
    Pending,
}

/// An evaluation of a Phenotype.
///
/// A Protein passes when ALL Ribosomes in the Phenome pass their evaluations
/// (with rRNA criteria scoped to the Gene).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Protein {
    id: uuid::Uuid,
    phenotype_id: uuid::Uuid,
    phenome_id: uuid::Uuid,
    gene_id: uuid::Uuid,
    result: ProteinResult,
    genome_id: GenomeId,
    created_at: DateTime<Utc>,
}

impl Protein {
    pub fn new(
        id: uuid::Uuid,
        phenotype_id: uuid::Uuid,
        phenome_id: uuid::Uuid,
        gene_id: uuid::Uuid,
        result: ProteinResult,
        genome_id: GenomeId,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            phenotype_id,
            phenome_id,
            gene_id,
            result,
            genome_id,
            created_at,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn phenotype_id(&self) -> &uuid::Uuid {
        &self.phenotype_id
    }

    pub fn phenome_id(&self) -> &uuid::Uuid {
        &self.phenome_id
    }

    pub fn gene_id(&self) -> &uuid::Uuid {
        &self.gene_id
    }

    pub fn result(&self) -> &ProteinResult {
        &self.result
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

pub trait ProteinRepository {
    fn save(&mut self, protein: Protein);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Protein>;
}

pub struct InMemoryProteinRepository {
    entries: std::collections::HashMap<uuid::Uuid, Protein>,
}

impl InMemoryProteinRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryProteinRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ProteinRepository for InMemoryProteinRepository {
    fn save(&mut self, protein: Protein) {
        self.entries.insert(*protein.id(), protein);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Protein> {
        self.entries.get(id)
    }
}