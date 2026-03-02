use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::{GenomeId, InsulatorId, TfId};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Insulator {
    pub id: InsulatorId,
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct InsulatorPlacement {
    pub insulator_id: InsulatorId,
    pub strategy: InsulatorPlacementStrategy,
    pub region: String,
    pub active: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsulatorPlacementStrategy {
    SharedCluster,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Genome {
    pub id: GenomeId,
    pub insulator_id: InsulatorId,
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Tf {
    pub id: TfId,
    pub insulator_id: InsulatorId,
    pub display_name: String,
    pub external_subject: Option<String>,
    pub identity_provider: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
