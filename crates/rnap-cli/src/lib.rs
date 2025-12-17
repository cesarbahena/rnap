use clap::Parser;

#[derive(Parser)]
#[command(name = "rnap")]
#[command(about = "Requirements Normalization and Assessment Platform")]
pub enum Cli {
    Seed {
        #[arg(short, long, help = "Path to seeds directory")]
        path: Option<String>,
    },
    Create {
        #[arg(help = "Genotype kind (e.g., FEAT, BUG)")]
        kind: String,
        #[arg(help = "Gene name (e.g., user authentication)")]
        name: String,
    },
    Mutate {
        #[arg(help = "Gene name (e.g., FEAT-0001-user-auth)")]
        gene: String,
        #[arg(help = "Trait assignments (e.g., title=Hello) and context")]
        args: Vec<String>,
        #[arg(short = 'a', long, help = "Append to array trait")]
        append: bool,
        #[arg(short = 'r', long, help = "Replace collection trait")]
        replace: bool,
        #[arg(long, default_value = "human", help = "Author of the mutation")]
        by: String,
    },
    Transcribe {
        #[arg(help = "Gene name (e.g., FEAT-0001-user-auth)")]
        gene: String,
    },
    /// Manage structured requirements (DNA entries)
    Dna {
        #[command(subcommand)]
        subcommand: DnaSubcommand,
    },
    /// Manage domain nodes (Chromosomes)
    Chromosome {
        #[command(subcommand)]
        subcommand: ChromosomeSubcommand,
    },
    /// Manage relationships between domain nodes (Quiasmas)
    Quiasma {
        #[command(subcommand)]
        subcommand: QuiasmaSubcommand,
    },
}

#[derive(clap::Subcommand)]
pub enum DnaSubcommand {
    /// Create a new DNA entry
    Create {
        #[arg(help = "The requirement content")]
        content: String,
    },
    /// List all DNA entries
    List,
}

#[derive(clap::Subcommand)]
pub enum ChromosomeSubcommand {
    /// Create a new domain node
    Create {
        #[arg(help = "The chromosome name")]
        name: String,
        #[arg(help = "Description (optional)")]
        description: Option<String>,
    },
    /// List all chromosomes
    List,
}

#[derive(clap::Subcommand)]
pub enum QuiasmaSubcommand {
    /// Create a new relationship
    Create {
        #[arg(help = "Source chromosome name")]
        source: String,
        #[arg(help = "Target chromosome name")]
        target: String,
        #[arg(help = "Relationship type: DeliversTo, Uses, DependsOn, Calls, Contains, AttachTo")]
        rel_type: String,
        #[arg(help = "Description (optional)")]
        description: Option<String>,
    },
    /// List all relationships
    List,
}

#[derive(Debug)]
pub struct CreateGeneResult {
    pub gene_id: uuid::Uuid,
    pub gene_name: String,
}

pub async fn run_seeds(pool: &sqlx::PgPool, seeds_path: &str) -> Result<usize, String> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(seeds_path);
    if !path.exists() {
        return Err(format!("Seeds directory not found: {}", seeds_path));
    }

    let mut ran = 0;
    let mut entries: Vec<_> = fs::read_dir(path)
        .map_err(|e| format!("Failed to read seeds directory: {}", e))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "sql"))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let sql = fs::read_to_string(entry.path())
            .map_err(|e| format!("Failed to read seed file: {}", e))?;
        
        sqlx::query(&sql)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to execute seed {}: {}", entry.file_name().to_string_lossy(), e))?;
        
        println!("Ran seed: {}", entry.file_name().to_string_lossy());
        ran += 1;
    }

    Ok(ran)
}

