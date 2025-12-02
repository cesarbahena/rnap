use rnap_genome::GenomeId;

/// A named group of Ribosomes (QA environment/profile).
///
/// A Genome has one or more Phenomes. Examples: "Staging checks", "Production checks".
/// A Protein references a Phenome to know which Ribosomes to run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Phenome {
    id: uuid::Uuid,
    name: String,
    genome_id: GenomeId,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum PhenomeError {
    #[error("phenome name must not be empty")]
    EmptyName,
}

impl Phenome {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        genome_id: GenomeId,
    ) -> Result<Self, PhenomeError> {
        if name.trim().is_empty() {
            return Err(PhenomeError::EmptyName);
        }
        Ok(Self { id, name, genome_id })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }
}

pub trait PhenomeRepository {
    fn save(&mut self, phenome: Phenome);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Phenome>;
}

pub struct InMemoryPhenomeRepository {
    entries: std::collections::HashMap<uuid::Uuid, Phenome>,
}

impl InMemoryPhenomeRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryPhenomeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl PhenomeRepository for InMemoryPhenomeRepository {
    fn save(&mut self, phenome: Phenome) {
        self.entries.insert(*phenome.id(), phenome);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Phenome> {
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
    fn phenome_can_be_created_with_name() {
        let gid = genome_id();
        let phenome = Phenome::new(
            uuid::Uuid::new_v4(),
            "Staging checks".to_string(),
            gid,
        )
        .unwrap();

        assert_eq!(phenome.name(), "Staging checks");
        assert_eq!(phenome.genome_id(), &gid);
    }

    #[test]
    fn phenome_rejects_empty_name() {
        let result = Phenome::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            genome_id(),
        );
        assert_eq!(result, Err(PhenomeError::EmptyName));
    }

    #[test]
    fn in_memory_phenome_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let phenome = Phenome::new(
            id,
            "Production checks".to_string(),
            genome_id(),
        )
        .unwrap();

        let mut repo = InMemoryPhenomeRepository::new();
        repo.save(phenome);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.name(), "Production checks");
    }
}