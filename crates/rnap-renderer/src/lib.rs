//! Diagram renderer for Structurizr DSL output.

use rnap_genome::GenomeId;
use rnap_organism::Organism;
use rnap_chromosome::Chromosome;
use rnap_channel::Channel;

pub trait DiagramRenderer: Send + Sync {
    fn render_workspace(&self, name: &str, description: &str) -> String;
    fn render_organism(&self, organism: &Organism) -> String;
    fn render_cell(&self, cell: &Cell) -> String;
    fn render_organelle(&self, organelle: &Organelle) -> String;
    fn render_chromosome(&self, chromosome: &Chromosome) -> String;
    fn render_channel(&self, channel: &Channel) -> String;
    fn render_enterprise_start(&self, name: Option<&str>) -> String;
    fn render_enterprise_end(&self) -> String;
    fn render_group_start(&self, name: &str) -> String;
    fn render_group_end(&self) -> String;
    fn render_view(&self, view: &LocusView) -> String;
    fn render_styles(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Cell {
    id: uuid::Uuid,
    name: String,
    description: String,
    tags: Vec<String>,
    genome_id: GenomeId,
}

impl Cell {
    pub fn new(id: uuid::Uuid, name: String, description: String, genome_id: GenomeId) -> Self {
        Self { id, name, description, tags: Vec::new(), genome_id }
    }
    pub fn id(&self) -> &uuid::Uuid { &self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn description(&self) -> &str { &self.description }
    pub fn tags(&self) -> &[String] { &self.tags }
    pub fn genome_id(&self) -> &GenomeId { &self.genome_id }
}

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
    Service, Worker, Database, Queue, Frontend, Infrastructure,
}

impl Organelle {
    pub fn new(id: uuid::Uuid, name: String, kind: OrganelleKind, cell_id: uuid::Uuid, genome_id: GenomeId) -> Self {
        Self { id, name, description: String::new(), technology: None, kind, cell_id, tags: Vec::new(), genome_id }
    }
    pub fn id(&self) -> &uuid::Uuid { &self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn description(&self) -> &str { &self.description }
    pub fn technology(&self) -> Option<&str> { self.technology.as_deref() }
    pub fn kind(&self) -> &OrganelleKind { &self.kind }
    pub fn cell_id(&self) -> &uuid::Uuid { &self.cell_id }
    pub fn tags(&self) -> &[String] { &self.tags }
    pub fn genome_id(&self) -> &GenomeId { &self.genome_id }
    pub fn with_description(mut self, description: String) -> Self { self.description = description; self }
    pub fn with_technology(mut self, technology: String) -> Self { self.technology = Some(technology); self }
}

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
    SystemLandscape, SystemContext, Container, Component,
}

impl LocusView {
    pub fn new(id: uuid::Uuid, name: String, view_type: ViewType, genome_id: GenomeId) -> Self {
        Self { id, name, view_type, scope_id: None, scope_name: None, genome_id }
    }
    pub fn id(&self) -> &uuid::Uuid { &self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn view_type(&self) -> &ViewType { &self.view_type }
    pub fn scope_id(&self) -> Option<&uuid::Uuid> { self.scope_id.as_ref() }
    pub fn scope_name(&self) -> Option<&str> { self.scope_name.as_deref() }
    pub fn genome_id(&self) -> &GenomeId { &self.genome_id }
    pub fn with_scope(mut self, scope_id: uuid::Uuid, scope_name: String) -> Self {
        self.scope_id = Some(scope_id);
        self.scope_name = Some(scope_name);
        self
    }
}

pub struct StructurizrRenderer;

impl StructurizrRenderer {
    pub fn new() -> Self { Self }
}

impl Default for StructurizrRenderer {
    fn default() -> Self { Self::new() }
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
            format!("    person \"{}\" \"{}\"", organism.name(), organism.description())
        }
    }

    fn render_cell(&self, cell: &Cell) -> String {
        if cell.description().is_empty() {
            format!("    softwareSystem \"{}\"", cell.name())
        } else {
            format!("    softwareSystem \"{}\" \"{}\"", cell.name(), cell.description())
        }
    }

    fn render_organelle(&self, organelle: &Organelle) -> String {
        let name = organelle.name();
        let desc = organelle.description();
        let tech = organelle.technology();
        match (desc, tech) {
            (_, Some(tech)) if !desc.is_empty() => format!("        container \"{}\" \"{}\" \"{}\"", name, desc, tech),
            (_, Some(tech)) => format!("        container \"{}\" \"\" \"{}\"", name, tech),
            _ => format!("        container \"{}\"", name),
        }
    }

    fn render_chromosome(&self, chromosome: &Chromosome) -> String {
        if chromosome.description().is_empty() {
            format!("            component \"{}\"", chromosome.name())
        } else {
            format!("            component \"{}\" \"{}\"", chromosome.name(), chromosome.description())
        }
    }

    fn render_channel(&self, channel: &Channel) -> String {
        format!("    // {}: {}", channel.description(), channel.relationship_type().label())
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
            ViewType::SystemLandscape => "    systemLandscape {".to_string(),
            ViewType::SystemContext => format!("    systemContext {} {{", view.scope_name().unwrap_or("system")),
            ViewType::Container => format!("    container {} {{", view.scope_name().unwrap_or("system")),
            ViewType::Component => format!("    component {} {{", view.scope_name().unwrap_or("container")),
        }
    }

    fn render_styles(&self) -> String {
        "    styles {\n\
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
    }".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn genome_id() -> GenomeId { GenomeId::new() }

    #[test]
    fn structurizr_renders_organism() {
        let renderer = StructurizrRenderer::new();
        let organism = Organism::new(
            uuid::Uuid::new_v4(), "Customer".to_string(), 
            rnap_organism::OrganismKind::Human, "Online shopper".to_string(), genome_id()
        ).unwrap();
        let output = renderer.render_organism(&organism);
        assert_eq!(output, "    person \"Customer\" \"Online shopper\"");
    }
}
