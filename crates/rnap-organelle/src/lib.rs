//! Organelle — runtime unit inside a Cell (C4 Container level).
//!
//! An Organelle is a deployable/runnable component within a Cell.
//! Examples: API service, worker, database, queue processor.
//! Maps to C4 Container in Structurizr.

use rnap_genome::GenomeId;

/// The kind of runtime unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OrganelleKind {
    /// A service that handles requests (API, gRPC, etc.)
    Service,
    /// A background worker that processes tasks
    Worker,
    /// A data store (database, cache, etc.)
    Database,
    /// A message queue or broker
    Queue,
}

impl OrganelleKind {
    pub fn label(&self) -> &'static str {
        match self {
            OrganelleKind::Service => "Service",
            OrganelleKind::Worker => "Worker",
            OrganelleKind::Database => "Database",
            OrganelleKind::Queue => "Queue",
        }
    }
}

/// A runtime unit inside a Cell.
///
/// An Organelle is a deployable or runnable component (API service, worker,
/// database, queue processor). It belongs to exactly one Cell.
///
/// Maps to Structurizr's `container` element.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Organelle {
    id: uuid::Uuid,
    name: String,
    description: String,
    kind: OrganelleKind,
    /// Technology stack (e.g., "Java/Spring Boot", "PostgreSQL", "RabbitMQ")
    technology: String,
    /// The Cell this Organelle belongs to (required)
    cell_id: uuid::Uuid,
    genome_id: GenomeId,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum OrganelleError {
    #[error("organelle name must not be empty")]
    EmptyName,
    #[error("organelle technology must not be empty")]
    EmptyTechnology,
}

