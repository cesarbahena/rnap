use rnap_genome::GenomeId;

/// The type of domain graph view, following C4 model levels.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ViewType {
    /// System Landscape view — shows all systems and actors
    SystemLandscape,
    /// Context view — shows one system and its interactions
    Context,
    /// Container view — shows the containers within a system
    Container,
    /// Component view — shows the components within a container
    Component,
}

impl ViewType {
    /// Returns the C4-style label for this view type.
    pub fn label(&self) -> &'static str {
        match self {
            ViewType::SystemLandscape => "System Landscape",
            ViewType::Context => "Context",
            ViewType::Container => "Container",
            ViewType::Component => "Component",
        }
    }
}

/// The scope of a Locus — which entities are included in the view.
///
/// Each field is a set of UUIDs referencing Chromosomes, Organisms,
/// and Quiasmas respectively. Duplicates are silently ignored.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Scope {
    chromosome_ids: Vec<uuid::Uuid>,
    organism_ids: Vec<uuid::Uuid>,
    quiasma_ids: Vec<uuid::Uuid>,
}

impl Scope {
    /// Creates a new empty scope.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a chromosome ID to the scope. Ignores duplicates.
    pub fn add_chromosome(&mut self, id: uuid::Uuid) {
        if !self.chromosome_ids.contains(&id) {
            self.chromosome_ids.push(id);
        }
    }

    /// Adds an organism ID to the scope. Ignores duplicates.
    pub fn add_organism(&mut self, id: uuid::Uuid) {
        if !self.organism_ids.contains(&id) {
            self.organism_ids.push(id);
        }
    }

    /// Adds a quiasma ID to the scope. Ignores duplicates.
    pub fn add_quiasma(&mut self, id: uuid::Uuid) {
        if !self.quiasma_ids.contains(&id) {
            self.quiasma_ids.push(id);
        }
    }

    /// Returns the chromosome IDs in this scope.
    pub fn chromosome_ids(&self) -> &[uuid::Uuid] {
        &self.chromosome_ids
    }

    /// Returns the organism IDs in this scope.
    pub fn organism_ids(&self) -> &[uuid::Uuid] {
        &self.organism_ids
    }

    /// Returns the quiasma IDs in this scope.
    pub fn quiasma_ids(&self) -> &[uuid::Uuid] {
        &self.quiasma_ids
    }
}

/// A declarative view of the domain graph.
///
/// A Locus selects a subset of Chromosomes, Organisms, and Quiasmas
/// from a specific perspective, renderable as C4-style views and
/// exportable to Structurizr DSL.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Locus {
    id: uuid::Uuid,
    name: String,
    view_type: ViewType,
    scope: Scope,
    genome_id: GenomeId,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum LocusError {
    #[error("locus name must not be empty")]
    EmptyName,
}

impl Locus {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        view_type: ViewType,
        genome_id: GenomeId,
    ) -> Result<Self, LocusError> {
        if name.trim().is_empty() {
            return Err(LocusError::EmptyName);
        }
        Ok(Self {
            id,
            name,
            view_type,
            scope: Scope::new(),
            genome_id,
        })
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn view_type(&self) -> &ViewType {
        &self.view_type
    }

    pub fn scope(&self) -> &Scope {
        &self.scope
    }

    /// Adds a chromosome to the scope. Returns &mut self for chaining.
    pub fn add_chromosome_to_scope(&mut self, id: uuid::Uuid) {
        self.scope.add_chromosome(id);
    }

    /// Adds an organism to the scope. Returns &mut self for chaining.
    pub fn add_organism_to_scope(&mut self, id: uuid::Uuid) {
        self.scope.add_organism(id);
    }

    /// Adds a quiasma to the scope. Returns &mut self for chaining.
    pub fn add_quiasma_to_scope(&mut self, id: uuid::Uuid) {
        self.scope.add_quiasma(id);
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }
}