pub fn create_gene(
    genotype_repo: &dyn rnap_genotype::GenotypeRepository,
    gene_repo: &mut dyn rnap_gene::GeneRepository,
    genome_id: rnap_genome::GenomeId,
    kind: &str,
    name: &str,
) -> Result<CreateGeneResult, String> {
    let genotype = genotype_repo
        .find_by_kind(kind)
        .ok_or_else(|| format!("genotype kind '{}' not found", kind))?;

    let slug: String = name.to_lowercase().replace(' ', "-");
    let gene_name = format!("{}-0001-{}", kind, slug);
    let gene_id = uuid::Uuid::new_v4();
    let gene = rnap_gene::Gene::new(gene_id, gene_name.clone(), genome_id, *genotype.genome_id());

    gene_repo.save(gene);

    Ok(CreateGeneResult { gene_id, gene_name })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_gene_finds_genotype_and_saves_gene() {
        use rnap_gene::GeneRepository;
        use rnap_genotype::GenotypeRepository;

        let genome_id = rnap_genome::GenomeId::new();
        let genotype = rnap_genotype::Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            genome_id,
            vec![rnap_genotype::Trait::new(
                "title".to_string(),
                rnap_genotype::TraitState::Dominant,
            )],
        )
        .unwrap();

        let genotype_repo = rnap_genotype::InMemoryGenotypeRepository::new(
            vec![("FEAT".to_string(), genotype)].into_iter().collect(),
        );

        let mut gene_repo = rnap_gene::InMemoryGeneRepository::new();

        let result = create_gene(
            &genotype_repo,
            &mut gene_repo,
            genome_id,
            "FEAT",
            "user authentication",
        );

        assert!(result.is_ok());
        let created = result.unwrap();
        assert!(created.gene_name.starts_with("FEAT-0001-"));

        let saved = gene_repo.find_by_id(&created.gene_id).unwrap();
        assert_eq!(saved.name(), created.gene_name);
    }

    #[test]
    fn create_gene_returns_error_when_kind_not_found() {
        let genotype_repo =
            rnap_genotype::InMemoryGenotypeRepository::new(std::collections::HashMap::new());
        let mut gene_repo = rnap_gene::InMemoryGeneRepository::new();
        let genome_id = rnap_genome::GenomeId::new();

        let result = create_gene(
            &genotype_repo,
            &mut gene_repo,
            genome_id,
            "BUG",
            "crash report",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn transcribe_gene_computes_state() {
        use rnap_gene::GeneRepository;
        use rnap_genotype::GenotypeRepository;

        let genome_id = rnap_genome::GenomeId::new();
        let genotype = rnap_genotype::Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            genome_id,
            vec![rnap_genotype::Trait::new(
                "title".to_string(),
                rnap_genotype::TraitState::Dominant,
            )],
        )
        .unwrap();

        let genotype_repo = rnap_genotype::InMemoryGenotypeRepository::new(
            vec![("FEAT".to_string(), genotype.clone())]
                .into_iter()
                .collect(),
        );

        let mut gene_repo = rnap_gene::InMemoryGeneRepository::new();
        let gene_name = "FEAT-0001-user-auth".to_string();
        let mut gene = rnap_gene::Gene::new(
            uuid::Uuid::new_v4(),
            gene_name.clone(),
            genome_id,
            *genotype.genome_id(),
        );
        gene_repo.save(gene);

        let stored = gene_repo.find_by_name(&gene_name).unwrap();
        let state = rnap_gene::GeneService::current_state(stored);

        assert!(state.is_empty());
    }

    #[test]
    fn transcribe_after_mutation_shows_updated_value() {
        use rnap_gene::{By, GeneRepository, GeneService, Mutation};
        use rnap_genotype::GenotypeRepository;

        let genome_id = rnap_genome::GenomeId::new();
        let genotype = rnap_genotype::Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            genome_id,
            vec![rnap_genotype::Trait::new(
                "title".to_string(),
                rnap_genotype::TraitState::Dominant,
            )],
        )
        .unwrap();

        let genotype_repo = rnap_genotype::InMemoryGenotypeRepository::new(
            vec![("FEAT".to_string(), genotype.clone())]
                .into_iter()
                .collect(),
        );

        let mut gene_repo = rnap_gene::InMemoryGeneRepository::new();
        let mut gene = rnap_gene::Gene::new(
            uuid::Uuid::new_v4(),
            "FEAT-0001-user-auth".to_string(),
            genome_id,
            *genotype.genome_id(),
        );

        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            *gene.id(),
            "title".to_string(),
            serde_json::json!("User authentication flow"),
            By::Human,
            "Initial requirement".to_string(),
            chrono::Utc::now(),
        );

        GeneService::validate_and_append(&mut gene, mutation, &genotype).unwrap();
        gene_repo.save(gene);

        let stored = gene_repo.find_by_name("FEAT-0001-user-auth").unwrap();
        let state = GeneService::current_state(stored);

        assert_eq!(state.len(), 1);
        assert_eq!(
            state.get("title").unwrap().value(),
            &serde_json::json!("User authentication flow")
        );
    }
}
