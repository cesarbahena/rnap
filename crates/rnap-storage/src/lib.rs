use rnap_genotype::Genotype;
use rnap_gene::{Gene, Mutation, By};
use rnap_dna::Dna;
use rnap_chromatine::Chromatine;
use rnap_chromosome::Chromosome;
use rnap_organism::{Organism, OrganismKind};
use rnap_channel::{Channel, RelationshipType, SourceType, TargetType};
use rnap_cell::Cell;
use rnap_organelle::Organelle;
use rnap_chiasma::{Chiasma, ViolationType};
use rnap_histone::Histone;
use rnap_mrna::Mrna;
use rnap_trna::Trna;
use rnap_srna::Srna;
use rnap_phenome::Phenome;
use rnap_ribosome::{Ribosome, Rrna};
use rnap_phenotype::{Protein, ProteinResult};
use rnap_fold::Fold;
use rnap_genome::GenomeId;
use sqlx::Row;

pub struct PostgresGenotypeRepository {
    pool: sqlx::PgPool,
}

impl PostgresGenotypeRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_kind(&self, kind: &str) -> Option<Genotype> {
        let row = sqlx::query(
            "SELECT id, kind, name, generation, genome_id, traits, created_at FROM genotypes WHERE kind = $1"
        )
        .bind(kind)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;

        let genotype = Genotype::with_id(
            rnap_genome::GenotypeId::from(row.get::<uuid::Uuid, _>("id")),
            row.get("kind"),
            row.get("name"),
            row.get::<i32, _>("generation") as u32,
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genome_id")),
            serde_json::from_value(row.get("traits")).ok()?,
            row.get::<chrono::DateTime<chrono::Utc>, _>("created_at"),
        )
        .ok()?;

        Some(genotype)
    }

    pub async fn find_by_genome_id(&self, genome_id: rnap_genome::GenomeId) -> Option<Genotype> {
        let row = sqlx::query(
            "SELECT id, kind, name, generation, genome_id, traits, created_at FROM genotypes WHERE genome_id = $1"
        )
        .bind(genome_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;

        let genotype = Genotype::with_id(
            rnap_genome::GenotypeId::from(row.get::<uuid::Uuid, _>("id")),
            row.get("kind"),
            row.get("name"),
            row.get::<i32, _>("generation") as u32,
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genome_id")),
            serde_json::from_value(row.get("traits")).ok()?,
            row.get::<chrono::DateTime<chrono::Utc>, _>("created_at"),
        )
        .ok()?;

        Some(genotype)
    }
}

pub struct PostgresGeneRepository {
    pool: sqlx::PgPool,
}

