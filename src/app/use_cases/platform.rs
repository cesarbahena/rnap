use super::super::*;

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
        self.record_signal(
            insulator_id,
            None,
            SignalType::InsulatorProvisioned,
            SignalTarget::Insulator(insulator_id),
            None,
            SignalPayload::Empty,
        );

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
        self.record_signal(
            input.insulator_id,
            None,
            SignalType::GenomeCreated,
            SignalTarget::Genome(genome.id),
            None,
            SignalPayload::Empty,
        );
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
        self.record_signal(
            input.insulator_id,
            Some(tf.id),
            SignalType::TfCreated,
            SignalTarget::Tf(tf.id),
            None,
            SignalPayload::Empty,
        );
        Ok(tf)
    }

    pub fn create_grn(&mut self, input: CreateGrn) -> Result<CreatedGrn, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_tf_in_insulator(input.activator, input.insulator_id)?;
        let name = require_text(input.name, DnapError::BlankGrnName)?;
        let now = SystemTime::now();
        let grn = Grn {
            id: self.allocate_grn_id(),
            genome_id: input.genome_id,
            name,
            activator: input.activator,
            state: GrnState::Triage,
            created_at: now,
            updated_at: now,
        };

        self.grns.insert(grn.id, grn.clone());
        self.record_signal(
            input.insulator_id,
            Some(input.activator),
            SignalType::GrnCreated,
            SignalTarget::Grn(grn.id),
            None,
            SignalPayload::Empty,
        );
        Ok(CreatedGrn { grn })
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
        let normalized_artifact = input
            .normalized_artifact
            .ok_or(DnapError::MissingNormalizedArtifact)?;
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
            normalized_artifact,
            created_at: now,
            updated_at: now,
        };
        let generation = GeneFamilyGeneration {
            id: generation_id,
            family_id,
            generation: 1,
            sequences,
            created_at: now,
        };

        self.gene_families.insert(family_id, family.clone());
        self.gene_family_generations
            .insert(generation_id, generation.clone());
        self.record_signal(
            input.insulator_id,
            Some(input.created_by),
            SignalType::GeneFamilyDefined,
            SignalTarget::GeneFamily(family_id),
            None,
            SignalPayload::Empty,
        );

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
}
