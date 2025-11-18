use rnap_genotype::{Genotype, GenotypeRepository};
use rnap_gene::{Gene, GeneRepository};
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
            "INSERT INTO genes (id, name, genome_id, genotype_id) VALUES ($1, $2, $3, $4)"
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
        let row = sqlx::query(
            "SELECT id, name, genome_id, genotype_id FROM genes WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;

        Some(Gene::new(
            row.get("id"),
            row.get("name"),
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genome_id")),
            rnap_genome::GenomeId::from(row.get::<uuid::Uuid, _>("genotype_id")),
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
        sqlx::query("INSERT INTO genomes (id, name) VALUES ($1, $2)")
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
            "INSERT INTO genotypes (id, kind, name, generation, genome_id, traits) VALUES ($1, $2, $3, $4, $5, $6)"
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
}