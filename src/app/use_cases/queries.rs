use super::super::*;

impl Dnap {
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

    pub fn enhancer_context(&self, enhancer_locus_id: LocusId) -> Option<&EnhancerContext> {
        self.enhancer_contexts.get(&enhancer_locus_id)
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
}
