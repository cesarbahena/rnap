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

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum GeneError {
    #[error("mutation gene_id {mutation_gene_id} does not match gene {gene_id}")]
    MutationGeneIdMismatch {
        gene_id: uuid::Uuid,
        mutation_gene_id: uuid::Uuid,
    },
    #[error("unknown trait key: {0}")]
    UnknownTraitKey(String),
    #[error("trait is vestigial: {0}")]
    TraitIsVestigial(String),
    #[error("tenant isolation violation: gene genome {gene_genome} does not match genotype genome {genotype_genome}")]
    TenantIsolationViolation {
        gene_genome: String,
        genotype_genome: String,
    },
}

pub struct GeneService;

impl GeneService {
    pub fn validate_and_append(
        gene: &mut Gene,
        mutation: Mutation,
        genotype: &rnap_genotype::Genotype,
    ) -> Result<(), GeneError> {
        if mutation.gene_id() != gene.id() {
            return Err(GeneError::MutationGeneIdMismatch {
                gene_id: *gene.id(),
                mutation_gene_id: *mutation.gene_id(),
            });
        }
        if genotype.find_trait(mutation.trait_key()).is_none() {
            return Err(GeneError::UnknownTraitKey(mutation.trait_key().to_string()));
        }
        Ok(())
    }

    pub fn current_state(gene: &Gene) -> std::collections::HashMap<&str, &Mutation> {
        let mut state: std::collections::HashMap<&str, &Mutation> =
            std::collections::HashMap::new();
        for mutation in gene.mutations() {
            state.insert(mutation.trait_key(), mutation);
        }
        state
    }

    pub fn is_ready(gene: &Gene, genotype: &rnap_genotype::Genotype) -> bool {
        let state = Self::current_state(gene);
        genotype
            .traits()
            .iter()
            .filter(|t| t.state().is_required())
            .all(|t| state.contains_key(t.key()))
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

    #[test]
    fn gene_service_rejects_mutation_with_wrong_gene_id() {
        let gene_id = uuid::Uuid::new_v4();
        let wrong_gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenomeId::new();
        let mut gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            genome_id,
            genotype_id,
        );
        let genotype = rnap_genotype::Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            rnap_genome::GenomeId::new(),
            vec![],
        )
        .unwrap();

        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            wrong_gene_id,
            "title".to_string(),
            serde_json::json!("Hello"),
            By::Human,
            "initial requirement".to_string(),
            chrono::Utc::now(),
        );

        let result = GeneService::validate_and_append(&mut gene, mutation, &genotype);
        assert!(result.is_err());
    }

    #[test]
    fn gene_service_rejects_unknown_trait_key() {
        let gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenomeId::new();
        let mut gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            genome_id,
            genotype_id,
        );
        let genotype = rnap_genotype::Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            rnap_genome::GenomeId::new(),
            vec![rnap_genotype::Trait::new(
                "title".to_string(),
                rnap_genotype::TraitState::Dominant,
            )],
        )
        .unwrap();

        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "nonexistent".to_string(),
            serde_json::json!("value"),
            By::Human,
            "unknown trait".to_string(),
            chrono::Utc::now(),
        );

        let result = GeneService::validate_and_append(&mut gene, mutation, &genotype);
        assert_eq!(
            result,
            Err(GeneError::UnknownTraitKey("nonexistent".to_string()))
        );
    }
}