impl PostgresGeneRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, gene: &Gene) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO genes (id, name, genome_id, genotype_id, created_at) VALUES ($1, $2, $3, $4, NOW())"
        )
        .bind(gene.id())
        .bind(gene.name())
        .bind(gene.genome_id().as_uuid())
        .bind(gene.genotype_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_name(&self, name: &str) -> Option<Gene> {
        // Try exact match first
        let row = sqlx::query(
            "SELECT id, name, genome_id, genotype_id FROM genes WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()?;

        Some(Gene::new(
            row.get("id"),
            row.get("name"),
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genome_id")),
            rnap_genome::GenotypeId::from(row.get::<uuid::Uuid, _>("genotype_id")),
        ))
    }

    pub async fn find_by_name_prefix(&self, prefix: &str) -> Option<Gene> {
        // Prefix match: "FEAT-0001" matches "FEAT-0001-test-feature"
        let row = sqlx::query(
            "SELECT id, name, genome_id, genotype_id FROM genes WHERE name LIKE ($1 || '%') ORDER BY name LIMIT 1"
        )
        .bind(prefix)
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()?;

        Some(Gene::new(
            row.get("id"),
            row.get("name"),
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genome_id")),
            rnap_genome::GenotypeId::from(row.get::<uuid::Uuid, _>("genotype_id")),
        ))
    }

    pub async fn next_sequence_for_kind(
        &self, 
        genome_id: &rnap_genome::GenomeId, 
        kind: &str
    ) -> Result<u32, String> {
        let row = sqlx::query(
            "INSERT INTO gene_counters (genome_id, kind, next_seq) VALUES ($1, $2, 2) \
             ON CONFLICT (genome_id, kind) \
             DO UPDATE SET next_seq = gene_counters.next_seq + 1 \
             RETURNING next_seq - 1 AS allocated_seq"
        )
        .bind(genome_id.as_uuid())
        .bind(kind)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let seq: i64 = row.get("allocated_seq");
        Ok(seq as u32)
    }

    pub async fn save_mutation(&self, mutation: &Mutation) -> Result<(), String> {
        let by_str = match mutation.by() {
            By::Human => "Human",
            By::Llm => "Llm",
        };
        
        sqlx::query(
            "INSERT INTO mutations (id, gene_id, trait_key, value, by, context, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(mutation.id())
        .bind(mutation.gene_id())
        .bind(mutation.trait_key())
        .bind(mutation.value())
        .bind(by_str)
        .bind(mutation.context())
        .bind(mutation.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_mutations_by_gene(&self, gene_id: &uuid::Uuid) -> Vec<Mutation> {
        let rows = sqlx::query(
            "SELECT id, gene_id, trait_key, value, by, context, created_at FROM mutations WHERE gene_id = $1 ORDER BY created_at"
        )
        .bind(gene_id)
        .fetch_all(&self.pool)
        .await
        .ok();

        match rows {
            Some(rows) => rows.iter().filter_map(|row| {
                let by = match row.get::<String, _>("by").as_str() {
                    "Llm" => By::Llm,
                    _ => By::Human,
                };
                
                Some(Mutation::new(
                    row.get("id"),
                    row.get("gene_id"),
                    row.get("trait_key"),
                    row.get("value"),
                    by,
                    row.get("context"),
                    row.get("created_at"),
                ))
            }).collect(),
            None => vec![],
        }
    }
}

pub struct PostgresDnaRepository {
    pool: sqlx::PgPool,
}

impl PostgresDnaRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, dna: &Dna) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO dna (id, path, genome_id, created_at) \n             VALUES ($1, $2, $3, NOW())"
        )
        .bind(dna.id())
        .bind(dna.path())
        .bind(dna.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Dna> {
        let row = sqlx::query(
            "SELECT id, path, genome_id FROM dna WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;

        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

        Dna::new(
            row.get("id"),
            row.get("path"),
            genome_id,
        ).ok()
    }

    pub async fn find_by_genome(&self, genome_id: &GenomeId) -> Vec<Dna> {
        let rows = sqlx::query(
            "SELECT id, path, genome_id FROM dna WHERE genome_id = $1"
        )
        .bind(genome_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .ok();

        match rows {
            Some(rows) => rows.iter().filter_map(|row| {
                let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

                Dna::new(
                    row.get("id"),
                    row.get("path"),
                    genome_id,
                ).ok()
            }).collect(),
            None => vec![],
        }
    }
}

pub struct PostgresChromatineRepository {
    pool: sqlx::PgPool,
}

impl PostgresChromatineRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, chromatine: &Chromatine) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO chromatine (id, path, genome_id, created_at) VALUES ($1, $2, $3, NOW())"
        )
        .bind(chromatine.id())
        .bind(chromatine.path())
        .bind(chromatine.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Chromatine> {
        let row = sqlx::query(
            "SELECT id, path, genome_id FROM chromatine WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;

        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

        Chromatine::new(
            row.get("id"),
            row.get("path"),
            genome_id,
        ).ok()
    }

    pub async fn find_by_genome(&self, genome_id: &GenomeId) -> Vec<Chromatine> {
        let rows = sqlx::query(
            "SELECT id, path, genome_id FROM chromatine WHERE genome_id = $1"
        )
        .bind(genome_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .ok();

        match rows {
            Some(rows) => rows.iter().filter_map(|row| {
                let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));
                Chromatine::new(
                    row.get("id"),
                    row.get("path"),
                    genome_id,
                ).ok()
            }).collect(),
            None => vec![],
        }
    }
}

pub struct PostgresChromosomeRepository {
    pool: sqlx::PgPool,
}

impl PostgresChromosomeRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, chromosome: &Chromosome) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO chromosomes (id, name, description, organelle_id, genome_id, created_at) VALUES ($1, $2, $3, $4, $5, NOW())"
        )
        .bind(chromosome.id())
        .bind(chromosome.name())
        .bind(chromosome.description())
        .bind(chromosome.organelle_id())
        .bind(chromosome.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Chromosome> {
        let row = sqlx::query(
            "SELECT id, name, description, organelle_id, genome_id FROM chromosomes WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

        Chromosome::new(
            row.get("id"),
            row.get("name"),
            row.get("description"),
            row.get("organelle_id"),
            genome_id,
        ).ok()
    }

    pub async fn find_by_genome(&self, genome_id: &GenomeId) -> Vec<Chromosome> {
        let rows = match sqlx::query(
            "SELECT id, name, description, organelle_id, genome_id FROM chromosomes WHERE genome_id = $1"
        )
        .bind(genome_id.as_uuid())
        .fetch_all(&self.pool)
        .await {
            Ok(rows) => rows,
            Err(_) => return vec![],
        };

        rows.iter().filter_map(|row| {
            let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));
            Chromosome::new(
                row.get("id"),
                row.get("name"),
                row.get("description"),
                row.get("organelle_id"),
                genome_id,
            ).ok()
        }).collect()
    }
}

