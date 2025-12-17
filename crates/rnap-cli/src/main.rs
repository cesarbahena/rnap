use clap::Parser;
use rnap_cli::Cli;
use rnap_dna::Dna;
use rnap_chromosome::Chromosome;
use rnap_quiasma::{Quiasma, RelationshipType, SourceType, TargetType};
use rnap_gene::{By, Gene, GeneService, Mutation};
use rnap_genome::GenomeId;
use rnap_storage::{
    PostgresDnaRepository,
    PostgresChromosomeRepository,
    PostgresQuiasmaRepository,
    PostgresGenotypeRepository,
    PostgresGeneRepository,
};
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
    let dna_repo = PostgresDnaRepository::new(pool.clone());
    let chromosome_repo = PostgresChromosomeRepository::new(pool.clone());
    let quiasma_repo = PostgresQuiasmaRepository::new(pool.clone());

    // Default genome ID (seeded in DB)
    let default_genome_id = GenomeId::from(
        uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
    );

    match cli {
        Cli::Seed { path } => {
            let seeds_path = path.unwrap_or_else(|| "seeds".to_string());
            let result = rt.block_on(async {
                rnap_cli::run_seeds(&pool, &seeds_path).await
            });
            match result {
                Ok(count) => println!("Seeded {} file(s)", count),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Cli::Create { kind, name } => {
            let result = rt.block_on(async {
                let genotype = genotype_repo.find_by_kind(&kind).await;
                match genotype {
                    Some(_genotype) => {
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
                let exact = gene_repo.find_by_name(&gene).await;
                let prefix = gene_repo.find_by_name_prefix(&gene).await;
                let found = exact.or(prefix).ok_or_else(|| format!("gene '{}' not found", gene))?;

                let genotype = genotype_repo.find_by_kind("FEAT").await
                    .ok_or_else(|| "FEAT genotype not found".to_string())?;

                let mut mutable_gene = Gene::new(
                    *found.id(),
                    found.name().to_string(),
                    *found.genome_id(),
                    *found.genotype_id(),
                );

                for arg in args {
                    if let Some((key, value)) = arg.split_once('=') {
                        let mutation = Mutation::new(
                            uuid::Uuid::new_v4(),
                            *found.id(),
                            key.to_string(),
                            serde_json::json!(value),
                            if by == "llm" { By::Llm } else { By::Human },
                            format!("via CLI: {}", arg),
                            chrono::Utc::now(),
                        );

                        GeneService::validate_and_append(&mut mutable_gene, mutation.clone(), &genotype)
                            .map_err(|e| e.to_string())?;
                        
                        gene_repo.save_mutation(&mutation).await?;
                    }
                }

                println!("Mutation applied to {}", found.name());
                Ok(())
            });

            if let Err(e) = result {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Cli::Transcribe { gene } => {
            let result: Result<(), String> = rt.block_on(async {
                let exact = gene_repo.find_by_name(&gene).await;
                let prefix = gene_repo.find_by_name_prefix(&gene).await;
                let found = exact.or(prefix).ok_or_else(|| format!("gene '{}' not found", gene))?;

                let genotype = genotype_repo.find_by_kind("FEAT").await
                    .ok_or_else(|| "FEAT genotype not found".to_string())?;

                let mutations = gene_repo.find_mutations_by_gene(found.id()).await;
                
                let mut state: std::collections::HashMap<&str, &Mutation> = std::collections::HashMap::new();
                for m in &mutations {
                    state.insert(m.trait_key(), m);
                }

                println!("Gene: {}", found.name());
                println!("ID: {}", found.id());
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
        Cli::Dna { subcommand } => {
            let result: Result<(), String> = rt.block_on(async {
                match subcommand {
                    rnap_cli::DnaSubcommand::Create { content } => {
                        let dna = Dna::new(
                            uuid::Uuid::new_v4(),
                            content.clone(),
                            default_genome_id,
                        ).map_err(|e| e.to_string())?;
                        
                        dna_repo.save(&dna).await?;
                        
                        println!("Created DNA: {}", dna.id());
                        println!("Content: {}", content);
                        Ok(())
                    }
                    rnap_cli::DnaSubcommand::List => {
                        let entries = dna_repo.find_by_genome(&default_genome_id).await;
                        
                        println!("DNA Entries:");
                        if entries.is_empty() {
                            println!("  (no entries)");
                        } else {
                            for dna in entries {
                                println!("  - {}: {}", dna.id(), dna.content());
                            }
                        }
                        Ok(())
                    }
                }
            });

            if let Err(e) = result {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Cli::Chromosome { subcommand } => {
            let result: Result<(), String> = rt.block_on(async {
                match subcommand {
                    rnap_cli::ChromosomeSubcommand::Create { name, description } => {
                        let desc = description.unwrap_or_default();
                        let chromosome = Chromosome::new(
                            uuid::Uuid::new_v4(),
                            name.clone(),
                            desc.clone(),
                            default_genome_id,
                        ).map_err(|e| e.to_string())?;
                        
                        chromosome_repo.save(&chromosome).await?;
                        
                        println!("Created Chromosome: {}", chromosome.id());
                        println!("Name: {}", name);
                        if !desc.is_empty() {
                            println!("Description: {}", desc);
                        }
                        Ok(())
                    }
                    rnap_cli::ChromosomeSubcommand::List => {
                        let entries = chromosome_repo.find_by_genome(&default_genome_id).await;
                        
                        println!("Chromosomes (Domain Nodes):");
                        if entries.is_empty() {
                            println!("  (no entries)");
                        } else {
                            for chrom in entries {
                                println!("  - {}: {}", chrom.id(), chrom.name());
                                if !chrom.description().is_empty() {
                                    println!("    {}", chrom.description());
                                }
                            }
                        }
                        Ok(())
                    }
                }
            });

            if let Err(e) = result {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Cli::Quiasma { subcommand } => {
            let result: Result<(), String> = rt.block_on(async {
                match subcommand {
                    rnap_cli::QuiasmaSubcommand::Create { source, target, rel_type, description } => {
                        // For demo, we'll create quiasma with placeholder IDs
                        // In a real app, we'd look up chromosome IDs by name
                        let source_id = uuid::Uuid::new_v4();
                        let target_id = uuid::Uuid::new_v4();
                        
                        let relationship = match rel_type.to_lowercase().as_str() {
                            "deliversto" => RelationshipType::DeliversTo,
                            "uses" => RelationshipType::Uses,
                            "dependson" => RelationshipType::DependsOn,
                            "calls" => RelationshipType::Calls,
                            "contains" => RelationshipType::Contains,
                            "attachto" => RelationshipType::AttachTo,
                            _ => return Err(format!("Unknown relationship type: {}", rel_type)),
                        };
                        
                        let quiasma = Quiasma::new(
                            uuid::Uuid::new_v4(),
                            source_id,
                            SourceType::Chromosome,
                            target_id,
                            TargetType::Chromosome,
                            relationship,
                            description.unwrap_or_default(),
                            default_genome_id,
                        ).map_err(|e| e.to_string())?;
                        
                        quiasma_repo.save(&quiasma).await?;
                        
                        println!("Created Quiasma (Relationship): {}", quiasma.id());
                        println!("Type: {} -> {}", source, target);
                        println!("Relationship: {}", rel_type);
                        Ok(())
                    }
                    rnap_cli::QuiasmaSubcommand::List => {
                        // For now, just show a message - would need find_all method
                        println!("Relationships:");
                        println!("  (use quiasma create to add relationships)");
                        Ok(())
                    }
                }
            });

            if let Err(e) = result {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
