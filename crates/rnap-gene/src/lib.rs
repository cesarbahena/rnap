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
    genotype_id: rnap_genome::GenotypeId,
    mutations: Vec<Mutation>,
}

impl Gene {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        genome_id: rnap_genome::GenomeId,
        genotype_id: rnap_genome::GenotypeId,
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

    pub fn genotype_id(&self) -> rnap_genome::GenotypeId {
        self.genotype_id
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
    #[error("unknown trait key: {0}")]
    UnknownTraitKey(String),
    #[error("ambiguous trait key: {0} — use the full canonical name")]
    AmbiguousTraitKey(String),
    #[error("trait is vestigial: {0}")]
    TraitIsVestigial(String),
}

pub struct GeneService;

impl GeneService {
    pub fn validate_and_append(
        gene: &mut Gene,
        trait_key: String,
        value: serde_json::Value,
        by: By,
        context: String,
        genotype: &rnap_genotype::Genotype,
    ) -> Result<Mutation, GeneError> {
        let now = chrono::Utc::now();

        // Resolve trait_key via match_trait (supports partial/typo matching)
        let matched = match rnap_genotype::match_trait(&trait_key, genotype.traits()) {
            rnap_genotype::TraitMatch::Exact { matched } => matched,
            rnap_genotype::TraitMatch::Ambiguous { .. } => {
                return Err(GeneError::AmbiguousTraitKey(trait_key.clone()));
            }
            rnap_genotype::TraitMatch::NotFound => {
                return Err(GeneError::UnknownTraitKey(trait_key.clone()));
            }
        };

        // Vestigial traits cannot be written
        if !matched.dominance().is_writable() {
            return Err(GeneError::TraitIsVestigial(trait_key.clone()));
        }

        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            *gene.id(),
            matched.key().to_string(), // canonical key
            value,
            by,
            context,
            now,
        );

        gene.append_mutation(mutation.clone());
        Ok(mutation)
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
            .filter(|t| t.dominance().is_required())
            .all(|t| state.contains_key(t.key()))
    }
}

pub trait GeneRepository {
    fn save(&mut self, gene: Gene);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Gene>;
    fn find_by_name(&self, name: &str) -> Option<&Gene>;
    fn find_by_name_prefix(&self, prefix: &str) -> Option<&Gene>;
    /// Returns the next sequence number for a given (genome_id, kind).
    /// Override in Postgres-backed repositories for atomic sequencing.
    fn next_sequence_for_kind(&mut self, _genome_id: &rnap_genome::GenomeId, _kind: &str) -> Result<u32, String> {
        Err("in-memory repository does not support sequence generation".to_string())
    }
}

pub struct InMemoryGeneRepository {
    genes: std::collections::HashMap<uuid::Uuid, Gene>,
    sequences: std::collections::HashMap<(rnap_genome::GenomeId, String), u32>,
}

impl InMemoryGeneRepository {
    pub fn new() -> Self {
        Self {
            genes: std::collections::HashMap::new(),
            sequences: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryGeneRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneRepository for InMemoryGeneRepository {
    fn save(&mut self, gene: Gene) {
        self.genes.insert(*gene.id(), gene);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Gene> {
        self.genes.get(id)
    }

    fn find_by_name(&self, name: &str) -> Option<&Gene> {
        self.genes.values().find(|g| g.name() == name)
    }

    fn find_by_name_prefix(&self, prefix: &str) -> Option<&Gene> {
        self.genes
            .values()
            .find(|g| g.name().starts_with(prefix))
    }

    fn next_sequence_for_kind(&mut self, genome_id: &rnap_genome::GenomeId, kind: &str) -> Result<u32, String> {
        let key = (*genome_id, kind.to_string());
        let next = self.sequences.get(&key).copied().unwrap_or(0) + 1;
        self.sequences.insert(key, next);
        Ok(next)
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
        let genotype_id = rnap_genome::GenotypeId::new();
        let gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            genome_id,
            genotype_id,
        );
        assert_eq!(gene.id(), &gene_id);
        assert_eq!(gene.name(), "FEAT-0001-user-auth");
        assert_eq!(gene.genome_id(), &genome_id);
        assert_eq!(gene.genotype_id(), genotype_id);
        assert!(gene.mutations().is_empty());
    }

    #[test]
    fn gene_service_rejects_unknown_trait_key() {
        let gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenotypeId::new();
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
            genome_id,
            vec![rnap_genotype::Trait::new(
                "title".to_string(),
                rnap_genotype::Dominance::Dominant,
            )],
        )
        .unwrap();

        let result = GeneService::validate_and_append(
            &mut gene,
            "nonexistent".to_string(),
            serde_json::json!("value"),
            By::Human,
            "unknown trait".to_string(),
            &genotype,
        );
        assert!(matches!(result, Err(GeneError::UnknownTraitKey(_))));
    }

    #[test]
    fn gene_service_rejects_ambiguous_trait_key() {
        let gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenotypeId::new();
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
            genome_id,
            vec![
                rnap_genotype::Trait::new("title".to_string(), rnap_genotype::Dominance::Dominant),
                rnap_genotype::Trait::new("timestamp".to_string(), rnap_genotype::Dominance::Recessive),
            ],
        )
        .unwrap();

        // "time" matches both "title" and "timestamp" — ambiguous
        let result = GeneService::validate_and_append(
            &mut gene,
            "time".to_string(),
            serde_json::json!("value"),
            By::Human,
            "ambiguous key".to_string(),
            &genotype,
        );
        assert!(matches!(result, Err(GeneError::AmbiguousTraitKey(_))));
    }

    #[test]
    fn gene_service_accepts_partial_trait_key() {
        let gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenotypeId::new();
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
            genome_id,
            vec![rnap_genotype::Trait::new(
                "title".to_string(),
                rnap_genotype::Dominance::Dominant,
            )],
        )
        .unwrap();

        // "tit" is a unique prefix match for "title"
        let result = GeneService::validate_and_append(
            &mut gene,
            "tit".to_string(),
            serde_json::json!("Hello"),
            By::Human,
            "partial key".to_string(),
            &genotype,
        );
        assert!(result.is_ok());
        // Canonical key should be "title", not "tit"
        assert_eq!(gene.mutations()[0].trait_key(), "title");
    }

