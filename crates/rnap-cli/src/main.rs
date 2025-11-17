use clap::Parser;
use rnap_cli::Cli;
use rnap_gene::{Gene, GeneRepository, InMemoryGeneRepository};
use rnap_genome::GenomeId;
use rnap_genotype::{Genotype, GenotypeRepository, InMemoryGenotypeRepository, Trait, TraitState};
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    let cli = Cli::parse();

    // Seed a default genotype for FEAT
    let genome_id = GenomeId::new();
    let feat_genotype = Genotype::new(
        "FEAT".to_string(),
        "Feature Request".to_string(),
        1,
        genome_id,
        vec![
            Trait::new("title".to_string(), TraitState::Dominant),
            Trait::new("description".to_string(), TraitState::Recessive),
        ],
    )
    .unwrap();

    let mut genotype_repo = InMemoryGenotypeRepository::new(
        vec![("FEAT".to_string(), feat_genotype)]
            .into_iter()
            .collect(),
    );

    let mut gene_repo = InMemoryGeneRepository::new();

    match cli {
        Cli::Create { kind, name } => {
            match rnap_cli::create_gene(&mut genotype_repo, &mut gene_repo, genome_id, &kind, &name)
            {
                Ok(result) => {
                    println!("Created gene: {}", result.gene_name);
                    println!("Gene ID: {}", result.gene_id);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Cli::Mutate { .. } => {
            eprintln!("Mutate not implemented yet");
            std::process::exit(1);
        }
        Cli::Transcribe { .. } => {
            eprintln!("Transcribe not implemented yet");
            std::process::exit(1);
        }
    }
}
