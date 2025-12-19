use rnap_genome::GenomeId;

/// Represents a Receptor (Relationship) between domain elements.
///
/// In C4 diagrams, relationships are shown as arrows connecting elements.
/// A Receptor connects a source element to a target element with a description
/// and optional technology annotation.
#[derive(Debug, Clone)]
pub struct Receptor {
    id: uuid::Uuid,
    source_id: uuid::Uuid,
    source_name: String,
    target_id: uuid::Uuid,
    target_name: String,
    description: String,
    technology: Option<String>,
    genome_id: GenomeId,
}

impl Receptor {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn genome_id() -> GenomeId {
        GenomeId::new()
    }

    #[test]
    fn receptor_can_be_created_with_source_and_target() {
        let gid = genome_id();
        let receptor = Receptor::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "customer".to_string(),
            uuid::Uuid::new_v4(),
            "api".to_string(),
            gid,
        );

        assert_eq!(receptor.source_name(), "customer");
        assert_eq!(receptor.target_name(), "api");
    }

    #[test]
    fn receptor_allows_description_and_technology() {
        let gid = genome_id();
        let receptor = Receptor::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "api".to_string(),
            uuid::Uuid::new_v4(),
            "database".to_string(),
            gid,
        )
        .with_description("Reads from and writes to".to_string())
        .with_technology("PostgreSQL".to_string());

        assert_eq!(receptor.description(), "Reads from and writes to");
        assert_eq!(receptor.technology(), Some("PostgreSQL"));
    }
}
