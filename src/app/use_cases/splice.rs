use super::super::*;

impl Dnap {
    pub fn splice(&mut self, input: SpliceAllele) -> Result<SpliceResult, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_grn_in_genome(input.grn_id, input.genome_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

        let allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.grn_id,
            &input.gene_fqn,
        )?;
        let mut allele = self
            .alleles
            .get(&allele_id)
            .cloned()
            .ok_or(DnapError::AlleleNotFound)?;
        let unexpressed_mutation_ids = self
            .mutations
            .values()
            .filter(|mutation| {
                mutation.allele_id == allele.id && mutation.state == MutationState::Unexpressed
            })
            .map(|mutation| mutation.id)
            .collect::<Vec<_>>();
        if input.lgtm && unexpressed_mutation_ids.is_empty() {
            return Err(DnapError::LgtmRequiresUnexpressedMutation);
        }

        let now = SystemTime::now();
        let untranscribed_unexpressed_mutations =
            self.untranscribed_unexpressed_mutation_count(allele.id);
        let mut task_realizations = Vec::new();
        for task_realization_text in input.task_realization_texts {
            let text = require_text(task_realization_text, DnapError::BlankTaskRealizationText)?;
            let task_realization = TaskRealization {
                id: self.allocate_task_realization_id(),
                allele_id: allele.id,
                text,
                depends_on: Vec::new(),
                created_at: now,
            };
            self.task_realizations
                .insert(task_realization.id, task_realization.clone());
            self.record_signal(
                input.insulator_id,
                Some(input.created_by),
                SignalType::TaskRealizationCreated,
                SignalTarget::TaskRealization(task_realization.id),
                None,
                SignalPayload::Empty,
            );
            task_realizations.push(task_realization);
        }
        for mutation_id in &unexpressed_mutation_ids {
            let Some(mutation) = self.mutations.get_mut(mutation_id) else {
                continue;
            };
            mutation.state = MutationState::Expressing;
            mutation.updated_at = now;
        }

        allele.state = AlleleState::Expressing;
        allele.updated_at = now;
        self.alleles.insert(allele.id, allele.clone());
        if !unexpressed_mutation_ids.is_empty() {
            self.record_signal(
                input.insulator_id,
                Some(input.created_by),
                SignalType::MutationsExpressed,
                SignalTarget::Allele(allele.id),
                None,
                SignalPayload::Empty,
            );
        }

        Ok(SpliceResult {
            allele,
            task_realizations,
            untranscribed_unexpressed_mutations,
        })
    }

    pub fn translate(&self, input: TranslateAllele) -> Result<TranslatedAllele, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_grn_in_genome(input.grn_id, input.genome_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

        let allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.grn_id,
            &input.gene_fqn,
        )?;
        let allele = self
            .alleles
            .get(&allele_id)
            .cloned()
            .ok_or(DnapError::AlleleNotFound)?;
        let task_realizations = self.ordered_task_realizations(allele.id);
        if task_realizations.is_empty() {
            return Err(DnapError::TaskRealizationsNotFound);
        }

        Ok(TranslatedAllele {
            allele,
            task_realizations,
        })
    }
}
