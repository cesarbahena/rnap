use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub enum TraitState {
    Dominant,
    Recessive,
    Vestigial,
}

impl TraitState {
    pub fn is_required(&self) -> bool {
        matches!(self, TraitState::Dominant)
    }

    pub fn is_writable(&self) -> bool {
        matches!(self, TraitState::Dominant | TraitState::Recessive)
    }

    pub fn is_visible(&self) -> bool {
        matches!(self, TraitState::Dominant | TraitState::Recessive)
    }
}

pub struct Trait {
    key: String,
    state: TraitState,
}

impl Trait {
    pub fn new(key: String, state: TraitState) -> Self {
        Self { key, state }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn state(&self) -> &TraitState {
        &self.state
    }
}

pub enum By {
    Human,
    Llm,
}

pub struct Mutation {
    id: uuid::Uuid,
    gene_id: uuid::Uuid,
    trait_key: String,
    value: serde_json::Value,
    by: By,
    context: String,
    created_at: i64,
}

impl Mutation {
    pub fn new(
        id: uuid::Uuid,
        gene_id: uuid::Uuid,
        trait_key: String,
        value: serde_json::Value,
        by: By,
        context: String,
        created_at: i64,
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

    pub fn created_at(&self) -> i64 {
        self.created_at
    }
}

pub struct Genotype {
    kind: String,
    name: String,
    generation: u32,
    traits: Vec<Trait>,
}

#[derive(Debug, PartialEq)]
pub enum GenotypeError {
    DuplicateTraitKey(String),
}

impl Genotype {
    pub fn new(
        kind: String,
        name: String,
        generation: u32,
        traits: Vec<Trait>,
    ) -> Result<Self, GenotypeError> {
        let mut seen = HashSet::new();
        for t in &traits {
            if !seen.insert(t.key().to_string()) {
                return Err(GenotypeError::DuplicateTraitKey(t.key().to_string()));
            }
        }
        Ok(Self {
            kind,
            name,
            generation,
            traits,
        })
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn traits(&self) -> &[Trait] {
        &self.traits
    }

    pub fn find_trait(&self, key: &str) -> Option<&Trait> {
        self.traits.iter().find(|t| t.key() == key)
    }
}

pub struct Gene {
    id: uuid::Uuid,
    name: String,
    genotype: Arc<Genotype>,
    mutations: Vec<Mutation>,
}

#[derive(Debug, PartialEq)]
pub enum GeneError {
    MutationGeneIdMismatch,
    TraitIsVestigial(String),
}

impl Gene {
    pub fn new(id: uuid::Uuid, name: String, genotype: Arc<Genotype>) -> Self {
        Self {
            id,
            name,
            genotype,
            mutations: Vec::new(),
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn genotype(&self) -> &Arc<Genotype> {
        &self.genotype
    }

    pub fn mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    pub fn append_mutation(&mut self, mutation: Mutation) -> Result<(), GeneError> {
        if mutation.gene_id() != &self.id {
            return Err(GeneError::MutationGeneIdMismatch);
        }
        if let Some(trait_def) = self.genotype.find_trait(mutation.trait_key()) {
            if !trait_def.state().is_writable() {
                return Err(GeneError::TraitIsVestigial(
                    mutation.trait_key().to_string(),
                ));
            }
        }
        self.mutations.push(mutation);
        Ok(())
    }

    pub fn current_state(&self) -> HashMap<&str, &Mutation> {
        let mut state: HashMap<&str, &Mutation> = HashMap::new();
        for mutation in &self.mutations {
            state.insert(mutation.trait_key(), mutation);
        }
        state
    }

    pub fn is_ready(&self) -> bool {
        let state = self.current_state();
        self.genotype
            .traits()
            .iter()
            .filter(|t| t.state().is_required())
            .all(|t| state.contains_key(t.key()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_genotype() -> Arc<Genotype> {
        Arc::new(
            Genotype::new("FEAT".to_string(), "Feature Request".to_string(), 1, vec![]).unwrap(),
        )
    }

    fn test_genotype_with_traits(traits: Vec<Trait>) -> Arc<Genotype> {
        Arc::new(
            Genotype::new("FEAT".to_string(), "Feature Request".to_string(), 1, traits).unwrap(),
        )
    }

    #[test]
    fn trait_state_dominant_is_required_writable_and_visible() {
        let state = TraitState::Dominant;

        assert!(state.is_required());
        assert!(state.is_writable());
        assert!(state.is_visible());
    }

    #[test]
    fn trait_state_recessive_is_optional_writable_and_visible() {
        let state = TraitState::Recessive;

        assert!(!state.is_required());
        assert!(state.is_writable());
        assert!(state.is_visible());
    }

    #[test]
    fn trait_state_vestigial_is_not_required_not_writable_and_not_visible() {
        let state = TraitState::Vestigial;

        assert!(!state.is_required());
        assert!(!state.is_writable());
        assert!(!state.is_visible());
    }

    #[test]
    fn trait_has_key_and_state() {
        let trait_def = Trait::new("title".to_string(), TraitState::Dominant);

        assert_eq!(trait_def.key(), "title");
        assert!(trait_def.state().is_required());
    }

    #[test]
    fn genotype_has_kind_and_name() {
        let genotype =
            Genotype::new("FEAT".to_string(), "Feature Request".to_string(), 1, vec![]).unwrap();

        assert_eq!(genotype.kind(), "FEAT");
        assert_eq!(genotype.name(), "Feature Request");
    }

    #[test]
    fn genotype_exposes_its_traits() {
        let genotype = Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            vec![
                Trait::new("title".to_string(), TraitState::Dominant),
                Trait::new("description".to_string(), TraitState::Recessive),
            ],
        )
        .unwrap();

        assert_eq!(genotype.traits().len(), 2);
        assert_eq!(genotype.traits()[0].key(), "title");
        assert_eq!(genotype.traits()[1].key(), "description");
    }

    #[test]
    fn genotype_rejects_duplicate_trait_keys() {
        let result = Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            vec![
                Trait::new("title".to_string(), TraitState::Dominant),
                Trait::new("title".to_string(), TraitState::Recessive),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn mutation_can_be_created_with_all_fields() {
        let gene_id = uuid::Uuid::new_v4();
        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "title".to_string(),
            serde_json::json!("Hello"),
            By::Human,
            "initial requirement".to_string(),
            1000,
        );

        assert_eq!(mutation.gene_id(), &gene_id);
        assert_eq!(mutation.trait_key(), "title");
        assert_eq!(mutation.value(), &serde_json::json!("Hello"));
        assert!(matches!(mutation.by(), By::Human));
        assert_eq!(mutation.context(), "initial requirement");
    }

    #[test]
    fn gene_can_be_created_with_id_and_empty_mutations() {
        let gene_id = uuid::Uuid::new_v4();
        let genotype = test_genotype();
        let gene = Gene::new(gene_id, "FEAT-0001-user-auth".to_string(), genotype);

        assert_eq!(gene.id(), &gene_id);
        assert!(gene.mutations().is_empty());
    }

    #[test]
    fn gene_has_name() {
        let gene_id = uuid::Uuid::new_v4();
        let genotype = test_genotype();
        let gene = Gene::new(gene_id, "FEAT-0001-user-auth".to_string(), genotype);

        assert_eq!(gene.name(), "FEAT-0001-user-auth");
    }

    #[test]
    fn gene_can_append_a_mutation() {
        let gene_id = uuid::Uuid::new_v4();
        let genotype = test_genotype();
        let mut gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            Arc::clone(&genotype),
        );

        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "title".to_string(),
            serde_json::json!("Hello"),
            By::Human,
            "initial requirement".to_string(),
            1000,
        );

        gene.append_mutation(mutation).unwrap();

        assert_eq!(gene.mutations().len(), 1);
        assert_eq!(gene.mutations()[0].trait_key(), "title");
    }

    #[test]
    fn gene_rejects_mutation_with_wrong_gene_id() {
        let gene_id = uuid::Uuid::new_v4();
        let wrong_gene_id = uuid::Uuid::new_v4();
        let genotype = test_genotype();
        let mut gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            Arc::clone(&genotype),
        );

        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            wrong_gene_id,
            "title".to_string(),
            serde_json::json!("Hello"),
            By::Human,
            "initial requirement".to_string(),
            1000,
        );

        let result = gene.append_mutation(mutation);
        assert!(result.is_err());
    }

    #[test]
    fn gene_rejects_mutation_targeting_vestigial_trait() {
        let gene_id = uuid::Uuid::new_v4();
        let genotype = test_genotype_with_traits(vec![Trait::new(
            "deprecated_field".to_string(),
            TraitState::Vestigial,
        )]);
        let mut gene = Gene::new(gene_id, "FEAT-0001-user-auth".to_string(), genotype);

        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "deprecated_field".to_string(),
            serde_json::json!("value"),
            By::Human,
            "trying to write vestigial".to_string(),
            1000,
        );

        let result = gene.append_mutation(mutation);
        assert!(result.is_err());
    }

