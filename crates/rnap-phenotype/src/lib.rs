pub mod phenotype;
pub mod protein;

pub use phenotype::{InMemoryPhenotypeRepository, Phenotype, PhenotypeRepository};
pub use protein::{InMemoryProteinRepository, Protein, ProteinResult, ProteinRepository};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rnap_genome::GenomeId;

    fn genome_id() -> GenomeId {
        GenomeId::new()
    }

    // ── Phenotype tests ──

    #[test]
    fn phenotype_can_be_created_with_git_info() {
        let mrna_id = uuid::Uuid::new_v4();
        let now = Utc::now();

        let phenotype = Phenotype::new(
            uuid::Uuid::new_v4(),
            mrna_id,
            "a1b2c3d4e5f6".to_string(),
            "/repos/rnap".to_string(),
            "main".to_string(),
            "origin".to_string(),
            now,
        )
        .unwrap();

        assert_eq!(phenotype.mrna_id(), &mrna_id);
        assert_eq!(phenotype.commit_sha(), "a1b2c3d4e5f6");
        assert_eq!(phenotype.branch(), "main");
        assert_eq!(phenotype.remote(), "origin");
    }

    #[test]
    fn phenotype_rejects_empty_commit_sha() {
        let result = Phenotype::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            "/repos/rnap".to_string(),
            "main".to_string(),
            "origin".to_string(),
            Utc::now(),
        );
        assert!(!result.is_ok());
    }

    #[test]
    fn in_memory_phenotype_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let phenotype = Phenotype::new(
            id,
            uuid::Uuid::new_v4(),
            "abc123".to_string(),
            "/repo".to_string(),
            "main".to_string(),
            "origin".to_string(),
            Utc::now(),
        )
        .unwrap();

        let mut repo = InMemoryPhenotypeRepository::new();
        repo.save(phenotype);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.commit_sha(), "abc123");
    }

    // ── Protein tests ──

    #[test]
    fn protein_can_be_created_as_pending() {
        let gid = genome_id();
        let now = Utc::now();

        let protein = Protein::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(), // phenotype_id
            uuid::Uuid::new_v4(), // phenome_id
            uuid::Uuid::new_v4(), // gene_id
            ProteinResult::Pending,
            gid,
            now,
        );

        assert_eq!(protein.result(), &ProteinResult::Pending);
    }

    #[test]
    fn protein_result_variants() {
        assert!(matches!(ProteinResult::Pass, ProteinResult::Pass));
        assert!(matches!(ProteinResult::Fail, ProteinResult::Fail));
        assert!(matches!(ProteinResult::Pending, ProteinResult::Pending));
    }

    #[test]
    fn in_memory_protein_repo_saves_and_finds() {
        let id = uuid::Uuid::new_v4();
        let protein = Protein::new(
            id,
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            ProteinResult::Pass,
            genome_id(),
            Utc::now(),
        );

        let mut repo = InMemoryProteinRepository::new();
        repo.save(protein);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.result(), &ProteinResult::Pass);
    }
}