use serde::{Deserialize, Serialize};

use super::{
    Allele, Exon, GeneFamily, GeneFamilyGeneration, GenomeId, Grn, GrnId, Insulator, InsulatorId,
    InsulatorPlacement, InsulatorPlacementStrategy, Intron, IntronId, IntronSequence, Locus,
    Mutation, NormalizedArtifact, Sequence, SequenceType, SequenceValue, TfId, Transcriptome,
    Transposon,
};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ProvisionInsulator {
    pub name: String,
    pub placement_region: String,
    pub placement_strategy: Option<InsulatorPlacementStrategy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateGenome {
    pub insulator_id: InsulatorId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateTf {
    pub insulator_id: InsulatorId,
    pub display_name: String,
    pub external_subject: Option<String>,
    pub identity_provider: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateGrn {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub name: String,
    pub activator: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DefineGeneFamily {
    pub insulator_id: InsulatorId,
    pub genome_id: Option<GenomeId>,
    pub name: String,
    pub abbreviation: String,
    pub normalized_artifact: Option<NormalizedArtifact>,
    pub sequences: Vec<DefineSequence>,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DefineSequence {
    pub name: String,
    pub sequence_type: SequenceType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DefinedGeneFamily {
    pub family: GeneFamily,
    pub generation: GeneFamilyGeneration,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MutateNew {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub grn_id: GrnId,
    pub gene_family_abbreviation: String,
    pub locus_name: String,
    pub mutations: Vec<SequenceMutation>,
    pub causes: Vec<String>,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MutateExisting {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub grn_id: GrnId,
    pub gene_fqn: String,
    pub mutations: Vec<SequenceMutation>,
    pub causes: Vec<String>,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SequenceMutation {
    pub sequence_name: String,
    pub value: SequenceValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MutatedAllele {
    pub locus: Locus,
    pub transposon: Option<Transposon>,
    pub allele: Allele,
    pub mutations: Vec<Mutation>,
    pub gene_fqn: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TranscribeAllele {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub grn_id: GrnId,
    pub gene_fqn: String,
    pub full: bool,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TranscribedAllele {
    pub allele: Allele,
    pub transcriptome: Transcriptome,
    pub sequences: Vec<Sequence>,
    pub approval_comments_visible: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SpliceAllele {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub grn_id: GrnId,
    pub gene_fqn: String,
    pub exon_texts: Vec<String>,
    pub lgtm: bool,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SpliceResult {
    pub allele: Allele,
    pub exons: Vec<Exon>,
    pub untranscribed_unexpressed_mutations: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TranslateAllele {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub grn_id: GrnId,
    pub gene_fqn: String,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TranslatedAllele {
    pub allele: Allele,
    pub exons: Vec<Exon>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AttachEnhancerPromoter {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub grn_id: GrnId,
    pub enhancer_gene_fqn: String,
    pub promoter_gene_fqn: String,
    pub updated_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreateIntron {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub grn_id: GrnId,
    pub target_mrna_fqn: String,
    pub target_sequence_name: Option<String>,
    pub title: String,
    pub body: Option<String>,
    pub precursor: Option<IntronId>,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AppendIntronSequence {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub grn_id: GrnId,
    pub target_mrna_fqn: Option<String>,
    pub target_sequence_name: Option<String>,
    pub intron_title: String,
    pub body: Option<String>,
    pub follow_up_title: Option<String>,
    pub follow_up_body: Option<String>,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppendedIntronSequence {
    pub intron: Intron,
    pub sequence: Option<IntronSequence>,
    pub follow_up: Option<Intron>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntronSummary {
    pub intron: Intron,
    pub latest_sequence: Option<IntronSequence>,
    pub has_precursor: bool,
    pub child_count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IntronThread {
    pub intron: Intron,
    pub sequences: Vec<IntronSequence>,
    pub precursors: Vec<IntronSummary>,
    pub children: Vec<IntronSummary>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CreatedGrn {
    pub grn: Grn,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ProvisionedInsulator {
    pub insulator: Insulator,
    pub placement: InsulatorPlacement,
}
