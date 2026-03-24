use super::super::*;

impl Dnap {
    pub fn mutate_new(&mut self, input: MutateNew) -> Result<MutatedAllele, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_grn_in_genome(input.grn_id, input.genome_id)?;
        self.require_chromosome_in_genome(input.chromosome_id, input.genome_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

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
            self.require_no_active_allele(locus.id, input.grn_id)?;
        }

        let now = SystemTime::now();
        let locus = Locus {
            id: self.allocate_locus_id(),
            family_id: family.id,
            insulator_id: input.insulator_id,
            chromosome_id: input.chromosome_id,
            name: locus_name,
            created_at: now,
        };
        let transposon = Transposon {
            id: self.allocate_transposon_id(),
            locus_id: locus.id,
            gene_family_generation_id: generation.id,
            created_at: now,
        };
        let allele = Allele {
            id: self.allocate_allele_id(),
            chromosome_id: input.chromosome_id,
            grn_id: input.grn_id,
            locus_id: locus.id,
            gene_family_generation_id: generation.id,
            generation: 1,
            origin: AlleleOrigin::Transposon(transposon.id),
            state: AlleleState::Mutating,
            degraded_at: None,
            created_at: now,
            updated_at: now,
        };
        let mutations = self.build_mutations(BuildMutations {
            allele_id: allele.id,
            insulator_id: input.insulator_id,
            genome_id: input.genome_id,
            grn_id: input.grn_id,
            locus_id: locus.id,
            generation_id: generation.id,
            mutations: input.mutations,
            causes: input.causes,
            created_by: input.created_by,
            created_at: now,
        })?;
        let gene_fqn = self.gene_fqn(&family, &locus, allele.generation);

        self.loci.insert(locus.id, locus.clone());
        self.transposons.insert(transposon.id, transposon.clone());
        self.alleles.insert(allele.id, allele.clone());
        for mutation in &mutations {
            self.mutations.insert(mutation.id, mutation.clone());
        }
        self.record_signal(
            input.insulator_id,
            Some(input.created_by),
            SignalType::LocusCreated,
            SignalTarget::Locus(locus.id),
            None,
            SignalPayload::Empty,
        );
        self.record_signal(
            input.insulator_id,
            Some(input.created_by),
            SignalType::TransposonCreated,
            SignalTarget::Transposon(transposon.id),
            None,
            SignalPayload::Empty,
        );
        self.record_signal(
            input.insulator_id,
            Some(input.created_by),
            SignalType::AlleleCreated,
            SignalTarget::Allele(allele.id),
            None,
            SignalPayload::Empty,
        );
        for mutation in &mutations {
            self.record_signal(
                input.insulator_id,
                Some(input.created_by),
                SignalType::MutationChanged,
                SignalTarget::Mutation(mutation.id),
                None,
                SignalPayload::Empty,
            );
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
        self.require_grn_in_genome(input.grn_id, input.genome_id)?;
        self.require_chromosome_in_genome(input.chromosome_id, input.genome_id)?;
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
        if matches!(allele.state, AlleleState::Selected | AlleleState::Degraded) {
            return Err(DnapError::AlleleCannotMutate);
        }

        let now = SystemTime::now();
        let mutations = self.build_mutations(BuildMutations {
            allele_id: allele.id,
            insulator_id: input.insulator_id,
            genome_id: input.genome_id,
            grn_id: input.grn_id,
            locus_id: allele.locus_id,
            generation_id: allele.gene_family_generation_id,
            mutations: input.mutations,
            causes: input.causes,
            created_by: input.created_by,
            created_at: now,
        })?;
        if allele.state == AlleleState::Expressing && !mutations.is_empty() {
            allele.state = AlleleState::Mutating;
        }
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
        for mutation in &mutations {
            self.record_signal(
                input.insulator_id,
                Some(input.created_by),
                SignalType::MutationChanged,
                SignalTarget::Mutation(mutation.id),
                None,
                SignalPayload::Empty,
            );
        }

        Ok(MutatedAllele {
            locus,
            transposon: None,
            allele,
            mutations,
            gene_fqn,
        })
    }
}
