use chrono::{DateTime, Utc};
use rnap_genome::GenomeId;

/// A tasklist generated from an mRNA.
///
/// tRNA lanes contain anticodons (tasks) that contribute to implementing
/// the parent mRNA. Multiple tRNAs can exist per mRNA for parallel work.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trna {
    id: uuid::Uuid,
    mrna_id: uuid::Uuid,
    anticodons: Vec<String>,
    worktree: String,
    genome_id: GenomeId,
    created_at: DateTime<Utc>,
}

impl Trna {
    pub fn new(
        id: uuid::Uuid,
        mrna_id: uuid::Uuid,
        anticodons: Vec<String>,
        worktree: String,
        genome_id: GenomeId,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            mrna_id,
            anticodons,
            worktree,
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

    pub fn anticodons(&self) -> &[String] {
        &self.anticodons
    }

    pub fn worktree(&self) -> &str {
        &self.worktree
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
    fn trna_can_be_created_with_anticodons() {
        let mrna_id = uuid::Uuid::new_v4();
        let gid = genome_id();
        let now = Utc::now();

        let trna = Trna::new(
            uuid::Uuid::new_v4(),
            mrna_id,
            vec![
                "Set up database schema".to_string(),
                "Write API endpoint".to_string(),
            ],
            "/worktrees/feature".to_string(),
            gid,
            now,
        );

        assert_eq!(trna.mrna_id(), &mrna_id);
        assert_eq!(trna.anticodons().len(), 2);
        assert_eq!(trna.anticodons()[0], "Set up database schema");
        assert_eq!(trna.worktree(), "/worktrees/feature");
    }

    #[test]
    fn in_memory_trna_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let trna = Trna::new(
            id,
            uuid::Uuid::new_v4(),
            vec!["Implement feature".to_string()],
            "/worktrees/feature".to_string(),
            genome_id(),
            Utc::now(),
        );

        let mut repo = InMemoryTrnaRepository::new();
        repo.save(trna);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.anticodons().len(), 1);
    }
}
