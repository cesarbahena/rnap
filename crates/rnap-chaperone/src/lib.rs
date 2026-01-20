use chrono::{DateTime, Utc};

/// QA commentary attached to a fold.
///
/// Chaperones are shown alongside tRNAs when listing tasks.
/// They represent QA feedback that needs attention.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Chaperone {
    id: uuid::Uuid,
    fold_id: uuid::Uuid,
    content: String,
    created_at: DateTime<Utc>,
}

impl Chaperone {
    pub fn new(
        id: uuid::Uuid,
        fold_id: uuid::Uuid,
        content: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            fold_id,
            content,
            created_at,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn fold_id(&self) -> &uuid::Uuid {
        &self.fold_id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

pub trait ChaperoneRepository {
    fn save(&mut self, chaperone: Chaperone);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Chaperone>;
    fn find_by_fold_id(&self, fold_id: &uuid::Uuid) -> Vec<&Chaperone>;
}

pub struct InMemoryChaperoneRepository {
    entries: std::collections::HashMap<uuid::Uuid, Chaperone>,
}

impl InMemoryChaperoneRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryChaperoneRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaperoneRepository for InMemoryChaperoneRepository {
    fn save(&mut self, chaperone: Chaperone) {
        self.entries.insert(*chaperone.id(), chaperone);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Chaperone> {
        self.entries.get(id)
    }

    fn find_by_fold_id(&self, fold_id: &uuid::Uuid) -> Vec<&Chaperone> {
        self.entries
            .values()
            .filter(|c| c.fold_id() == fold_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaperone_can_be_created() {
        let fold_id = uuid::Uuid::new_v4();
        let chaperone = Chaperone::new(
            uuid::Uuid::new_v4(),
            fold_id,
            "Add loading spinner".to_string(),
            Utc::now(),
        );

        assert_eq!(chaperone.fold_id(), &fold_id);
        assert_eq!(chaperone.content(), "Add loading spinner");
    }

    #[test]
    fn in_memory_chaperone_repo_saves_and_finds() {
        let fold_id = uuid::Uuid::new_v4();
        let id = uuid::Uuid::new_v4();
        let chaperone = Chaperone::new(
            id,
            fold_id,
            "Fix aria labels".to_string(),
            Utc::now(),
        );

        let mut repo = InMemoryChaperoneRepository::new();
        repo.save(chaperone);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.content(), "Fix aria labels");
    }

    #[test]
    fn in_memory_chaperone_repo_finds_by_fold_id() {
        let fold1 = uuid::Uuid::new_v4();
        let fold2 = uuid::Uuid::new_v4();

        let c1 = Chaperone::new(uuid::Uuid::new_v4(), fold1, "Issue 1".to_string(), Utc::now());
        let c2 = Chaperone::new(uuid::Uuid::new_v4(), fold1, "Issue 2".to_string(), Utc::now());
        let c3 = Chaperone::new(uuid::Uuid::new_v4(), fold2, "Issue 3".to_string(), Utc::now());

        let mut repo = InMemoryChaperoneRepository::new();
        repo.save(c1);
        repo.save(c2);
        repo.save(c3);

        let found = repo.find_by_fold_id(&fold1);
        assert_eq!(found.len(), 2);
    }
}
