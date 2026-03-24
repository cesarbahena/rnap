use super::super::*;

impl Dnap {
    pub fn create_semantic_narrowing(
        &mut self,
        input: CreateSemanticNarrowing,
    ) -> Result<SemanticNarrowing, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_grn_in_genome(input.grn_id, input.genome_id)?;
        self.require_chromosome_in_genome(input.chromosome_id, input.genome_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

        let (target_mrna_locus_id, target_sequence_definition_id) = self
            .resolve_semantic_narrowing_target(
                input.insulator_id,
                input.genome_id,
                input.grn_id,
                &input.target_mrna_fqn,
                input.target_sequence_name.as_deref(),
            )?;
        if let Some(precursor) = input.precursor {
            let precursor = self
                .semantic_narrowings
                .get(&precursor)
                .ok_or(DnapError::SemanticNarrowingNotFound)?;
            if precursor.target_mrna_locus_id != target_mrna_locus_id
                || precursor.target_sequence_definition_id != target_sequence_definition_id
            {
                return Err(DnapError::SemanticNarrowingNotFound);
            }
        }
        let title = require_text(input.title, DnapError::BlankSemanticNarrowingTitle)?;
        let body = input
            .body
            .map(|body| require_text(body, DnapError::BlankSemanticNarrowingTitle))
            .transpose()?;
        let normalized_title = normalize_match_text(&title);
        if self.semantic_narrowings.values().any(|semantic_narrowing| {
            semantic_narrowing.target_mrna_locus_id == target_mrna_locus_id
                && semantic_narrowing.target_sequence_definition_id == target_sequence_definition_id
                && semantic_narrowing.precursor == input.precursor
                && semantic_narrowing.normalized_title == normalized_title
        }) {
            return Err(DnapError::DuplicateSemanticNarrowingTitle);
        }

        let title_scope_hash = semantic_narrowing_title_scope_hash(
            target_mrna_locus_id,
            target_sequence_definition_id,
            input.precursor,
            &normalized_title,
        );
        let semantic_narrowing = SemanticNarrowing {
            id: self.allocate_semantic_narrowing_id(),
            target_mrna_locus_id,
            target_sequence_definition_id,
            precursor: input.precursor,
            title,
            body,
            normalized_title,
            title_scope_hash,
            created_at: SystemTime::now(),
        };
        self.semantic_narrowings
            .insert(semantic_narrowing.id, semantic_narrowing.clone());
        self.record_signal(
            input.insulator_id,
            Some(input.created_by),
            SignalType::SemanticNarrowingCreated,
            SignalTarget::SemanticNarrowing(semantic_narrowing.id),
            None,
            SignalPayload::Empty,
        );

        Ok(semantic_narrowing)
    }

    pub fn append_semantic_narrowing_sequence(
        &mut self,
        input: AppendSemanticNarrowingSequence,
    ) -> Result<AppendedSemanticNarrowingSequence, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_grn_in_genome(input.grn_id, input.genome_id)?;
        self.require_chromosome_in_genome(input.chromosome_id, input.genome_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

        let target = match input.target_mrna_fqn.as_deref() {
            Some(target_mrna_fqn) => Some(self.resolve_semantic_narrowing_target(
                input.insulator_id,
                input.genome_id,
                input.grn_id,
                target_mrna_fqn,
                input.target_sequence_name.as_deref(),
            )?),
            None => None,
        };
        let semantic_narrowing = self.resolve_semantic_narrowing(
            input.insulator_id,
            input.genome_id,
            input.grn_id,
            input.created_by,
            input.semantic_narrowing_title.as_str(),
            target,
        )?;
        let sequence = input
            .body
            .map(|body| {
                let body = require_text(body, DnapError::BlankSemanticNarrowingAnswer)?;
                let sequence = SemanticNarrowingSequence {
                    id: self.allocate_semantic_narrowing_sequence_id(),
                    semantic_narrowing_id: semantic_narrowing.id,
                    body,
                    created_at: SystemTime::now(),
                };
                self.semantic_narrowing_sequences
                    .insert(sequence.id, sequence.clone());
                self.record_signal(
                    input.insulator_id,
                    Some(input.created_by),
                    SignalType::SemanticNarrowingAnswered,
                    SignalTarget::SemanticNarrowingSequence(sequence.id),
                    None,
                    SignalPayload::Empty,
                );
                Ok(sequence)
            })
            .transpose()?;
        let follow_up = match input.follow_up_title {
            Some(title) => Some(
                self.create_semantic_narrowing(CreateSemanticNarrowing {
                    insulator_id: input.insulator_id,
                    genome_id: input.genome_id,
                    chromosome_id: input.chromosome_id,
                    grn_id: input.grn_id,
                    target_mrna_fqn: self
                        .loci
                        .get(&semantic_narrowing.target_mrna_locus_id)
                        .map(|locus| locus.name.clone())
                        .ok_or(DnapError::SemanticNarrowingNotFound)?,
                    target_sequence_name: semantic_narrowing
                        .target_sequence_definition_id
                        .and_then(|id| {
                            self.sequence_definition_name(
                                semantic_narrowing.target_mrna_locus_id,
                                id,
                            )
                        }),
                    title,
                    body: input.follow_up_body,
                    precursor: Some(semantic_narrowing.id),
                    created_by: input.created_by,
                })?,
            ),
            None => None,
        };

        Ok(AppendedSemanticNarrowingSequence {
            semantic_narrowing,
            sequence,
            follow_up,
        })
    }

