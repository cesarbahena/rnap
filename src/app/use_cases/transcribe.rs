use super::super::*;

impl Dnap {
    pub fn transcribe(&mut self, input: TranscribeAllele) -> Result<TranscribedAllele, DnapError> {
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
                        .map(|cursor| {
                            (
                                cursor.last_rendered_mutation_id,
                                cursor.last_rendered_sequence_hash.clone(),
                            )
                        });
                    let latest = latest_cursors
                        .iter()
                        .find(|cursor| cursor.sequence_definition_id == sequence.definition_id)
                        .map(|cursor| {
                            (
                                cursor.last_rendered_mutation_id,
                                cursor.last_rendered_sequence_hash.clone(),
                            )
                        });
                    previous != latest
                })
                .collect()
        };

        let now = SystemTime::now();
        let transcriptome = match self.transcriptomes.get(&allele.id).cloned() {
            Some(mut transcriptome) => {
                transcriptome.sequences = latest_cursors;
                transcriptome.updated_at = now;
                transcriptome
            }
            None => Transcriptome {
                id: self.allocate_transcriptome_id(),
                locus_id: allele.locus_id,
                allele_id: allele.id,
                sequences: latest_cursors,
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
}
