use std::collections::BTreeMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

mod commands;
mod encoding;
mod error;
mod genes;
mod identity;
mod ids;
mod workflow;
mod use_cases {
    pub(super) mod exploration;
    pub(super) mod introns;
    pub(super) mod mutate;
    pub(super) mod platform;
    pub(super) mod queries;
    pub(super) mod splice;
    pub(super) mod transcribe;
}

pub use commands::*;
pub use encoding::*;
pub use error::*;
pub use genes::*;
pub use identity::*;
pub use ids::*;
pub use workflow::*;

#[derive(Serialize, Deserialize, Default)]
pub struct Dnap {
    next_insulator_id: u64,
    next_genome_id: u64,
    next_grn_id: u64,
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
    next_intron_id: u64,
    next_intron_sequence_id: u64,
    insulators: BTreeMap<InsulatorId, Insulator>,
    placements: BTreeMap<InsulatorId, InsulatorPlacement>,
    genomes: BTreeMap<GenomeId, Genome>,
    grns: BTreeMap<GrnId, Grn>,
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
    enhancer_contexts: BTreeMap<LocusId, EnhancerContext>,
    introns: BTreeMap<IntronId, Intron>,
    intron_sequences: BTreeMap<IntronSequenceId, IntronSequence>,
}

pub(super) struct BuildMutations {
    pub allele_id: AlleleId,
    pub insulator_id: InsulatorId,
    pub genome_id: GenomeId,
    pub grn_id: GrnId,
    pub locus_id: LocusId,
    pub generation_id: GeneFamilyGenerationId,
    pub mutations: Vec<SequenceMutation>,
    pub causes: Vec<String>,
    pub created_by: TfId,
    pub created_at: SystemTime,
}

impl Dnap {
    pub fn gene_family_generation(
        &self,
        id: GeneFamilyGenerationId,
    ) -> Option<&GeneFamilyGeneration> {
        self.gene_family_generations.get(&id)
    }

    pub(super) fn require_insulator(&self, id: InsulatorId) -> Result<(), DnapError> {
        self.insulators
            .contains_key(&id)
            .then_some(())
            .ok_or(DnapError::InsulatorNotFound)
    }