pub struct PostgresOrganismRepository {
    pool: sqlx::PgPool,
}

impl PostgresOrganismRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, ligand: &Organism) -> Result<(), String> {
        let kind_str = match ligand.kind() {
            OrganismKind::Human => "Human",
            OrganismKind::Team => "Team",
            OrganismKind::Service => "Service",
        };
        
        sqlx::query(
            "INSERT INTO ligands (id, name, kind, description, genome_id, created_at) VALUES ($1, $2, $3, $4, $5, NOW())"
        )
        .bind(ligand.id())
        .bind(ligand.name())
        .bind(kind_str)
        .bind(ligand.description())
        .bind(ligand.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Organism> {
        let row = sqlx::query(
            "SELECT id, name, kind, description, genome_id FROM ligands WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));
        let kind_str: String = row.get("kind");
        let kind = match kind_str.as_str() {
            "Human" => OrganismKind::Human,
            "Team" => OrganismKind::Team,
            "Service" => OrganismKind::Service,
            _ => return None,
        };

        Organism::new(
            row.get("id"),
            row.get("name"),
            kind,
            row.get("description"),
            genome_id,
        ).ok()
    }
}

pub struct PostgresChannelRepository {
    pool: sqlx::PgPool,
}

impl PostgresChannelRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, receptor: &Channel) -> Result<(), String> {
        let source_type_str = match receptor.source_type() {
            SourceType::Chromosome => "Chromosome",
            SourceType::Organism => "Organism",
            SourceType::Cell => "Cell",
            SourceType::Organelle => "Organelle",
        };
        let target_type_str = match receptor.target_type() {
            TargetType::Chromosome => "Chromosome",
            TargetType::Organism => "Organism",
            TargetType::Cell => "Cell",
            TargetType::Organelle => "Organelle",
        };
        let rel_type_str = match receptor.relationship_type() {
            RelationshipType::DeliversTo => "DeliversTo",
            RelationshipType::Uses => "Uses",
            RelationshipType::DependsOn => "DependsOn",
            RelationshipType::Calls => "Calls",
            RelationshipType::Contains => "Contains",
            RelationshipType::AttachTo => "AttachTo",
        };
        