pub trait LocusRepository {
    fn save(&mut self, locus: Locus);
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Locus>;
}

pub struct InMemoryLocusRepository {
    entries: std::collections::HashMap<uuid::Uuid, Locus>,
}

impl InMemoryLocusRepository {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryLocusRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl LocusRepository for InMemoryLocusRepository {
    fn save(&mut self, locus: Locus) {
        self.entries.insert(*locus.id(), locus);
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<&Locus> {
        self.entries.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locus_can_be_created_with_name_and_view_type() {
        let genome_id = GenomeId::new();
        let locus = Locus::new(
            uuid::Uuid::new_v4(),
            "System Overview".to_string(),
            ViewType::SystemLandscape,
            genome_id,
        )
        .unwrap();

        assert_eq!(locus.name(), "System Overview");
        assert_eq!(locus.view_type(), &ViewType::SystemLandscape);
        assert_eq!(locus.genome_id(), &genome_id);
    }

    #[test]
    fn locus_rejects_empty_name() {
        let genome_id = GenomeId::new();
        let result = Locus::new(
            uuid::Uuid::new_v4(),
            "   ".to_string(),
            ViewType::Context,
            genome_id,
        );
        assert_eq!(result, Err(LocusError::EmptyName));
    }

    #[test]
    fn view_type_has_all_c4_levels() {
        assert_eq!(ViewType::SystemLandscape.label(), "System Landscape");
        assert_eq!(ViewType::Context.label(), "Context");
        assert_eq!(ViewType::Container.label(), "Container");
        assert_eq!(ViewType::Component.label(), "Component");
    }

    #[test]
    fn locus_scope_starts_empty_and_accepts_entities() {
        let genome_id = GenomeId::new();
        let mut locus = Locus::new(
            uuid::Uuid::new_v4(),
            "Context View".to_string(),
            ViewType::Context,
            genome_id,
        ).unwrap();

        // Scope starts empty
        assert!(locus.scope().chromosome_ids().is_empty());
        assert!(locus.scope().organism_ids().is_empty());
        assert!(locus.scope().quiasma_ids().is_empty());

        // Add entities
        let chrom_id = uuid::Uuid::new_v4();
        let org_id = uuid::Uuid::new_v4();
        let quiasma_id = uuid::Uuid::new_v4();

        locus.add_chromosome_to_scope(chrom_id);
        locus.add_organism_to_scope(org_id);
        locus.add_quiasma_to_scope(quiasma_id);

        assert_eq!(locus.scope().chromosome_ids(), &[chrom_id]);
        assert_eq!(locus.scope().organism_ids(), &[org_id]);
        assert_eq!(locus.scope().quiasma_ids(), &[quiasma_id]);
    }

    #[test]
    fn scope_deduplicates_added_entities() {
        let genome_id = GenomeId::new();
        let mut locus = Locus::new(
            uuid::Uuid::new_v4(),
            "System Landscape".to_string(),
            ViewType::SystemLandscape,
            genome_id,
        ).unwrap();

        let chrom_id = uuid::Uuid::new_v4();
        locus.add_chromosome_to_scope(chrom_id);
        locus.add_chromosome_to_scope(chrom_id); // duplicate

        assert_eq!(locus.scope().chromosome_ids().len(), 1);
    }

    #[test]
    fn in_memory_locus_repo_saves_and_finds() {
        let genome_id = GenomeId::new();
        let id = uuid::Uuid::new_v4();
        let mut locus = Locus::new(
            id,
            "Context View".to_string(),
            ViewType::Context,
            genome_id,
        ).unwrap();

        let chrom_id = uuid::Uuid::new_v4();
        locus.add_chromosome_to_scope(chrom_id);

        let mut repo = InMemoryLocusRepository::new();
        repo.save(locus);

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.name(), "Context View");
        assert_eq!(found.scope().chromosome_ids(), &[chrom_id]);
    }
}