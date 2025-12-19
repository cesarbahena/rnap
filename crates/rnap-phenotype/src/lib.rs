pub mod phenotype;
pub mod protein;

pub use phenotype::{InMemoryPhenotypeRepository, Phenotype, PhenotypeError, PhenotypeRepository};
pub use protein::{InMemoryProteinRepository, Protein, ProteinRepository, ProteinResult};
