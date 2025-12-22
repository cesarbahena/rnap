use rnap_genome::GenomeId;

/// The type of node at the source end of a channel edge.
///
/// Maps to C4 model levels:
/// - Organism = Person (external actors)
/// - Cell = Software System (top-level owned systems)
/// - Organelle = Container (runtime units inside a Cell)
/// - Chromosome = Component (internal modules inside an Organelle)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SourceType {
    Organism,
    Cell,
    Organelle,
    Chromosome,
}

/// The type of node at the target end of a channel edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TargetType {
    Organism,
    Cell,
    Organelle,
    Chromosome,
}

/// The kind of relationship between two domain nodes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RelationshipType {
    DeliversTo,
    Uses,
    DependsOn,
    Calls,
    Contains,
    AttachTo,
}

impl RelationshipType {
    pub fn label(&self) -> &'static str {
        match self {
            RelationshipType::DeliversTo => "delivers to",
            RelationshipType::Uses => "uses",
            RelationshipType::DependsOn => "depends on",
            RelationshipType::Calls => "calls",
            RelationshipType::Contains => "contains",
            RelationshipType::AttachTo => "attached to",
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Channel {
    id: uuid::Uuid,
    source_id: uuid::Uuid,
    source_type: SourceType,
    target_id: uuid::Uuid,
    target_type: TargetType,
    relationship_type: RelationshipType,
    description: String,
    genome_id: GenomeId,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ChannelError {
    #[error("source and target must be different")]
    SameSourceAndTarget,
}

impl Channel {
    pub fn new(
        id: uuid::Uuid,
        source_id: uuid::Uuid,
        source_type: SourceType,
        target_id: uuid::Uuid,
        target_type: TargetType,
        relationship_type: RelationshipType,
        description: String,
        genome_id: GenomeId,
    ) -> Result<Self, ChannelError> {
        if source_id == target_id {
            return Err(ChannelError::SameSourceAndTarget);
        }
        Ok(Self {
            id, source_id, source_type, target_id, target_type, relationship_type, description, genome_id,
        })
    }
    pub fn id(&self) -> &uuid::Uuid { &self.id }
    pub fn source_id(&self) -> &uuid::Uuid { &self.source_id }
    pub fn source_type(&self) -> &SourceType { &self.source_type }
    pub fn target_id(&self) -> &uuid::Uuid { &self.target_id }
    pub fn target_type(&self) -> &TargetType { &self.target_type }
    pub fn relationship_type(&self) -> &RelationshipType { &self.relationship_type }
    pub fn description(&self) -> &str { &self.description }
    pub fn genome_id(&self) -> &GenomeId { &self.genome_id }
}

pub trait ChannelRepository {
    fn save(&mut self, channel: Channel);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Channel>;
}

pub struct InMemoryChannelRepository {
    entries: std::collections::HashMap<uuid::Uuid, Channel>,
}

impl InMemoryChannelRepository {
    pub fn new() -> Self { Self { entries: std::collections::HashMap::new() } }
}
impl Default for InMemoryChannelRepository { fn default() -> Self { Self::new() } }


impl ChannelRepository for InMemoryChannelRepository {
    fn save(&mut self, channel: Channel) { self.entries.insert(*channel.id(), channel); }
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Channel> { self.entries.get(id) }
}

#[cfg(test)] mod tests {
    use super::*;
    fn genome_id() -> GenomeId { GenomeId::new() }
    
    #[test] fn channel_can_be_created() {
        let gid = genome_id();
        let ch = Channel::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            SourceType::Organism,
            uuid::Uuid::new_v4(),
            TargetType::Cell,
            RelationshipType::Uses,
            "uses".to_string(),
            gid
        ).unwrap();
        assert_eq!(ch.target_type(), &TargetType::Cell);
    }
    
    #[test] fn channel_organelle_to_organelle() {
        let gid = genome_id();
        let ch = Channel::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            SourceType::Organelle,
            uuid::Uuid::new_v4(),
            TargetType::Organelle,
            RelationshipType::Calls,
            "API calls database".to_string(),
            gid
        ).unwrap();
        assert_eq!(ch.source_type(), &SourceType::Organelle);
        assert_eq!(ch.target_type(), &TargetType::Organelle);
    }
    
    #[test] fn channel_cell_to_cell() {
        let gid = genome_id();
        let ch = Channel::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            SourceType::Cell,
            uuid::Uuid::new_v4(),
            TargetType::Cell,
            RelationshipType::DependsOn,
            "auth depends on payment".to_string(),
            gid
        ).unwrap();
        assert_eq!(ch.source_type(), &SourceType::Cell);
        assert_eq!(ch.target_type(), &TargetType::Cell);
    }
    
    #[test] fn channel_rejects_self_loop() {
        let gid = genome_id();
        let id = uuid::Uuid::new_v4();
        let result = Channel::new(id, id, SourceType::Cell, id, TargetType::Cell, RelationshipType::DependsOn, "self".to_string(), gid);
        assert_eq!(result, Err(ChannelError::SameSourceAndTarget));
    }
}