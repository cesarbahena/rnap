use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fold {
    id: uuid::Uuid,
    trna_id: uuid::Uuid,
    commit: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FoldError {
    #[error("commit must not be empty")]
    EmptyCommit,
}

impl Fold {
    pub fn new(
        id: uuid::Uuid,
        trna_id: uuid::Uuid,
        commit: String,
        created_at: DateTime<Utc>,
    ) -> Result<Self, FoldError> {
        if commit.trim().is_empty() {
            return Err(FoldError::EmptyCommit);
        }
        Ok(Self {
            id,
            trna_id,
            commit,
            created_at,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn trna_id(&self) -> &uuid::Uuid {
        &self.trna_id
    }

    pub fn commit(&self) -> &str {
        &self.commit
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

pub trait FoldRepository {
    fn save(&mut self, fold: Fold);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Fold>;
    fn find_by_trna_id(&self, trna_id: &uuid::Uuid) -> Vec<&Fold>;
}

pub struct InMemoryFoldRepository {
    entries: std::collections::HashMap<uuid::Uuid, Fold>,
}

impl InMemoryFoldRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryFoldRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FoldRepository for InMemoryFoldRepository {
    fn save(&mut self, fold: Fold) {
        self.entries.insert(*fold.id(), fold);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Fold> {
        self.entries.get(id)
    }

    fn find_by_trna_id(&self, trna_id: &uuid::Uuid) -> Vec<&Fold> {
        self.entries
            .values()
            .filter(|fold| fold.trna_id() == trna_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold_can_be_created() {
        let id = uuid::Uuid::new_v4();
        let trna_id = uuid::Uuid::new_v4();
        let now = Utc::now();

        let fold = Fold::new(id, trna_id, "abc123def".to_string(), now).unwrap();

        assert_eq!(fold.commit(), "abc123def");
        assert_eq!(fold.trna_id(), &trna_id);
    }

    #[test]
    fn fold_rejects_empty_commit() {
        let result = Fold::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            Utc::now(),
        );
        assert_eq!(result, Err(FoldError::EmptyCommit));
    }

    #[test]
    fn in_memory_fold_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let fold = Fold::new(
            id,
            uuid::Uuid::new_v4(),
            "def456".to_string(),
            Utc::now(),
        )
        .unwrap();

        let mut repo = InMemoryFoldRepository::new();
        repo.save(fold);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.commit(), "def456");
    }

    #[test]
    fn in_memory_fold_repo_finds_by_trna_id() {
        let trna_id = uuid::Uuid::new_v4();
        let fold1 = Fold::new(uuid::Uuid::new_v4(), trna_id, "abc".to_string(), Utc::now()).unwrap();
        let fold2 = Fold::new(uuid::Uuid::new_v4(), trna_id, "def".to_string(), Utc::now()).unwrap();
        let other_trna = uuid::Uuid::new_v4();
        let fold3 = Fold::new(uuid::Uuid::new_v4(), other_trna, "ghi".to_string(), Utc::now()).unwrap();

        let mut repo = InMemoryFoldRepository::new();
        repo.save(fold1);
        repo.save(fold2);
        repo.save(fold3);

        let found = repo.find_by_trna_id(&trna_id);
        assert_eq!(found.len(), 2);
    }
}