    #[test]
    fn gene_current_state_returns_last_mutation_per_trait() {
        let gene_id = uuid::Uuid::new_v4();
        let genotype = test_genotype_with_traits(vec![
            Trait::new("title".to_string(), TraitState::Dominant),
            Trait::new("status".to_string(), TraitState::Recessive),
        ]);
        let mut gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            Arc::clone(&genotype),
        );

        gene.append_mutation(Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "title".to_string(),
            serde_json::json!("First title"),
            By::Human,
            "first".to_string(),
            1000,
        ))
        .unwrap();

        gene.append_mutation(Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "status".to_string(),
            serde_json::json!("draft"),
            By::Human,
            "initial status".to_string(),
            2000,
        ))
        .unwrap();

        gene.append_mutation(Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "title".to_string(),
            serde_json::json!("Updated title"),
            By::Llm,
            "refined".to_string(),
            3000,
        ))
        .unwrap();

        let state = gene.current_state();

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
    fn gene_is_ready_when_all_dominant_traits_have_mutations() {
        let gene_id = uuid::Uuid::new_v4();
        let genotype = test_genotype_with_traits(vec![
            Trait::new("title".to_string(), TraitState::Dominant),
            Trait::new("status".to_string(), TraitState::Recessive),
        ]);
        let mut gene = Gene::new(
            gene_id,
            "FEAT-0001-user-auth".to_string(),
            Arc::clone(&genotype),
        );

        gene.append_mutation(Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "title".to_string(),
            serde_json::json!("Hello"),
            By::Human,
            "initial".to_string(),
            1000,
        ))
        .unwrap();

        assert!(gene.is_ready());

        let empty_gene = Gene::new(
            uuid::Uuid::new_v4(),
            "FEAT-0002-another".to_string(),
            Arc::clone(&genotype),
        );
        assert!(!empty_gene.is_ready());
    }
}
