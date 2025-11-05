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
}
