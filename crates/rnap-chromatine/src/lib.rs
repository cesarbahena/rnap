#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Chromatine {
    id: uuid::Uuid,
    url: String,
    genome_id: rnap_genome::GenomeId,
}

impl Chromatine {
    pub fn new(id: uuid::Uuid, url: String, genome_id: rnap_genome::GenomeId) -> Result<Self, ChromatineError> {
        if url.trim().is_empty() {
            return Err(ChromatineError::EmptyUrl);
        }
        Ok(Self { id, url, genome_id })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn genome_id(&self) -> &rnap_genome::GenomeId {
        &self.genome_id
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ChromatineError {
    #[error("chromatine URL must not be empty")]
    EmptyUrl,
}

pub trait ChromatineRepository {
    fn save(&mut self, chromatine: Chromatine);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Chromatine>;
}

pub struct InMemoryChromatineRepository {
    entries: std::collections::HashMap<uuid::Uuid, Chromatine>,
}

impl InMemoryChromatineRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryChromatineRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromatineRepository for InMemoryChromatineRepository {
    fn save(&mut self, chromatine: Chromatine) {
        self.entries.insert(*chromatine.id(), chromatine);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Chromatine> {
        self.entries.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rnap_genome::GenomeId;

    #[test]
    fn chromatine_can_be_created_with_id_url_and_genome_id() {
        let genome_id = GenomeId::new();
        let chromatine = Chromatine::new(
            uuid::Uuid::new_v4(),
            "https://docs.example.com/prd-v2.pdf".to_string(),
            genome_id,
        ).unwrap();

        assert_eq!(chromatine.url(), "https://docs.example.com/prd-v2.pdf");
        assert_eq!(chromatine.genome_id(), &genome_id);
    }

    #[test]
    fn chromatine_rejects_empty_url() {
        let genome_id = GenomeId::new();
        let result = Chromatine::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            genome_id,
        );
        assert_eq!(result, Err(ChromatineError::EmptyUrl));
    }

    #[test]
    fn in_memory_chromatine_repo_saves_and_finds() {
        let genome_id = GenomeId::new();
        let id = uuid::Uuid::new_v4();
        let chromatine = Chromatine::new(
            id,
            "https://docs.example.com/prd.pdf".to_string(),
            genome_id,
        ).unwrap();

        let mut repo = InMemoryChromatineRepository::new();
        repo.save(chromatine);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.url(), "https://docs.example.com/prd.pdf");
    }
}