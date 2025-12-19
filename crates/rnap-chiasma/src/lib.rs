//! Chiasma — evidence of technical debt from domain boundary violations.

use rnap_genome::GenomeId;

/// Type of architecture violation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ViolationType {
    CrossBoundaryDependency,
    CircularDependency,
    DomainLeakage,
    BidirectionalCoupling,
    SharedMutability,
    GodObject,
}

/// Chiasma — mark of domain leakage / technical debt.
///
/// NOT a relationship. This documents when chromosomes (domains) leak
/// into each other, violating architectural boundaries.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Chiasma {
    id: uuid::Uuid,
    source_chromosome_id: uuid::Uuid,
    source_chromosome_name: String,
    target_chromosome_id: uuid::Uuid,
    target_chromosome_name: String,
    violation_type: ViolationType,
    description: String,
    documented_at: chrono::DateTime<chrono::Utc>,
    genome_id: GenomeId,
}

impl Chiasma {
    pub fn new(
        id: uuid::Uuid,
        source_chromosome_id: uuid::Uuid,
        source_chromosome_name: String,
        target_chromosome_id: uuid::Uuid,
        target_chromosome_name: String,
        violation_type: ViolationType,
        description: String,
        genome_id: GenomeId,
    ) -> Self {
        Self {
            id,
            source_chromosome_id,
            source_chromosome_name,
            target_chromosome_id,
            target_chromosome_name,
            violation_type,
            description,
            documented_at: chrono::Utc::now(),
            genome_id,
        }
    }

    pub fn id(&self) -> &uuid::Uuid { &self.id }
    pub fn source_chromosome_id(&self) -> &uuid::Uuid { &self.source_chromosome_id }
    pub fn source_chromosome_name(&self) -> &str { &self.source_chromosome_name }
    pub fn target_chromosome_id(&self) -> &uuid::Uuid { &self.target_chromosome_id }
    pub fn target_chromosome_name(&self) -> &str { &self.target_chromosome_name }
    pub fn violation_type(&self) -> &ViolationType { &self.violation_type }
    pub fn description(&self) -> &str { &self.description }
    pub fn documented_at(&self) -> &chrono::DateTime<chrono::Utc> { &self.documented_at }
    pub fn genome_id(&self) -> &GenomeId { &self.genome_id }
}

pub trait ChiasmaRepository {
    fn save(&mut self, chiasma: Chiasma);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Chiasma>;
    fn find_all(&self) -> Vec<&Chiasma>;
}

pub struct InMemoryChiasmaRepository {
    entries: std::collections::HashMap<uuid::Uuid, Chiasma>,
}

impl InMemoryChiasmaRepository {
    pub fn new() -> Self { Self { entries: std::collections::HashMap::new() } }
}
impl Default for InMemoryChiasmaRepository { fn default() -> Self { Self::new() } }

impl ChiasmaRepository for InMemoryChiasmaRepository {
    fn save(&mut self, chiasma: Chiasma) { self.entries.insert(*chiasma.id(), chiasma); }
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Chiasma> { self.entries.get(id) }
    fn find_all(&self) -> Vec<&Chiasma> { self.entries.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn genome_id() -> GenomeId { GenomeId::new() }

    #[test]
    fn chiasma_documents_domain_leakage() {
        let gid = genome_id();
        let c = Chiasma::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(), "Order Service".to_string(),
            uuid::Uuid::new_v4(), "User Service".to_string(),
            ViolationType::CrossBoundaryDependency,
            "Order directly accesses User Repository".to_string(),
            gid,
        );
        assert_eq!(c.source_chromosome_name(), "Order Service");
        assert_eq!(c.violation_type(), &ViolationType::CrossBoundaryDependency);
    }

    #[test]
    fn chiasma_repo_find_all() {
        let gid = genome_id();
        let mut repo = InMemoryChiasmaRepository::new();
        repo.save(Chiasma::new(uuid::Uuid::new_v4(), uuid::Uuid::new_v4(), "A".to_string(), uuid::Uuid::new_v4(), "B".to_string(), ViolationType::DomainLeakage, "leak".to_string(), gid.clone()));
        repo.save(Chiasma::new(uuid::Uuid::new_v4(), uuid::Uuid::new_v4(), "C".to_string(), uuid::Uuid::new_v4(), "D".to_string(), ViolationType::CircularDependency, "cycle".to_string(), gid));
        assert_eq!(repo.find_all().len(), 2);
    }
}
