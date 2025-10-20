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

pub struct Genotype {
    version: u32,
    traits: Vec<Trait>,
}

impl Genotype {
    pub fn new(version: u32, traits: Vec<Trait>) -> Self {
        Self { version, traits }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn traits(&self) -> &[Trait] {
        &self.traits
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
        let genotype = Genotype::new(1, vec![]);

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
        );

        assert_eq!(genotype.traits().len(), 2);
        assert_eq!(genotype.traits()[0].key(), "title");
        assert_eq!(genotype.traits()[1].key(), "description");
    }
}
