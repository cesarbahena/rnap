use chrono::{DateTime, Utc};
use rnap_genome::GenomeId;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Histone {
    id: uuid::Uuid,
    title: String,
    decision: String,
    context: String,
    genome_id: GenomeId,
    mutation_id: Option<uuid::Uuid>,
    gene_id: Option<uuid::Uuid>,
    dna_id: Option<uuid::Uuid>,
    created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum HistoneError {
    #[error("histone title must not be empty")]
    EmptyTitle,
    #[error("histone decision must not be empty")]
    EmptyDecision,
    #[error("histone context must not be empty")]
    EmptyContext,
    #[error("histone must be linked to at least one relevant change (mutation_id, gene_id, or dna_id)")]
    NoLinkedChange,
}

impl Histone {
    pub fn new(
        id: uuid::Uuid,
        title: String,
        decision: String,
        context: String,
        genome_id: GenomeId,
        mutation_id: Option<uuid::Uuid>,
        gene_id: Option<uuid::Uuid>,
        dna_id: Option<uuid::Uuid>,
        created_at: DateTime<Utc>,
    ) -> Result<Self, HistoneError> {
        if title.trim().is_empty() {
            return Err(HistoneError::EmptyTitle);
        }
        if decision.trim().is_empty() {
            return Err(HistoneError::EmptyDecision);
        }
        if context.trim().is_empty() {
            return Err(HistoneError::EmptyContext);
        }
        if mutation_id.is_none() && gene_id.is_none() && dna_id.is_none() {
            return Err(HistoneError::NoLinkedChange);
        }
        Ok(Self {
            id,
            title,
            decision,
            context,
            genome_id,
            mutation_id,
            gene_id,
            dna_id,
            created_at,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn decision(&self) -> &str {
        &self.decision
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }

    pub fn mutation_id(&self) -> Option<&uuid::Uuid> {
        self.mutation_id.as_ref()
    }

    pub fn gene_id(&self) -> Option<&uuid::Uuid> {
        self.gene_id.as_ref()
    }

    pub fn dna_id(&self) -> Option<&uuid::Uuid> {
        self.dna_id.as_ref()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

pub trait HistoneRepository {
    fn save(&mut self, histone: Histone);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Histone>;
}

pub struct InMemoryHistoneRepository {
    entries: std::collections::HashMap<uuid::Uuid, Histone>,
}

impl InMemoryHistoneRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryHistoneRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoneRepository for InMemoryHistoneRepository {
    fn save(&mut self, histone: Histone) {
        self.entries.insert(*histone.id(), histone);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Histone> {
        self.entries.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn genome_id() -> GenomeId {
        GenomeId::new()
    }

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    #[test]
    fn histone_rejects_empty_title() {
        let result = Histone::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            "some decision".to_string(),
            "some context".to_string(),
            genome_id(),
            None,
            Some(uuid::Uuid::new_v4()),
            None,
            now(),
        );
        assert_eq!(result, Err(HistoneError::EmptyTitle));
    }

    #[test]
    fn histone_rejects_empty_decision() {
        let result = Histone::new(
            uuid::Uuid::new_v4(),
            "some title".to_string(),
            "   ".to_string(),
            "some context".to_string(),
            genome_id(),
            None,
            Some(uuid::Uuid::new_v4()),
            None,
            now(),
        );
        assert_eq!(result, Err(HistoneError::EmptyDecision));
    }

    #[test]
    fn histone_rejects_empty_context() {
        let result = Histone::new(
            uuid::Uuid::new_v4(),
            "some title".to_string(),
            "some decision".to_string(),
            "   ".to_string(),
            genome_id(),
            None,
            Some(uuid::Uuid::new_v4()),
            None,
            now(),
        );
        assert_eq!(result, Err(HistoneError::EmptyContext));
    }

    #[test]
    fn histone_rejects_when_no_linked_change() {
        let result = Histone::new(
            uuid::Uuid::new_v4(),
            "some title".to_string(),
            "some decision".to_string(),
            "some context".to_string(),
            genome_id(),
            None,
            None,
            None,
            now(),
        );
        assert_eq!(result, Err(HistoneError::NoLinkedChange));
    }

    #[test]
    fn in_memory_histone_repo_saves_and_finds() {
        let genome_id = genome_id();
        let id = uuid::Uuid::new_v4();
        let histone = Histone::new(
            id,
            "Use PostgreSQL".to_string(),
            "Chosen for row-level security".to_string(),
            "Need multi-tenant isolation".to_string(),
            genome_id,
            Some(uuid::Uuid::new_v4()),
            None,
            None,
            now(),
        )
        .unwrap();

        let mut repo = InMemoryHistoneRepository::new();
        repo.save(histone);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.title(), "Use PostgreSQL");
    }

    #[test]
    fn histone_can_be_created_with_required_fields_and_at_least_one_link() {
        let genome_id = genome_id();
        let gene_id = uuid::Uuid::new_v4();
        let created_at = now();

        let histone = Histone::new(
            uuid::Uuid::new_v4(),
            "Use PostgreSQL".to_string(),
            "PostgreSQL chosen for multi-tenant isolation".to_string(),
            "Need relational DB with row-level security".to_string(),
            genome_id,
            None,
            Some(gene_id),
            None,
            created_at,
        )
        .unwrap();

        assert_eq!(histone.title(), "Use PostgreSQL");
        assert_eq!(histone.gene_id(), Some(&gene_id));
        assert_eq!(histone.mutation_id(), None);
        assert_eq!(histone.dna_id(), None);
    }
}