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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

    fn with_state(&self, new_state: TraitState) -> Self {
        Self {
            key: self.key.clone(),
            state: new_state,
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum GenotypeError {
    #[error("duplicate trait key: {0}")]
    DuplicateTraitKey(String),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

    pub fn evolve(&self, transitions: Vec<TraitTransition>) -> Result<Genotype, EvolutionError> {
        let mut new_traits: Vec<Trait> = self.traits.clone();

        for transition in transitions {
            let idx = new_traits.iter().position(|t| t.key() == transition.key);

            let Some(idx) = idx else {
                return Err(EvolutionError::UnknownTraitKey(transition.key.clone()));
            };

            if matches!(new_traits[idx].state(), TraitState::Vestigial) {
                return Err(EvolutionError::VestigialTraitCannotTransition {
                    key: transition.key.clone(),
                });
            }

            new_traits[idx] = new_traits[idx].with_state(transition.new_state);
        }

        Genotype::new(
            self.kind.clone(),
            self.name.clone(),
            self.generation + 1,
            self.genome_id,
            new_traits,
        )
        .map_err(|e| match e {
            GenotypeError::DuplicateTraitKey(key) => EvolutionError::DuplicateTraitKey(key),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraitTransition {
    pub key: String,
    pub new_state: TraitState,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum EvolutionError {
    #[error("unknown trait key: {0}")]
    UnknownTraitKey(String),
    #[error("vestigial trait cannot transition: {key}")]
    VestigialTraitCannotTransition { key: String },
    #[error("duplicate trait key: {0}")]
    DuplicateTraitKey(String),
}

pub trait GenotypeRepository {
    fn find_by_kind(&self, kind: &str) -> Option<Genotype>;
}

pub struct InMemoryGenotypeRepository {
    genotypes: std::collections::HashMap<String, Genotype>,
}

impl InMemoryGenotypeRepository {
    pub fn new(genotypes: std::collections::HashMap<String, Genotype>) -> Self {
        Self { genotypes }
    }
}

impl GenotypeRepository for InMemoryGenotypeRepository {
    fn find_by_kind(&self, kind: &str) -> Option<Genotype> {
        self.genotypes.get(kind).cloned()
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

    #[test]
    fn evolve_transitions_dominant_to_recessive() {
        let genome_id = GenomeId::new();
        let genotype = Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            genome_id,
            vec![
                Trait::new("title".to_string(), TraitState::Dominant),
                Trait::new("description".to_string(), TraitState::Recessive),
            ],
        )
        .unwrap();

        let evolved = genotype
            .evolve(vec![TraitTransition {
                key: "title".to_string(),
                new_state: TraitState::Recessive,
            }])
            .unwrap();

        assert_eq!(evolved.generation(), 2);
        assert_eq!(evolved.genome_id(), &genome_id);
        assert_eq!(evolved.kind(), "FEAT");
        let title_trait = evolved.find_trait("title").unwrap();
        assert!(matches!(title_trait.state(), TraitState::Recessive));
    }

    #[test]
    fn evolve_rejects_vestigial_trait_transition() {
        let genotype = Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            GenomeId::new(),
            vec![Trait::new("deprecated".to_string(), TraitState::Vestigial)],
        )
        .unwrap();

        let result = genotype.evolve(vec![TraitTransition {
            key: "deprecated".to_string(),
            new_state: TraitState::Dominant,
        }]);
        assert_eq!(
            result,
            Err(EvolutionError::VestigialTraitCannotTransition {
                key: "deprecated".to_string()
            })
        );
    }

    #[test]
    fn evolve_transitions_active_to_vestigial() {
        let genotype = Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            GenomeId::new(),
            vec![Trait::new("notes".to_string(), TraitState::Recessive)],
        )
        .unwrap();

        let evolved = genotype
            .evolve(vec![TraitTransition {
                key: "notes".to_string(),
                new_state: TraitState::Vestigial,
            }])
            .unwrap();

        let notes_trait = evolved.find_trait("notes").unwrap();
        assert!(matches!(notes_trait.state(), TraitState::Vestigial));
    }

    #[test]
    fn in_memory_genotype_repo_finds_by_kind() {
        let genome_id = GenomeId::new();
        let genotype = Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            genome_id,
            vec![Trait::new("title".to_string(), TraitState::Dominant)],
        )
        .unwrap();

        let repo = InMemoryGenotypeRepository::new(
            vec![("FEAT".to_string(), genotype.clone())]
                .into_iter()
                .collect(),
        );

        let found = repo.find_by_kind("FEAT").unwrap();
        assert_eq!(found.kind(), "FEAT");
        assert_eq!(found.traits().len(), 1);
    }
}
