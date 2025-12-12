use rnap_genotype::Genotype;
use rnap_gene::{Gene, Mutation, By};
use rnap_dna::Dna;
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
            "SELECT id, kind, name, generation, genome_id, traits FROM genotypes WHERE kind = $1"
        )
        .bind(kind)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;

        let genotype = Genotype::new(
            row.get("kind"),
            row.get("name"),
            row.get::<i32, _>("generation") as u32,
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genome_id")),
            serde_json::from_value(row.get("traits")).ok()?,
        )
        .ok()?;

        Some(genotype)
    }

pub async fn find_by_genome_id(&self, genome_id: rnap_genome::GenomeId) -> Option<Genotype> {
        let row = sqlx::query(
            "SELECT id, kind, name, generation, genome_id, traits FROM genotypes WHERE genome_id = $1"
        )
        .bind(genome_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;

        let genotype = Genotype::new(
            row.get("kind"),
            row.get("name"),
            row.get::<i32, _>("generation") as u32,
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genome_id")),
            serde_json::from_value(row.get("traits")).ok()?,
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
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genotype_id")),
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
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genotype_id")),
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
        let chromatine_refs: Vec<String> = dna.chromatine_refs().to_vec();
        
        sqlx::query(
            "INSERT INTO dna (id, content, chromatine_refs, genome_id, created_at) \n             VALUES ($1, $2, $3, $4, NOW())"
        )
        .bind(dna.id())
        .bind(dna.content())
        .bind(&chromatine_refs)
        .bind(dna.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Dna> {
        let row = sqlx::query(
            "SELECT id, content, chromatine_refs, genome_id FROM dna WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;

        let chromatine_refs: Vec<String> = row.get("chromatine_refs");
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

        let mut dna = Dna::new(
            row.get("id"),
            row.get("content"),
            genome_id,
        ).ok()?;

        // Restore chromatine_refs (Dna::new initializes empty)
        for ref_url in chromatine_refs {
            dna.add_chromatine_ref(ref_url);
        }

        Some(dna)
    }

    pub async fn find_by_genome(&self, genome_id: &GenomeId) -> Vec<Dna> {
        let rows = sqlx::query(
            "SELECT id, content, chromatine_refs, genome_id FROM dna WHERE genome_id = $1"
        )
        .bind(genome_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .ok();

        match rows {
            Some(rows) => rows.iter().filter_map(|row| {
                let chromatine_refs: Vec<String> = row.get("chromatine_refs");
                let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

                let mut dna = Dna::new(
                    row.get("id"),
                    row.get("content"),
                    genome_id,
                ).ok()?;

                for ref_url in chromatine_refs {
                    dna.add_chromatine_ref(ref_url);
                }

                Some(dna)
            }).collect(),
            None => vec![],
        }
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
            {"key": "title", "state": "Dominant"},
            {"key": "description", "state": "Recessive"}
        ]);

        sqlx::query(
            "INSERT INTO genotypes (id, kind, name, generation, genome_id, traits, created_at) VALUES ($1, $2, $3, $4, $5, $6, NOW())"
        )
        .bind(uuid::Uuid::new_v4())
        .bind("FEAT")
        .bind("Feature Request")
        .bind(1i32)
        .bind(genome_id.as_uuid())
        .bind(traits)
        .execute(&pool)
        .await
        .unwrap();

        let repo = PostgresGenotypeRepository::new(pool);
        let found = repo.find_by_kind("FEAT").await.unwrap();

        assert_eq!(found.kind(), "FEAT");
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
        let content = "Users must be able to reset their password";

        // Create DNA with chromatine refs
        let mut dna = Dna::new(
            dna_id,
            content.to_string(),
            genome_id,
        ).unwrap();
        dna.add_chromatine_ref("https://docs.example.com/prd.pdf".to_string());

        let repo = PostgresDnaRepository::new(pool.clone());
        repo.save(&dna).await.unwrap();

        // Find it back
        let found = repo.find_by_id(&dna_id).await.unwrap();
        assert_eq!(found.content(), content);
        assert_eq!(found.genome_id(), &genome_id);
        assert_eq!(found.chromatine_refs().len(), 1);
        assert_eq!(found.chromatine_refs()[0], "https://docs.example.com/prd.pdf");
    }
}