use chrono::{DateTime, Utc};

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
    pub fn new(id: uuid::Uuid, mrna_id: uuid::Uuid, commit_sha: String, root_git_directory: String, branch: String, remote: String, created_at: DateTime<Utc>) -> Result<Self, FoldError> {
        if commit_sha.trim().is_empty() { return Err(FoldError::EmptyCommitSha); }
        Ok(Self { id, mrna_id, commit_sha, root_git_directory, branch, remote, created_at })
    }
    pub fn id(&self) -> &uuid::Uuid { &self.id }
    pub fn mrna_id(&self) -> &uuid::Uuid { &self.mrna_id }
    pub fn commit_sha(&self) -> &str { &self.commit_sha }
    pub fn root_git_directory(&self) -> &str { &self.root_git_directory }
    pub fn branch(&self) -> &str { &self.branch }
    pub fn remote(&self) -> &str { &self.remote }
    pub fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn fold_can_be_created() {
        let f = Fold::new(uuid::Uuid::new_v4(), uuid::Uuid::new_v4(), "abc123".to_string(), "/repo".to_string(), "main".to_string(), "origin".to_string(), Utc::now()).unwrap();
        assert_eq!(f.commit_sha(), "abc123");
    }
    #[test] fn fold_rejects_empty_sha() {
        let r = Fold::new(uuid::Uuid::new_v4(), uuid::Uuid::new_v4(), "   ".to_string(), "/repo".to_string(), "main".to_string(), "origin".to_string(), Utc::now());
        assert_eq!(r, Err(FoldError::EmptyCommitSha));
    }
}