    pub(super) fn require_genome_in_insulator(
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

    pub(super) fn require_grn_in_genome(
        &self,
        id: GrnId,
        genome_id: GenomeId,
    ) -> Result<(), DnapError> {
        let grn = self.grns.get(&id).ok_or(DnapError::GrnNotFound)?;
        if grn.genome_id == genome_id {
            Ok(())
        } else {
            Err(DnapError::GrnGenomeMismatch)
        }
    }

    pub(super) fn require_tf_in_insulator(
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

    pub(super) fn require_available_abbreviation(
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

    pub(super) fn require_no_active_allele(
        &self,
        locus_id: LocusId,
        grn_id: GrnId,
    ) -> Result<(), DnapError> {
        let duplicate = self.alleles.values().any(|allele| {
            allele.locus_id == locus_id
                && allele.grn_id == grn_id
                && !matches!(allele.state, AlleleState::Selected | AlleleState::Degraded)
        });
        if duplicate {
            Err(DnapError::DuplicateActiveAllele)
        } else {
            Ok(())
        }
    }

    pub(super) fn find_locus(
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

    fn locus_has_artifact(&self, locus_id: LocusId, normalized_artifact: ArtifactKind) -> bool {
        let Some(locus) = self.loci.get(&locus_id) else {
            return false;
        };
        let Some(family) = self.gene_families.get(&locus.family_id) else {
            return false;
        };
        matches!(
            (family.normalized_artifact, normalized_artifact),
            (NormalizedArtifact::Promoter, ArtifactKind::Promoter)
                | (
                    NormalizedArtifact::EnterpriseNegotiationHandoverCertificate,
                    ArtifactKind::Enhancer
                )
                | (
                    NormalizedArtifact::ManagedRequirement,
                    ArtifactKind::ManagedRequirement,
                )
        )
    }

    pub(super) fn resolve_intron_target(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        grn_id: GrnId,
        target_mrna_fqn: &str,
        target_sequence_name: Option<&str>,
    ) -> Result<(LocusId, Option<SequenceDefinitionId>), DnapError> {
        let allele_id =
            self.resolve_active_allele_id(insulator_id, genome_id, grn_id, target_mrna_fqn)?;
        let allele = self
            .alleles
            .get(&allele_id)
            .ok_or(DnapError::AlleleNotFound)?;
        if !self.locus_has_artifact(allele.locus_id, ArtifactKind::ManagedRequirement) {
            return Err(DnapError::IntronTargetRequired);
        }
        let sequence_definition_id = target_sequence_name
            .map(|sequence_name| {
                let generation = self
                    .gene_family_generations
                    .get(&allele.gene_family_generation_id)
                    .ok_or(DnapError::GeneFamilyNotFound)?;
                resolve_sequence_definition(generation, sequence_name)
                    .map(|definition| definition.id)
            })
            .transpose()?;
        Ok((allele.locus_id, sequence_definition_id))
    }

    pub(super) fn resolve_intron(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        _grn_id: GrnId,
        created_by: TfId,
        title: &str,
        target: Option<(LocusId, Option<SequenceDefinitionId>)>,
    ) -> Result<Intron, DnapError> {
        let normalized = normalize_match_text(title);
        let mut matches = self
            .introns
            .values()
            .filter(|intron| {
                self.loci
                    .get(&intron.target_mrna_locus_id)
                    .is_some_and(|locus| {
                        locus.insulator_id == insulator_id && locus.genome_id == genome_id
                    })
            })
            .filter(|intron| {
                target
                    .map(|(target_mrna_locus_id, target_sequence_definition_id)| {
                        intron.target_mrna_locus_id == target_mrna_locus_id
                            && intron.target_sequence_definition_id == target_sequence_definition_id
                    })
                    .unwrap_or(true)
            })
            .filter(|intron| intron.normalized_title == normalized)
            .cloned()
            .collect::<Vec<_>>();
        if matches.is_empty() {
            matches = self
                .introns
                .values()
                .filter(|intron| {
                    self.loci
                        .get(&intron.target_mrna_locus_id)
                        .is_some_and(|locus| {
                            locus.insulator_id == insulator_id && locus.genome_id == genome_id
                        })
                })
                .filter(|intron| {
                    target
                        .map(|(target_mrna_locus_id, target_sequence_definition_id)| {
                            intron.target_mrna_locus_id == target_mrna_locus_id
                                && intron.target_sequence_definition_id
                                    == target_sequence_definition_id
                        })
                        .unwrap_or(true)
                })
                .filter(|intron| intron.normalized_title.starts_with(&normalized))
                .cloned()
                .collect();
        }

        match matches.as_slice() {
            [intron] => Ok(intron.clone()),
            [] => {
                let _ = created_by;
                Err(DnapError::IntronNotFound)
            }
            _ => Err(DnapError::AmbiguousIntron),
        }
    }

    pub(super) fn intron_summary(&self, intron: &Intron) -> IntronSummary {
        let latest_sequence = self.intron_sequences_for(intron.id).into_iter().last();
        let child_count = self
            .introns
            .values()
            .filter(|candidate| candidate.precursor == Some(intron.id))
            .count();
        IntronSummary {
            intron: intron.clone(),
            latest_sequence,
            has_precursor: intron.precursor.is_some(),
            child_count,
        }
    }

    pub(super) fn intron_sequences_for(&self, intron_id: IntronId) -> Vec<IntronSequence> {
        self.intron_sequences
            .values()
            .filter(|sequence| sequence.intron_id == intron_id)
            .cloned()
            .collect()
    }

    pub(super) fn sequence_definition_name(
        &self,
        target_mrna_locus_id: LocusId,
        sequence_definition_id: SequenceDefinitionId,
    ) -> Option<String> {
        let locus = self.loci.get(&target_mrna_locus_id)?;
        let family = self.gene_families.get(&locus.family_id)?;
        let generation = self
            .gene_family_generations
            .get(&family.current_generation_id)?;
        generation
            .sequences
            .iter()
            .find(|definition| definition.id == sequence_definition_id)
            .map(|definition| definition.name.clone())
    }

    pub(super) fn latest_intron_sequence(&self, intron_id: IntronId) -> Option<IntronSequence> {
        self.intron_sequences
            .values()
            .rfind(|sequence| sequence.intron_id == intron_id)
            .cloned()
    }

    pub(super) fn mutation_context(
        &self,
        target_mrna_locus_id: LocusId,
        sequence_definition_id: SequenceDefinitionId,
        causes: &[IntronId],
    ) -> Vec<MutationContext> {
        self.introns
            .values()
            .filter(|intron| intron.target_mrna_locus_id == target_mrna_locus_id)
            .filter(|intron| {
                intron.target_sequence_definition_id.is_none()
                    || intron.target_sequence_definition_id == Some(sequence_definition_id)
            })
            .filter_map(|intron| {
                let latest = self.latest_intron_sequence(intron.id);
                if causes.contains(&intron.id) {
                    latest.map(|sequence| MutationContext::Cause(intron.id, sequence.id))
                } else {
                    Some(match latest {
                        Some(sequence) => MutationContext::AnsweredContext(intron.id, sequence.id),
                        None => MutationContext::UnansweredContext(intron.id),
                    })
                }
            })
            .collect()
    }

    pub(super) fn build_mutations(
        &mut self,
        input: BuildMutations,
    ) -> Result<Vec<Mutation>, DnapError> {
        let generation = self
            .gene_family_generations
            .get(&input.generation_id)
            .ok_or(DnapError::GeneFamilyNotFound)?
            .clone();
        let mut open_by_sequence = self
            .mutations
            .values()
            .filter(|mutation| {
                mutation.allele_id == input.allele_id
                    && mutation.state == MutationState::Unexpressed
            })
            .map(|mutation| (mutation.sequence_definition_id, mutation.clone()))
            .collect::<BTreeMap<_, _>>();
        let resolved_causes = input
            .causes
            .iter()
            .map(|cause| {
                let intron = self.resolve_intron(
                    input.insulator_id,
                    input.genome_id,
                    input.grn_id,
                    input.created_by,
                    cause,
                    None,
                )?;
                if self.latest_intron_sequence(intron.id).is_none() {
                    return Err(DnapError::IntronCauseRequiresAnswer);
                }
                Ok(intron.id)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut touched = Vec::<SequenceDefinitionId>::new();
        let mut requested = Vec::<(SequenceDefinitionId, SequenceValue)>::new();
        for mutation in input.mutations {
            let sequence_name =
                require_text(mutation.sequence_name, DnapError::BlankMutationSequenceName)?;
            let definition = resolve_sequence_definition(&generation, &sequence_name)?;
            if !value_matches_sequence_type(&mutation.value, definition.sequence_type) {
                return Err(DnapError::SequenceValueTypeMismatch);
            }
            requested.push((definition.id, mutation.value));
            if !touched.contains(&definition.id) {
                touched.push(definition.id);
            }
        }

        let mut used_causes = Vec::<IntronId>::new();
        for (definition_id, value) in requested {
            let context = self.mutation_context(input.locus_id, definition_id, &resolved_causes);
            for context_item in &context {
                if let MutationContext::Cause(intron_id, _) = context_item {
                    if !used_causes.contains(intron_id) {
                        used_causes.push(*intron_id);
                    }
                }
            }
            if let Some(existing) = open_by_sequence.get_mut(&definition_id) {
                existing.value = value;
                existing.context = context;
                existing.updated_at = input.created_at;
            } else {
                open_by_sequence.insert(
                    definition_id,
                    Mutation {
                        id: self.allocate_mutation_id(),
                        allele_id: input.allele_id,
                        sequence_definition_id: definition_id,
                        value,
                        context,
                        state: MutationState::Unexpressed,
                        created_at: input.created_at,
                        updated_at: input.created_at,
                    },
                );
            }
        }
        if resolved_causes
            .iter()
            .any(|cause| !used_causes.contains(cause))
        {
            return Err(DnapError::IntronNotFound);
        }
        Ok(touched
            .into_iter()
            .filter_map(|definition_id| open_by_sequence.remove(&definition_id))
            .collect())
    }

    pub(super) fn resolve_active_allele_id(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        grn_id: GrnId,
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
                    && allele.grn_id == grn_id
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
                let locus_name = normalize_match_text(&locus.name);
                (normalized == full || normalized == without_generation || normalized == locus_name)
                    .then_some(allele.id)
            })
            .collect::<Vec<_>>();

        match matches.as_slice() {
            [allele_id] => Ok(*allele_id),
            [] => Err(DnapError::GeneFqnNotFound),
            _ => Err(DnapError::AmbiguousGeneFqn),
        }
    }

    pub(super) fn latest_sequence_cursors(
        &self,
        allele_id: AlleleId,
    ) -> Vec<TranscriptSequenceCursor> {
        let mut latest = BTreeMap::<SequenceDefinitionId, (MutationId, SequenceHash)>::new();
        for mutation in self
            .mutations
            .values()
            .filter(|mutation| mutation.allele_id == allele_id)
        {
            latest.insert(
                mutation.sequence_definition_id,
                (mutation.id, sequence_hash(&mutation.value)),
            );
        }
        latest
            .into_iter()
            .map(|(sequence_definition_id, (mutation_id, sequence_hash))| {
                TranscriptSequenceCursor {
                    sequence_definition_id,
                    last_rendered_mutation_id: Some(mutation_id),
                    last_rendered_sequence_hash: Some(sequence_hash),
                }
            })
            .collect()
    }

    pub(super) fn untranscribed_unexpressed_mutation_count(&self, allele_id: AlleleId) -> usize {
        let previous_cursors = self
            .transcriptomes
            .get(&allele_id)
            .map(|transcriptome| transcriptome.sequences.as_slice())
            .unwrap_or(&[]);

        self.mutations
            .values()
            .filter(|mutation| {
                mutation.allele_id == allele_id
                    && mutation.state == MutationState::Unexpressed
                    && previous_cursors
                        .iter()
                        .find(|cursor| {
                            cursor.sequence_definition_id == mutation.sequence_definition_id
                        })
                        .map(|cursor| {
                            cursor.last_rendered_mutation_id == Some(mutation.id)
                                && cursor.last_rendered_sequence_hash
                                    == Some(sequence_hash(&mutation.value))
                        })
                        != Some(true)
            })
            .count()
    }

    pub(super) fn ordered_exons(&self, allele_id: AlleleId) -> Vec<Exon> {
        let mut remaining = self
            .exons
            .values()
            .filter(|exon| exon.allele_id == allele_id)
            .cloned()
            .collect::<Vec<_>>();
        let mut ordered = Vec::<Exon>::new();

        while !remaining.is_empty() {
            let before = remaining.len();
            let mut index = 0;
            while index < remaining.len() {
                let ready = remaining[index]
                    .depends_on
                    .iter()
                    .all(|dependency| ordered.iter().any(|exon| exon.id == *dependency));
                if ready {
                    ordered.push(remaining.remove(index));
                } else {
                    index += 1;
                }
            }
            if remaining.len() == before {
                ordered.extend(remaining);
                break;
            }
        }

        ordered
    }

    pub(super) fn gene_fqn(&self, family: &GeneFamily, locus: &Locus, generation: u32) -> String {
        format!(
            "{}-{}-{:04}",
            family.abbreviation,
            slugify(&locus.name),
            generation
        )
    }

    pub(super) fn gene_fqn_without_generation(&self, family: &GeneFamily, locus: &Locus) -> String {
        format!("{}-{}", family.abbreviation, slugify(&locus.name))
    }

    pub(super) fn allocate_insulator_id(&mut self) -> InsulatorId {
        self.next_insulator_id += 1;
        InsulatorId(self.next_insulator_id)
    }

    pub(super) fn allocate_genome_id(&mut self) -> GenomeId {
        self.next_genome_id += 1;
        GenomeId(self.next_genome_id)
    }

    pub(super) fn allocate_tf_id(&mut self) -> TfId {
        self.next_tf_id += 1;
        TfId(self.next_tf_id)
    }

    pub(super) fn allocate_grn_id(&mut self) -> GrnId {
        self.next_grn_id += 1;
        GrnId(self.next_grn_id)
    }

    pub(super) fn allocate_gene_family_id(&mut self) -> GeneFamilyId {
        self.next_gene_family_id += 1;
        GeneFamilyId(self.next_gene_family_id)
    }

    pub(super) fn allocate_gene_family_generation_id(&mut self) -> GeneFamilyGenerationId {
        self.next_gene_family_generation_id += 1;
        GeneFamilyGenerationId(self.next_gene_family_generation_id)
    }

    pub(super) fn allocate_sequence_definition_id(&mut self) -> SequenceDefinitionId {
        self.next_sequence_definition_id += 1;
        SequenceDefinitionId(self.next_sequence_definition_id)
    }

    pub(super) fn allocate_locus_id(&mut self) -> LocusId {
        self.next_locus_id += 1;
        LocusId(self.next_locus_id)
    }

    pub(super) fn allocate_transposon_id(&mut self) -> TransposonId {
        self.next_transposon_id += 1;
        TransposonId(self.next_transposon_id)
    }

    pub(super) fn allocate_allele_id(&mut self) -> AlleleId {
        self.next_allele_id += 1;
        AlleleId(self.next_allele_id)
    }

    pub(super) fn allocate_mutation_id(&mut self) -> MutationId {
        self.next_mutation_id += 1;
        MutationId(self.next_mutation_id)
    }

    pub(super) fn allocate_chromosome_id(&mut self) -> ChromosomeId {
        self.next_chromosome_id += 1;
        ChromosomeId(self.next_chromosome_id)
    }

    pub(super) fn allocate_transcriptome_id(&mut self) -> TranscriptomeId {
        self.next_transcriptome_id += 1;
        TranscriptomeId(self.next_transcriptome_id)
    }

    pub(super) fn allocate_exon_id(&mut self) -> ExonId {
        self.next_exon_id += 1;
        ExonId(self.next_exon_id)
    }

    pub(super) fn allocate_intron_id(&mut self) -> IntronId {
        self.next_intron_id += 1;
        IntronId(self.next_intron_id)
    }

    pub(super) fn allocate_intron_sequence_id(&mut self) -> IntronSequenceId {
        self.next_intron_sequence_id += 1;
        IntronSequenceId(self.next_intron_sequence_id)
    }
}

#[derive(Clone, Copy)]
pub(super) enum ArtifactKind {
    Promoter,
    Enhancer,
    ManagedRequirement,
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
) -> Result<&'a SequenceDefinition, DnapError> {
    let normalized = normalize_match_text(name);
    if let Some(exact) = generation
        .sequences
        .iter()
        .find(|definition| normalize_match_text(&definition.name) == normalized)
    {
        return Ok(exact);
    }

    let matches = generation
        .sequences
        .iter()
        .filter(|definition| normalize_match_text(&definition.name).starts_with(&normalized))
        .collect::<Vec<_>>();

    match matches.as_slice() {
        [definition] => Ok(*definition),
        [] => Err(DnapError::SequenceDefinitionNotFound),
        _ => Err(DnapError::AmbiguousSequenceDefinition),
    }
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

fn sequence_hash(value: &SequenceValue) -> SequenceHash {
    let serialized = match value {
        SequenceValue::String(value) => format!("string:{value}"),
        SequenceValue::StringVec(value) => format!("string_vec:{value:?}"),
        SequenceValue::Int(value) => format!("int:{value}"),
        SequenceValue::IntVec(value) => format!("int_vec:{value:?}"),
        SequenceValue::Float(value) => format!("float:{value:?}"),
        SequenceValue::FloatVec(value) => format!("float_vec:{value:?}"),
        SequenceValue::Bool(value) => format!("bool:{value}"),
        SequenceValue::BoolVec(value) => format!("bool_vec:{value:?}"),
        SequenceValue::GeneRef(value) => format!("gene:{value:?}"),
        SequenceValue::GeneRefVec(value) => format!("gene_vec:{value:?}"),
    };

    // FNV-1a is enough here: this is a deterministic render cursor, not a security hash.
    let mut hash = 0xcbf29ce484222325u64;
    for byte in serialized.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    SequenceHash(format!("{hash:016x}"))
}

fn intron_title_scope_hash(
    target_mrna_locus_id: LocusId,
    target_sequence_definition_id: Option<SequenceDefinitionId>,
    precursor: Option<IntronId>,
    normalized_title: &str,
) -> String {
    let serialized = format!(
        "{target_mrna_locus_id:?}:{target_sequence_definition_id:?}:{precursor:?}:{normalized_title}"
    );
    let mut hash = 0xcbf29ce484222325u64;
    for byte in serialized.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests;
