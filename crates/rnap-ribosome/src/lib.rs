pub mod ribosome;
pub mod rrna;

pub use ribosome::{InMemoryRibosomeRepository, Ribosome, RibosomeError, RibosomeRepository};
pub use rrna::{InMemoryRrnaRepository, Rrna, RrnaError, RrnaRepository};

#[cfg(test)]
mod tests {
    use super::*;
    use rnap_genome::GenomeId;
    use serde_json::json;

    fn genome_id() -> GenomeId {
        GenomeId::new()
    }

    // ── Ribosome tests ──

    #[test]
    fn ribosome_can_be_created_with_name_and_config() {
        let gid = genome_id();
        let config = json!({"steps": ["lint", "test"]});

        let ribosome = Ribosome::new(
            uuid::Uuid::new_v4(),
            "CI".to_string(),
            config.clone(),
            uuid::Uuid::new_v4(), // phenome_id
            gid,
        )
        .unwrap();

        assert_eq!(ribosome.name(), "CI");
        assert_eq!(ribosome.config(), &config);
    }

    #[test]
    fn ribosome_rejects_empty_name() {
        let result = Ribosome::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            json!({}),
            uuid::Uuid::new_v4(),
            genome_id(),
        );
        assert_eq!(result, Err(RibosomeError::EmptyName));
    }

    #[test]
    fn in_memory_ribosome_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let ribosome = Ribosome::new(
            id,
            "Security Scan".to_string(),
            json!({"scanner": "bandit"}),
            uuid::Uuid::new_v4(),
            genome_id(),
        )
        .unwrap();

        let mut repo = InMemoryRibosomeRepository::new();
        repo.save(ribosome);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.name(), "Security Scan");
    }

    // ── rRNA tests ──

    #[test]
    fn rrna_can_be_created_with_criteria() {
        let gid = genome_id();
        let rrna = Rrna::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(), // ribosome_id
            uuid::Uuid::new_v4(), // gene_id
            "coverage >= 90%".to_string(),
            gid,
        )
        .unwrap();

        assert_eq!(rrna.criteria(), "coverage >= 90%");
    }

    #[test]
    fn rrna_rejects_empty_criteria() {
        let result = Rrna::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            genome_id(),
        );
        assert_eq!(result, Err(RrnaError::EmptyCriteria));
    }

    #[test]
    fn in_memory_rrna_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let rrna = Rrna::new(
            id,
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "no clippy warnings".to_string(),
            genome_id(),
        )
        .unwrap();

        let mut repo = InMemoryRrnaRepository::new();
        repo.save(rrna);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.criteria(), "no clippy warnings");
    }
}