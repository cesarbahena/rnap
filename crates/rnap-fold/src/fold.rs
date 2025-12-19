use chrono::{DateTime, Utc};

/// An implementation snapshot — captures WHAT was built at a point in time.
///
/// Links what was implemented (git state) to what was intended (mRNA).
/// 
/// Note: This was previously called "Phenotype" but was renamed to "Fold"
/// to make room for the rename: Phenome → Phenotype.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fold {
    id: uuid::Uuid,
    mrna_id: uuid::Uuid,
    commit_sha: String,
    root_git_directory: String,
    branch: String,
    remote: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FoldError {
    #[error("commit SHA must not be empty")]
    EmptyCommitSha,
}

impl Fold {
    pub fn new(
        id: uuid::Uuid,
        mrna_id: uuid::Uuid,
        commit_sha: String,
        root_git_directory: String,
        branch: String,
        remote: String,
        created_at: DateTime<Utc>,
    ) -> Result<Self, FoldError> {
        if commit_sha.trim().is_empty() {
            return Err(FoldError::EmptyCommitSha);
        }
        Ok(Self {
            id,
            mrna_id,
            commit_sha,
            root_git_directory,
            branch,
            remote,
            created_at,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn mrna_id(&self) -> &uuid::Uuid {
        &self.mrna_id
    }

    pub fn commit_sha(&self) -> &str {
        &self.commit_sha
    }

    pub fn root_git_directory(&self) -> &str {
        &self.root_git_directory
    }

    pub fn branch(&self) -> &str {
        &self.branch
    }

    pub fn remote(&self) -> &str {
        &self.remote
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

pub trait FoldRepository {
    fn save(&mut self, fold: Fold);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Fold>;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold_can_be_created_with_commit_sha() {
        let id = uuid::Uuid::new_v4();
        let mrna_id = uuid::Uuid::new_v4();
        let now = Utc::now();

        let fold = Fold::new(
            id,
            mrna_id,
            "abc123def".to_string(),
            "/home/user/project".to_string(),
            "main".to_string(),
            "origin".to_string(),
            now,
        )
        .unwrap();

        assert_eq!(fold.commit_sha(), "abc123def");
        assert_eq!(fold.mrna_id(), &mrna_id);
        assert_eq!(fold.branch(), "main");
    }

    #[test]
    fn fold_rejects_empty_commit_sha() {
        let result = Fold::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            "/home/user/project".to_string(),
            "main".to_string(),
            "origin".to_string(),
            Utc::now(),
        );
        assert_eq!(result, Err(FoldError::EmptyCommitSha));
    }

    #[test]
    fn in_memory_fold_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let fold = Fold::new(
            id,
            uuid::Uuid::new_v4(),
            "def456".to_string(),
            "/home/user/project".to_string(),
            "feature".to_string(),
            "origin".to_string(),
            Utc::now(),
        )
        .unwrap();

        let mut repo = InMemoryFoldRepository::new();
        repo.save(fold);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.commit_sha(), "def456");
    }
}
