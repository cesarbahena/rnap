use chrono::{DateTime, Utc};

/// An implementation snapshot — captures WHAT was built at a point in time.
///
/// Links what was implemented (git state) to what was intended (mRNA).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Phenotype {
    id: uuid::Uuid,
    mrna_id: uuid::Uuid,
    commit_sha: String,
    root_git_directory: String,
    branch: String,
    remote: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum PhenotypeError {
    #[error("commit SHA must not be empty")]
    EmptyCommitSha,
}

impl Phenotype {
    pub fn new(
        id: uuid::Uuid,
        mrna_id: uuid::Uuid,
        commit_sha: String,
        root_git_directory: String,
        branch: String,
        remote: String,
        created_at: DateTime<Utc>,
    ) -> Result<Self, PhenotypeError> {
        if commit_sha.trim().is_empty() {
            return Err(PhenotypeError::EmptyCommitSha);
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

pub trait PhenotypeRepository {
    fn save(&mut self, phenotype: Phenotype);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Phenotype>;
}

pub struct InMemoryPhenotypeRepository {
    entries: std::collections::HashMap<uuid::Uuid, Phenotype>,
}

impl InMemoryPhenotypeRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryPhenotypeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl PhenotypeRepository for InMemoryPhenotypeRepository {
    fn save(&mut self, phenotype: Phenotype) {
        self.entries.insert(*phenotype.id(), phenotype);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Phenotype> {
        self.entries.get(id)
    }
}