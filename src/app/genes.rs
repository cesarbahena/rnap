use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::{
    AlleleId, ChromosomeId, GeneFamilyGenerationId, GeneFamilyId, GeneId, GenomeId, GrnId,
    InsulatorId, LocusId, MutationId, NormalizedArtifact, SemanticNarrowingId,
    SemanticNarrowingSequenceId, SequenceDefinitionId, SequenceType, TransposonId,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GeneFamily {
    pub id: GeneFamilyId,
    pub insulator_id: InsulatorId,
    pub genome_id: Option<GenomeId>,
    pub name: String,
    pub abbreviation: String,
    pub current_generation_id: GeneFamilyGenerationId,
    pub normalized_artifact: NormalizedArtifact,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GeneFamilyGeneration {
    pub id: GeneFamilyGenerationId,
    pub family_id: GeneFamilyId,
    pub generation: u32,
    pub sequences: Vec<SequenceDefinition>,
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SequenceDefinition {
    pub id: SequenceDefinitionId,
    pub name: String,
    pub sequence_type: SequenceType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Locus {
    pub id: LocusId,
    pub family_id: GeneFamilyId,
    pub insulator_id: InsulatorId,
    pub chromosome_id: ChromosomeId,
    pub name: String,
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Transposon {
    pub id: TransposonId,
    pub locus_id: LocusId,
    pub gene_family_generation_id: GeneFamilyGenerationId,
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlleleOrigin {
    Gene(GeneId),
    Transposon(TransposonId),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlleleState {
    Mutating,
    Expressing,
    Selected,
    Degraded,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Allele {
    pub id: AlleleId,
    pub chromosome_id: ChromosomeId,
    pub grn_id: GrnId,
    pub locus_id: LocusId,
    pub gene_family_generation_id: GeneFamilyGenerationId,
    pub generation: u32,
    pub origin: AlleleOrigin,
    pub state: AlleleState,
    pub degraded_at: Option<SystemTime>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Sequence {
    pub definition_id: SequenceDefinitionId,
    pub name: String,
    pub value: SequenceValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SequenceValue {
    String(String),
    StringVec(Vec<String>),
    Int(i64),
    IntVec(Vec<i64>),
    Float(f64),
    FloatVec(Vec<f64>),
    Bool(bool),
    BoolVec(Vec<bool>),
    GeneRef(GeneId),
    GeneRefVec(Vec<GeneId>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Mutation {
    pub id: MutationId,
    pub allele_id: AlleleId,
    pub sequence_definition_id: SequenceDefinitionId,
    pub value: SequenceValue,
    pub context: Vec<MutationContext>,
    pub state: MutationState,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum MutationContext {
    Cause(SemanticNarrowingId, SemanticNarrowingSequenceId),
    AnsweredContext(SemanticNarrowingId, SemanticNarrowingSequenceId),
    UnansweredContext(SemanticNarrowingId),
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SequenceHash(pub(crate) String);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum MutationState {
    Unexpressed,
    Expressing,
}
