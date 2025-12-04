use chrono::{DateTime, Utc};
use rnap_genome::GenomeId;

/// An atomic learning from an implementation task.
///
/// sRNA entries start as DB records and can be promoted (exported to AGENTS.md)
/// for future LLM sessions, feeding back into the regulatory cycle.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Srna {
    id: uuid::Uuid,
    content: String,
    task_context: String,
    promoted: bool,
    genome_id: GenomeId,
    created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SrnaError {
    #[error("sRNA content must not be empty")]
    EmptyContent,
    #[error("sRNA task context must not be empty")]
    EmptyTaskContext,
}

impl Srna {
    pub fn new(
        id: uuid::Uuid,
        content: String,
        task_context: String,
        genome_id: GenomeId,
        created_at: DateTime<Utc>,
    ) -> Result<Self, SrnaError> {
        if content.trim().is_empty() {
            return Err(SrnaError::EmptyContent);
        }
        if task_context.trim().is_empty() {
            return Err(SrnaError::EmptyTaskContext);
        }
        Ok(Self {
            id,
            content,
            task_context,
            promoted: false,
            genome_id,
            created_at,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn task_context(&self) -> &str {
        &self.task_context
    }

    pub fn promoted(&self) -> bool {
        self.promoted
    }

    /// Marks this sRNA as promoted (exported to AGENTS.md).
    pub fn promote(&mut self) {
        self.promoted = true;
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

pub trait SrnaRepository {
    fn save(&mut self, srna: Srna);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Srna>;
}

pub struct InMemorySrnaRepository {
    entries: std::collections::HashMap<uuid::Uuid, Srna>,
}

impl InMemorySrnaRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemorySrnaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl SrnaRepository for InMemorySrnaRepository {
    fn save(&mut self, srna: Srna) {
        self.entries.insert(*srna.id(), srna);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Srna> {
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
    fn srna_can_be_created_with_content_and_task_context() {
        let gid = genome_id();
        let now = Utc::now();

        let srna = Srna::new(
            uuid::Uuid::new_v4(),
            "Always validate tenant isolation at the repository layer".to_string(),
            "Implementing mRNA freeze logic".to_string(),
            gid,
            now,
        )
        .unwrap();

        assert_eq!(
            srna.content(),
            "Always validate tenant isolation at the repository layer"
        );
        assert_eq!(srna.task_context(), "Implementing mRNA freeze logic");
        assert!(!srna.promoted());
        assert_eq!(srna.genome_id(), &gid);
    }

    #[test]
    fn srna_rejects_empty_content() {
        let result = Srna::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            "some task".to_string(),
            genome_id(),
            Utc::now(),
        );
        assert_eq!(result, Err(SrnaError::EmptyContent));
    }

    #[test]
    fn srna_rejects_empty_task_context() {
        let result = Srna::new(
            uuid::Uuid::new_v4(),
            "some learning".to_string(),
            "   ".to_string(),
            genome_id(),
            Utc::now(),
        );
        assert_eq!(result, Err(SrnaError::EmptyTaskContext));
    }

    #[test]
    fn srna_can_be_promoted() {
        let mut srna = Srna::new(
            uuid::Uuid::new_v4(),
            "Learning".to_string(),
            "Task context".to_string(),
            genome_id(),
            Utc::now(),
        )
        .unwrap();

        assert!(!srna.promoted());
        srna.promote();
        assert!(srna.promoted());
    }

    #[test]
    fn in_memory_srna_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let srna = Srna::new(
            id,
            "Learning".to_string(),
            "Task context".to_string(),
            genome_id(),
            Utc::now(),
        )
        .unwrap();

        let mut repo = InMemorySrnaRepository::new();
        repo.save(srna);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.content(), "Learning");
    }
}