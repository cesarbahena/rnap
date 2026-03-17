use super::super::*;

impl Dnap {
    pub fn create_intron(&mut self, input: CreateIntron) -> Result<Intron, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_grn_in_genome(input.grn_id, input.genome_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

        let (target_mrna_locus_id, target_sequence_definition_id) = self.resolve_intron_target(
            input.insulator_id,
            input.genome_id,
            input.grn_id,
            &input.target_mrna_fqn,
            input.target_sequence_name.as_deref(),
        )?;
        if let Some(precursor) = input.precursor {
            let precursor = self
                .introns
                .get(&precursor)
                .ok_or(DnapError::IntronNotFound)?;
            if precursor.target_mrna_locus_id != target_mrna_locus_id
                || precursor.target_sequence_definition_id != target_sequence_definition_id
            {
                return Err(DnapError::IntronNotFound);
            }
        }
        let title = require_text(input.title, DnapError::BlankIntronTitle)?;
        let body = input
            .body
            .map(|body| require_text(body, DnapError::BlankIntronTitle))
            .transpose()?;
        let normalized_title = normalize_match_text(&title);
        if self.introns.values().any(|intron| {
            intron.target_mrna_locus_id == target_mrna_locus_id
                && intron.target_sequence_definition_id == target_sequence_definition_id
                && intron.precursor == input.precursor
                && intron.normalized_title == normalized_title
        }) {
            return Err(DnapError::DuplicateIntronTitle);
        }

        let title_scope_hash = intron_title_scope_hash(
            target_mrna_locus_id,
            target_sequence_definition_id,
            input.precursor,
            &normalized_title,
        );
        let intron = Intron {
            id: self.allocate_intron_id(),
            target_mrna_locus_id,
            target_sequence_definition_id,
            precursor: input.precursor,
            title,
            body,
            normalized_title,
            title_scope_hash,
            created_at: SystemTime::now(),
        };
        self.introns.insert(intron.id, intron.clone());

        Ok(intron)
    }

    pub fn append_intron_sequence(
        &mut self,
        input: AppendIntronSequence,
    ) -> Result<AppendedIntronSequence, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_grn_in_genome(input.grn_id, input.genome_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

        let target = match input.target_mrna_fqn.as_deref() {
            Some(target_mrna_fqn) => Some(self.resolve_intron_target(
                input.insulator_id,
                input.genome_id,
                input.grn_id,
                target_mrna_fqn,
                input.target_sequence_name.as_deref(),
            )?),
            None => None,
        };
        let intron = self.resolve_intron(
            input.insulator_id,
            input.genome_id,
            input.grn_id,
            input.created_by,
            input.intron_title.as_str(),
            target,
        )?;
        let sequence = input
            .body
            .map(|body| {
                let body = require_text(body, DnapError::BlankIntronAnswer)?;
                let sequence = IntronSequence {
                    id: self.allocate_intron_sequence_id(),
                    intron_id: intron.id,
                    body,
                    created_at: SystemTime::now(),
                };
                self.intron_sequences.insert(sequence.id, sequence.clone());
                Ok(sequence)
            })
            .transpose()?;
        let follow_up = match input.follow_up_title {
            Some(title) => Some(
                self.create_intron(CreateIntron {
                    insulator_id: input.insulator_id,
                    genome_id: input.genome_id,
                    grn_id: input.grn_id,
                    target_mrna_fqn: self
                        .loci
                        .get(&intron.target_mrna_locus_id)
                        .map(|locus| locus.name.clone())
                        .ok_or(DnapError::IntronNotFound)?,
                    target_sequence_name: intron.target_sequence_definition_id.and_then(|id| {
                        self.sequence_definition_name(intron.target_mrna_locus_id, id)
                    }),
                    title,
                    body: input.follow_up_body,
                    precursor: Some(intron.id),
                    created_by: input.created_by,
                })?,
            ),
            None => None,
        };

        Ok(AppendedIntronSequence {
            intron,
            sequence,
            follow_up,
        })
    }

    pub fn intron_summaries_for(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        grn_id: GrnId,
        created_by: TfId,
        target_mrna_fqn: &str,
        target_sequence_name: Option<&str>,
    ) -> Result<Vec<IntronSummary>, DnapError> {
        self.require_insulator(insulator_id)?;
        self.require_genome_in_insulator(genome_id, insulator_id)?;
        self.require_grn_in_genome(grn_id, genome_id)?;
        self.require_tf_in_insulator(created_by, insulator_id)?;
        let (target_mrna_locus_id, target_sequence_definition_id) = self.resolve_intron_target(
            insulator_id,
            genome_id,
            grn_id,
            target_mrna_fqn,
            target_sequence_name,
        )?;
        Ok(self
            .introns
            .values()
            .filter(|intron| {
                intron.target_mrna_locus_id == target_mrna_locus_id
                    && intron.target_sequence_definition_id == target_sequence_definition_id
            })
            .filter(|intron| intron.precursor.is_none())
            .map(|intron| self.intron_summary(intron))
            .collect())
    }

    pub fn intron_thread(
        &self,
        insulator_id: InsulatorId,
        genome_id: GenomeId,
        grn_id: GrnId,
        created_by: TfId,
        intron_title: &str,
        target: Option<(&str, Option<&str>)>,
    ) -> Result<IntronThread, DnapError> {
        self.require_insulator(insulator_id)?;
        self.require_genome_in_insulator(genome_id, insulator_id)?;
        self.require_grn_in_genome(grn_id, genome_id)?;
        self.require_tf_in_insulator(created_by, insulator_id)?;
        let target = target
            .map(|(target_mrna_fqn, target_sequence_name)| {
                self.resolve_intron_target(
                    insulator_id,
                    genome_id,
                    grn_id,
                    target_mrna_fqn,
                    target_sequence_name,
                )
            })
            .transpose()?;
        let intron = self.resolve_intron(
            insulator_id,
            genome_id,
            grn_id,
            created_by,
            intron_title,
            target,
        )?;
        let sequences = self.intron_sequences_for(intron.id);
        let mut precursors = Vec::new();
        if let Some(precursor_id) = intron.precursor {
            if let Some(precursor) = self.introns.get(&precursor_id) {
                precursors.push(self.intron_summary(precursor));
            }
        }
        let children = self
            .introns
            .values()
            .filter(|candidate| candidate.precursor == Some(intron.id))
            .map(|child| self.intron_summary(child))
            .collect();

        Ok(IntronThread {
            intron,
            sequences,
            precursors,
            children,
        })
    }

    pub fn intron_thread_by_id(&self, intron_id: IntronId) -> Result<IntronThread, DnapError> {
        let intron = self
            .introns
            .get(&intron_id)
            .cloned()
            .ok_or(DnapError::IntronNotFound)?;
        let sequences = self.intron_sequences_for(intron.id);
        let mut precursors = Vec::new();
        if let Some(precursor_id) = intron.precursor {
            if let Some(precursor) = self.introns.get(&precursor_id) {
                precursors.push(self.intron_summary(precursor));
            }
        }
        let children = self
            .introns
            .values()
            .filter(|candidate| candidate.precursor == Some(intron.id))
            .map(|child| self.intron_summary(child))
            .collect();

        Ok(IntronThread {
            intron,
            sequences,
            precursors,
            children,
        })
    }
}
