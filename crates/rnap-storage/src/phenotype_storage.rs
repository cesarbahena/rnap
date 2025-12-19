pub struct PostgresPhenotypeRepository {
    pool: sqlx::PgPool,
}

impl PostgresPhenotypeRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, phenotype: &Phenotype) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO phenotypes (id, name, genome_id, created_at) VALUES ($1, $2, $3, NOW())"
        )
        .bind(phenotype.id())
        .bind(phenotype.name())
        .bind(phenotype.genome_id().as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Option<Phenotype> {
        let row = sqlx::query(
            "SELECT id, name, genome_id FROM phenotypes WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        let row = row?;
        let genome_id = GenomeId::from(row.get::<uuid::Uuid, _>("genome_id"));

        Phenotype::new(
            row.get("id"),
            row.get("name"),
            genome_id,
        ).ok()
    }
}
