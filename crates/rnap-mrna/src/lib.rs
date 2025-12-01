use chrono::{DateTime, Utc};
use rnap_genome::GenomeId;

/// A frozen snapshot of pending mutation IDs for a gene.
///
/// mRNA captures what an agent sees at implementation time.
/// Since mutations are immutable, only IDs are needed (not deep copies).
/// Version number is incremented on each freeze.
/// Scoped to a genome for tenant isolation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Mrna {
    id: uuid::Uuid,
    gene_id: uuid::Uuid,
    version: u32,
    mutation_ids: Vec<uuid::Uuid>,
    genome_id: GenomeId,
    created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum MrnaError {
    #[error("mRNA must have at least one mutation")]
    EmptyMutations,
}

impl Mrna {
    pub fn new(
        id: uuid::Uuid,
        gene_id: uuid::Uuid,
        version: u32,
        mutation_ids: Vec<uuid::Uuid>,
        genome_id: GenomeId,
        created_at: DateTime<Utc>,
    ) -> Result<Self, MrnaError> {
        if mutation_ids.is_empty() {
            return Err(MrnaError::EmptyMutations);
        }
        Ok(Self {
            id,
            gene_id,
            version,
            mutation_ids,
            genome_id,
            created_at,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn gene_id(&self) -> &uuid::Uuid {
        &self.gene_id
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn mutation_ids(&self) -> &[uuid::Uuid] {
        &self.mutation_ids
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

pub trait MrnaRepository {
    fn save(&mut self, mrna: Mrna);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Mrna>;
}

pub struct InMemoryMrnaRepository {
    entries: std::collections::HashMap<uuid::Uuid, Mrna>,
}

impl InMemoryMrnaRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryMrnaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MrnaRepository for InMemoryMrnaRepository {
    fn save(&mut self, mrna: Mrna) {
        self.entries.insert(*mrna.id(), mrna);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Mrna> {
        self.entries.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn genome_id() -> GenomeId {
        GenomeId::new()
    }

    #[test]
    fn mrna_can_be_created_with_mutations() {
        let gene_id = uuid::Uuid::new_v4();
        let mutation_id = uuid::Uuid::new_v4();
        let now = Utc::now();
        let gid = genome_id();

        let mrna = Mrna::new(
            uuid::Uuid::new_v4(),
            gene_id,
            1,
            vec![mutation_id],
            gid,
            now,
        )
        .unwrap();

        assert_eq!(mrna.gene_id(), &gene_id);
        assert_eq!(mrna.version(), 1);
        assert_eq!(mrna.mutation_ids(), &[mutation_id]);
        assert_eq!(mrna.genome_id(), &gid);
    }

    #[test]
    fn mrna_rejects_empty_mutations() {
        let result = Mrna::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            1,
            vec![],
            genome_id(),
            Utc::now(),
        );
        assert_eq!(result, Err(MrnaError::EmptyMutations));
    }

    #[test]
    fn in_memory_mrna_repo_saves_and_finds() {
        let gid = genome_id();
        let id = uuid::Uuid::new_v4();
        let mutation_id = uuid::Uuid::new_v4();
        let mrna = Mrna::new(
            id,
            uuid::Uuid::new_v4(),
            1,
            vec![mutation_id],
            gid,
            Utc::now(),
        )
        .unwrap();

        let mut repo = InMemoryMrnaRepository::new();
        repo.save(mrna);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.version(), 1);
        assert_eq!(found.mutation_ids(), &[mutation_id]);
    }
}