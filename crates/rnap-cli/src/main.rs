use clap::Parser;
use rnap_cli::Cli;
use rnap_gene::{By, Gene, GeneRepository, GeneService, Mutation};
use rnap_genome::GenomeId;
use rnap_genotype::{Genotype, GenotypeRepository, Trait, TraitState};
use rnap_storage::{PostgresGenotypeRepository, PostgresGeneRepository};
use std::env;

fn main() {
    let cli = Cli::parse();

    // Get DATABASE_URL from env or use default
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://rnap:rnap@localhost:5432/rnap".to_string()
    });

    // Connect to Postgres (sync wrapper)
    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool = rt.block_on(sqlx::PgPool::connect(&db_url)).expect("Failed to connect to DB");

    let genotype_repo = PostgresGenotypeRepository::new(pool.clone());
    let gene_repo = PostgresGeneRepository::new(pool.clone());

    // Default genome ID (seeded in DB)
    let default_genome_id = GenomeId::from(
        uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
    );

    match cli {
        Cli::Create { kind, name } => {
            let result = rt.block_on(async {
                let genotype = genotype_repo.find_by_kind(&kind).await;
                match genotype {
                    Some(genotype) => {
                        // Use known genotype ID from seeded DB (00000000-0000-0000-0000-000000000002)
                        let genotype_id = GenomeId::from(
                            uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
                        );
                        
                        let gene_repo = PostgresGeneRepository::new(pool.clone());
                        let next_seq = gene_repo.next_sequence_for_kind(&default_genome_id, &kind).await?;
                        let slug: String = name.to_lowercase().replace(' ', "-");
                        let gene_name = format!("{}-{:04}-{}", kind, next_seq, slug);
                        let gene_id = uuid::Uuid::new_v4();
                        let gene = Gene::new(
                            gene_id,
                            gene_name.clone(),
                            default_genome_id,
                            genotype_id,
                        );

                        gene_repo.save(&gene).await?;

                        Ok(rnap_cli::CreateGeneResult { gene_id, gene_name })
                    }
                    None => Err(format!("genotype kind '{}' not found", kind))
                }
            });

            match result {
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
        Cli::Mutate { gene, args, by, .. } => {
            let result: Result<(), String> = rt.block_on(async {
                let gene = gene_repo.find_by_name(&gene).await
                    .ok_or_else(|| format!("gene '{}' not found", gene))?;

                // For now, get any FEAT genotype (simplified - in real app, get by genotype_id)
                let genotype = genotype_repo.find_by_kind("FEAT").await
                    .ok_or_else(|| "FEAT genotype not found".to_string())?;

                let mut mutable_gene = Gene::new(
                    *gene.id(),
                    gene.name().to_string(),
                    *gene.genome_id(),
                    *gene.genotype_id(),
                );

                // Parse args: "title=Hello world" -> trait_key="title", value="Hello world"
                for arg in args {
                    if let Some((key, value)) = arg.split_once('=') {
                        let mutation = Mutation::new(
                            uuid::Uuid::new_v4(),
                            *gene.id(),
                            key.to_string(),
                            serde_json::json!(value),
                            if by == "llm" { By::Llm } else { By::Human },
                            format!("via CLI: {}", arg),
                            chrono::Utc::now(),
                        );

                        GeneService::validate_and_append(&mut mutable_gene, mutation.clone(), &genotype)
                            .map_err(|e| e.to_string())?;
                        
                        // Save mutation to DB
                        gene_repo.save_mutation(&mutation).await?;
                    }
                }

                println!("Mutation applied to {}", gene.name());

                Ok(())
            });

            if let Err(e) = result {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Cli::Transcribe { gene } => {
            let result: Result<(), String> = rt.block_on(async {
                let gene = gene_repo.find_by_name(&gene).await
                    .ok_or_else(|| format!("gene '{}' not found", gene))?;

                let genotype = genotype_repo.find_by_kind("FEAT").await
                    .ok_or_else(|| "FEAT genotype not found".to_string())?;

                // Load mutations from DB
                let mutations = gene_repo.find_mutations_by_gene(gene.id()).await;
                
                // Build state from mutations
                let mut state: std::collections::HashMap<&str, &Mutation> = std::collections::HashMap::new();
                for m in &mutations {
                    state.insert(m.trait_key(), m);
                }

                println!("Gene: {}", gene.name());
                println!("ID: {}", gene.id());
                println!("Kind: {}", genotype.kind());
                println!("Generation: {}", genotype.generation());
                println!("");
                println!("Traits:");
                for t in genotype.traits() {
                    println!("  - {} ({:?})", t.key(), t.state());
                }
                println!("");
                println!("State:");
                if state.is_empty() {
                    println!("  (no mutations)");
                } else {
                    for (key, m) in &state {
                        println!("  {} = {}", key, m.value());
                    }
                }

                Ok(())
            });

            if let Err(e) = result {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}