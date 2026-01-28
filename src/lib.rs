use std::collections::BTreeMap;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InsulatorId(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenomeId(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TfId(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GeneFamilyId(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GeneFamilyGenerationId(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SequenceDefinitionId(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Insulator {
    pub id: InsulatorId,
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsulatorPlacement {
    pub insulator_id: InsulatorId,
    pub strategy: InsulatorPlacementStrategy,
    pub region: String,
    pub active: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsulatorPlacementStrategy {
    SharedCluster,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Genome {
    pub id: GenomeId,
    pub insulator_id: InsulatorId,
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tf {
    pub id: TfId,
    pub insulator_id: InsulatorId,
    pub display_name: String,
    pub external_subject: Option<String>,
    pub identity_provider: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneFamily {
    pub id: GeneFamilyId,
    pub insulator_id: InsulatorId,
    pub genome_id: Option<GenomeId>,
    pub name: String,
    pub abbreviation: String,
    pub current_generation_id: GeneFamilyGenerationId,
    pub encodes: EncodingType,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneFamilyGeneration {
    pub id: GeneFamilyGenerationId,
    pub family_id: GeneFamilyId,
    pub generation: u32,
    pub sequences: Vec<SequenceDefinition>,
    pub created_by: TfId,
    pub created_at: SystemTime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SequenceDefinition {
    pub id: SequenceDefinitionId,
    pub name: String,
    pub sequence_type: SequenceType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SequenceType {
    String,
    StringVec,
    Int,
    IntVec,
    Float,
    FloatVec,
    Bool,
    BoolVec,
    Gene,
    GeneVec,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodingType {
    RNA(RnaType),
    GRN(GrnType),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrnType {
    Promoter,
    Telomere,
    Centromere,
    Silencer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RnaType {
    Translation(TranslationRnaType),
    Regulatory(RegulatoryRnaType),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranslationRnaType {
    MRNA,
    RRNA,
    TRNA,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegulatoryRnaType {
    SnRNA,
    SiRNA,
    TmRNA,
    GRNA,
    MiRNA,
    PiRNA,
    ERNA,
    SnoRNA,
    CrRNA,
    TracrRNA,
    LncRNA,
    CircRNA,
    SgRNA,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvisionInsulator {
    pub name: String,
    pub placement_region: String,
    pub placement_strategy: Option<InsulatorPlacementStrategy>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateGenome {
    pub insulator_id: InsulatorId,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateTf {
    pub insulator_id: InsulatorId,
    pub display_name: String,
    pub external_subject: Option<String>,
    pub identity_provider: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefineGeneFamily {
    pub insulator_id: InsulatorId,
    pub genome_id: Option<GenomeId>,
    pub name: String,
    pub abbreviation: String,
    pub encodes: Option<EncodingType>,
    pub sequences: Vec<DefineSequence>,
    pub created_by: TfId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefineSequence {
    pub name: String,
    pub sequence_type: SequenceType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinedGeneFamily {
    pub family: GeneFamily,
    pub generation: GeneFamilyGeneration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvisionedInsulator {
    pub insulator: Insulator,
    pub placement: InsulatorPlacement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DnapError {
    BlankInsulatorName,
    BlankPlacementRegion,
    BlankGenomeName,
    BlankTfDisplayName,
    BlankGeneFamilyName,
    BlankGeneFamilyAbbreviation,
    BlankSequenceDefinitionName,
    DuplicateSequenceDefinitionName,
    DuplicateGeneFamilyAbbreviation,
    MissingEncodingType,
    InsulatorNotFound,
    GenomeNotFound,
    GenomeInsulatorMismatch,
    TfNotFound,
    TfInsulatorMismatch,
}

#[derive(Default)]
pub struct Dnap {
    next_insulator_id: u64,
    next_genome_id: u64,
    next_tf_id: u64,
    next_gene_family_id: u64,
    next_gene_family_generation_id: u64,
    next_sequence_definition_id: u64,
    insulators: BTreeMap<InsulatorId, Insulator>,
    placements: BTreeMap<InsulatorId, InsulatorPlacement>,
    genomes: BTreeMap<GenomeId, Genome>,
    tfs: BTreeMap<TfId, Tf>,
    gene_families: BTreeMap<GeneFamilyId, GeneFamily>,
    gene_family_generations: BTreeMap<GeneFamilyGenerationId, GeneFamilyGeneration>,
}

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
        Ok(tf)
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
        let encodes = input.encodes.ok_or(DnapError::MissingEncodingType)?;
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
            encodes,
            created_at: now,
            updated_at: now,
        };
        let generation = GeneFamilyGeneration {
            id: generation_id,
            family_id,
            generation: 1,
            sequences,
            created_by: input.created_by,
            created_at: now,
        };

        self.gene_families.insert(family_id, family.clone());
        self.gene_family_generations
            .insert(generation_id, generation.clone());

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

    pub fn gene_family_generation(
        &self,
        id: GeneFamilyGenerationId,
    ) -> Option<&GeneFamilyGeneration> {
        self.gene_family_generations.get(&id)
    }

    fn require_insulator(&self, id: InsulatorId) -> Result<(), DnapError> {
        self.insulators
            .contains_key(&id)
            .then_some(())
            .ok_or(DnapError::InsulatorNotFound)
    }

    fn require_genome_in_insulator(
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

    fn require_tf_in_insulator(
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

    fn require_available_abbreviation(
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

    fn allocate_insulator_id(&mut self) -> InsulatorId {
        self.next_insulator_id += 1;
        InsulatorId(self.next_insulator_id)
    }

    fn allocate_genome_id(&mut self) -> GenomeId {
        self.next_genome_id += 1;
        GenomeId(self.next_genome_id)
    }

    fn allocate_tf_id(&mut self) -> TfId {
        self.next_tf_id += 1;
        TfId(self.next_tf_id)
    }

    fn allocate_gene_family_id(&mut self) -> GeneFamilyId {
        self.next_gene_family_id += 1;
        GeneFamilyId(self.next_gene_family_id)
    }

    fn allocate_gene_family_generation_id(&mut self) -> GeneFamilyGenerationId {
        self.next_gene_family_generation_id += 1;
        GeneFamilyGenerationId(self.next_gene_family_generation_id)
    }

    fn allocate_sequence_definition_id(&mut self) -> SequenceDefinitionId {
        self.next_sequence_definition_id += 1;
        SequenceDefinitionId(self.next_sequence_definition_id)
    }
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
    value.trim().to_ascii_lowercase()
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
            sequences: vec![sequence("title"), sequence("problem")],
            created_by,
        })
        .expect("valid gene family")
    }

    fn sequence(name: &str) -> DefineSequence {
        DefineSequence {
            name: name.to_owned(),
            sequence_type: SequenceType::String,
        }
    }

    fn prd_encoding() -> EncodingType {
        EncodingType::RNA(RnaType::Translation(TranslationRnaType::MRNA))
    }
}
