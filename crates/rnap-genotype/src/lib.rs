#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum GenotypeError {
    #[error("duplicate trait key: {0}")]
    DuplicateTraitKey(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Genotype {
    kind: String,
    name: String,
    generation: u32,
    genome_id: rnap_genome::GenomeId,
    traits: Vec<Trait>,
}

impl Genotype {
    pub fn new(
        kind: String,
        name: String,
        generation: u32,
        genome_id: rnap_genome::GenomeId,
        traits: Vec<Trait>,
    ) -> Result<Self, GenotypeError> {
        let mut seen = std::collections::HashSet::new();
        for t in &traits {
            if !seen.insert(t.key().to_string()) {
                return Err(GenotypeError::DuplicateTraitKey(t.key().to_string()));
            }
        }
        Ok(Self {
            kind,
            name,
            generation,
            genome_id,
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

    pub fn genome_id(&self) -> &rnap_genome::GenomeId {
        &self.genome_id
    }

    pub fn traits(&self) -> &[Trait] {
        &self.traits
    }

    pub fn find_trait(&self, key: &str) -> Option<&Trait> {
        self.traits.iter().find(|t| t.key() == key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rnap_genome::GenomeId;

    fn test_genotype() -> Genotype {
        Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            GenomeId::new(),
            vec![],
        )
        .unwrap()
    }

    fn test_genotype_with_traits(traits: Vec<Trait>) -> Genotype {
        Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            GenomeId::new(),
            traits,
        )
        .unwrap()
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
        let t = Trait::new("title".to_string(), TraitState::Dominant);
        assert_eq!(t.key(), "title");
        assert!(t.state().is_required());
    }

    #[test]
    fn genotype_has_kind_and_name() {
        let genome_id = GenomeId::new();
        let genotype = Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            genome_id,
            vec![],
        )
        .unwrap();
        assert_eq!(genotype.kind(), "FEAT");
        assert_eq!(genotype.name(), "Feature Request");
        assert_eq!(genotype.genome_id(), &genome_id);
    }

    #[test]
    fn genotype_exposes_its_traits() {
        let genotype = test_genotype_with_traits(vec![
            Trait::new("title".to_string(), TraitState::Dominant),
            Trait::new("description".to_string(), TraitState::Recessive),
        ]);
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
            GenomeId::new(),
            vec![
                Trait::new("title".to_string(), TraitState::Dominant),
                Trait::new("title".to_string(), TraitState::Recessive),
            ],
        );
        assert!(result.is_err());
    }
}
