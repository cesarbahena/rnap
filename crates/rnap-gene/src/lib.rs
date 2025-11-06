#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum By {
    Human,
    Llm,
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
}
