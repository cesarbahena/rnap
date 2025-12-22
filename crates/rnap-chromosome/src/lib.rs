//! Chromosome — internal module inside an Organelle (C4 Component level).
//!
//! A Chromosome is a code-level element inside an Organelle (e.g., classes,
//! modules). It always belongs to exactly one Organelle.
//! Maps to C4 Component in Structurizr.

use rnap_genome::GenomeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Error)]
pub enum ChromosomeError {
    #[error("name cannot be empty")]
    EmptyName,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chromosome {
    id: Uuid,
    name: String,
    description: String,
    /// The Organelle this Chromosome belongs to (required).
    /// Chromosomes are always inside an Organelle (like components inside containers).
    organelle_id: Uuid,
    genome_id: GenomeId,
}

impl Chromosome {
    pub fn new(
        id: Uuid,
        name: String,
        description: String,
        organelle_id: Uuid,
        genome_id: GenomeId,
    ) -> Result<Self, ChromosomeError> {
        if name.trim().is_empty() {
            return Err(ChromosomeError::EmptyName);
        }
        Ok(Self {
            id,
            name,
            description,
            organelle_id,
            genome_id,
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn organelle_id(&self) -> &Uuid {
        &self.organelle_id
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }
}

pub trait ChromosomeRepository: Send + Sync {
    fn save(&self, chromosome: Chromosome) -> Result<(), RepositoryError>;
    fn find_by_id(&self, id: &Uuid) -> Option<Chromosome>;
    /// Find all chromosomes belonging to a specific organelle.
    fn find_by_organelle_id(&self, organelle_id: &Uuid) -> Vec<Chromosome>;
}

#[derive(Debug, Clone, Error)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,
    #[error("save failed: {0}")]
    SaveFailed(String),
}

impl From<String> for RepositoryError {
    fn from(err: String) -> Self {
        RepositoryError::SaveFailed(err)
    }
}

pub struct InMemoryChromosomeRepository {
    chromosomes: RwLock<HashMap<Uuid, Chromosome>>,
}

impl InMemoryChromosomeRepository {
    pub fn new() -> Self {
        Self {
            chromosomes: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryChromosomeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromosomeRepository for InMemoryChromosomeRepository {
    fn save(&self, chromosome: Chromosome) -> Result<(), RepositoryError> {
        let mut chromosomes = self.chromosomes.write().map_err(|e| e.to_string())?;
        chromosomes.insert(*chromosome.id(), chromosome);
        Ok(())
    }

    fn find_by_id(&self, id: &Uuid) -> Option<Chromosome> {
        let chromosomes = self.chromosomes.read().ok()?;
        chromosomes.get(id).cloned()
    }

    fn find_by_organelle_id(&self, organelle_id: &Uuid) -> Vec<Chromosome> {
        let chromosomes = self.chromosomes.read().unwrap();
        chromosomes
            .values()
            .filter(|c| c.organelle_id() == organelle_id)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn organelle_id() -> Uuid {
        Uuid::new_v4()
    }

    #[test]
    fn chromosome_requires_non_empty_name() {
        let genome_id = GenomeId::new();
        let result = Chromosome::new(
            Uuid::new_v4(),
            "".to_string(),
            "desc".to_string(),
            organelle_id(),
            genome_id,
        );
        assert!(matches!(result, Err(ChromosomeError::EmptyName)));
    }

    #[test]
    fn chromosome_requires_whitespace_only_name() {
        let genome_id = GenomeId::new();
        let result = Chromosome::new(
            Uuid::new_v4(),
            "   ".to_string(),
            "desc".to_string(),
            organelle_id(),
            genome_id,
        );
        assert!(matches!(result, Err(ChromosomeError::EmptyName)));
    }

    #[test]
    fn chromosome_allows_empty_description() {
        let genome_id = GenomeId::new();
        let result = Chromosome::new(
            Uuid::new_v4(),
            "User Management".to_string(),
            "".to_string(),
            organelle_id(),
            genome_id,
        );
        assert!(result.is_ok());
        let chromosome = result.unwrap();
        assert_eq!(chromosome.name(), "User Management");
        assert_eq!(chromosome.description(), "");
    }

    #[test]
    fn chromosome_belongs_to_organelle() {
        let genome_id = GenomeId::new();
        let oid = organelle_id();
        let result = Chromosome::new(
            Uuid::new_v4(),
            "Order Service".to_string(),
            "Handles order processing".to_string(),
            oid,
            genome_id,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().organelle_id(), &oid);
    }

    #[test]
    fn chromosome_persists_and_retrieves() {
        let repo = InMemoryChromosomeRepository::new();
        let genome_id = GenomeId::new();
        let oid = organelle_id();
        let id = Uuid::new_v4();

        let chromosome = Chromosome::new(
            id,
            "Order Service".to_string(),
            "Handles order processing".to_string(),
            oid,
            genome_id,
        )
        .unwrap();

        repo.save(chromosome.clone()).unwrap();
        let found = repo.find_by_id(&id).unwrap();

        assert_eq!(found.name(), "Order Service");
        assert_eq!(found.description(), "Handles order processing");
        assert_eq!(found.organelle_id(), &oid);
    }

    #[test]
    fn in_memory_chromosome_repository_find_by_organelle_id() {
        let repo = InMemoryChromosomeRepository::new();
        let genome_id = GenomeId::new();
        let oid = organelle_id();

        let c1 = Chromosome::new(
            Uuid::new_v4(),
            "Module A".to_string(),
            "A".to_string(),
            oid,
            genome_id,
        )
        .unwrap();

        let c2 = Chromosome::new(
            Uuid::new_v4(),
            "Module B".to_string(),
            "B".to_string(),
            oid,
            genome_id,
        )
        .unwrap();

        // Different organelle
        let c3 = Chromosome::new(
            Uuid::new_v4(),
            "Module C".to_string(),
            "C".to_string(),
            Uuid::new_v4(),
            genome_id,
        )
        .unwrap();

        repo.save(c1).unwrap();
        repo.save(c2).unwrap();
        repo.save(c3).unwrap();

        let in_organelle = repo.find_by_organelle_id(&oid);
        assert_eq!(in_organelle.len(), 2);
    }
}