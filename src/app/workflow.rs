use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::{
    AlleleId, LocusId, MutationId, SemanticNarrowingId, SemanticNarrowingSequenceId,
    SequenceDefinitionId, SequenceHash, TaskRealizationId, TranscriptomeId,
};

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
