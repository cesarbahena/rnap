#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Dna {
    id: uuid::Uuid,
    content: String,
    chromatine_refs: Vec<String>,
    genome_id: rnap_genome::GenomeId,
}

impl Dna {
    pub fn new(id: uuid::Uuid, content: String, genome_id: rnap_genome::GenomeId) -> Result<Self, DnaError> {
        if content.trim().is_empty() {
            return Err(DnaError::EmptyContent);
        }
        Ok(Self {
            id,
            content,
            chromatine_refs: vec![],
            genome_id,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn chromatine_refs(&self) -> &[String] {
        &self.chromatine_refs
    }

    pub fn add_chromatine_ref(&mut self, url: String) {
        if !url.trim().is_empty() && !self.chromatine_refs.contains(&url) {
            self.chromatine_refs.push(url);
        }
    }

    pub fn genome_id(&self) -> &rnap_genome::GenomeId {
        &self.genome_id
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DnaError {
    #[error("DNA content must not be empty")]
    EmptyContent,
}

pub trait DnaRepository {
    fn save(&mut self, dna: Dna);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Dna>;
}

pub struct InMemoryDnaRepository {
    entries: std::collections::HashMap<uuid::Uuid, Dna>,
}

impl InMemoryDnaRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryDnaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl DnaRepository for InMemoryDnaRepository {
    fn save(&mut self, dna: Dna) {
        self.entries.insert(*dna.id(), dna);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Dna> {
        self.entries.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rnap_genome::GenomeId;

    #[test]
    fn dna_can_be_created_with_id_content_and_genome_id() {
        let genome_id = GenomeId::new();
        let dna = Dna::new(
            uuid::Uuid::new_v4(),
            "Users must be able to reset their password".to_string(),
            genome_id,
        ).unwrap();

        assert_eq!(
            dna.content(),
            "Users must be able to reset their password"
        );
        assert_eq!(dna.genome_id(), &genome_id);
    }

    #[test]
    fn dna_rejects_empty_content() {
        let genome_id = GenomeId::new();
        let result = Dna::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            genome_id,
        );
        assert_eq!(result, Err(DnaError::EmptyContent));
    }

    #[test]
    fn dna_can_be_created_with_empty_chromatine_refs() {
        let genome_id = GenomeId::new();
        let dna = Dna::new(
            uuid::Uuid::new_v4(),
            "Users must authenticate before accessing data".to_string(),
            genome_id,
        ).unwrap();

        assert!(dna.chromatine_refs().is_empty());
    }

    #[test]
    fn dna_add_chromatine_ref_appends_without_duplicates() {
        let genome_id = GenomeId::new();
        let mut dna = Dna::new(
            uuid::Uuid::new_v4(),
            "Users must authenticate before accessing data".to_string(),
            genome_id,
        ).unwrap();

        dna.add_chromatine_ref("https://docs.example.com/prd.pdf".to_string());
        dna.add_chromatine_ref("https://docs.example.com/prd.pdf".to_string()); // duplicate ignored
        dna.add_chromatine_ref("https://stakeholder.interview/notes".to_string());

        assert_eq!(dna.chromatine_refs().len(), 2);
        assert_eq!(dna.chromatine_refs()[0], "https://docs.example.com/prd.pdf");
        assert_eq!(dna.chromatine_refs()[1], "https://stakeholder.interview/notes");
    }

    #[test]
    fn dna_ignores_empty_chromatine_refs() {
        let genome_id = GenomeId::new();
        let mut dna = Dna::new(
            uuid::Uuid::new_v4(),
            "Some requirement".to_string(),
            genome_id,
        ).unwrap();

        dna.add_chromatine_ref("   ".to_string());

        assert!(dna.chromatine_refs().is_empty());
    }

    #[test]
    fn in_memory_dna_repo_saves_and_finds_dna() {
        let genome_id = GenomeId::new();
        let id = uuid::Uuid::new_v4();
        let dna = Dna::new(
            id,
            "Users must be able to reset their password".to_string(),
            genome_id,
        ).unwrap();

        let mut repo = InMemoryDnaRepository::new();
        repo.save(dna);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.content(), "Users must be able to reset their password");
    }
}