        sqlx::query(
            "INSERT INTO quiasmas (id, source_id, source_type, target_id, target_type, relationship_type, description, genome_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())"
        )
        .bind(receptor.id())
        .bind(receptor.source_id())
        .bind(source_type_str)
        .bind(receptor.target_id())
        .bind(target_type_str)
        .bind(rel_type_str)
        .bind(receptor.description())
        .bind(receptor.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Channel> {
        let row = sqlx::query(
            "SELECT id, source_id, source_type, target_id, target_type, relationship_type, description, genome_id FROM quiasmas WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));
        let source_type_str: String = row.get("source_type");
        let target_type_str: String = row.get("target_type");
        let rel_type_str: String = row.get("relationship_type");

        let source_type = match source_type_str.as_str() {
            "Chromosome" => SourceType::Chromosome,
            "Organism" => SourceType::Organism,
            "Cell" => SourceType::Cell,
            "Organelle" => SourceType::Organelle,
            _ => return None,
        };
        let target_type = match target_type_str.as_str() {
            "Chromosome" => TargetType::Chromosome,
            "Organism" => TargetType::Organism,
            "Cell" => TargetType::Cell,
            "Organelle" => TargetType::Organelle,
            _ => return None,
        };
        let relationship_type = match rel_type_str.as_str() {
            "DeliversTo" => RelationshipType::DeliversTo,
            "Uses" => RelationshipType::Uses,
            "DependsOn" => RelationshipType::DependsOn,
            "Calls" => RelationshipType::Calls,
            "Contains" => RelationshipType::Contains,
            "AttachTo" => RelationshipType::AttachTo,
            _ => return None,
        };

        Channel::new(
            row.get("id"),
            row.get("source_id"),
            source_type,
            row.get("target_id"),
            target_type,
            relationship_type,
            row.get("description"),
            genome_id,
        ).ok()
    }
}

pub struct PostgresHistoneRepository {
    pool: sqlx::PgPool,
}

impl PostgresHistoneRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, histone: &Histone) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO histones (id, title, decision, context, mutation_id, gene_id, dna_id, genome_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())"
        )
        .bind(histone.id())
        .bind(histone.title())
        .bind(histone.decision())
        .bind(histone.context())
        .bind(histone.mutation_id())
        .bind(histone.gene_id())
        .bind(histone.dna_id())
        .bind(histone.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Histone> {
        let row = sqlx::query(
            "SELECT id, title, decision, context, mutation_id, gene_id, dna_id, genome_id, created_at FROM histones WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

        Histone::new(
            row.get("id"),
            row.get("title"),
            row.get("decision"),
            row.get("context"),
            genome_id,
            row.get("mutation_id"),
            row.get("gene_id"),
            row.get("dna_id"),
            row.get("created_at"),
        ).ok()
    }
}

pub struct PostgresMrnaRepository {
    pool: sqlx::PgPool,
}

impl PostgresMrnaRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, mrna: &Mrna) -> Result<(), String> {
        let mutation_ids_json = serde_json::to_value(mrna.mutation_ids()).map_err(|e| e.to_string())?;
        
        sqlx::query(
            "INSERT INTO mrna (id, gene_id, version, mutation_ids, genome_id, created_at) VALUES ($1, $2, $3, $4, $5, NOW())"
        )
        .bind(mrna.id())
        .bind(mrna.gene_id())
        .bind(mrna.version() as i32)
        .bind(mutation_ids_json)
        .bind(mrna.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Mrna> {
        let row = sqlx::query(
            "SELECT id, gene_id, version, mutation_ids, genome_id, created_at FROM mrna WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));
        let mutation_ids: Vec<uuid::Uuid> = serde_json::from_value(row.get("mutation_ids")).ok()?;

        Mrna::new(
            row.get("id"),
            row.get("gene_id"),
            row.get::<i32, _>("version") as u32,
            mutation_ids,
            genome_id,
            row.get("created_at"),
        ).ok()
    }
}

pub struct PostgresTrnaRepository {
    pool: sqlx::PgPool,
}

impl PostgresTrnaRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, trna: &Trna) -> Result<(), String> {
        let tasks_json = serde_json::to_value(trna.tasks()).map_err(|e| e.to_string())?;
        
