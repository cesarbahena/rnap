use super::super::*;

impl Dnap {
    pub fn attach_enhancer_promoter(
        &mut self,
        input: AttachEnhancerPromoter,
    ) -> Result<EnhancerContext, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_grn_in_genome(input.grn_id, input.genome_id)?;
        self.require_tf_in_insulator(input.updated_by, input.insulator_id)?;

        let enhancer_allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.grn_id,
            &input.enhancer_gene_fqn,
        )?;
        let promoter_allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.grn_id,
            &input.promoter_gene_fqn,
        )?;
        let enhancer_locus_id = self
            .alleles
            .get(&enhancer_allele_id)
            .map(|allele| allele.locus_id)
            .ok_or(DnapError::AlleleNotFound)?;
        let promoter_locus_id = self
            .alleles
            .get(&promoter_allele_id)
            .map(|allele| allele.locus_id)
            .ok_or(DnapError::AlleleNotFound)?;
        if !self.locus_has_artifact(enhancer_locus_id, ArtifactKind::Enhancer) {
            return Err(DnapError::EnhancerContextEnhancerRequired);
        }
        if !self.locus_has_artifact(promoter_locus_id, ArtifactKind::Promoter) {
            return Err(DnapError::EnhancerContextPromoterRequired);
        }

        let context = EnhancerContext {
            enhancer_locus_id,
            promoter_locus_id,
            updated_at: SystemTime::now(),
        };
        self.enhancer_contexts
            .insert(enhancer_locus_id, context.clone());
        Ok(context)
    }
}
