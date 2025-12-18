//! Diagram renderer trait and types for framework-agnostic diagram generation.
//!
//! This crate provides:
//! - A `DiagramRenderer` trait that defines the interface for rendering
//! - Domain types (Cell, Organelle, Channel, LocusView) for rendering purposes
//! - A `StructurizrRenderer` implementation for Structurizr DSL output
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │  Domain Model (rnap-chromosome,    │
//! │  rnap-organism, etc.)              │
//! └─────────────────────────────────────┘
//!                    │
//!                    ▼
//! ┌─────────────────────────────────────┐
//! │  DiagramRenderer trait              │
//! └─────────────────────────────────────┘
//!                    │
//!        ┌───────────┴───────────┐
//!        ▼                       ▼
//! ┌──────────────┐        ┌──────────────┐
//! │Structurizr   │        │ Future:      │
//! │Renderer      │        │PlantUML, etc.│
//! └──────────────┘        └──────────────┘
//! ```

use rnap_genome::GenomeId;
use rnap_organism::Organism;
use rnap_chromosome::Chromosome;

/// Trait for rendering domain model entities to diagram formats.
///
/// Implementors are responsible for translating the framework-agnostic
/// domain model into their specific output format (DSL, JSON, etc.)
pub trait DiagramRenderer: Send + Sync {
    /// Renders the workspace/root element with name and description.
    fn render_workspace(&self, name: &str, description: &str) -> String;

    /// Renders a person/organism element.
    fn render_organism(&self, organism: &Organism) -> String;

    /// Renders a software system/cell element.
    fn render_cell(&self, cell: &Cell) -> String;

    /// Renders a container/organelle element.
    fn render_organelle(&self, organelle: &Organelle) -> String;

    /// Renders a component/chromosome element.
    fn render_chromosome(&self, chromosome: &Chromosome) -> String;

    /// Renders a relationship/channel between elements.
    fn render_channel(&self, channel: &Channel) -> String;

    /// Renders an enterprise boundary grouping.
    fn render_enterprise_start(&self, name: Option<&str>) -> String;

    /// Closes an enterprise boundary.
    fn render_enterprise_end(&self) -> String;

    /// Renders a group boundary (e.g., "Backend Services").
    fn render_group_start(&self, name: &str) -> String;

    /// Closes a group boundary.
    fn render_group_end(&self) -> String;

    /// Renders a view definition (context, container, component).
    fn render_view(&self, view: &LocusView) -> String;

    /// Renders element styles.
    fn render_styles(&self) -> String;

    /// Renders a relationship technology (optional).
    fn render_technology(&self, technology: &str) -> String {
        format!("\"{}\"", technology)
    }
}

// ============
// Domain Types for Rendering
// ============

/// Represents a Cell (Software System) for rendering
#[derive(Debug, Clone)]
pub struct Cell {
    id: uuid::Uuid,
    name: String,
    description: String,
    tags: Vec<String>,
    genome_id: GenomeId,
}

impl Cell {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        genome_id: GenomeId,
    ) -> Self {
        Self {
            id,
            name,
            description,
            tags: Vec::new(),
            genome_id,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}

/// Represents an Organelle (Container) for rendering
#[derive(Debug, Clone)]
pub struct Organelle {
    id: uuid::Uuid,
    name: String,
    description: String,
    technology: Option<String>,
    kind: OrganelleKind,
    cell_id: uuid::Uuid,
    tags: Vec<String>,
    genome_id: GenomeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganelleKind {
    Service,
    Worker,
    Database,
    Queue,
    Frontend,
    Infrastructure,
}

impl Organelle {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        kind: OrganelleKind,
        cell_id: uuid::Uuid,
        genome_id: GenomeId,
    ) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            technology: None,
            kind,
            cell_id,
            tags: Vec::new(),
            genome_id,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    pub fn kind(&self) -> &OrganelleKind {
        &self.kind
    }

    pub fn cell_id(&self) -> &uuid::Uuid {
        &self.cell_id
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_technology(mut self, technology: String) -> Self {
        self.technology = Some(technology);
        self
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
}

/// Represents a Channel (Relationship) for rendering
#[derive(Debug, Clone)]
pub struct Channel {
    id: uuid::Uuid,
    source_id: uuid::Uuid,
    source_name: String,
    target_id: uuid::Uuid,
    target_name: String,
    description: String,
    technology: Option<String>,
    genome_id: GenomeId,
}

impl Channel {
    pub fn new(
        id: uuid::Uuid,
        source_id: uuid::Uuid,
        source_name: String,
        target_id: uuid::Uuid,
        target_name: String,
        genome_id: GenomeId,
    ) -> Self {
        Self {
            id,
            source_id,
            source_name,
            target_id,
            target_name,
            description: String::new(),
            technology: None,
            genome_id,
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn source_id(&self) -> &uuid::Uuid {
        &self.source_id
    }

    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    pub fn target_id(&self) -> &uuid::Uuid {
        &self.target_id
    }

    pub fn target_name(&self) -> &str {
        &self.target_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn technology(&self) -> Option<&str> {
        self.technology.as_deref()
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_technology(mut self, technology: String) -> Self {
        self.technology = Some(technology);
        self
    }
}

/// Represents a Locus (View) for rendering
#[derive(Debug, Clone)]
pub struct LocusView {
    id: uuid::Uuid,
    name: String,
    view_type: ViewType,
    scope_id: Option<uuid::Uuid>,
    scope_name: Option<String>,
    genome_id: GenomeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewType {
    SystemLandscape,
    SystemContext,
    Container,
    Component,
}

impl LocusView {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        view_type: ViewType,
        genome_id: GenomeId,
    ) -> Self {
        Self {
            id,
            name,
            view_type,
            scope_id: None,
            scope_name: None,
            genome_id,
        }
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

    pub fn scope_id(&self) -> Option<&uuid::Uuid> {
        self.scope_id.as_ref()
    }

    pub fn scope_name(&self) -> Option<&str> {
        self.scope_name.as_deref()
    }

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }

    pub fn with_scope(mut self, scope_id: uuid::Uuid, scope_name: String) -> Self {
        self.scope_id = Some(scope_id);
        self.scope_name = Some(scope_name);
        self
    }
}

// ============
// Structurizr Renderer Implementation
// ============

/// Structurizr DSL renderer
pub struct StructurizrRenderer;

impl StructurizrRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StructurizrRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagramRenderer for StructurizrRenderer {
    fn render_workspace(&self, name: &str, description: &str) -> String {
        if description.is_empty() {
            format!("workspace \"{}\" {{\n", name)
        } else {
            format!("workspace \"{}\" \"{}\" {{\n", name, description)
        }
    }

    fn render_organism(&self, organism: &Organism) -> String {
        if organism.description().is_empty() {
            format!("    person \"{}\"", organism.name())
        } else {
            format!(
                "    person \"{}\" \"{}\"",
                organism.name(),
                organism.description()
            )
        }
    }

    fn render_cell(&self, cell: &Cell) -> String {
        if cell.description().is_empty() {
            format!("    softwareSystem \"{}\"", cell.name())
        } else {
            format!(
                "    softwareSystem \"{}\" \"{}\"",
                cell.name(),
                cell.description()
            )
        }
    }

    fn render_organelle(&self, organelle: &Organelle) -> String {
        let name = organelle.name();
        let desc = organelle.description();
        let tech = organelle.technology();

        match (desc, tech) {
            (_, Some(tech)) if !desc.is_empty() => {
                format!(
                    "        container \"{}\" \"{}\" \"{}\"",
                    name, desc, tech
                )
            }
            (_, Some(tech)) => {
                format!("        container \"{}\" \"\" \"{}\"", name, tech)
            }
            _ => {
                format!("        container \"{}\"", name)
            }
        }
    }

    fn render_chromosome(&self, chromosome: &Chromosome) -> String {
        if chromosome.description().is_empty() {
            format!("            component \"{}\"", chromosome.name())
        } else {
            format!(
                "            component \"{}\" \"{}\"",
                chromosome.name(),
                chromosome.description()
            )
        }
    }

    fn render_channel(&self, channel: &Channel) -> String {
        let desc = channel.description();
        let tech = channel.technology();

        match (desc, tech) {
            (_, Some(tech)) if !desc.is_empty() => {
                format!(
                    "        {} -> {} \"{}\" \"{}\"",
                    channel.source_name(),
                    channel.target_name(),
                    desc,
                    tech
                )
            }
            (_, Some(tech)) => {
                format!(
                    "        {} -> {} \"\" \"{}\"",
                    channel.source_name(),
                    channel.target_name(),
                    tech
                )
            }
            _ if !desc.is_empty() => {
                format!(
                    "        {} -> {} \"{}\"",
                    channel.source_name(),
                    channel.target_name(),
                    desc
                )
            }
            _ => {
                format!(
                    "        {} -> {}",
                    channel.source_name(),
                    channel.target_name()
                )
            }
        }
    }

    fn render_enterprise_start(&self, name: Option<&str>) -> String {
        match name {
            Some(n) => format!("    enterprise \"{}\" {{", n),
            None => "    enterprise {".to_string(),
        }
    }

    fn render_enterprise_end(&self) -> String {
        "    }".to_string()
    }

    fn render_group_start(&self, name: &str) -> String {
        format!("        group \"{}\" {{", name)
    }

    fn render_group_end(&self) -> String {
        "        }".to_string()
    }

    fn render_view(&self, view: &LocusView) -> String {
        match view.view_type() {
            ViewType::SystemLandscape => {
                "    systemLandscape {".to_string()
            }
            ViewType::SystemContext => {
                let scope = view.scope_name().unwrap_or("system");
                format!("    systemContext {} {{", scope)
            }
            ViewType::Container => {
                let scope = view.scope_name().unwrap_or("system");
                format!("    container {} {{", scope)
            }
            ViewType::Component => {
                let scope = view.scope_name().unwrap_or("container");
                format!("    component {} {{", scope)
            }
        }
    }

    fn render_styles(&self) -> String {
        let styles = "    styles {\n\
        element \"Element\" {\n\
            background \"#438DD5\"\n\
            color \"#FFFFFF\"\n\
        }\n\
        element \"Person\" {\n\
            shape \"Person\"\n\
        }\n\
        element \"Database\" {\n\
            shape \"Cylinder\"\n\
        }\n\
    }";
        styles.to_string()
    }
}

// ============
// Tests
// ============

#[cfg(test)]
mod tests {
    use super::*;
    use rnap_organism::OrganismKind;

    fn genome_id() -> GenomeId {
        GenomeId::new()
    }

    #[test]
    fn structurizr_renders_organism_without_description() {
        let renderer = StructurizrRenderer::new();
        let organism = Organism::new(
            uuid::Uuid::new_v4(),
            "Customer".to_string(),
            OrganismKind::Human,
            "".to_string(),
            genome_id(),
        )
        .unwrap();

        let output = renderer.render_organism(&organism);
        assert_eq!(output, "    person \"Customer\"");
    }

    #[test]
    fn structurizr_renders_organism_with_description() {
        let renderer = StructurizrRenderer::new();
        let organism = Organism::new(
            uuid::Uuid::new_v4(),
            "Customer".to_string(),
            OrganismKind::Human,
            "Online shopper".to_string(),
            genome_id(),
        )
        .unwrap();

        let output = renderer.render_organism(&organism);
        assert_eq!(output, "    person \"Customer\" \"Online shopper\"");
    }

    #[test]
    fn structurizr_renders_cell() {
        let renderer = StructurizrRenderer::new();
        let cell = Cell::new(
            uuid::Uuid::new_v4(),
            "Commerce Platform".to_string(),
            "Handles e-commerce".to_string(),
            genome_id(),
        );

        let output = renderer.render_cell(&cell);
        assert_eq!(
            output,
            "    softwareSystem \"Commerce Platform\" \"Handles e-commerce\""
        );
    }

    #[test]
    fn structurizr_renders_organelle_with_technology() {
        let renderer = StructurizrRenderer::new();
        let organelle = Organelle::new(
            uuid::Uuid::new_v4(),
            "API Gateway".to_string(),
            OrganelleKind::Service,
            uuid::Uuid::new_v4(),
            genome_id(),
        )
        .with_description("REST API".to_string())
        .with_technology("Rust/Actix".to_string());

        let output = renderer.render_organelle(&organelle);
        assert_eq!(
            output,
            "        container \"API Gateway\" \"REST API\" \"Rust/Actix\""
        );
    }

    #[test]
    fn structurizr_renders_organelle_with_technology_no_description() {
        let renderer = StructurizrRenderer::new();
        let organelle = Organelle::new(
            uuid::Uuid::new_v4(),
            "API Gateway".to_string(),
            OrganelleKind::Service,
            uuid::Uuid::new_v4(),
            genome_id(),
        )
        .with_technology("Rust/Actix".to_string());

        let output = renderer.render_organelle(&organelle);
        assert_eq!(output, "        container \"API Gateway\" \"\" \"Rust/Actix\"");
    }

    #[test]
    fn structurizr_renders_channel_with_technology() {
        let renderer = StructurizrRenderer::new();
        let channel = Channel::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "customer".to_string(),
            uuid::Uuid::new_v4(),
            "api".to_string(),
            genome_id(),
        )
        .with_description("Uses".to_string())
        .with_technology("REST".to_string());

        let output = renderer.render_channel(&channel);
        assert_eq!(output, "        customer -> api \"Uses\" \"REST\"");
    }

    #[test]
    fn structurizr_renders_channel_description_only() {
        let renderer = StructurizrRenderer::new();
        let channel = Channel::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "api".to_string(),
            uuid::Uuid::new_v4(),
            "database".to_string(),
            genome_id(),
        )
        .with_description("Reads from and writes to".to_string());

        let output = renderer.render_channel(&channel);
        assert_eq!(output, "        api -> database \"Reads from and writes to\"");
    }

    #[test]
    fn structurizr_renders_channel_no_description_no_technology() {
        let renderer = StructurizrRenderer::new();
        let channel = Channel::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "api".to_string(),
            uuid::Uuid::new_v4(),
            "database".to_string(),
            genome_id(),
        );

        let output = renderer.render_channel(&channel);
        assert_eq!(output, "        api -> database");
    }

    #[test]
    fn structurizr_renders_enterprise() {
        let renderer = StructurizrRenderer::new();

        let start = renderer.render_enterprise_start(Some("Our Company"));
        assert_eq!(start, "    enterprise \"Our Company\" {");

        let end = renderer.render_enterprise_end();
        assert_eq!(end, "    }");
    }

    #[test]
    fn structurizr_renders_enterprise_without_name() {
        let renderer = StructurizrRenderer::new();

        let start = renderer.render_enterprise_start(None);
        assert_eq!(start, "    enterprise {");
    }

    #[test]
    fn structurizr_renders_group() {
        let renderer = StructurizrRenderer::new();

        let start = renderer.render_group_start("Backend Services");
        assert_eq!(start, "        group \"Backend Services\" {");

        let end = renderer.render_group_end();
        assert_eq!(end, "        }");
    }

    #[test]
    fn structurizr_renders_system_context_view() {
        let renderer = StructurizrRenderer::new();

        let view = LocusView::new(
            uuid::Uuid::new_v4(),
            "System Context".to_string(),
            ViewType::SystemContext,
            genome_id(),
        )
        .with_scope(uuid::Uuid::new_v4(), "commerce".to_string());

        let output = renderer.render_view(&view);
        assert_eq!(output, "    systemContext commerce {");
    }

    #[test]
    fn structurizr_renders_container_view() {
        let renderer = StructurizrRenderer::new();

        let view = LocusView::new(
            uuid::Uuid::new_v4(),
            "Container View".to_string(),
            ViewType::Container,
            genome_id(),
        )
        .with_scope(uuid::Uuid::new_v4(), "commerce".to_string());

        let output = renderer.render_view(&view);
        assert_eq!(output, "    container commerce {");
    }

    #[test]
    fn structurizr_renders_workspace() {
        let renderer = StructurizrRenderer::new();

        let output = renderer.render_workspace("Commerce", "E-commerce system");
        assert_eq!(output, "workspace \"Commerce\" \"E-commerce system\" {\n");
    }

    #[test]
    fn structurizr_renders_workspace_without_description() {
        let renderer = StructurizrRenderer::new();

        let output = renderer.render_workspace("Commerce", "");
        assert_eq!(output, "workspace \"Commerce\" {\n");
    }

    #[test]
    fn structurizr_renders_chromosome() {
        let renderer = StructurizrRenderer::new();
        let genome_id = genome_id();
        let chromosome = Chromosome::new(
            uuid::Uuid::new_v4(),
            "Cart".to_string(),
            "Shopping cart logic".to_string(),
            genome_id.clone(),
        )
        .unwrap();

        let output = renderer.render_chromosome(&chromosome);
        assert_eq!(
            output,
            "            component \"Cart\" \"Shopping cart logic\""
        );
    }

    #[test]
    fn structurizr_renders_styles() {
        let renderer = StructurizrRenderer::new();
        let styles = renderer.render_styles();

        assert!(styles.contains("element \"Element\""));
        assert!(styles.contains("element \"Person\""));
        assert!(styles.contains("element \"Database\""));
        assert!(styles.contains("shape \"Person\""));
        assert!(styles.contains("shape \"Cylinder\""));
    }

    #[test]
    fn organelle_kind_to_tag() {
        let service = OrganelleKind::Service;
        let database = OrganelleKind::Database;
        let worker = OrganelleKind::Worker;
        let queue = OrganelleKind::Queue;

        // These would be used for tag-based filtering
        assert_eq!(format!("{:?}", service), "Service");
        assert_eq!(format!("{:?}", database), "Database");
        assert_eq!(format!("{:?}", worker), "Worker");
        assert_eq!(format!("{:?}", queue), "Queue");
    }
}