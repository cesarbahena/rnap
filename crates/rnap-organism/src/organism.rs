use rnap_genome::GenomeId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OrganismKind {
    Human,
    Team,
    Service,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Organism {
    id: uuid::Uuid,
    name: String,
    kind: OrganismKind,
    description: String,
    genome_id: GenomeId,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum OrganismError {
    #[error("organism name must not be empty")]
    EmptyName,
}

impl Organism {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        kind: OrganismKind,
        description: String,
        genome_id: GenomeId,
    ) -> Result<Self, OrganismError> {
        if name.trim().is_empty() {
            return Err(OrganismError::EmptyName);
        }
        Ok(Self { id, name, kind, description, genome_id })
    }
    pub fn id(&self) -> &uuid::Uuid { &self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn kind(&self) -> &OrganismKind { &self.kind }
    pub fn description(&self) -> &str { &self.description }
    pub fn genome_id(&self) -> &GenomeId { &self.genome_id }
}

pub trait OrganismRepository {
    fn save(&mut self, organism: Organism);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Organism>;
}

pub struct InMemoryOrganismRepository {
    entries: std::collections::HashMap<uuid::Uuid, Organism>,
}

impl InMemoryOrganismRepository {
    pub fn new() -> Self { Self { entries: std::collections::HashMap::new() } }
}
impl Default for InMemoryOrganismRepository { fn default() -> Self { Self::new() } }

impl OrganismRepository for InMemoryOrganismRepository {
    fn save(&mut self, organism: Organism) { self.entries.insert(*organism.id(), organism); }
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Organism> { self.entries.get(id) }
}

#[cfg(test)] mod tests {
    use super::*;
    fn genome_id() -> GenomeId { GenomeId::new() }
    
    #[test] fn organism_can_be_created() {
        let gid = genome_id();
        let org = Organism::new(uuid::Uuid::new_v4(), "Platform Team".to_string(), OrganismKind::Team, "Handles infra".to_string(), gid).unwrap();
        assert_eq!(org.name(), "Platform Team");
    }
    #[test] fn organism_rejects_empty_name() {
        let result = Organism::new(uuid::Uuid::new_v4(), "   ".to_string(), OrganismKind::Human, "desc".to_string(), genome_id());
        assert_eq!(result, Err(OrganismError::EmptyName));
    }
}
