use std::collections::BTreeMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InsulatorId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenomeId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TfId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GeneFamilyId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GeneFamilyGenerationId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SequenceDefinitionId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LocusId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TransposonId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AlleleId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MutationId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GeneId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChromosomeId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TranscriptomeId(u64);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExonId(u64);

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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GeneFamily {
    pub id: GeneFamilyId,
    pub insulator_id: InsulatorId,
    pub genome_id: Option<GenomeId>,
    pub name: String,
    pub abbreviation: String,
    pub current_generation_id: GeneFamilyGenerationId,
    pub encodes: EncodingType,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GeneFamilyGeneration {
    pub id: GeneFamilyGenerationId,
    pub family_id: GeneFamilyId,
    pub generation: u32,
    pub sequences: Vec<SequenceDefinition>,
    pub created_by: TfId,
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
    pub genome_id: GenomeId,
    pub name: String,
    pub created_at: SystemTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Transposon {
    pub id: TransposonId,
    pub locus_id: LocusId,
    pub gene_family_generation_id: GeneFamilyGenerationId,
    pub created_by: TfId,
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
    Spliced,
    StaleSplice,
    StaleTranscript,
    Selected,
    Degraded,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Allele {
    pub id: AlleleId,
    pub genome_id: GenomeId,
    pub locus_id: LocusId,
    pub gene_family_generation_id: GeneFamilyGenerationId,
    pub generation: u32,
    pub origin: AlleleOrigin,
    pub state: AlleleState,
    pub created_by: TfId,
    pub degraded_at: Option<SystemTime>,
    pub degraded_by: Option<TfId>,
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
    pub created_by: TfId,
    pub created_at: SystemTime,
}

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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum SequenceType {
    String,
    StringVec,
    Int,
    IntVec,
    Float,
    FloatVec,
    Bool,
    BoolVec,
    Gene,
    GeneVec,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodingType {
    RNA(RnaType),
    GRN(GrnType),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrnType {
    Promoter,
    Telomere,
    Centromere,
    Silencer,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RnaType {
    Translation(TranslationRnaType),
    Regulatory(RegulatoryRnaType),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranslationRnaType {
    MRNA,
    RRNA,
    TRNA,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegulatoryRnaType {
    SnRNA,
    SiRNA,
    TmRNA,
    GRNA,
    MiRNA,
    PiRNA,
    ERNA,
    SnoRNA,
    CrRNA,
    TracrRNA,
    LncRNA,
    CircRNA,
    SgRNA,
}

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
pub struct DefineGeneFamily {
    pub insulator_id: InsulatorId,
    pub genome_id: Option<GenomeId>,
    pub name: String,
    pub abbreviation: String,
    pub encodes: Option<EncodingType>,
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
    pub gene_family_abbreviation: String,
    pub locus_name: String,
    pub mutations: Vec<SequenceMutation>,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MutateExisting {
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub gene_fqn: String,
    pub mutations: Vec<SequenceMutation>,
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
    pub gene_fqn: String,
    pub exon_texts: Vec<String>,
    pub lgtm: bool,
    pub created_by: TfId,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SplicedAllele {
    pub allele: Allele,
    pub exons: Vec<Exon>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ProvisionedInsulator {
    pub insulator: Insulator,
    pub placement: InsulatorPlacement,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum DnapError {
    BlankInsulatorName,
    BlankPlacementRegion,
    BlankGenomeName,
    BlankTfDisplayName,
    BlankGeneFamilyName,
    BlankGeneFamilyAbbreviation,
    BlankSequenceDefinitionName,
    DuplicateSequenceDefinitionName,
    DuplicateGeneFamilyAbbreviation,
    DuplicateActiveAllele,
    MissingEncodingType,
    MissingMutation,
    BlankLocusName,
    BlankGeneFamilyLookup,
    BlankMutationSequenceName,
    BlankExonText,
    GeneFamilyNotFound,
    SequenceDefinitionNotFound,
    SequenceValueTypeMismatch,
    GeneFqnNotFound,
    AmbiguousGeneFqn,
    AlleleNotFound,
    AlleleCannotMutate,
    StaleSpliceRequiresTranscribe,
    LgtmRequiresStaleTranscript,
    InsulatorNotFound,
    GenomeNotFound,
    GenomeInsulatorMismatch,
    TfNotFound,
    TfInsulatorMismatch,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Dnap {
    next_insulator_id: u64,
    next_genome_id: u64,
    next_tf_id: u64,
    next_gene_family_id: u64,
    next_gene_family_generation_id: u64,
    next_sequence_definition_id: u64,
    next_locus_id: u64,
    next_transposon_id: u64,
    next_allele_id: u64,
    next_mutation_id: u64,
    next_chromosome_id: u64,
    next_transcriptome_id: u64,
    next_exon_id: u64,
    insulators: BTreeMap<InsulatorId, Insulator>,
    placements: BTreeMap<InsulatorId, InsulatorPlacement>,
    genomes: BTreeMap<GenomeId, Genome>,
    tfs: BTreeMap<TfId, Tf>,
    gene_families: BTreeMap<GeneFamilyId, GeneFamily>,
    gene_family_generations: BTreeMap<GeneFamilyGenerationId, GeneFamilyGeneration>,
    loci: BTreeMap<LocusId, Locus>,
    transposons: BTreeMap<TransposonId, Transposon>,
    alleles: BTreeMap<AlleleId, Allele>,
    mutations: BTreeMap<MutationId, Mutation>,
    chromosomes: BTreeMap<LocusId, Chromosome>,
    transcriptomes: BTreeMap<AlleleId, Transcriptome>,
    exons: BTreeMap<ExonId, Exon>,
}

impl Dnap {
    pub fn provision_insulator(
        &mut self,
        input: ProvisionInsulator,
    ) -> Result<ProvisionedInsulator, DnapError> {
        let name = require_text(input.name, DnapError::BlankInsulatorName)?;
        let region = require_text(input.placement_region, DnapError::BlankPlacementRegion)?;
        let now = SystemTime::now();
        let insulator_id = self.allocate_insulator_id();
        let insulator = Insulator {
            id: insulator_id,
            name,
            created_at: now,
            updated_at: now,
        };
        let placement = InsulatorPlacement {
            insulator_id,
            strategy: input
                .placement_strategy
                .unwrap_or(InsulatorPlacementStrategy::SharedCluster),
            region,
            active: true,
            created_at: now,
            updated_at: now,
        };

        self.insulators.insert(insulator_id, insulator.clone());
        self.placements.insert(insulator_id, placement.clone());

        Ok(ProvisionedInsulator {
            insulator,
            placement,
        })
    }

    pub fn create_genome(&mut self, input: CreateGenome) -> Result<Genome, DnapError> {
        self.require_insulator(input.insulator_id)?;
        let name = require_text(input.name, DnapError::BlankGenomeName)?;
        let now = SystemTime::now();
        let genome = Genome {
            id: self.allocate_genome_id(),
            insulator_id: input.insulator_id,
            name,
            created_at: now,
            updated_at: now,
        };

        self.genomes.insert(genome.id, genome.clone());
        Ok(genome)
    }

    pub fn create_tf(&mut self, input: CreateTf) -> Result<Tf, DnapError> {
        self.require_insulator(input.insulator_id)?;
        let display_name = require_text(input.display_name, DnapError::BlankTfDisplayName)?;
        let now = SystemTime::now();
        let tf = Tf {
            id: self.allocate_tf_id(),
            insulator_id: input.insulator_id,
            display_name,
            external_subject: input.external_subject,
            identity_provider: input.identity_provider,
            created_at: now,
            updated_at: now,
        };

        self.tfs.insert(tf.id, tf.clone());
        Ok(tf)
    }

    pub fn define_gene_family(
        &mut self,
        input: DefineGeneFamily,
    ) -> Result<DefinedGeneFamily, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;
        if let Some(genome_id) = input.genome_id {
            self.require_genome_in_insulator(genome_id, input.insulator_id)?;
        }

        let name = require_text(input.name, DnapError::BlankGeneFamilyName)?;
        let abbreviation =
            require_text(input.abbreviation, DnapError::BlankGeneFamilyAbbreviation)?;
        let encodes = input.encodes.ok_or(DnapError::MissingEncodingType)?;
        self.require_available_abbreviation(input.insulator_id, input.genome_id, &abbreviation)?;

        let mut seen_sequences = BTreeMap::new();
        let mut sequences = Vec::with_capacity(input.sequences.len());
        for sequence in input.sequences {
            let sequence_name =
                require_text(sequence.name, DnapError::BlankSequenceDefinitionName)?;
            let normalized = normalize_match_text(&sequence_name);
            if seen_sequences.insert(normalized, ()).is_some() {
                return Err(DnapError::DuplicateSequenceDefinitionName);
            }
            sequences.push(SequenceDefinition {
                id: self.allocate_sequence_definition_id(),
                name: sequence_name,
                sequence_type: sequence.sequence_type,
            });
        }

        let now = SystemTime::now();
        let family_id = self.allocate_gene_family_id();
        let generation_id = self.allocate_gene_family_generation_id();
        let family = GeneFamily {
            id: family_id,
            insulator_id: input.insulator_id,
            genome_id: input.genome_id,
            name,
            abbreviation,
            current_generation_id: generation_id,
            encodes,
            created_at: now,
            updated_at: now,
        };
        let generation = GeneFamilyGeneration {
            id: generation_id,
            family_id,
            generation: 1,
            sequences,
            created_by: input.created_by,
            created_at: now,
        };

        self.gene_families.insert(family_id, family.clone());
        self.gene_family_generations
            .insert(generation_id, generation.clone());

        Ok(DefinedGeneFamily { family, generation })
    }

    pub fn resolve_gene_family(
        &self,
        insulator_id: InsulatorId,
        genome_id: Option<GenomeId>,
        abbreviation: &str,
    ) -> Option<&GeneFamily> {
        let normalized = normalize_match_text(abbreviation);
        if let Some(genome_id) = genome_id {
            if let Some(family) = self.gene_families.values().find(|family| {
                family.insulator_id == insulator_id
                    && family.genome_id == Some(genome_id)
                    && normalize_match_text(&family.abbreviation) == normalized
            }) {
                return Some(family);
            }
        }

        self.gene_families.values().find(|family| {
            family.insulator_id == insulator_id
                && family.genome_id.is_none()
                && normalize_match_text(&family.abbreviation) == normalized
        })
    }

    pub fn mutate_new(&mut self, input: MutateNew) -> Result<MutatedAllele, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;
        if input.mutations.is_empty() {
            return Err(DnapError::MissingMutation);
        }

        let family_lookup = require_text(
            input.gene_family_abbreviation,
            DnapError::BlankGeneFamilyLookup,
        )?;
        let family = self
            .resolve_gene_family(input.insulator_id, Some(input.genome_id), &family_lookup)
            .ok_or(DnapError::GeneFamilyNotFound)?
            .clone();
        let generation = self
            .gene_family_generations
            .get(&family.current_generation_id)
            .ok_or(DnapError::GeneFamilyNotFound)?
            .clone();
        let locus_name = require_text(input.locus_name, DnapError::BlankLocusName)?;

        if let Some(locus) = self.find_locus(input.genome_id, family.id, &locus_name) {
            self.require_no_active_allele(locus.id, input.created_by)?;
        }

        let now = SystemTime::now();
        let locus = Locus {
            id: self.allocate_locus_id(),
            family_id: family.id,
            insulator_id: input.insulator_id,
            genome_id: input.genome_id,
            name: locus_name,
            created_at: now,
        };
        let transposon = Transposon {
            id: self.allocate_transposon_id(),
            locus_id: locus.id,
            gene_family_generation_id: generation.id,
            created_by: input.created_by,
            created_at: now,
        };
        let allele = Allele {
            id: self.allocate_allele_id(),
            genome_id: input.genome_id,
            locus_id: locus.id,
            gene_family_generation_id: generation.id,
            generation: 1,
            origin: AlleleOrigin::Transposon(transposon.id),
            state: AlleleState::Mutating,
            created_by: input.created_by,
            degraded_at: None,
            degraded_by: None,
            created_at: now,
            updated_at: now,
        };
        let chromosome = Chromosome {
            id: self.allocate_chromosome_id(),
            genome_id: input.genome_id,
            locus_id: locus.id,
            genes: Vec::new(),
            alleles: vec![allele.id],
        };

        let mutations = self.build_mutations(
            allele.id,
            generation.id,
            input.mutations,
            input.created_by,
            now,
        )?;
        let gene_fqn = self.gene_fqn(&family, &locus, allele.generation);

        self.loci.insert(locus.id, locus.clone());
        self.transposons.insert(transposon.id, transposon.clone());
        self.alleles.insert(allele.id, allele.clone());
        self.chromosomes.insert(locus.id, chromosome);
        for mutation in &mutations {
            self.mutations.insert(mutation.id, mutation.clone());
        }

        Ok(MutatedAllele {
            locus,
            transposon: Some(transposon),
            allele,
            mutations,
            gene_fqn,
        })
    }

    pub fn mutate_existing(&mut self, input: MutateExisting) -> Result<MutatedAllele, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;
        if input.mutations.is_empty() {
            return Err(DnapError::MissingMutation);
        }

        let allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.created_by,
            &input.gene_fqn,
        )?;
        let mut allele = self
            .alleles
            .get(&allele_id)
            .cloned()
            .ok_or(DnapError::AlleleNotFound)?;
        if matches!(allele.state, AlleleState::Selected | AlleleState::Degraded) {
            return Err(DnapError::AlleleCannotMutate);
        }

        let now = SystemTime::now();
        let mutations = self.build_mutations(
            allele.id,
            allele.gene_family_generation_id,
            input.mutations,
            input.created_by,
            now,
        )?;
        allele.state = match allele.state {
            AlleleState::Spliced | AlleleState::StaleTranscript => AlleleState::StaleSplice,
            state => state,
        };
        allele.updated_at = now;

        let locus = self
            .loci
            .get(&allele.locus_id)
            .cloned()
            .ok_or(DnapError::AlleleNotFound)?;
        let family = self
            .gene_families
            .get(&locus.family_id)
            .ok_or(DnapError::GeneFamilyNotFound)?;
        let gene_fqn = self.gene_fqn(family, &locus, allele.generation);

        self.alleles.insert(allele.id, allele.clone());
        for mutation in &mutations {
            self.mutations.insert(mutation.id, mutation.clone());
        }

        Ok(MutatedAllele {
            locus,
            transposon: None,
            allele,
            mutations,
            gene_fqn,
        })
    }

    pub fn transcribe(&mut self, input: TranscribeAllele) -> Result<TranscribedAllele, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

        let allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.created_by,
            &input.gene_fqn,
        )?;
        let mut allele = self
            .alleles
            .get(&allele_id)
            .cloned()
            .ok_or(DnapError::AlleleNotFound)?;
        let previous_cursors = self
            .transcriptomes
            .get(&allele.id)
            .map(|transcriptome| transcriptome.sequences.clone())
            .unwrap_or_default();
        let projection = self.project_allele(allele.id)?;
        let latest_cursors = self.latest_sequence_cursors(allele.id);
        let sequences = if input.full {
            projection
        } else {
            projection
                .into_iter()
                .filter(|sequence| {
                    let previous = previous_cursors
                        .iter()
                        .find(|cursor| cursor.sequence_definition_id == sequence.definition_id)
                        .and_then(|cursor| cursor.last_rendered_mutation_id);
                    let latest = latest_cursors
                        .iter()
                        .find(|cursor| cursor.sequence_definition_id == sequence.definition_id)
                        .and_then(|cursor| cursor.last_rendered_mutation_id);
                    previous != latest
                })
                .collect()
        };

        let now = SystemTime::now();
        if allele.state == AlleleState::StaleSplice {
            allele.state = AlleleState::StaleTranscript;
            allele.updated_at = now;
            self.alleles.insert(allele.id, allele.clone());
        }

        let chromosome_id = self
            .chromosomes
            .get(&allele.locus_id)
            .map(|chromosome| chromosome.id)
            .ok_or(DnapError::AlleleNotFound)?;
        let transcriptome = match self.transcriptomes.get(&allele.id).cloned() {
            Some(mut transcriptome) => {
                transcriptome.sequences = latest_cursors;
                transcriptome.updated_at = now;
                transcriptome
            }
            None => Transcriptome {
                id: self.allocate_transcriptome_id(),
                chromosome_id,
                allele_id: allele.id,
                sequences: latest_cursors,
                created_by: input.created_by,
                created_at: now,
                updated_at: now,
            },
        };
        self.transcriptomes.insert(allele.id, transcriptome.clone());

        Ok(TranscribedAllele {
            allele,
            transcriptome,
            sequences,
            approval_comments_visible: true,
        })
    }

    pub fn splice(&mut self, input: SpliceAllele) -> Result<SplicedAllele, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

        let allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.created_by,
            &input.gene_fqn,
        )?;
        let mut allele = self
            .alleles
            .get(&allele_id)
            .cloned()
            .ok_or(DnapError::AlleleNotFound)?;
        if allele.state == AlleleState::StaleSplice {
            return Err(DnapError::StaleSpliceRequiresTranscribe);
        }
        if input.lgtm && allele.state != AlleleState::StaleTranscript {
            return Err(DnapError::LgtmRequiresStaleTranscript);
        }

        let now = SystemTime::now();
        let mut exons = Vec::new();
        for exon_text in input.exon_texts {
            let text = require_text(exon_text, DnapError::BlankExonText)?;
            let exon = Exon {
                id: self.allocate_exon_id(),
                allele_id: allele.id,
                text,
                depends_on: Vec::new(),
                created_by: input.created_by,
                created_at: now,
            };
            self.exons.insert(exon.id, exon.clone());
            exons.push(exon);
        }

        allele.state = AlleleState::Spliced;
        allele.updated_at = now;
        self.alleles.insert(allele.id, allele.clone());

        Ok(SplicedAllele { allele, exons })
    }

    pub fn project_allele(&self, allele_id: AlleleId) -> Result<Vec<Sequence>, DnapError> {
        let allele = self
            .alleles
            .get(&allele_id)
            .ok_or(DnapError::AlleleNotFound)?;
        let generation = self
            .gene_family_generations
            .get(&allele.gene_family_generation_id)
            .ok_or(DnapError::GeneFamilyNotFound)?;
        let mut latest = BTreeMap::<SequenceDefinitionId, &Mutation>::new();
        for mutation in self
            .mutations
            .values()
            .filter(|mutation| mutation.allele_id == allele_id)
        {
            latest.insert(mutation.sequence_definition_id, mutation);
        }

        Ok(generation
            .sequences
            .iter()
            .filter_map(|definition| {
                latest.get(&definition.id).map(|mutation| Sequence {
                    definition_id: definition.id,
                    name: definition.name.clone(),
                    value: mutation.value.clone(),
                })
            })
            .collect())
    }

    pub fn locus(&self, id: LocusId) -> Option<&Locus> {
        self.loci.get(&id)
    }

    pub fn allele(&self, id: AlleleId) -> Option<&Allele> {
        self.alleles.get(&id)
    }

    pub fn transcriptome(&self, allele_id: AlleleId) -> Option<&Transcriptome> {
        self.transcriptomes.get(&allele_id)
    }

    pub fn find_insulator_by_name(&self, name: &str) -> Option<&Insulator> {
        let normalized = normalize_match_text(name);
        self.insulators
            .values()
            .find(|insulator| normalize_match_text(&insulator.name) == normalized)
    }

    pub fn find_genome_by_name(&self, insulator_id: InsulatorId, name: &str) -> Option<&Genome> {
        let normalized = normalize_match_text(name);
        self.genomes.values().find(|genome| {
            genome.insulator_id == insulator_id && normalize_match_text(&genome.name) == normalized
        })
    }

    pub fn find_tf_by_display_name(&self, insulator_id: InsulatorId, name: &str) -> Option<&Tf> {
        let normalized = normalize_match_text(name);
        self.tfs.values().find(|tf| {
            tf.insulator_id == insulator_id && normalize_match_text(&tf.display_name) == normalized
        })
    }

    pub fn insulator(&self, id: InsulatorId) -> Option<&Insulator> {
        self.insulators.get(&id)
    }

    pub fn active_placement(&self, insulator_id: InsulatorId) -> Option<&InsulatorPlacement> {
        self.placements
            .get(&insulator_id)
            .filter(|placement| placement.active)
    }

    pub fn genome(&self, id: GenomeId) -> Option<&Genome> {
        self.genomes.get(&id)
    }

    pub fn tf(&self, id: TfId) -> Option<&Tf> {
        self.tfs.get(&id)
    }

    pub fn gene_family(&self, id: GeneFamilyId) -> Option<&GeneFamily> {
        self.gene_families.get(&id)
    }

    pub fn gene_family_generation(
        &self,
        id: GeneFamilyGenerationId,
    ) -> Option<&GeneFamilyGeneration> {
        self.gene_family_generations.get(&id)
    }

    fn require_insulator(&self, id: InsulatorId) -> Result<(), DnapError> {
        self.insulators
            .contains_key(&id)
            .then_some(())
            .ok_or(DnapError::InsulatorNotFound)
    }

    fn require_genome_in_insulator(
        &self,
        id: GenomeId,
        insulator_id: InsulatorId,
    ) -> Result<(), DnapError> {
        let genome = self.genomes.get(&id).ok_or(DnapError::GenomeNotFound)?;
        if genome.insulator_id == insulator_id {
            Ok(())
        } else {
            Err(DnapError::GenomeInsulatorMismatch)
        }
    }

    fn require_tf_in_insulator(
        &self,
        id: TfId,
        insulator_id: InsulatorId,
    ) -> Result<(), DnapError> {
        let tf = self.tfs.get(&id).ok_or(DnapError::TfNotFound)?;
        if tf.insulator_id == insulator_id {
            Ok(())
        } else {
            Err(DnapError::TfInsulatorMismatch)
        }
    }

    fn require_available_abbreviation(
        &self,
        insulator_id: InsulatorId,
        genome_id: Option<GenomeId>,
        abbreviation: &str,
    ) -> Result<(), DnapError> {
        let normalized = normalize_match_text(abbreviation);
        let duplicate = self.gene_families.values().any(|family| {
            family.insulator_id == insulator_id
                && family.genome_id == genome_id
                && normalize_match_text(&family.abbreviation) == normalized
        });
        if duplicate {
            Err(DnapError::DuplicateGeneFamilyAbbreviation)
        } else {
            Ok(())
        }
    }

    fn require_no_active_allele(
        &self,
        locus_id: LocusId,
        created_by: TfId,
    ) -> Result<(), DnapError> {
        let duplicate = self.alleles.values().any(|allele| {
            allele.locus_id == locus_id
                && allele.created_by == created_by
                && !matches!(allele.state, AlleleState::Selected | AlleleState::Degraded)
        });
        if duplicate {
            Err(DnapError::DuplicateActiveAllele)
        } else {
            Ok(())
        }
    }

    fn find_locus(
        &self,
        genome_id: GenomeId,
        family_id: GeneFamilyId,
        name: &str,
    ) -> Option<&Locus> {
        let normalized = normalize_match_text(name);
        self.loci.values().find(|locus| {
            locus.genome_id == genome_id
                && locus.family_id == family_id
                && normalize_match_text(&locus.name) == normalized
        })
    }

    fn build_mutations(
        &mut self,
        allele_id: AlleleId,
        generation_id: GeneFamilyGenerationId,
        mutations: Vec<SequenceMutation>,
        created_by: TfId,
        created_at: SystemTime,
    ) -> Result<Vec<Mutation>, DnapError> {
        let generation = self
            .gene_family_generations
            .get(&generation_id)
            .ok_or(DnapError::GeneFamilyNotFound)?
            .clone();
        let mut built = Vec::with_capacity(mutations.len());
        for mutation in mutations {
            let sequence_name =
                require_text(mutation.sequence_name, DnapError::BlankMutationSequenceName)?;
            let definition = resolve_sequence_definition(&generation, &sequence_name)
                .ok_or(DnapError::SequenceDefinitionNotFound)?;
            if !value_matches_sequence_type(&mutation.value, definition.sequence_type) {
                return Err(DnapError::SequenceValueTypeMismatch);
            }
            built.push(Mutation {
                id: self.allocate_mutation_id(),
                allele_id,
                sequence_definition_id: definition.id,
                value: mutation.value,
                created_by,
                created_at,
            });
        }
        Ok(built)
    }

    fn resolve_active_allele_id(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        created_by: TfId,
        gene_fqn: &str,
    ) -> Result<AlleleId, DnapError> {
        let normalized = normalize_match_text(gene_fqn);
        if normalized.is_empty() {
            return Err(DnapError::GeneFqnNotFound);
        }

        let matches = self
            .alleles
            .values()
            .filter(|allele| {
                allele.genome_id == genome_id
                    && allele.created_by == created_by
                    && !matches!(allele.state, AlleleState::Selected | AlleleState::Degraded)
            })
            .filter_map(|allele| {
                let locus = self.loci.get(&allele.locus_id)?;
                if locus.insulator_id != insulator_id {
                    return None;
                }
                let family = self.gene_families.get(&locus.family_id)?;
                let full = normalize_match_text(&self.gene_fqn(family, locus, allele.generation));
                let without_generation =
                    normalize_match_text(&self.gene_fqn_without_generation(family, locus));
                (normalized == full || normalized == without_generation).then_some(allele.id)
            })
            .collect::<Vec<_>>();

        match matches.as_slice() {
            [allele_id] => Ok(*allele_id),
            [] => Err(DnapError::GeneFqnNotFound),
            _ => Err(DnapError::AmbiguousGeneFqn),
        }
    }

    fn latest_sequence_cursors(&self, allele_id: AlleleId) -> Vec<TranscriptSequenceCursor> {
        let mut latest = BTreeMap::<SequenceDefinitionId, MutationId>::new();
        for mutation in self
            .mutations
            .values()
            .filter(|mutation| mutation.allele_id == allele_id)
        {
            latest.insert(mutation.sequence_definition_id, mutation.id);
        }
        latest
            .into_iter()
            .map(
                |(sequence_definition_id, mutation_id)| TranscriptSequenceCursor {
                    sequence_definition_id,
                    last_rendered_mutation_id: Some(mutation_id),
                },
            )
            .collect()
    }

    fn gene_fqn(&self, family: &GeneFamily, locus: &Locus, generation: u32) -> String {
        format!(
            "{}-{}-{:04}",
            family.abbreviation,
            slugify(&locus.name),
            generation
        )
    }

    fn gene_fqn_without_generation(&self, family: &GeneFamily, locus: &Locus) -> String {
        format!("{}-{}", family.abbreviation, slugify(&locus.name))
    }

    fn allocate_insulator_id(&mut self) -> InsulatorId {
        self.next_insulator_id += 1;
        InsulatorId(self.next_insulator_id)
    }

    fn allocate_genome_id(&mut self) -> GenomeId {
        self.next_genome_id += 1;
        GenomeId(self.next_genome_id)
    }

    fn allocate_tf_id(&mut self) -> TfId {
        self.next_tf_id += 1;
        TfId(self.next_tf_id)
    }

    fn allocate_gene_family_id(&mut self) -> GeneFamilyId {
        self.next_gene_family_id += 1;
        GeneFamilyId(self.next_gene_family_id)
    }

    fn allocate_gene_family_generation_id(&mut self) -> GeneFamilyGenerationId {
        self.next_gene_family_generation_id += 1;
        GeneFamilyGenerationId(self.next_gene_family_generation_id)
    }

    fn allocate_sequence_definition_id(&mut self) -> SequenceDefinitionId {
        self.next_sequence_definition_id += 1;
        SequenceDefinitionId(self.next_sequence_definition_id)
    }

    fn allocate_locus_id(&mut self) -> LocusId {
        self.next_locus_id += 1;
        LocusId(self.next_locus_id)
    }

    fn allocate_transposon_id(&mut self) -> TransposonId {
        self.next_transposon_id += 1;
        TransposonId(self.next_transposon_id)
    }

    fn allocate_allele_id(&mut self) -> AlleleId {
        self.next_allele_id += 1;
        AlleleId(self.next_allele_id)
    }

    fn allocate_mutation_id(&mut self) -> MutationId {
        self.next_mutation_id += 1;
        MutationId(self.next_mutation_id)
    }

    fn allocate_chromosome_id(&mut self) -> ChromosomeId {
        self.next_chromosome_id += 1;
        ChromosomeId(self.next_chromosome_id)
    }

    fn allocate_transcriptome_id(&mut self) -> TranscriptomeId {
        self.next_transcriptome_id += 1;
        TranscriptomeId(self.next_transcriptome_id)
    }

    fn allocate_exon_id(&mut self) -> ExonId {
        self.next_exon_id += 1;
        ExonId(self.next_exon_id)
    }
}

fn require_text(value: String, error: DnapError) -> Result<String, DnapError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_owned())
    }
}

fn normalize_match_text(value: &str) -> String {
    value
        .trim()
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect()
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;
    for character in value.trim().chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }
    if slug.ends_with('-') {
        slug.pop();
    }
    slug
}

fn resolve_sequence_definition<'a>(
    generation: &'a GeneFamilyGeneration,
    name: &str,
) -> Option<&'a SequenceDefinition> {
    let normalized = normalize_match_text(name);
    generation
        .sequences
        .iter()
        .find(|definition| normalize_match_text(&definition.name) == normalized)
}

