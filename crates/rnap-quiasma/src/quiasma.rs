use rnap_genome::GenomeId;

/// The type of node at the source end of a quiasma edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SourceType {
    Chromosome,
    Organism,
}

/// The type of node at the target end of a quiasma edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TargetType {
    Chromosome,
    Organism,
}

/// The kind of relationship between two domain nodes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RelationshipType {
    /// C4-level: system provides a service to another system
    DeliversTo,
    /// C4-level: system uses another system
    Uses,
    /// C4-level: system depends on another system
    DependsOn,
    /// C4-level: system calls an actor's interface
    Calls,
    /// C4-level: system contains another system
    Contains,
    /// C4-level: system attaches to an actor
    AttachTo,
}

impl RelationshipType {
    /// Returns the C4-style label for this relationship type.
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

/// A directed edge in the domain graph connecting domain nodes and actors.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Quiasma {
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
pub enum QuiasmaError {
    #[error("source and target must be different")]
    SameSourceAndTarget,
}

impl Quiasma {
    pub fn new(
        id: uuid::Uuid,
        source_id: uuid::Uuid,
        source_type: SourceType,
        target_id: uuid::Uuid,
        target_type: TargetType,
        relationship_type: RelationshipType,
        description: String,
        genome_id: GenomeId,
    ) -> Result<Self, QuiasmaError> {
        if source_id == target_id {
            return Err(QuiasmaError::SameSourceAndTarget);
        }
        Ok(Self {
            id,
            source_id,
            source_type,
            target_id,
            target_type,
            relationship_type,
            description,
            genome_id,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn source_id(&self) -> &uuid::Uuid {
        &self.source_id
    }

    pub fn source_type(&self) -> &SourceType {
        &self.source_type
    }

    pub fn target_id(&self) -> &uuid::Uuid {
        &self.target_id
    }

    pub fn target_type(&self) -> &TargetType {
        &self.target_type
    }

    pub fn relationship_type(&self) -> &RelationshipType {
        &self.relationship_type
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }
}

pub trait QuiasmaRepository {
    fn save(&mut self, quiasma: Quiasma);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Quiasma>;
}

pub struct InMemoryQuiasmaRepository {
    entries: std::collections::HashMap<uuid::Uuid, Quiasma>,
}

impl InMemoryQuiasmaRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryQuiasmaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl QuiasmaRepository for InMemoryQuiasmaRepository {
    fn save(&mut self, quiasma: Quiasma) {
        self.entries.insert(*quiasma.id(), quiasma);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Quiasma> {
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
    fn quiasma_can_be_created_with_source_and_target_nodes() {
        let genome_id = genome_id();
        let source_id = uuid::Uuid::new_v4();
        let target_id = uuid::Uuid::new_v4();

        let quiasma = Quiasma::new(
            uuid::Uuid::new_v4(),
            source_id,
            SourceType::Chromosome,
            target_id,
            TargetType::Organism,
            RelationshipType::Uses,
            "Order Service uses the Payment Service".to_string(),
            genome_id,
        )
        .unwrap();

        assert_eq!(quiasma.source_id(), &source_id);
        assert_eq!(quiasma.source_type(), &SourceType::Chromosome);
        assert_eq!(quiasma.target_id(), &target_id);
        assert_eq!(quiasma.target_type(), &TargetType::Organism);
        assert_eq!(quiasma.relationship_type(), &RelationshipType::Uses);
        assert_eq!(
            quiasma.description(),
            "Order Service uses the Payment Service"
        );
        assert_eq!(quiasma.genome_id(), &genome_id);
    }

    #[test]
    fn quiasma_rejects_same_source_and_target() {
        let genome_id = genome_id();
        let id = uuid::Uuid::new_v4();

        let result = Quiasma::new(
            id,
            id,
            SourceType::Chromosome,
            id,
            TargetType::Organism,
            RelationshipType::DependsOn,
            "self-loop".to_string(),
            genome_id,
        );
        assert_eq!(result, Err(QuiasmaError::SameSourceAndTarget));
    }

    #[test]
    fn relationship_type_has_all_c4_levels() {
        assert_eq!(RelationshipType::DeliversTo.label(), "delivers to");
        assert_eq!(RelationshipType::Uses.label(), "uses");
        assert_eq!(RelationshipType::DependsOn.label(), "depends on");
        assert_eq!(RelationshipType::Calls.label(), "calls");
        assert_eq!(RelationshipType::Contains.label(), "contains");
        assert_eq!(RelationshipType::AttachTo.label(), "attached to");
    }

    #[test]
    fn in_memory_quiasma_repo_saves_and_finds() {
        let genome_id = genome_id();
        let id = uuid::Uuid::new_v4();
        let quiasma = Quiasma::new(
            id,
            uuid::Uuid::new_v4(),
            SourceType::Chromosome,
            uuid::Uuid::new_v4(),
            TargetType::Chromosome,
            RelationshipType::DeliversTo,
            "API delivers to client".to_string(),
            genome_id,
        )
        .unwrap();

        let mut repo = InMemoryQuiasmaRepository::new();
        repo.save(quiasma);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.relationship_type(), &RelationshipType::DeliversTo);
    }
}
