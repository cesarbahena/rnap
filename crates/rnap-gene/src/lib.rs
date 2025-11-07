#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum By {
    Human,
    Llm,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Mutation {
    id: uuid::Uuid,
    gene_id: uuid::Uuid,
    trait_key: String,
    value: serde_json::Value,
    by: By,
    context: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl Mutation {
    pub fn new(
        id: uuid::Uuid,
        gene_id: uuid::Uuid,
        trait_key: String,
        value: serde_json::Value,
        by: By,
        context: String,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            gene_id,
            trait_key,
            value,
            by,
            context,
            created_at,
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn gene_id(&self) -> &uuid::Uuid {
        &self.gene_id
    }

    pub fn trait_key(&self) -> &str {
        &self.trait_key
    }

    pub fn value(&self) -> &serde_json::Value {
        &self.value
    }

    pub fn by(&self) -> &By {
        &self.by
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Gene {
    id: uuid::Uuid,
    name: String,
    genome_id: rnap_genome::GenomeId,
    genotype_id: rnap_genome::GenomeId,
    mutations: Vec<Mutation>,
}

impl Gene {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        genome_id: rnap_genome::GenomeId,
        genotype_id: rnap_genome::GenomeId,
    ) -> Self {
        Self {
            id,
            name,
            genome_id,
            genotype_id,
            mutations: Vec::new(),
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn genome_id(&self) -> &rnap_genome::GenomeId {
        &self.genome_id
    }

    pub fn genotype_id(&self) -> &rnap_genome::GenomeId {
        &self.genotype_id
    }

    pub fn mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    pub fn append_mutation(&mut self, mutation: Mutation) {
        self.mutations.push(mutation);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn by_enum_has_human_and_llm_variants() {
        assert_eq!(By::Human, By::Human);
        assert_eq!(By::Llm, By::Llm);
        assert_ne!(By::Human, By::Llm);
    }

    #[test]
    fn mutation_can_be_created_with_all_fields() {
        let gene_id = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();
        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "title".to_string(),
            serde_json::json!("Hello"),
            By::Human,
            "initial requirement".to_string(),
            now,
        );
        assert_eq!(mutation.gene_id(), &gene_id);
        assert_eq!(mutation.trait_key(), "title");
        assert_eq!(mutation.value(), &serde_json::json!("Hello"));
        assert!(matches!(mutation.by(), By::Human));
        assert_eq!(mutation.context(), "initial requirement");
        assert_eq!(mutation.created_at(), now);
    }

    #[test]
    fn gene_can_be_created_with_id_name_genome_id_genotype_id() {
        let gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenomeId::new();
        let gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            genome_id,
            genotype_id,
        );
        assert_eq!(gene.id(), &gene_id);
        assert_eq!(gene.name(), "FEAT-0001-user-auth");
        assert_eq!(gene.genome_id(), &genome_id);
        assert_eq!(gene.genotype_id(), &genotype_id);
        assert!(gene.mutations().is_empty());
    }
}
