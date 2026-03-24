use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::{
    AlleleId, ChromosomeId, GeneFamilyId, GenomeId, GrnId, InsulatorId, LocusId, MutationId,
    SemanticNarrowingId, SemanticNarrowingSequenceId, SequenceDefinitionId, SequenceHash, SignalId,
    TaskRealizationId, TfId, TranscriptomeId, TransposonId,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Chromosome {
    pub id: ChromosomeId,
    pub genome_id: GenomeId,
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub degraded_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Transcriptome {
    pub id: TranscriptomeId,
    pub locus_id: LocusId,
    pub allele_id: AlleleId,
    pub sequences: Vec<TranscriptSequenceCursor>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TranscriptSequenceCursor {
    pub sequence_definition_id: SequenceDefinitionId,
    pub last_rendered_mutation_id: Option<MutationId>,
    pub last_rendered_sequence_hash: Option<SequenceHash>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TaskRealization {
    pub id: TaskRealizationId,
    pub allele_id: AlleleId,
    pub text: String,
    pub depends_on: Vec<TaskRealizationId>,
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SemanticNarrowing {
    pub id: SemanticNarrowingId,
    pub target_mrna_locus_id: LocusId,
    pub target_sequence_definition_id: Option<SequenceDefinitionId>,
    pub precursor: Option<SemanticNarrowingId>,
    pub title: String,
    pub body: Option<String>,
    pub normalized_title: String,
    pub title_scope_hash: String,
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SemanticNarrowingSequence {
    pub id: SemanticNarrowingSequenceId,
    pub semantic_narrowing_id: SemanticNarrowingId,
    pub body: String,
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Signal {
    pub id: SignalId,
    pub insulator_id: InsulatorId,
    pub tf_id: Option<TfId>,
    pub signal_type: SignalType,
    pub target: SignalTarget,
    pub occurred_at: SystemTime,
    pub reason: Option<String>,
    pub payload: SignalPayload,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum SignalType {
    InsulatorProvisioned,
    GenomeCreated,
    TfCreated,
    GrnCreated,
    ChromosomeCreated,
    GeneFamilyDefined,
    LocusCreated,
    TransposonCreated,
    AlleleCreated,
    MutationChanged,
    TaskRealizationCreated,
    MutationsExpressed,
    SemanticNarrowingCreated,
    SemanticNarrowingAnswered,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum SignalTarget {
    Insulator(InsulatorId),
    Genome(GenomeId),
    Grn(GrnId),
    Chromosome(ChromosomeId),
    Tf(TfId),
    GeneFamily(GeneFamilyId),
    Locus(LocusId),
    Transposon(TransposonId),
    Allele(AlleleId),
    Mutation(MutationId),
    TaskRealization(TaskRealizationId),
    SemanticNarrowing(SemanticNarrowingId),
    SemanticNarrowingSequence(SemanticNarrowingSequenceId),
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum SignalPayload {
    Empty,
    Text(String),
}
