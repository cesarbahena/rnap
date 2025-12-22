//! Cell — top-level software system boundary (C4 Software System level).
//!
//! A Cell represents an owned system with clear ownership and responsibility.
//! Maps to C4 Software System in Structurizr.

use rnap_genome::GenomeId;

/// A top-level software system boundary.
///
/// A Cell is the top-level owned system in the domain graph. It contains
/// Organelles (runtime units) and can have Channels to other Cells, Organelles,
/// and Organisms.
///
/// Maps to Structurizr's `softwareSystem` element.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cell {
    id: uuid::Uuid,
    name: String,
    description: String,
    genome_id: GenomeId,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum CellError {
    #[error("cell name must not be empty")]
    EmptyName,
}

impl Cell {
    /// Creates a new Cell with the given parameters.
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        genome_id: GenomeId,
    ) -> Result<Self, CellError> {
        if name.trim().is_empty() {
            return Err(CellError::EmptyName);
        }
        Ok(Self {
            id,
            name,
            description,
            genome_id,
        })
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

    pub fn genome_id(&self) -> &GenomeId {
        &self.genome_id
    }
}

/// Repository trait for Cell persistence.
pub trait CellRepository: Send + Sync {
    fn save(&mut self, cell: Cell) -> Result<(), CellRepositoryError>;
    fn find_by_id(&self, id: &uuid::Uuid) -> Option<Cell>;
    fn find_all(&self) -> Vec<Cell>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum CellRepositoryError {
    #[error("cell not found")]
    NotFound,
    #[error("save failed: {0}")]
    SaveFailed(String),
}

impl From<String> for CellRepositoryError {
    fn from(err: String) -> Self {
        CellRepositoryError::SaveFailed(err)
    }
}

/// In-memory Cell repository for testing.
pub struct InMemoryCellRepository {
    cells: std::sync::RwLock<std::collections::HashMap<uuid::Uuid, Cell>>,
}

impl InMemoryCellRepository {
    pub fn new() -> Self {
        Self {
            cells: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

impl Default for InMemoryCellRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl CellRepository for InMemoryCellRepository {
    fn save(&mut self, cell: Cell) -> Result<(), CellRepositoryError> {
        let mut cells = self.cells.write().map_err(|e| e.to_string())?;
        cells.insert(*cell.id(), cell);
        Ok(())
    }

    fn find_by_id(&self, id: &uuid::Uuid) -> Option<Cell> {
        let cells = self.cells.read().ok()?;
        cells.get(id).cloned()
    }

    fn find_all(&self) -> Vec<Cell> {
        let cells = self.cells.read().unwrap();
        cells.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn genome_id() -> GenomeId {
        GenomeId::new()
    }

    #[test]
    fn cell_can_be_created_with_name_and_description() {
        let gid = genome_id();
        let cell = Cell::new(
            uuid::Uuid::new_v4(),
            "Order System".to_string(),
            "Handles order lifecycle from creation to fulfillment".to_string(),
            gid,
        )
        .unwrap();

        assert_eq!(cell.name(), "Order System");
        assert_eq!(
            cell.description(),
            "Handles order lifecycle from creation to fulfillment"
        );
        assert_eq!(cell.genome_id(), &gid);
    }

    #[test]
    fn cell_rejects_empty_name() {
        let gid = genome_id();
        let result = Cell::new(uuid::Uuid::new_v4(), "".to_string(), "desc".to_string(), gid);
        assert_eq!(result, Err(CellError::EmptyName));
    }

    #[test]
    fn cell_rejects_whitespace_only_name() {
        let gid = genome_id();
        let result = Cell::new(uuid::Uuid::new_v4(), "   ".to_string(), "desc".to_string(), gid);
        assert_eq!(result, Err(CellError::EmptyName));
    }

    #[test]
    fn cell_allows_empty_description() {
        let gid = genome_id();
        let cell = Cell::new(uuid::Uuid::new_v4(), "Auth System".to_string(), "".to_string(), gid).unwrap();
        assert_eq!(cell.name(), "Auth System");
        assert_eq!(cell.description(), "");
    }

    #[test]
    fn in_memory_cell_repository_saves_and_finds() {
        let gid = genome_id();
        let id = uuid::Uuid::new_v4();
        let cell = Cell::new(
            id,
            "Payment System".to_string(),
            "Processes payments".to_string(),
            gid,
        )
        .unwrap();

        let mut repo = InMemoryCellRepository::new();
        repo.save(cell).unwrap();

        let found = repo.find_by_id(&id).unwrap();
        assert_eq!(found.name(), "Payment System");
    }

    #[test]
    fn in_memory_cell_repository_find_all() {
        let gid = genome_id();
        let cell1 = Cell::new(uuid::Uuid::new_v4(), "System A".to_string(), "A".to_string(), gid).unwrap();
        let cell2 = Cell::new(uuid::Uuid::new_v4(), "System B".to_string(), "B".to_string(), gid).unwrap();

        let mut repo = InMemoryCellRepository::new();
        repo.save(cell1).unwrap();
        repo.save(cell2).unwrap();

        let all = repo.find_all();
        assert_eq!(all.len(), 2);
    }
}