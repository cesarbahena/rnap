use std::collections::BTreeMap;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InsulatorId(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenomeId(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TfId(u64);

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
    InsulatorNotFound,
}

#[derive(Default)]
pub struct Dnap {
    next_insulator_id: u64,
    next_genome_id: u64,
    next_tf_id: u64,
    insulators: BTreeMap<InsulatorId, Insulator>,
    placements: BTreeMap<InsulatorId, InsulatorPlacement>,
    genomes: BTreeMap<GenomeId, Genome>,
    tfs: BTreeMap<TfId, Tf>,
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

    fn require_insulator(&self, id: InsulatorId) -> Result<(), DnapError> {
        self.insulators
            .contains_key(&id)
            .then_some(())
            .ok_or(DnapError::InsulatorNotFound)
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
}

fn require_text(value: String, error: DnapError) -> Result<String, DnapError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_owned())
    }
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

    fn provision_acme(dnap: &mut Dnap) -> ProvisionedInsulator {
        dnap.provision_insulator(ProvisionInsulator {
            name: "Acme".to_owned(),
            placement_region: "us-east-1".to_owned(),
            placement_strategy: None,
        })
        .expect("valid provisioning")
    }
}