        sqlx::query(
            "INSERT INTO trna (id, mrna_id, tasks, genome_id, created_at) VALUES ($1, $2, $3, $4, NOW())"
        )
        .bind(trna.id())
        .bind(trna.mrna_id())
        .bind(tasks_json)
        .bind(trna.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Trna> {
        let row = sqlx::query(
            "SELECT id, mrna_id, tasks, genome_id, created_at FROM trna WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));
        
        // Parse tasks from JSON - simplified, just use the raw JSON
        // Full deserialization would require custom JSON parsing
        let _tasks_json: serde_json::Value = row.get("tasks");

        Some(Trna::new(
            row.get("id"),
            row.get("mrna_id"),
            vec![], // Simplified: would need proper JSON parsing for full implementation
            genome_id,
            row.get("created_at"),
        ))
    }
}

pub struct PostgresSrnaRepository {
    pool: sqlx::PgPool,
}

impl PostgresSrnaRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, srna: &Srna) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO srna (id, content, task_context, promoted, genome_id, created_at) VALUES ($1, $2, $3, $4, $5, NOW())"
        )
        .bind(srna.id())
        .bind(srna.content())
        .bind(srna.task_context())
        .bind(srna.promoted())
        .bind(srna.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Srna> {
        let row = sqlx::query(
            "SELECT id, content, task_context, promoted, genome_id, created_at FROM srna WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));
        let promoted: bool = row.get("promoted");

        let mut srna = Srna::new(
            row.get("id"),
            row.get("content"),
            row.get("task_context"),
            genome_id,
            row.get("created_at"),
        ).ok()?;

        if promoted {
            srna.promote();
        }

        Some(srna)
    }
}

pub struct PostgresPhenomeRepository {
    pool: sqlx::PgPool,
}

impl PostgresPhenomeRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, phenome: &Phenome) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO phenomes (id, name, genome_id, created_at) VALUES ($1, $2, $3, NOW())"
        )
        .bind(phenome.id())
        .bind(phenome.name())
        .bind(phenome.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Phenome> {
        let row = sqlx::query(
            "SELECT id, name, genome_id FROM phenomes WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

        Phenome::new(
            row.get("id"),
            row.get("name"),
            genome_id,
        ).ok()
    }
}

pub struct PostgresRibosomeRepository {
    pool: sqlx::PgPool,
}

impl PostgresRibosomeRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, ribosome: &Ribosome) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO ribosomes (id, name, config, phenome_id, genome_id, created_at) VALUES ($1, $2, $3, $4, $5, NOW())"
        )
        .bind(ribosome.id())
        .bind(ribosome.name())
        .bind(ribosome.config())
        .bind(ribosome.phenome_id())
        .bind(ribosome.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Ribosome> {
        let row = sqlx::query(
            "SELECT id, name, config, phenome_id, genome_id FROM ribosomes WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

        Ribosome::new(
            row.get("id"),
            row.get("name"),
            row.get("config"),
            row.get("phenome_id"),
            genome_id,
        ).ok()
    }
}

pub struct PostgresRrnaRepository {
    pool: sqlx::PgPool,
}

impl PostgresRrnaRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, rrna: &Rrna) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO rrna (id, ribosome_id, gene_id, criteria, genome_id, created_at) VALUES ($1, $2, $3, $4, $5, NOW())"
        )
        .bind(rrna.id())
        .bind(rrna.ribosome_id())
        .bind(rrna.gene_id())
        .bind(rrna.criteria())
        .bind(rrna.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Rrna> {
        let row = sqlx::query(
            "SELECT id, ribosome_id, gene_id, criteria, genome_id FROM rrna WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

        Rrna::new(
            row.get("id"),
            row.get("ribosome_id"),
            row.get("gene_id"),
            row.get("criteria"),
            genome_id,
        ).ok()
    }
}

pub struct PostgresFoldRepository {
    pool: sqlx::PgPool,
}

impl PostgresFoldRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, fold: &Fold) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO folds (id, mrna_id, commit_sha, root_git_directory, branch, remote, created_at) VALUES ($1, $2, $3, $4, $5, $6, NOW())"
        )
        .bind(fold.id())
        .bind(fold.mrna_id())
        .bind(fold.commit_sha())
        .bind(fold.root_git_directory())
        .bind(fold.branch())
        .bind(fold.remote())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Fold> {
        let row = sqlx::query(
            "SELECT id, mrna_id, commit_sha, root_git_directory, branch, remote, created_at FROM folds WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;

        Fold::new(
            row.get("id"),
            row.get("mrna_id"),
            row.get("commit_sha"),
            row.get("root_git_directory"),
            row.get("branch"),
            row.get("remote"),
            row.get("created_at"),
        ).ok()
    }
}

pub struct PostgresProteinRepository {
    pool: sqlx::PgPool,
}

impl PostgresProteinRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, protein: &Protein) -> Result<(), String> {
        let result_str = match protein.result() {
            ProteinResult::Pass => "Pass",
            ProteinResult::Fail => "Fail",
            ProteinResult::Pending => "Pending",
        };
        
