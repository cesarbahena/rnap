use rnap_genome::GenomeId;

/// A named group of Ribosomes (QA environment/profile).
///
/// A Genome has one or more Phenotypes. Examples: "Staging checks", "Production checks".
/// A Protein references a Phenotype to know which Ribosomes to run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Phenotype {
    id: uuid::Uuid,
    name: String,
    genome_id: GenomeId,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum PhenotypeError {
    #[error("phenotype name must not be empty")]
    EmptyName,
}

impl Phenotype {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        genome_id: GenomeId,
    ) -> Result<Self, PhenotypeError> {
        if name.trim().is_empty() {
            return Err(PhenotypeError::EmptyName);
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

pub trait PhenotypeRepository {
    fn save(&mut self, phenotype: Phenotype);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Phenotype>;
}

pub struct InMemoryPhenotypeRepository {
    entries: std::collections::HashMap<uuid::Uuid, Phenotype>,
}

impl InMemoryPhenotypeRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryPhenotypeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl PhenotypeRepository for InMemoryPhenotypeRepository {
    fn save(&mut self, phenotype: Phenotype) {
        self.entries.insert(*phenotype.id(), phenotype);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Phenotype> {
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
    fn phenotype_can_be_created_with_name() {
        let gid = genome_id();
        let phenotype = Phenotype::new(
            uuid::Uuid::new_v4(),
            "Staging checks".to_string(),
            gid,
        )
        .unwrap();

        assert_eq!(phenotype.name(), "Staging checks");
        assert_eq!(phenotype.genome_id(), &gid);
    }

    #[test]
    fn phenotype_rejects_empty_name() {
        let result = Phenotype::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            genome_id(),
        );
        assert_eq!(result, Err(PhenotypeError::EmptyName));
    }

    #[test]
    fn in_memory_phenotype_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let phenotype = Phenotype::new(
            id,
            "Production checks".to_string(),
            genome_id(),
        )
        .unwrap();

        let mut repo = InMemoryPhenotypeRepository::new();
        repo.save(phenotype);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.name(), "Production checks");
    }
}
