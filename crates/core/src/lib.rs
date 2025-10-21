use std::collections::HashSet;

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

pub enum Actor {
    Human,
    Llm,
}

pub struct Mutation {
    id: uuid::Uuid,
    gene_id: uuid::Uuid,
    trait_key: String,
    value: serde_json::Value,
    actor: Actor,
    context: String,
    created_at: i64,
}

impl Mutation {
    pub fn new(
        id: uuid::Uuid,
        gene_id: uuid::Uuid,
        trait_key: String,
        value: serde_json::Value,
        actor: Actor,
        context: String,
        created_at: i64,
    ) -> Self {
        Self {
            id,
            gene_id,
            trait_key,
            value,
            actor,
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

    pub fn actor(&self) -> &Actor {
        &self.actor
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn created_at(&self) -> i64 {
        self.created_at
    }
}

pub struct Genotype {
    version: u32,
    traits: Vec<Trait>,
}

#[derive(Debug, PartialEq)]
pub enum GenotypeError {
    DuplicateTraitKey(String),
}

impl Genotype {
    pub fn new(version: u32, traits: Vec<Trait>) -> Result<Self, GenotypeError> {
        let mut seen = HashSet::new();
        for t in &traits {
            if !seen.insert(t.key().to_string()) {
                return Err(GenotypeError::DuplicateTraitKey(t.key().to_string()));
            }
        }
        Ok(Self { version, traits })
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn traits(&self) -> &[Trait] {
        &self.traits
    }
}

pub struct Gene {
    id: uuid::Uuid,
    mutations: Vec<Mutation>,
}

#[derive(Debug, PartialEq)]
pub enum GeneError {
    MutationGeneIdMismatch,
}

impl Gene {
    pub fn new(id: uuid::Uuid) -> Self {
        Self {
            id,
            mutations: Vec::new(),
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    pub fn append_mutation(&mut self, mutation: Mutation) -> Result<(), GeneError> {
        if mutation.gene_id() != &self.id {
            return Err(GeneError::MutationGeneIdMismatch);
        }
        self.mutations.push(mutation);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn genotype_has_version() {
        let genotype = Genotype::new(1, vec![]).unwrap();

        assert_eq!(genotype.version(), 1);
    }

    #[test]
    fn genotype_exposes_its_traits() {
        let genotype = Genotype::new(
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
            Actor::Human,
            "initial requirement".to_string(),
            1000,
        );

        assert_eq!(mutation.gene_id(), &gene_id);
        assert_eq!(mutation.trait_key(), "title");
        assert_eq!(mutation.value(), &serde_json::json!("Hello"));
        assert!(matches!(mutation.actor(), Actor::Human));
        assert_eq!(mutation.context(), "initial requirement");
    }

    #[test]
    fn gene_can_be_created_with_id_and_empty_mutations() {
        let gene_id = uuid::Uuid::new_v4();
        let gene = Gene::new(gene_id);

        assert_eq!(gene.id(), &gene_id);
        assert!(gene.mutations().is_empty());
    }

    #[test]
    fn gene_can_append_a_mutation() {
        let gene_id = uuid::Uuid::new_v4();
        let mut gene = Gene::new(gene_id);

        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            gene_id,
            "title".to_string(),
            serde_json::json!("Hello"),
            Actor::Human,
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
        let mut gene = Gene::new(gene_id);

        let mutation = Mutation::new(
            uuid::Uuid::new_v4(),
            wrong_gene_id,
            "title".to_string(),
            serde_json::json!("Hello"),
            Actor::Human,
            "initial requirement".to_string(),
            1000,
        );

        let result = gene.append_mutation(mutation);
        assert!(result.is_err());
    }
}