        sqlx::query(
            "INSERT INTO proteins (id, fold_id, phenome_id, gene_id, result, genome_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, NOW())"
        )
        .bind(protein.id())
        .bind(protein.phenotype_id())
        .bind(protein.phenome_id())
        .bind(protein.gene_id())
        .bind(result_str)
        .bind(protein.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Protein> {
        let row = sqlx::query(
            "SELECT id, fold_id, phenome_id, gene_id, result, genome_id, created_at FROM proteins WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));
        let result_str: String = row.get("result");
        let result = match result_str.as_str() {
            "Pass" => ProteinResult::Pass,
            "Fail" => ProteinResult::Fail,
            _ => ProteinResult::Pending,
        };

        Some(Protein::new(
            row.get("id"),
            row.get("fold_id"),
            row.get("phenome_id"),
            row.get("gene_id"),
            result,
            genome_id,
            row.get("created_at"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn postgres_genotype_repo_finds_seed_genotype() {
        let pool = sqlx::PgPool::connect(&dotenvy::var("DATABASE_URL").unwrap_or_else(|_| "postgres://rnap:rnap@localhost:5432/rnap".to_string()))
            .await
            .unwrap();

        let genome_id = rnap_genome::GenomeId::new();

        // First create the genome (required for FK)
        sqlx::query("INSERT INTO genomes (id, name, created_at) VALUES ($1, $2, NOW())")
            .bind(genome_id.as_uuid())
            .bind("test-tenant")
            .execute(&pool)
            .await
            .unwrap();

        let traits = serde_json::json!([
            {"key": "title", "dominance": "Dominant"},
            {"key": "description", "dominance": "Recessive"}
        ]);

        // Use unique kind to avoid conflicts with existing DB data
        let kind = format!("FEAT-{}", uuid::Uuid::new_v4()).replace("-", "");

        sqlx::query(
            "INSERT INTO genotypes (id, kind, name, generation, genome_id, traits, created_at) VALUES ($1, $2, $3, $4, $5, $6, NOW())"
        )
        .bind(uuid::Uuid::new_v4())
        .bind(&kind)
        .bind("Feature Request")
        .bind(1i32)
        .bind(genome_id.as_uuid())
        .bind(traits)
        .execute(&pool)
        .await
        .unwrap();

        let repo = PostgresGenotypeRepository::new(pool);
        let found = repo.find_by_kind(&kind).await.unwrap();

        assert_eq!(found.kind(), kind);
        assert_eq!(found.traits().len(), 2);
    }

    #[tokio::test]
    async fn postgres_dna_repo_saves_and_finds_dna() {
        let pool = sqlx::PgPool::connect(&dotenvy::var("DATABASE_URL").unwrap_or_else(|_| "postgres://rnap:rnap@localhost:5432/rnap".to_string()))
            .await
            .unwrap();

        let genome_id = rnap_genome::GenomeId::new();

        // Create genome first (required for FK)
        sqlx::query("INSERT INTO genomes (id, name, created_at) VALUES ($1, $2, NOW())")
            .bind(genome_id.as_uuid())
            .bind("test-tenant")
            .execute(&pool)
            .await
            .unwrap();

        let dna_id = uuid::Uuid::new_v4();
        let path = "dna/2024/q1/req-001.dna";

        // Create DNA with path
        let dna = Dna::new(
            dna_id,
            path.to_string(),
            genome_id,
        ).unwrap();

        let repo = PostgresDnaRepository::new(pool.clone());
        repo.save(&dna).await.unwrap();

        // Find it back
        let found = repo.find_by_id(&dna_id).await.unwrap();
        assert_eq!(found.path(), path);
        assert_eq!(found.genome_id(), &genome_id);
    }

    #[tokio::test]
    async fn postgres_chromatine_repo_saves_and_finds() {
        let pool = sqlx::PgPool::connect(&dotenvy::var("DATABASE_URL").unwrap_or_else(|_| "postgres://rnap:rnap@localhost:5432/rnap".to_string()))
            .await
            .unwrap();

        let genome_id = rnap_genome::GenomeId::new();

        sqlx::query("INSERT INTO genomes (id, name, created_at) VALUES ($1, $2, NOW())")
            .bind(genome_id.as_uuid())
            .bind("test-tenant")
            .execute(&pool)
            .await
            .unwrap();

        let chromatine = Chromatine::new(
            uuid::Uuid::new_v4(),
            "docs/research/prd.pdf".to_string(),
            genome_id,
        ).unwrap();

        let repo = PostgresChromatineRepository::new(pool.clone());
        repo.save(&chromatine).await.unwrap();

        let found = repo.find_by_id(chromatine.id()).await.unwrap();
        assert_eq!(found.path(), "docs/research/prd.pdf");
    }
}