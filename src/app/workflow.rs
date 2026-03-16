use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::{
    AlleleId, ChromosomeId, ExonId, GeneId, GenomeId, IntronId, IntronSequenceId, LocusId,
    MutationId, SequenceDefinitionId, SequenceHash, TfId, TranscriptomeId,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Chromosome {
    pub id: ChromosomeId,
    pub genome_id: GenomeId,
    pub locus_id: LocusId,
    pub genes: Vec<GeneId>,
    pub alleles: Vec<AlleleId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Transcriptome {
    pub id: TranscriptomeId,
    pub chromosome_id: ChromosomeId,
    pub allele_id: AlleleId,
    pub sequences: Vec<TranscriptSequenceCursor>,
    pub created_by: TfId,
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
pub struct Exon {
    pub id: ExonId,
    pub allele_id: AlleleId,
    pub text: String,
    pub depends_on: Vec<ExonId>,
    pub created_by: TfId,
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EnhancerContext {
    pub enhancer_locus_id: LocusId,
    pub promoter_locus_id: LocusId,
    pub updated_by: TfId,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Intron {
    pub id: IntronId,
    pub target_mrna_locus_id: LocusId,
    pub target_sequence_definition_id: Option<SequenceDefinitionId>,
    pub precursor: Option<IntronId>,
    pub title: String,
    pub body: Option<String>,
    pub normalized_title: String,
    pub title_scope_hash: String,
    pub created_by: TfId,
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct IntronSequence {
    pub id: IntronSequenceId,
    pub intron_id: IntronId,
    pub body: String,
    pub created_by: TfId,
    pub created_at: SystemTime,
}
