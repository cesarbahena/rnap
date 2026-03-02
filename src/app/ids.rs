use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InsulatorId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenomeId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TfId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GeneFamilyId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GeneFamilyGenerationId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SequenceDefinitionId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LocusId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransposonId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AlleleId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MutationId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GeneId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IntronId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IntronSequenceId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChromosomeId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TranscriptomeId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExonId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExplorationGraphId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExplorationNodeId(pub(crate) u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExplorationEdgeId(pub(crate) u64);

impl ExplorationGraphId {
    pub fn from_raw(value: u64) -> Self {
        Self(value)
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

impl ExplorationNodeId {
    pub fn from_raw(value: u64) -> Self {
        Self(value)
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

impl ExplorationEdgeId {
    pub fn raw(self) -> u64 {
        self.0
    }
}