    pub fn semantic_narrowing_summaries_for(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        grn_id: GrnId,
        created_by: TfId,
        target_mrna_fqn: &str,
        target_sequence_name: Option<&str>,
    ) -> Result<Vec<SemanticNarrowingSummary>, DnapError> {
        self.require_insulator(insulator_id)?;
        self.require_genome_in_insulator(genome_id, insulator_id)?;
        self.require_grn_in_genome(grn_id, genome_id)?;
        self.require_tf_in_insulator(created_by, insulator_id)?;
        let (target_mrna_locus_id, target_sequence_definition_id) = self
            .resolve_semantic_narrowing_target(
                insulator_id,
                genome_id,
                grn_id,
                target_mrna_fqn,
                target_sequence_name,
            )?;
        Ok(self
            .semantic_narrowings
            .values()
            .filter(|semantic_narrowing| {
                semantic_narrowing.target_mrna_locus_id == target_mrna_locus_id
                    && semantic_narrowing.target_sequence_definition_id
                        == target_sequence_definition_id
            })
            .filter(|semantic_narrowing| semantic_narrowing.precursor.is_none())
            .map(|semantic_narrowing| self.semantic_narrowing_summary(semantic_narrowing))
            .collect())
    }

    pub fn semantic_narrowing_thread(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        grn_id: GrnId,
        created_by: TfId,
        semantic_narrowing_title: &str,
        target: Option<(&str, Option<&str>)>,
    ) -> Result<SemanticNarrowingThread, DnapError> {
        self.require_insulator(insulator_id)?;
        self.require_genome_in_insulator(genome_id, insulator_id)?;
        self.require_grn_in_genome(grn_id, genome_id)?;
        self.require_tf_in_insulator(created_by, insulator_id)?;
        let target = target
            .map(|(target_mrna_fqn, target_sequence_name)| {
                self.resolve_semantic_narrowing_target(
                    insulator_id,
                    genome_id,
                    grn_id,
                    target_mrna_fqn,
                    target_sequence_name,
                )
            })
            .transpose()?;
        let semantic_narrowing = self.resolve_semantic_narrowing(
            insulator_id,
            genome_id,
            grn_id,
            created_by,
            semantic_narrowing_title,
            target,
        )?;
        let sequences = self.semantic_narrowing_sequences_for(semantic_narrowing.id);
        let mut precursors = Vec::new();
        if let Some(precursor_id) = semantic_narrowing.precursor {
            if let Some(precursor) = self.semantic_narrowings.get(&precursor_id) {
                precursors.push(self.semantic_narrowing_summary(precursor));
            }
        }
        let children = self
            .semantic_narrowings
            .values()
            .filter(|candidate| candidate.precursor == Some(semantic_narrowing.id))
            .map(|child| self.semantic_narrowing_summary(child))
            .collect();

        Ok(SemanticNarrowingThread {
            semantic_narrowing,
            sequences,
            precursors,
            children,
        })
    }

    pub fn semantic_narrowing_thread_by_id(
        &self,
        semantic_narrowing_id: SemanticNarrowingId,
    ) -> Result<SemanticNarrowingThread, DnapError> {
        let semantic_narrowing = self
            .semantic_narrowings
            .get(&semantic_narrowing_id)
            .cloned()
            .ok_or(DnapError::SemanticNarrowingNotFound)?;
        let sequences = self.semantic_narrowing_sequences_for(semantic_narrowing.id);
        let mut precursors = Vec::new();
        if let Some(precursor_id) = semantic_narrowing.precursor {
            if let Some(precursor) = self.semantic_narrowings.get(&precursor_id) {
                precursors.push(self.semantic_narrowing_summary(precursor));
            }
        }
        let children = self
            .semantic_narrowings
            .values()
            .filter(|candidate| candidate.precursor == Some(semantic_narrowing.id))
            .map(|child| self.semantic_narrowing_summary(child))
            .collect();

        Ok(SemanticNarrowingThread {
            semantic_narrowing,
            sequences,
            precursors,
            children,
        })
    }
}