fn value_matches_sequence_type(value: &SequenceValue, sequence_type: SequenceType) -> bool {
    matches!(
        (value, sequence_type),
        (SequenceValue::String(_), SequenceType::String)
            | (SequenceValue::StringVec(_), SequenceType::StringVec)
            | (SequenceValue::Int(_), SequenceType::Int)
            | (SequenceValue::IntVec(_), SequenceType::IntVec)
            | (SequenceValue::Float(_), SequenceType::Float)
            | (SequenceValue::FloatVec(_), SequenceType::FloatVec)
            | (SequenceValue::Bool(_), SequenceType::Bool)
            | (SequenceValue::BoolVec(_), SequenceType::BoolVec)
            | (SequenceValue::GeneRef(_), SequenceType::Gene)
            | (SequenceValue::GeneRefVec(_), SequenceType::GeneVec)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_blank_human_entered_labels() {
        let mut dnap = Dnap::default();

        assert_eq!(
            dnap.provision_insulator(ProvisionInsulator {
                name: " ".to_owned(),
                placement_region: "us-east-1".to_owned(),
                placement_strategy: None,
            }),
            Err(DnapError::BlankInsulatorName)
        );

        let provisioned = provision_acme(&mut dnap);

        assert_eq!(
            dnap.create_genome(CreateGenome {
                insulator_id: provisioned.insulator.id,
                name: "\t".to_owned(),
            }),
            Err(DnapError::BlankGenomeName)
        );

        assert_eq!(
            dnap.create_tf(CreateTf {
                insulator_id: provisioned.insulator.id,
                display_name: "\n".to_owned(),
                external_subject: None,
                identity_provider: None,
            }),
            Err(DnapError::BlankTfDisplayName)
        );
    }

    #[test]
    fn requires_explicit_non_blank_placement_region() {
        let mut dnap = Dnap::default();

        assert_eq!(
            dnap.provision_insulator(ProvisionInsulator {
                name: "Acme".to_owned(),
                placement_region: " ".to_owned(),
                placement_strategy: None,
            }),
            Err(DnapError::BlankPlacementRegion)
        );
    }

    #[test]
    fn defaults_omitted_placement_strategy_to_shared_cluster() {
        let mut dnap = Dnap::default();

        let provisioned = dnap
            .provision_insulator(ProvisionInsulator {
                name: "Acme".to_owned(),
                placement_region: "us-east-1".to_owned(),
                placement_strategy: None,
            })
            .expect("valid provisioning");

        assert_eq!(
            provisioned.placement.strategy,
            InsulatorPlacementStrategy::SharedCluster
        );
    }

    #[test]
    fn rejects_missing_insulator_ownership_for_genome_and_tf() {
        let mut dnap = Dnap::default();
        let missing = InsulatorId(404);

        assert_eq!(
            dnap.create_genome(CreateGenome {
                insulator_id: missing,
                name: "Billing Platform".to_owned(),
            }),
            Err(DnapError::InsulatorNotFound)
        );

        assert_eq!(
            dnap.create_tf(CreateTf {
                insulator_id: missing,
                display_name: "Cesar".to_owned(),
                external_subject: None,
                identity_provider: None,
            }),
            Err(DnapError::InsulatorNotFound)
        );
    }

    #[test]
    fn rejects_blank_gene_family_inputs() {
        let mut dnap = Dnap::default();
        let provisioned = provision_acme(&mut dnap);
        let tf = create_cesar(&mut dnap, provisioned.insulator.id);

        assert_eq!(
            dnap.define_gene_family(DefineGeneFamily {
                insulator_id: provisioned.insulator.id,
                genome_id: None,
                name: " ".to_owned(),
                abbreviation: "PRD".to_owned(),
                encodes: Some(prd_encoding()),
                sequences: vec![sequence("title")],
                created_by: tf.id,
            }),
            Err(DnapError::BlankGeneFamilyName)
        );

        assert_eq!(
            dnap.define_gene_family(DefineGeneFamily {
                insulator_id: provisioned.insulator.id,
                genome_id: None,
                name: "Product Requirements Document".to_owned(),
                abbreviation: "\n".to_owned(),
                encodes: Some(prd_encoding()),
                sequences: vec![sequence("title")],
                created_by: tf.id,
            }),
            Err(DnapError::BlankGeneFamilyAbbreviation)
        );

        assert_eq!(
            dnap.define_gene_family(DefineGeneFamily {
                insulator_id: provisioned.insulator.id,
                genome_id: None,
                name: "Product Requirements Document".to_owned(),
                abbreviation: "PRD".to_owned(),
                encodes: Some(prd_encoding()),
                sequences: vec![sequence(" ")],
                created_by: tf.id,
            }),
            Err(DnapError::BlankSequenceDefinitionName)
        );
    }

    #[test]
    fn rejects_duplicate_sequence_names_inside_one_generation() {
        let mut dnap = Dnap::default();
        let provisioned = provision_acme(&mut dnap);
        let tf = create_cesar(&mut dnap, provisioned.insulator.id);

        assert_eq!(
            dnap.define_gene_family(DefineGeneFamily {
                insulator_id: provisioned.insulator.id,
                genome_id: None,
                name: "Product Requirements Document".to_owned(),
                abbreviation: "PRD".to_owned(),
                encodes: Some(prd_encoding()),
                sequences: vec![sequence("Title"), sequence("title")],
                created_by: tf.id,
            }),
            Err(DnapError::DuplicateSequenceDefinitionName)
        );
    }

    #[test]
    fn requires_encoding_type_for_gene_family() {
        let mut dnap = Dnap::default();
        let provisioned = provision_acme(&mut dnap);
        let tf = create_cesar(&mut dnap, provisioned.insulator.id);

        assert_eq!(
            dnap.define_gene_family(DefineGeneFamily {
                insulator_id: provisioned.insulator.id,
                genome_id: None,
                name: "Product Requirements Document".to_owned(),
                abbreviation: "PRD".to_owned(),
                encodes: None,
                sequences: vec![sequence("title")],
                created_by: tf.id,
            }),
            Err(DnapError::MissingEncodingType)
        );
    }

    #[test]
    fn allows_genome_scoped_gene_family_to_shadow_insulator_abbreviation() {
        let mut dnap = Dnap::default();
        let provisioned = provision_acme(&mut dnap);
        let genome = create_billing_genome(&mut dnap, provisioned.insulator.id);
        let tf = create_cesar(&mut dnap, provisioned.insulator.id);

        let tenant_prd = define_gene_family(
            &mut dnap,
            provisioned.insulator.id,
            None,
            tf.id,
            "Product Requirements Document",
            "PRD",
        );
        let project_prd = define_gene_family(
            &mut dnap,
            provisioned.insulator.id,
            Some(genome.id),
            tf.id,
            "Billing PRD",
            "PRD",
        );

        assert_ne!(tenant_prd.family.id, project_prd.family.id);
    }

    #[test]
    fn rejects_duplicate_abbreviations_in_the_same_effective_scope() {
        let mut dnap = Dnap::default();
        let provisioned = provision_acme(&mut dnap);
        let genome = create_billing_genome(&mut dnap, provisioned.insulator.id);
        let tf = create_cesar(&mut dnap, provisioned.insulator.id);

        define_gene_family(
            &mut dnap,
            provisioned.insulator.id,
            None,
            tf.id,
            "Product Requirements Document",
            "PRD",
        );

        assert_eq!(
            dnap.define_gene_family(DefineGeneFamily {
                insulator_id: provisioned.insulator.id,
                genome_id: None,
                name: "Another Product Requirements Document".to_owned(),
                abbreviation: "prd".to_owned(),
                encodes: Some(prd_encoding()),
                sequences: vec![sequence("title")],
                created_by: tf.id,
            }),
            Err(DnapError::DuplicateGeneFamilyAbbreviation)
        );

        define_gene_family(
            &mut dnap,
            provisioned.insulator.id,
            Some(genome.id),
            tf.id,
            "Billing Product Requirements Document",
            "PRD",
        );

        assert_eq!(
            dnap.define_gene_family(DefineGeneFamily {
                insulator_id: provisioned.insulator.id,
                genome_id: Some(genome.id),
                name: "Duplicate Billing PRD".to_owned(),
                abbreviation: "prd".to_owned(),
                encodes: Some(prd_encoding()),
                sequences: vec![sequence("title")],
                created_by: tf.id,
            }),
            Err(DnapError::DuplicateGeneFamilyAbbreviation)
        );
    }

    #[test]
    fn resolves_genome_override_before_insulator_default() {
        let mut dnap = Dnap::default();
        let provisioned = provision_acme(&mut dnap);
        let genome = create_billing_genome(&mut dnap, provisioned.insulator.id);
        let tf = create_cesar(&mut dnap, provisioned.insulator.id);

        let tenant_prd = define_gene_family(
            &mut dnap,
            provisioned.insulator.id,
            None,
            tf.id,
            "Product Requirements Document",
            "PRD",
        );
        let project_prd = define_gene_family(
            &mut dnap,
            provisioned.insulator.id,
            Some(genome.id),
            tf.id,
            "Billing Product Requirements Document",
            "PRD",
        );

        assert_eq!(
            dnap.resolve_gene_family(provisioned.insulator.id, Some(genome.id), "prd")
                .map(|family| family.id),
            Some(project_prd.family.id)
        );
        assert_eq!(
            dnap.resolve_gene_family(provisioned.insulator.id, None, "prd")
                .map(|family| family.id),
            Some(tenant_prd.family.id)
        );
    }

    #[test]
    fn mutate_new_requires_initial_mutation_and_creates_candidate_work() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Feature Requirements Specification",
            "FRS",
        );

        assert_eq!(
            dnap.mutate_new(MutateNew {
                insulator_id,
                genome_id,
                gene_family_abbreviation: "FRS".to_owned(),
                locus_name: "Checkout".to_owned(),
                mutations: Vec::new(),
                created_by: tf_id,
            }),
            Err(DnapError::MissingMutation)
        );

        let mutated = dnap
            .mutate_new(MutateNew {
                insulator_id,
                genome_id,
                gene_family_abbreviation: "frs".to_owned(),
                locus_name: "Checkout".to_owned(),
                mutations: vec![mutation(
                    "Some Section",
                    SequenceValue::String("Draft".to_owned()),
                )],
                created_by: tf_id,
            })
            .expect("new candidate work");

        assert_eq!(mutated.locus.name, "Checkout");
        assert_eq!(mutated.allele.state, AlleleState::Mutating);
        assert_eq!(mutated.gene_fqn, "FRS-checkout-0001");
        assert!(mutated.transposon.is_some());
        assert_eq!(
            dnap.project_allele(mutated.allele.id)
                .expect("candidate projection"),
            vec![Sequence {
                definition_id: mutated.mutations[0].sequence_definition_id,
                name: "Some Section".to_owned(),
                value: SequenceValue::String("Draft".to_owned()),
            }]
        );
    }

    #[test]
    fn mutation_sequence_matching_is_kebab_case_insensitive_and_type_checked() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Feature Requirements Specification",
            "FRS",
        );

        let mutated = dnap
            .mutate_new(MutateNew {
                insulator_id,
                genome_id,
                gene_family_abbreviation: "FRS".to_owned(),
                locus_name: "Checkout".to_owned(),
                mutations: vec![mutation(
                    "some-section",
                    SequenceValue::String("Draft".to_owned()),
                )],
                created_by: tf_id,
            })
            .expect("kebab matched sequence");

        assert_eq!(
            dnap.mutate_existing(MutateExisting {
                insulator_id,
                genome_id,
                gene_fqn: mutated.gene_fqn,
                mutations: vec![mutation("some-section", SequenceValue::Bool(true))],
                created_by: tf_id,
            }),
            Err(DnapError::SequenceValueTypeMismatch)
        );
    }

    #[test]
    fn fqn_generation_may_be_omitted_when_it_resolves_one_active_allele() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Feature Requirements Specification",
            "FRS",
        );
        dnap.mutate_new(MutateNew {
            insulator_id,
            genome_id,
            gene_family_abbreviation: "FRS".to_owned(),
            locus_name: "Checkout".to_owned(),
            mutations: vec![mutation(
                "Some Section",
                SequenceValue::String("Draft".to_owned()),
            )],
            created_by: tf_id,
        })
        .expect("new candidate work");

        let mutated = dnap
            .mutate_existing(MutateExisting {
                insulator_id,
                genome_id,
                gene_fqn: "frs checkout".to_owned(),
                mutations: vec![mutation(
                    "Problem",
                    SequenceValue::String("Pain".to_owned()),
                )],
                created_by: tf_id,
            })
            .expect("omitted generation resolves");

        assert_eq!(mutated.gene_fqn, "FRS-checkout-0001");
        assert_eq!(mutated.allele.state, AlleleState::Mutating);
    }

    #[test]
    fn active_allele_fqn_resolution_is_scoped_to_the_creating_tf() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, owner_tf_id) = workspace(&mut dnap);
        let other_tf = dnap
            .create_tf(CreateTf {
                insulator_id,
                display_name: "Reviewer".to_owned(),
                external_subject: None,
                identity_provider: None,
            })
            .expect("valid tf");
        define_gene_family(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            owner_tf_id,
            "Feature Requirements Specification",
            "FRS",
        );
        dnap.mutate_new(MutateNew {
            insulator_id,
            genome_id,
            gene_family_abbreviation: "FRS".to_owned(),
            locus_name: "Checkout".to_owned(),
            mutations: vec![mutation(
                "Some Section",
                SequenceValue::String("Draft".to_owned()),
            )],
            created_by: owner_tf_id,
        })
        .expect("owner candidate work");

        assert_eq!(
            dnap.mutate_existing(MutateExisting {
                insulator_id,
                genome_id,
                gene_fqn: "FRS-checkout".to_owned(),
                mutations: vec![mutation(
                    "Problem",
                    SequenceValue::String("Cross-user edit".to_owned())
                )],
                created_by: other_tf.id,
            }),
            Err(DnapError::GeneFqnNotFound)
        );
    }

    #[test]
    fn stale_spliced_work_must_be_transcribed_before_lgtm_splice_acknowledgement() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Feature Requirements Specification",
            "FRS",
        );
        let mutated = dnap
            .mutate_new(MutateNew {
                insulator_id,
                genome_id,
                gene_family_abbreviation: "FRS".to_owned(),
                locus_name: "Checkout".to_owned(),
                mutations: vec![mutation(
                    "Some Section",
                    SequenceValue::String("Draft".to_owned()),
                )],
                created_by: tf_id,
            })
            .expect("new candidate work");

        dnap.splice(SpliceAllele {
            insulator_id,
            genome_id,
            gene_fqn: mutated.gene_fqn.clone(),
            exon_texts: vec!["Build checkout".to_owned()],
            lgtm: false,
            created_by: tf_id,
        })
        .expect("first splice");

        let stale = dnap
            .mutate_existing(MutateExisting {
                insulator_id,
                genome_id,
                gene_fqn: "FRS-checkout".to_owned(),
                mutations: vec![mutation(
                    "Some Section",
                    SequenceValue::String("Updated".to_owned()),
                )],
                created_by: tf_id,
            })
            .expect("mutate spliced work");
        assert_eq!(stale.allele.state, AlleleState::StaleSplice);

        assert_eq!(
            dnap.splice(SpliceAllele {
                insulator_id,
                genome_id,
                gene_fqn: "FRS-checkout".to_owned(),
                exon_texts: Vec::new(),
                lgtm: true,
                created_by: tf_id,
            }),
            Err(DnapError::StaleSpliceRequiresTranscribe)
        );

        let transcribed = dnap
            .transcribe(TranscribeAllele {
                insulator_id,
                genome_id,
                gene_fqn: "frs checkout".to_owned(),
                full: false,
                created_by: tf_id,
            })
            .expect("stale transcription");
        assert_eq!(transcribed.allele.state, AlleleState::StaleTranscript);
        assert!(transcribed.approval_comments_visible);

        let spliced = dnap
            .splice(SpliceAllele {
                insulator_id,
                genome_id,
                gene_fqn: "FRS-checkout".to_owned(),
                exon_texts: Vec::new(),
                lgtm: true,
                created_by: tf_id,
            })
            .expect("lgtm acknowledgement");
        assert_eq!(spliced.allele.state, AlleleState::Spliced);
        assert!(spliced.exons.is_empty());
    }

    #[test]
    fn transcriptome_tracks_render_cursor_per_sequence_without_storing_projection() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Feature Requirements Specification",
            "FRS",
        );
        let mutated = dnap
            .mutate_new(MutateNew {
                insulator_id,
                genome_id,
                gene_family_abbreviation: "FRS".to_owned(),
                locus_name: "Checkout".to_owned(),
                mutations: vec![
                    mutation("Some Section", SequenceValue::String("Draft".to_owned())),
                    mutation("Problem", SequenceValue::String("Pain".to_owned())),
                ],
                created_by: tf_id,
            })
            .expect("new candidate work");

        let first = dnap
            .transcribe(TranscribeAllele {
                insulator_id,
                genome_id,
                gene_fqn: mutated.gene_fqn.clone(),
                full: false,
                created_by: tf_id,
            })
            .expect("first transcript");
        assert_eq!(first.sequences.len(), 2);
        assert_eq!(first.transcriptome.sequences.len(), 2);

        let second = dnap
            .transcribe(TranscribeAllele {
                insulator_id,
                genome_id,
                gene_fqn: mutated.gene_fqn.clone(),
                full: false,
                created_by: tf_id,
            })
            .expect("unchanged transcript");
        assert!(second.sequences.is_empty());

        let changed = dnap
            .mutate_existing(MutateExisting {
                insulator_id,
                genome_id,
                gene_fqn: mutated.gene_fqn,
                mutations: vec![mutation(
                    "Problem",
                    SequenceValue::String("Sharper pain".to_owned()),
                )],
                created_by: tf_id,
            })
            .expect("change one sequence");
        let third = dnap
            .transcribe(TranscribeAllele {
                insulator_id,
                genome_id,
                gene_fqn: changed.gene_fqn,
                full: false,
                created_by: tf_id,
            })
            .expect("changed transcript");

        assert_eq!(third.sequences.len(), 1);
        assert_eq!(third.sequences[0].name, "Problem");
        assert_eq!(third.transcriptome.sequences.len(), 2);
    }

    fn provision_acme(dnap: &mut Dnap) -> ProvisionedInsulator {
        dnap.provision_insulator(ProvisionInsulator {
            name: "Acme".to_owned(),
            placement_region: "us-east-1".to_owned(),
            placement_strategy: None,
        })
        .expect("valid provisioning")
    }

    fn create_billing_genome(dnap: &mut Dnap, insulator_id: InsulatorId) -> Genome {
        dnap.create_genome(CreateGenome {
            insulator_id,
            name: "Billing Platform".to_owned(),
        })
        .expect("valid genome")
    }

    fn create_cesar(dnap: &mut Dnap, insulator_id: InsulatorId) -> Tf {
        dnap.create_tf(CreateTf {
            insulator_id,
            display_name: "Cesar".to_owned(),
            external_subject: None,
            identity_provider: None,
        })
        .expect("valid tf")
    }

    fn workspace(dnap: &mut Dnap) -> (InsulatorId, GenomeId, TfId) {
        let provisioned = provision_acme(dnap);
        let genome = create_billing_genome(dnap, provisioned.insulator.id);
        let tf = create_cesar(dnap, provisioned.insulator.id);
        (provisioned.insulator.id, genome.id, tf.id)
    }

    fn define_gene_family(
        dnap: &mut Dnap,
        insulator_id: InsulatorId,
        genome_id: Option<GenomeId>,
        created_by: TfId,
        name: &str,
        abbreviation: &str,
    ) -> DefinedGeneFamily {
        dnap.define_gene_family(DefineGeneFamily {
            insulator_id,
            genome_id,
            name: name.to_owned(),
            abbreviation: abbreviation.to_owned(),
            encodes: Some(prd_encoding()),
            sequences: vec![sequence("Some Section"), sequence("Problem")],
            created_by,
        })
        .expect("valid gene family")
    }

    fn sequence(name: &str) -> DefineSequence {
        DefineSequence {
            name: name.to_owned(),
            sequence_type: SequenceType::String,
        }
    }

    fn mutation(sequence_name: &str, value: SequenceValue) -> SequenceMutation {
        SequenceMutation {
            sequence_name: sequence_name.to_owned(),
            value,
        }
    }

    fn prd_encoding() -> EncodingType {
        EncodingType::RNA(RnaType::Translation(TranslationRnaType::MRNA))
    }
}