impl Organelle {
    /// Creates a new Organelle with the given parameters.
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        kind: OrganelleKind,
        technology: String,
        cell_id: uuid::Uuid,
        genome_id: GenomeId,
    ) -> Result<Self, OrganelleError> {
        if name.trim().is_empty() {
            return Err(OrganelleError::EmptyName);
        }
        if technology.trim().is_empty() {
            return Err(OrganelleError::EmptyTechnology);
        }
        Ok(Self {
            id,
            name,
            description,
            kind,
            technology,
            cell_id,
            genome_id,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn kind(&self) -> &OrganelleKind {
        &self.kind
    }

    pub fn technology(&self) -> &str {
        &self.technology
    }

    pub fn cell_id(&self) -> &uuid::Uuid {
        &self.cell_id
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }
}

/// Repository trait for Organelle persistence.
pub trait OrganelleRepository: Send + Sync {
    fn save(&mut self, organelle: Organelle) -> Result<(), OrganelleRepositoryError>;
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<Organelle>;
    /// Find all organelles belonging to a specific cell.
    fn find_by_cell_id(&self, cell_id: &uuid::Uuid) -> Vec<Organelle>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum OrganelleRepositoryError {
    #[error("organelle not found")]
    NotFound,
    #[error("save failed: {0}")]
    SaveFailed(String),
}

impl From<String> for OrganelleRepositoryError {
    fn from(err: String) -> Self {
        OrganelleRepositoryError::SaveFailed(err)
    }
}

/// In-memory Organelle repository for testing.
pub struct InMemoryOrganelleRepository {
    organelles: std::sync::RwLock<std::collections::HashMap<uuid::Uuid, Organelle>>,
}

impl InMemoryOrganelleRepository {
    pub fn new() -> Self {
        Self {
            organelles: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

impl Default for InMemoryOrganelleRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganelleRepository for InMemoryOrganelleRepository {
    fn save(&mut self, organelle: Organelle) -> Result<(), OrganelleRepositoryError> {
        let mut organelles = self.organelles.write().map_err(|e| e.to_string())?;
        organelles.insert(*organelle.id(), organelle);
        Ok(())
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<Organelle> {
        let organelles = self.organelles.read().ok()?;
        organelles.get(id).cloned()
    }

    fn find_by_cell_id(&self, cell_id: &uuid::Uuid) -> Vec<Organelle> {
        let organelles = self.organelles.read().unwrap();
        organelles
            .values()
            .filter(|o| o.cell_id() == cell_id)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn genome_id() -> GenomeId {
        GenomeId::new()
    }

    fn cell_id() -> uuid::Uuid {
        uuid::Uuid::new_v4()
    }

    #[test]
    fn organelle_can_be_created_with_all_fields() {
        let gid = genome_id();
        let cid = cell_id();
        let organelle = Organelle::new(
            uuid::Uuid::new_v4(),
            "Order API".to_string(),
            "Handles order CRUD operations".to_string(),
            OrganelleKind::Service,
            "Java/Spring Boot".to_string(),
            cid,
            gid,
        )
        .unwrap();

        assert_eq!(organelle.name(), "Order API");
        assert_eq!(organelle.kind(), &OrganelleKind::Service);
        assert_eq!(organelle.technology(), "Java/Spring Boot");
        assert_eq!(organelle.cell_id(), &cid);
    }

    #[test]
    fn organelle_rejects_empty_name() {
        let gid = genome_id();
        let result = Organelle::new(
            uuid::Uuid::new_v4(),
            "".to_string(),
            "desc".to_string(),
            OrganelleKind::Service,
            "Java".to_string(),
            cell_id(),
            gid,
        );
        assert_eq!(result, Err(OrganelleError::EmptyName));
    }

    #[test]
    fn organelle_rejects_empty_technology() {
        let gid = genome_id();
        let result = Organelle::new(
            uuid::Uuid::new_v4(),
            "Order API".to_string(),
            "desc".to_string(),
            OrganelleKind::Service,
            "".to_string(),
            cell_id(),
            gid,
        );
        assert_eq!(result, Err(OrganelleError::EmptyTechnology));
    }

    #[test]
    fn organelle_kind_has_label() {
        assert_eq!(OrganelleKind::Service.label(), "Service");
        assert_eq!(OrganelleKind::Worker.label(), "Worker");
        assert_eq!(OrganelleKind::Database.label(), "Database");
        assert_eq!(OrganelleKind::Queue.label(), "Queue");
    }

    #[test]
    fn in_memory_organelle_repository_saves_and_finds() {
        let gid = genome_id();
        let cid = cell_id();
        let id = uuid::Uuid::new_v4();
        let organelle = Organelle::new(
            id,
            "Payment Service".to_string(),
            "Processes payments".to_string(),
            OrganelleKind::Service,
            "Go".to_string(),
            cid,
            gid,
        )
        .unwrap();

        let mut repo = InMemoryOrganelleRepository::new();
        repo.save(organelle).unwrap();

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.name(), "Payment Service");
    }

    #[test]
    fn in_memory_organelle_repository_find_by_cell_id() {
        let gid = genome_id();
        let cid = cell_id();

        let organelle1 = Organelle::new(
            uuid::Uuid::new_v4(),
            "API".to_string(),
            "API".to_string(),
            OrganelleKind::Service,
            "Rust".to_string(),
            cid,
            gid,
        )
        .unwrap();

        let organelle2 = Organelle::new(
            uuid::Uuid::new_v4(),
            "DB".to_string(),
            "DB".to_string(),
            OrganelleKind::Database,
            "PostgreSQL".to_string(),
            cid,
            gid,
        )
        .unwrap();

        // Different cell
        let organelle3 = Organelle::new(
            uuid::Uuid::new_v4(),
            "Other".to_string(),
            "Other".to_string(),
            OrganelleKind::Worker,
            "Python".to_string(),
            uuid::Uuid::new_v4(),
            gid,
        )
        .unwrap();

        let mut repo = InMemoryOrganelleRepository::new();
        repo.save(organelle1).unwrap();
        repo.save(organelle2).unwrap();
        repo.save(organelle3).unwrap();

        let in_cell = repo.find_by_cell_id(&cid);
        assert_eq!(in_cell.len(), 2);
    }
}