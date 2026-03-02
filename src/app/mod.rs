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
    next_exploration_graph_id: u64,
    next_exploration_node_id: u64,
    next_exploration_edge_id: u64,
    next_intron_id: u64,
    next_intron_sequence_id: u64,
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
    exploration_graphs: BTreeMap<ExplorationGraphId, ExplorationGraph>,
    exploration_nodes: BTreeMap<ExplorationNodeId, ExplorationNode>,
    exploration_edges: BTreeMap<ExplorationEdgeId, ExplorationEdge>,
    enhancer_contexts: BTreeMap<LocusId, EnhancerContext>,
    introns: BTreeMap<IntronId, Intron>,
    intron_sequences: BTreeMap<IntronSequenceId, IntronSequence>,
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

    pub(super) fn find_locus_by_encoding(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        encoding: EncodingKind,
        name: &str,
    ) -> Option<&Locus> {
        let normalized = normalize_match_text(name);
        self.loci.values().find(|locus| {
            locus.insulator_id == insulator_id
                && locus.genome_id == genome_id
                && normalize_match_text(&locus.name) == normalized
                && self.locus_has_encoding(locus.id, encoding)
        })
    }

    pub(super) fn require_locus_encoding(
        &self,
        locus_id: LocusId,
        encoding: EncodingKind,
    ) -> Result<(), DnapError> {
        if self.locus_has_encoding(locus_id, encoding) {
            Ok(())
        } else {
            match encoding {
                EncodingKind::Promoter => Err(DnapError::ExplorationGraphPromoterRequired),
                EncodingKind::ERna => Err(DnapError::ExplorationNodeErnaRequired),
                EncodingKind::Enhancer => Err(DnapError::EnhancerContextEnhancerRequired),
                EncodingKind::MRna => Err(DnapError::IntronTargetRequired),
            }
        }
    }

    fn locus_has_encoding(&self, locus_id: LocusId, encoding: EncodingKind) -> bool {
        let Some(locus) = self.loci.get(&locus_id) else {
            return false;
        };
        let Some(family) = self.gene_families.get(&locus.family_id) else {
            return false;
        };
        match (family.encodes, encoding) {
            (EncodingType::GRN(GrnType::Promoter), EncodingKind::Promoter) => true,
            (EncodingType::GRN(GrnType::Enhancer), EncodingKind::Enhancer) => true,
            (
                EncodingType::RNA(RnaType::Translation(TranslationRnaType::ERna)),
                EncodingKind::ERna,
            ) => true,
            (
                EncodingType::RNA(RnaType::Translation(TranslationRnaType::MRna)),
                EncodingKind::MRna,
            ) => true,
            _ => false,
        }
    }

    pub(super) fn resolve_intron_target(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        created_by: TfId,
        target_mrna_fqn: &str,
        target_sequence_name: Option<&str>,
    ) -> Result<(LocusId, Option<SequenceDefinitionId>), DnapError> {
        let allele_id =
            self.resolve_active_allele_id(insulator_id, genome_id, created_by, target_mrna_fqn)?;
        let allele = self
            .alleles
            .get(&allele_id)
            .ok_or(DnapError::AlleleNotFound)?;
        if !self.locus_has_encoding(allele.locus_id, EncodingKind::MRna) {
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
            .filter(|sequence| sequence.intron_id == intron_id)
            .cloned()
            .last()
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
        allele_id: AlleleId,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        locus_id: LocusId,
        generation_id: GeneFamilyGenerationId,
        mutations: Vec<SequenceMutation>,
        causes: Vec<String>,
        created_by: TfId,
        created_at: SystemTime,
    ) -> Result<Vec<Mutation>, DnapError> {
        let generation = self
            .gene_family_generations
            .get(&generation_id)
            .ok_or(DnapError::GeneFamilyNotFound)?
            .clone();
        let mut open_by_sequence = self
            .mutations
            .values()
            .filter(|mutation| {
                mutation.allele_id == allele_id && mutation.state == MutationState::Unexpressed
            })
            .map(|mutation| (mutation.sequence_definition_id, mutation.clone()))
            .collect::<BTreeMap<_, _>>();
        let resolved_causes = causes
            .iter()
            .map(|cause| {
                let intron =
                    self.resolve_intron(insulator_id, genome_id, created_by, cause, None)?;
                if self.latest_intron_sequence(intron.id).is_none() {
                    return Err(DnapError::IntronCauseRequiresAnswer);
                }
                Ok(intron.id)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut touched = Vec::<SequenceDefinitionId>::new();
        let mut requested = Vec::<(SequenceDefinitionId, SequenceValue)>::new();
        for mutation in mutations {
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
            let context = self.mutation_context(locus_id, definition_id, &resolved_causes);
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
                existing.updated_at = created_at;
            } else {
                open_by_sequence.insert(
                    definition_id,
                    Mutation {
                        id: self.allocate_mutation_id(),
                        allele_id,
                        sequence_definition_id: definition_id,
                        value,
                        context,
                        state: MutationState::Unexpressed,
                        created_by,
                        created_at,
                        updated_at: created_at,
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

    pub(super) fn allocate_exploration_graph_id(&mut self) -> ExplorationGraphId {
        self.next_exploration_graph_id += 1;
        ExplorationGraphId(self.next_exploration_graph_id)
    }

    pub(super) fn allocate_exploration_node_id(&mut self) -> ExplorationNodeId {
        self.next_exploration_node_id += 1;
        ExplorationNodeId(self.next_exploration_node_id)
    }

    pub(super) fn allocate_exploration_edge_id(&mut self) -> ExplorationEdgeId {
        self.next_exploration_edge_id += 1;
        ExplorationEdgeId(self.next_exploration_edge_id)
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
pub(super) enum EncodingKind {
    Promoter,
    Enhancer,
    ERna,
    MRna,
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
    fn mutate_new_can_create_locus_transposon_and_allele_without_initial_mutations() {
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

        let empty = dnap
            .mutate_new(MutateNew {
                insulator_id,
                genome_id,
                gene_family_abbreviation: "FRS".to_owned(),
                locus_name: "Checkout".to_owned(),
                mutations: Vec::new(),
                causes: Vec::new(),
                created_by: tf_id,
            })
            .expect("new allele without initial mutations");

        assert_eq!(empty.locus.name, "Checkout");
        assert_eq!(empty.allele.state, AlleleState::Mutating);
        assert_eq!(empty.gene_fqn, "FRS-checkout-0001");
        assert!(empty.mutations.is_empty());
        assert!(empty.transposon.is_some());
        assert!(dnap
            .project_allele(empty.allele.id)
            .expect("empty projection")
            .is_empty());
    }

    #[test]
    fn mutate_new_can_create_initial_sequence_mutations() {
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
                gene_family_abbreviation: "frs".to_owned(),
                locus_name: "Checkout".to_owned(),
                mutations: vec![mutation(
                    "Some Section",
                    SequenceValue::String("Draft".to_owned()),
                )],
                causes: Vec::new(),
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
                causes: Vec::new(),
                created_by: tf_id,
            })
            .expect("kebab matched sequence");

        assert_eq!(
            dnap.mutate_existing(MutateExisting {
                insulator_id,
                genome_id,
                gene_fqn: mutated.gene_fqn,
                mutations: vec![mutation("some-section", SequenceValue::Bool(true))],
                causes: Vec::new(),
                created_by: tf_id,
            }),
            Err(DnapError::SequenceValueTypeMismatch)
        );
    }

    #[test]
    fn active_allele_can_be_resolved_by_locus_name_without_fuzzy_matching() {
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
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("new candidate work");

        let mutated = dnap
            .mutate_existing(MutateExisting {
                insulator_id,
                genome_id,
                gene_fqn: "checkout".to_owned(),
                mutations: vec![mutation("Prob", SequenceValue::String("Pain".to_owned()))],
                causes: Vec::new(),
                created_by: tf_id,
            })
            .expect("locus name resolves");

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
            causes: Vec::new(),
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
                causes: Vec::new(),
                created_by: other_tf.id,
            }),
            Err(DnapError::GeneFqnNotFound)
        );
    }

    #[test]
    fn lgtm_expresses_unexpressed_mutations_without_requiring_transcribe() {
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
                causes: Vec::new(),
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

        let unexpressed = dnap
            .mutate_existing(MutateExisting {
                insulator_id,
                genome_id,
                gene_fqn: "FRS-checkout".to_owned(),
                mutations: vec![mutation(
                    "Some Section",
                    SequenceValue::String("Updated".to_owned()),
                )],
                causes: Vec::new(),
                created_by: tf_id,
            })
            .expect("mutate spliced work");
        assert_eq!(unexpressed.allele.state, AlleleState::Mutating);

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
        assert_eq!(spliced.allele.state, AlleleState::Expressing);
        assert!(spliced.exons.is_empty());
        assert_eq!(spliced.untranscribed_unexpressed_mutations, 1);
    }

    #[test]
    fn translate_returns_exons_without_changing_allele_state() {
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
                causes: Vec::new(),
                created_by: tf_id,
            })
            .expect("new candidate work");
        dnap.splice(SpliceAllele {
            insulator_id,
            genome_id,
            gene_fqn: mutated.gene_fqn.clone(),
            exon_texts: vec![
                "Implement checkout API".to_owned(),
                "Add retry tests".to_owned(),
            ],
            lgtm: false,
            created_by: tf_id,
        })
        .expect("splice exons");

        let translated = dnap
            .translate(TranslateAllele {
                insulator_id,
                genome_id,
                gene_fqn: "checkout".to_owned(),
                created_by: tf_id,
            })
            .expect("translate exons");

        assert_eq!(translated.allele.state, AlleleState::Expressing);
        assert_eq!(
            translated
                .exons
                .iter()
                .map(|exon| exon.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Implement checkout API", "Add retry tests"]
        );
        assert_eq!(
            dnap.allele(translated.allele.id).expect("allele").state,
            AlleleState::Expressing
        );
    }

    #[test]
    fn translate_errors_when_the_active_allele_has_no_exons() {
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
            mutations: Vec::new(),
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("new candidate work");

        assert_eq!(
            dnap.translate(TranslateAllele {
                insulator_id,
                genome_id,
                gene_fqn: "checkout".to_owned(),
                created_by: tf_id,
            }),
            Err(DnapError::ExonsNotFound)
        );
    }

    #[test]
    fn creates_promoter_owned_exploration_graph_with_auto_created_erna_node() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family_with_encoding(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Story",
            "STR",
            EncodingType::GRN(GrnType::Promoter),
        );
        define_gene_family_with_encoding(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Exploration",
            "EXP",
            EncodingType::RNA(RnaType::Translation(TranslationRnaType::ERna)),
        );
        dnap.mutate_new(MutateNew {
            insulator_id,
            genome_id,
            gene_family_abbreviation: "STR".to_owned(),
            locus_name: "Checkout flow".to_owned(),
            mutations: Vec::new(),
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("promoter");

        let graph = dnap
            .create_exploration_graph(CreateExplorationGraph {
                insulator_id,
                genome_id,
                promoter_gene_fqn: "checkout-flow".to_owned(),
                name: "Event storm".to_owned(),
                created_by: tf_id,
            })
            .expect("graph");
        let added = dnap
            .add_exploration_node(AddExplorationNode {
                insulator_id,
                genome_id,
                graph_id: graph.graph.id,
                erna_locus_name: "Payment authorized".to_owned(),
                erna_family_abbreviation: Some("EXP".to_owned()),
                label: None,
                position_x: 120,
                position_y: 80,
                created_by: tf_id,
            })
            .expect("node");

        assert_eq!(graph.promoter_locus.name, "Checkout flow");
        assert_eq!(added.erna_locus.name, "Payment authorized");
        assert!(added.created_erna.is_some());
        assert_eq!(added.node.label, "Payment authorized");
        assert_eq!(added.node.position_x, 120);
        assert_eq!(dnap.exploration_nodes(graph.graph.id).len(), 1);
    }

    #[test]
    fn exploration_edges_connect_nodes_inside_one_graph_and_allow_cycles() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family_with_encoding(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Story",
            "STR",
            EncodingType::GRN(GrnType::Promoter),
        );
        define_gene_family_with_encoding(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Exploration",
            "EXP",
            EncodingType::RNA(RnaType::Translation(TranslationRnaType::ERna)),
        );
        dnap.mutate_new(MutateNew {
            insulator_id,
            genome_id,
            gene_family_abbreviation: "STR".to_owned(),
            locus_name: "Checkout flow".to_owned(),
            mutations: Vec::new(),
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("promoter");
        let graph = dnap
            .create_exploration_graph(CreateExplorationGraph {
                insulator_id,
                genome_id,
                promoter_gene_fqn: "checkout-flow".to_owned(),
                name: "Process map".to_owned(),
                created_by: tf_id,
            })
            .expect("graph");
        let first = add_erna_node(
            &mut dnap,
            insulator_id,
            genome_id,
            tf_id,
            graph.graph.id,
            "A",
        );
        let second = add_erna_node(
            &mut dnap,
            insulator_id,
            genome_id,
            tf_id,
            graph.graph.id,
            "B",
        );

        dnap.add_exploration_edge(AddExplorationEdge {
            insulator_id,
            genome_id,
            graph_id: graph.graph.id,
            from_node_id: first.node.id,
            to_node_id: second.node.id,
            label: Some("leads to".to_owned()),
            created_by: tf_id,
        })
        .expect("edge");
        dnap.add_exploration_edge(AddExplorationEdge {
            insulator_id,
            genome_id,
            graph_id: graph.graph.id,
            from_node_id: second.node.id,
            to_node_id: first.node.id,
            label: Some("loops".to_owned()),
            created_by: tf_id,
        })
        .expect("cycle edge");

        assert_eq!(dnap.exploration_edges(graph.graph.id).len(), 2);
    }

    #[test]
    fn enhancer_context_stores_promoter_property_on_enhancer_locus() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family_with_encoding(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Story",
            "STR",
            EncodingType::GRN(GrnType::Promoter),
        );
        define_gene_family_with_encoding(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Research",
            "RSH",
            EncodingType::GRN(GrnType::Enhancer),
        );
        let promoter = dnap
            .mutate_new(MutateNew {
                insulator_id,
                genome_id,
                gene_family_abbreviation: "STR".to_owned(),
                locus_name: "Checkout flow".to_owned(),
                mutations: Vec::new(),
                causes: Vec::new(),
                created_by: tf_id,
            })
            .expect("promoter");
        let enhancer = dnap
            .mutate_new(MutateNew {
                insulator_id,
                genome_id,
                gene_family_abbreviation: "RSH".to_owned(),
                locus_name: "Payment provider research".to_owned(),
                mutations: Vec::new(),
                causes: Vec::new(),
                created_by: tf_id,
            })
            .expect("enhancer");

        let context = dnap
            .attach_enhancer_promoter(AttachEnhancerPromoter {
                insulator_id,
                genome_id,
                enhancer_gene_fqn: "payment-provider-research".to_owned(),
                promoter_gene_fqn: "checkout-flow".to_owned(),
                updated_by: tf_id,
            })
            .expect("context");

        assert_eq!(context.enhancer_locus_id, enhancer.locus.id);
        assert_eq!(context.promoter_locus_id, promoter.locus.id);
        assert_eq!(
            dnap.enhancer_context(enhancer.locus.id)
                .expect("enhancer context")
                .promoter_locus_id,
            promoter.locus.id
        );
    }

    #[test]
    fn introns_target_mrna_and_can_chain_follow_ups() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family_with_encoding(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Requirement",
            "REQ",
            EncodingType::RNA(RnaType::Translation(TranslationRnaType::MRna)),
        );
        let target = dnap
            .mutate_new(MutateNew {
                insulator_id,
                genome_id,
                gene_family_abbreviation: "REQ".to_owned(),
                locus_name: "Checkout requirements".to_owned(),
                mutations: Vec::new(),
                causes: Vec::new(),
                created_by: tf_id,
            })
            .expect("target");

        let intron = dnap
            .create_intron(CreateIntron {
                insulator_id,
                genome_id,
                target_mrna_fqn: "checkout-requirements".to_owned(),
                target_sequence_name: None,
                title: "Clarify payment retries".to_owned(),
                body: None,
                precursor: None,
                created_by: tf_id,
            })
            .expect("intron");
        let answered = dnap
            .append_intron_sequence(AppendIntronSequence {
                insulator_id,
                genome_id,
                target_mrna_fqn: None,
                target_sequence_name: None,
                intron_title: "clarify-payment-retries".to_owned(),
                body: Some("Retry twice".to_owned()),
                follow_up_title: Some("Clarify retry ceiling".to_owned()),
                follow_up_body: None,
                created_by: tf_id,
            })
            .expect("follow up");

        assert_eq!(intron.target_mrna_locus_id, target.locus.id);
        assert_eq!(answered.intron.id, intron.id);
        assert_eq!(
            answered.sequence.expect("answer").body,
            "Retry twice".to_owned()
        );
        assert_eq!(
            answered.follow_up.expect("follow up").precursor,
            Some(intron.id)
        );
    }

    #[test]
    fn introns_reject_rrna_targets() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family_with_encoding(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Design",
            "DSN",
            EncodingType::RNA(RnaType::Translation(TranslationRnaType::RRna)),
        );
        dnap.mutate_new(MutateNew {
            insulator_id,
            genome_id,
            gene_family_abbreviation: "DSN".to_owned(),
            locus_name: "Checkout design".to_owned(),
            mutations: Vec::new(),
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("design");

        assert_eq!(
            dnap.create_intron(CreateIntron {
                insulator_id,
                genome_id,
                target_mrna_fqn: "checkout-design".to_owned(),
                target_sequence_name: None,
                title: "Clarify component boundary".to_owned(),
                body: None,
                precursor: None,
                created_by: tf_id,
            }),
            Err(DnapError::IntronTargetRequired)
        );
    }

    #[test]
    pub(super) fn mutation_context_captures_relevant_introns_and_explicit_causes() {
        let mut dnap = Dnap::default();
        let (insulator_id, genome_id, tf_id) = workspace(&mut dnap);
        define_gene_family_with_encoding(
            &mut dnap,
            insulator_id,
            Some(genome_id),
            tf_id,
            "Requirement",
            "REQ",
            EncodingType::RNA(RnaType::Translation(TranslationRnaType::MRna)),
        );
        dnap.mutate_new(MutateNew {
            insulator_id,
            genome_id,
            gene_family_abbreviation: "REQ".to_owned(),
            locus_name: "Checkout requirements".to_owned(),
            mutations: vec![mutation(
                "Some Section",
                SequenceValue::String("Draft".to_owned()),
            )],
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("target");
        let cause = dnap
            .create_intron(CreateIntron {
                insulator_id,
                genome_id,
                target_mrna_fqn: "checkout-requirements".to_owned(),
                target_sequence_name: Some("Some Section".to_owned()),
                title: "How strict is latency".to_owned(),
                body: None,
                precursor: None,
                created_by: tf_id,
            })
            .expect("cause intron");
        let cause_answer = dnap
            .append_intron_sequence(AppendIntronSequence {
                insulator_id,
                genome_id,
                target_mrna_fqn: None,
                target_sequence_name: None,
                intron_title: "how-strict".to_owned(),
                body: Some("Paid checkout under 100ms".to_owned()),
                follow_up_title: None,
                follow_up_body: None,
                created_by: tf_id,
            })
            .expect("cause answer")
            .sequence
            .expect("sequence");
        let unanswered = dnap
            .create_intron(CreateIntron {
                insulator_id,
                genome_id,
                target_mrna_fqn: "checkout-requirements".to_owned(),
                target_sequence_name: Some("Some Section".to_owned()),
                title: "Which users are included".to_owned(),
                body: None,
                precursor: None,
                created_by: tf_id,
            })
            .expect("unanswered intron");

        let mutated = dnap
            .mutate_existing(MutateExisting {
                insulator_id,
                genome_id,
                gene_fqn: "checkout-requirements".to_owned(),
                mutations: vec![mutation(
                    "Some Section",
                    SequenceValue::String("Paid checkout latency < 100ms".to_owned()),
                )],
                causes: vec!["how-strict".to_owned()],
                created_by: tf_id,
            })
            .expect("mutate with cause");

        assert!(mutated.mutations[0]
            .context
            .contains(&MutationContext::Cause(cause.id, cause_answer.id)));
        assert!(mutated.mutations[0]
            .context
            .contains(&MutationContext::UnansweredContext(unanswered.id)));
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
                causes: Vec::new(),
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
        let mutation_count = dnap
            .mutations
            .values()
            .filter(|mutation| mutation.allele_id == mutated.allele.id)
            .count();

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
                causes: Vec::new(),
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
        assert_eq!(
            dnap.mutations
                .values()
                .filter(|mutation| mutation.allele_id == changed.allele.id)
                .count(),
            mutation_count
        );
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

    fn define_gene_family_with_encoding(
        dnap: &mut Dnap,
        insulator_id: InsulatorId,
        genome_id: Option<GenomeId>,
        created_by: TfId,
        name: &str,
        abbreviation: &str,
        encodes: EncodingType,
    ) -> DefinedGeneFamily {
        dnap.define_gene_family(DefineGeneFamily {
            insulator_id,
            genome_id,
            name: name.to_owned(),
            abbreviation: abbreviation.to_owned(),
            encodes: Some(encodes),
            sequences: vec![sequence("Some Section")],
            created_by,
        })
        .expect("valid gene family")
    }

    fn add_erna_node(
        dnap: &mut Dnap,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        tf_id: TfId,
        graph_id: ExplorationGraphId,
        name: &str,
    ) -> AddedExplorationNode {
        dnap.add_exploration_node(AddExplorationNode {
            insulator_id,
            genome_id,
            graph_id,
            erna_locus_name: name.to_owned(),
            erna_family_abbreviation: Some("EXP".to_owned()),
            label: None,
            position_x: 0,
            position_y: 0,
            created_by: tf_id,
        })
        .expect("erna node")
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
        EncodingType::RNA(RnaType::Translation(TranslationRnaType::MRna))
    }
}