    #[test]
    fn gene_service_rejects_mutation_targeting_vestigial_trait() {
        let gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenotypeId::new();
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
            genome_id,
            vec![rnap_genotype::Trait::new(
                "deprecated_field".to_string(),
                rnap_genotype::Dominance::Vestigial,
            )],
        )
        .unwrap();

        let result = GeneService::validate_and_append(
            &mut gene,
            "deprecated_field".to_string(),
            serde_json::json!("value"),
            By::Human,
            "trying to write vestigial".to_string(),
            &genotype,
        );
        assert!(matches!(result, Err(GeneError::TraitIsVestigial(_))));
    }

    #[test]
    fn gene_service_current_state_returns_last_mutation_per_trait() {
        let gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenotypeId::new();
        let genotype = rnap_genotype::Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            genome_id,
            vec![
                rnap_genotype::Trait::new("title".to_string(), rnap_genotype::Dominance::Dominant),
                rnap_genotype::Trait::new("status".to_string(), rnap_genotype::Dominance::Recessive),
            ],
        )
        .unwrap();
        let mut gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            genome_id,
            genotype_id,
        );

        GeneService::validate_and_append(
            &mut gene,
            "title".to_string(),
            serde_json::json!("First title"),
            By::Human,
            "first".to_string(),
            &genotype,
        )
        .unwrap();

        GeneService::validate_and_append(
            &mut gene,
            "status".to_string(),
            serde_json::json!("draft"),
            By::Human,
            "initial status".to_string(),
            &genotype,
        )
        .unwrap();

        GeneService::validate_and_append(
            &mut gene,
            "title".to_string(),
            serde_json::json!("Updated title"),
            By::Llm,
            "refined".to_string(),
            &genotype,
        )
        .unwrap();

        let state = GeneService::current_state(&gene);
        assert_eq!(state.len(), 2);
        assert_eq!(
            state.get("title").unwrap().value(),
            &serde_json::json!("Updated title")
        );
        assert_eq!(
            state.get("status").unwrap().value(),
            &serde_json::json!("draft")
        );
    }

    #[test]
    fn gene_service_is_ready_when_all_dominant_traits_have_mutations() {
        let gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenotypeId::new();
        let genotype = rnap_genotype::Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            genome_id,
            vec![
                rnap_genotype::Trait::new("title".to_string(), rnap_genotype::Dominance::Dominant),
                rnap_genotype::Trait::new("status".to_string(), rnap_genotype::Dominance::Recessive),
            ],
        )
        .unwrap();
        let mut gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            genome_id,
            genotype_id,
        );

        GeneService::validate_and_append(
            &mut gene,
            "title".to_string(),
            serde_json::json!("Hello"),
            By::Human,
            "initial".to_string(),
            &genotype,
        )
        .unwrap();

        assert!(GeneService::is_ready(&gene, &genotype));

        let empty_gene = Gene::new(
            uuid::Uuid::new_v4(),
            "FEAT-0002-another".to_string(),
            genome_id,
            genotype_id,
        );
        assert!(!GeneService::is_ready(&empty_gene, &genotype));
    }

    #[test]
    fn in_memory_gene_repo_saves_and_finds_gene() {
        let gene_id = uuid::Uuid::new_v4();
        let genome_id = rnap_genome::GenomeId::new();
        let genotype_id = rnap_genome::GenotypeId::new();
        let gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            genome_id,
            genotype_id,
        );

        let mut repo = InMemoryGeneRepository::new();
        repo.save(gene);

        let found = repo.find_by_id(&gene_id).unwrap();
        assert_eq!(found.name(), "FEAT-0001-user-auth");
    }
}
