#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Dominance {
    Dominant,
    Recessive,
    Vestigial,
}

impl Dominance {
    pub fn is_required(&self) -> bool {
        matches!(self, Dominance::Dominant)
    }

    pub fn is_writable(&self) -> bool {
        matches!(self, Dominance::Dominant | Dominance::Recessive)
    }

    pub fn is_visible(&self) -> bool {
        matches!(self, Dominance::Dominant | Dominance::Recessive)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trait {
    key: String,
    dominance: Dominance,
}

impl Trait {
    pub fn new(key: String, dominance: Dominance) -> Self {
        Self { key, dominance }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn dominance(&self) -> &Dominance {
        &self.dominance
    }

    fn with_dominance(&self, new_dominance: Dominance) -> Self {
        Self {
            key: self.key.clone(),
            dominance: new_dominance,
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
    id: rnap_genome::GenotypeId,
    kind: String,
    name: String,
    generation: u32,
    genome_id: rnap_genome::GenomeId,
    traits: Vec<Trait>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl Genotype {
    /// New genotype without an explicit id — generates a random one and sets created_at to now.
    /// For in-memory / test contexts.
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
            id: rnap_genome::GenotypeId::new(),
            kind,
            name,
            generation,
            genome_id,
            traits,
            created_at: chrono::Utc::now(),
        })
    }

    /// New genotype with an explicit id and timestamp — used by storage layer when loading from DB.
    pub fn with_id(
        id: rnap_genome::GenotypeId,
        kind: String,
        name: String,
        generation: u32,
        genome_id: rnap_genome::GenomeId,
        traits: Vec<Trait>,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Self, GenotypeError> {
        let mut seen = std::collections::HashSet::new();
        for t in &traits {
            if !seen.insert(t.key().to_string()) {
                return Err(GenotypeError::DuplicateTraitKey(t.key().to_string()));
            }
        }
        Ok(Self {
            id,
            kind,
            name,
            generation,
            genome_id,
            traits,
            created_at,
        })
    }

    pub fn id(&self) -> rnap_genome::GenotypeId {
        self.id
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

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
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

            if matches!(new_traits[idx].dominance(), Dominance::Vestigial) {
                return Err(EvolutionError::VestigialTraitCannotTransition {
                    key: transition.key.clone(),
                });
            }

            new_traits[idx] = new_traits[idx].with_dominance(transition.new_dominance);
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
    pub new_dominance: Dominance,
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

/// Result of matching a user input against available traits.
#[derive(Debug, Clone, PartialEq)]
pub enum TraitMatch {
    /// Exact match on the canonical key.
    Exact { matched: Trait },
    /// Single non-exact match found.
    Ambiguous { matched: Trait },
    /// No match found.
    NotFound,
}

/// Matches a user input string against available traits.
///
/// Matching priority:
/// 1. Exact match (case-sensitive) → TraitMatch::Exact
/// 2. Single non-exact match (prefix or Levenshtein) → TraitMatch::Ambiguous
/// 3. Multiple candidates → TraitMatch::Ambiguous (caller should error)
/// 4. No match → TraitMatch::NotFound
///
/// Returns TraitMatch::Ambiguous for both single non-exact matches AND multiple
/// non-exact matches. Callers should error on Ambiguous unless it's an Exact match.
pub fn match_trait(input: &str, traits: &[Trait]) -> TraitMatch {
    let input_lower = input.to_lowercase();

    // Exact match (case-sensitive)
    if let Some(t) = traits.iter().find(|t| t.key() == input) {
        return TraitMatch::Exact { matched: t.clone() };
    }

    // Collect all non-exact candidates (prefix or Levenshtein)
    let candidates: Vec<Trait> = traits
        .iter()
        .filter(|t| {
            let key_lower = t.key().to_lowercase();
            // Prefix match
            key_lower.starts_with(&input_lower) || input_lower.starts_with(&key_lower)
                // Levenshtein match (distance ≤ 3 is considered close enough)
                || levenshtein_distance(&input_lower, &key_lower) <= 3
        })
        .cloned()
        .collect();

    match candidates.len() {
        0 => TraitMatch::NotFound,
        // Unique non-exact match acts as exact for UX (unique prefix/typo)
        1 => TraitMatch::Exact { matched: candidates.into_iter().next().unwrap() },
        // Multiple matches — error, caller shows the ambiguity
        _ => TraitMatch::Ambiguous { matched: candidates.into_iter().next().unwrap() },
    }
}

/// Levenshtein edit distance between two strings.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut matrix = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m {
        matrix[i][0] = i;
    }
    for j in 0..=n {
        matrix[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[m][n]
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
        let state = Dominance::Dominant;
        assert!(state.is_required());
        assert!(state.is_writable());
        assert!(state.is_visible());
    }

    #[test]
    fn trait_state_recessive_is_optional_writable_and_visible() {
        let state = Dominance::Recessive;
        assert!(!state.is_required());
        assert!(state.is_writable());
        assert!(state.is_visible());
    }

    #[test]
    fn trait_state_vestigial_is_not_required_not_writable_and_not_visible() {
        let state = Dominance::Vestigial;
        assert!(!state.is_required());
        assert!(!state.is_writable());
        assert!(!state.is_visible());
    }

    #[test]
    fn trait_has_key_and_state() {
        let t = Trait::new("title".to_string(), Dominance::Dominant);
        assert_eq!(t.key(), "title");
        assert!(t.dominance().is_required());
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
            Trait::new("title".to_string(), Dominance::Dominant),
            Trait::new("description".to_string(), Dominance::Recessive),
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
                Trait::new("title".to_string(), Dominance::Dominant),
                Trait::new("title".to_string(), Dominance::Recessive),
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
                Trait::new("title".to_string(), Dominance::Dominant),
                Trait::new("description".to_string(), Dominance::Recessive),
            ],
        )
        .unwrap();

        let evolved = genotype
            .evolve(vec![TraitTransition {
                key: "title".to_string(),
                new_dominance: Dominance::Recessive,
            }])
            .unwrap();

        assert_eq!(evolved.generation(), 2);
        assert_eq!(evolved.genome_id(), &genome_id);
        assert_eq!(evolved.kind(), "FEAT");
        let title_trait = evolved.find_trait("title").unwrap();
        assert!(matches!(title_trait.dominance(), Dominance::Recessive));
    }

    #[test]
    fn evolve_rejects_vestigial_trait_transition() {
        let genotype = Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            GenomeId::new(),
            vec![Trait::new("deprecated".to_string(), Dominance::Vestigial)],
        )
        .unwrap();

        let result = genotype.evolve(vec![TraitTransition {
            key: "deprecated".to_string(),
            new_dominance: Dominance::Dominant,
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
            vec![Trait::new("notes".to_string(), Dominance::Recessive)],
        )
        .unwrap();

        let evolved = genotype
            .evolve(vec![TraitTransition {
                key: "notes".to_string(),
                new_dominance: Dominance::Vestigial,
            }])
            .unwrap();

        let notes_trait = evolved.find_trait("notes").unwrap();
        assert!(matches!(notes_trait.dominance(), Dominance::Vestigial));
    }

    #[test]
    fn in_memory_genotype_repo_finds_by_kind() {
        let genome_id = GenomeId::new();
        let genotype = Genotype::new(
            "FEAT".to_string(),
            "Feature Request".to_string(),
            1,
            genome_id,
            vec![Trait::new("title".to_string(), Dominance::Dominant)],
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

    #[test]
    fn match_trait_exact_match() {
        let traits = vec![
            Trait::new("Title".to_string(), Dominance::Dominant),
            Trait::new("Description".to_string(), Dominance::Recessive),
        ];
        let result = match_trait("Title", &traits);
        assert!(matches!(result, TraitMatch::Exact { matched } if matched.key() == "Title"));
    }

    #[test]
    fn match_trait_prefix_match() {
        let traits = vec![
            Trait::new("Time to Live (days)".to_string(), Dominance::Dominant),
            Trait::new("Timestamp".to_string(), Dominance::Recessive),
        ];
        let result = match_trait("time", &traits);
        assert!(matches!(result, TraitMatch::Ambiguous { matched } if matched.key() == "Time to Live (days)"));
    }

    #[test]
    fn match_trait_not_found() {
        let traits = vec![
            Trait::new("Title".to_string(), Dominance::Dominant),
        ];
        let result = match_trait("nonexistent", &traits);
        assert!(matches!(result, TraitMatch::NotFound));
    }

    #[test]
    fn match_trait_ambiguous_returns_first() {
        // Both "Time to Live" and "Timestamp" match "time"
        let traits = vec![
            Trait::new("Time to Live (days)".to_string(), Dominance::Dominant),
            Trait::new("Timestamp".to_string(), Dominance::Recessive),
        ];
        let result = match_trait("time", &traits);
        // Should be Ambiguous (multiple matches), not Exact
        assert!(matches!(result, TraitMatch::Ambiguous { .. }));
    }

    #[test]
    fn match_trait_case_insensitive() {
        // "title" is unique prefix for "Title" — returns Exact (unique non-exact)
        let traits = vec![
            Trait::new("Title".to_string(), Dominance::Dominant),
        ];
        let result = match_trait("title", &traits);
        assert!(matches!(result, TraitMatch::Exact { matched } if matched.key() == "Title"));
    }

    #[test]
    fn match_trait_levenshtein_close_match() {
        // Levenshtein catches single-character typos — unique match returns Exact
        let traits = vec![
            Trait::new("Description".to_string(), Dominance::Dominant),
        ];
        // "Descripiton" is 2 characters off from "Description" (distance = 2 ≤ 3)
        let result = match_trait("Descripiton", &traits);
        assert!(matches!(result, TraitMatch::Exact { matched } if matched.key() == "Description"));
    }
}
