use clap::Parser;

#[derive(Parser)]
#[command(name = "rnap")]
#[command(about = "Requirements Normalization and Assessment Platform")]
pub enum Cli {
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
}

pub struct CreateGeneResult {
    pub gene_id: uuid::Uuid,
    pub gene_name: String,
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
}
