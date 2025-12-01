use chrono::{DateTime, Utc};
use rnap_genome::GenomeId;

/// The status of a task in a tRNA tasklist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
}

/// A single task within a tRNA tasklist.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Task {
    description: String,
    status: TaskStatus,
}

impl Task {
    pub fn new(description: String, status: TaskStatus) -> Result<Self, TrnaError> {
        if description.trim().is_empty() {
            return Err(TrnaError::EmptyTaskDescription);
        }
        Ok(Self { description, status })
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }
}

/// A mutable tasklist for an mRNA implementation context.
///
/// Must be generated before starting coding and must be updated
/// when tasks are completed or blocked.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trna {
    id: uuid::Uuid,
    mrna_id: uuid::Uuid,
    tasks: Vec<Task>,
    genome_id: GenomeId,
    created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TrnaError {
    #[error("task description must not be empty")]
    EmptyTaskDescription,
}

impl Trna {
    pub fn new(
        id: uuid::Uuid,
        mrna_id: uuid::Uuid,
        tasks: Vec<Task>,
        genome_id: GenomeId,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            mrna_id,
            tasks,
            genome_id,
            created_at,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn mrna_id(&self) -> &uuid::Uuid {
        &self.mrna_id
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

pub trait TrnaRepository {
    fn save(&mut self, trna: Trna);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Trna>;
}

pub struct InMemoryTrnaRepository {
    entries: std::collections::HashMap<uuid::Uuid, Trna>,
}

impl InMemoryTrnaRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryTrnaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl TrnaRepository for InMemoryTrnaRepository {
    fn save(&mut self, trna: Trna) {
        self.entries.insert(*trna.id(), trna);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Trna> {
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
    fn trna_can_be_created_with_tasks() {
        let mrna_id = uuid::Uuid::new_v4();
        let gid = genome_id();
        let now = Utc::now();

        let trna = Trna::new(
            uuid::Uuid::new_v4(),
            mrna_id,
            vec![
                Task::new("Set up database schema".to_string(), TaskStatus::Todo).unwrap(),
                Task::new("Write API endpoint".to_string(), TaskStatus::Todo).unwrap(),
            ],
            gid,
            now,
        );

        assert_eq!(trna.mrna_id(), &mrna_id);
        assert_eq!(trna.tasks().len(), 2);
        assert_eq!(trna.tasks()[0].description(), "Set up database schema");
        assert_eq!(trna.tasks()[0].status(), &TaskStatus::Todo);
    }

    #[test]
    fn task_rejects_empty_description() {
        let result = Task::new("   ".to_string(), TaskStatus::Todo);
        assert_eq!(result, Err(TrnaError::EmptyTaskDescription));
    }

    #[test]
    fn in_memory_trna_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let trna = Trna::new(
            id,
            uuid::Uuid::new_v4(),
            vec![Task::new("Implement feature".to_string(), TaskStatus::InProgress).unwrap()],
            genome_id(),
            Utc::now(),
        );

        let mut repo = InMemoryTrnaRepository::new();
        repo.save(trna);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.tasks().len(), 1);
        assert_eq!(found.tasks()[0].status(), &TaskStatus::InProgress);
    }
}