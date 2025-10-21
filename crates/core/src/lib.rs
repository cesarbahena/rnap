use serde::{Deserialize, Serialize};

pub type GeneId = String;
pub type Locus = String;

#[derive(Debug, Clone, Serialize)]
pub struct Mutation {
    id: String,
    gene_id: GeneId,
    locus: Locus,
    value: serde_json::Value,
    created_at: i64,
}

impl Mutation {
    pub fn new(
        id: String,
        gene_id: String,
        locus: Locus,
        value: serde_json::Value,
        created_at: i64,
    ) -> Self {
        Self {
            id,
            gene_id,
            locus,
            value,
            created_at,
        }
    }

    pub fn gene_id(&self) -> &GeneId {
        &self.gene_id
    }

    pub fn locus(&self) -> &Locus {
        &self.locus
    }

    pub fn value(&self) -> &serde_json::Value {
        &self.value
    }
}
