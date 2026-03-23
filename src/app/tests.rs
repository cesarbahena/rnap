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
            normalized_artifact: Some(prd_normalized_artifact()),
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
            normalized_artifact: Some(prd_normalized_artifact()),
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
            normalized_artifact: Some(prd_normalized_artifact()),
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
            normalized_artifact: Some(prd_normalized_artifact()),
            sequences: vec![sequence("Title"), sequence("title")],
            created_by: tf.id,
        }),
        Err(DnapError::DuplicateSequenceDefinitionName)
    );
}

#[test]
fn requires_normalized_artifact_type_for_gene_family() {
    let mut dnap = Dnap::default();
    let provisioned = provision_acme(&mut dnap);
    let tf = create_cesar(&mut dnap, provisioned.insulator.id);

    assert_eq!(
        dnap.define_gene_family(DefineGeneFamily {
            insulator_id: provisioned.insulator.id,
            genome_id: None,
            name: "Product Requirements Document".to_owned(),
            abbreviation: "PRD".to_owned(),
            normalized_artifact: None,
            sequences: vec![sequence("title")],
            created_by: tf.id,
        }),
        Err(DnapError::MissingNormalizedArtifact)
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
            normalized_artifact: Some(prd_normalized_artifact()),
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
            normalized_artifact: Some(prd_normalized_artifact()),
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

#[test]
fn mutate_new_can_create_locus_transposon_and_allele_without_initial_mutations() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Feature Requirements Specification",
        "FRS",
    );

    let empty = dnap
        .mutate_new(MutateNew {
            insulator_id,
            genome_id,
            grn_id,
            gene_family_abbreviation: "FRS".to_owned(),
            locus_name: "Checkout".to_owned(),
            mutations: Vec::new(),
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("new allele without initial mutations");

    assert_eq!(empty.locus.name, "Checkout");
    assert_eq!(empty.allele.state, AlleleState::Mutating);
    assert_eq!(empty.gene_fqn, "FRS-checkout-0001");
    assert!(empty.mutations.is_empty());
    assert!(empty.transposon.is_some());
    assert!(dnap
        .project_allele(empty.allele.id)
        .expect("empty projection")
        .is_empty());
}

#[test]
fn mutate_new_can_create_initial_sequence_mutations() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Feature Requirements Specification",
        "FRS",
    );
    let mutated = dnap
        .mutate_new(MutateNew {
            insulator_id,
            genome_id,
            grn_id,
            gene_family_abbreviation: "frs".to_owned(),
            locus_name: "Checkout".to_owned(),
            mutations: vec![mutation(
                "Some Section",
                SequenceValue::String("Draft".to_owned()),
            )],
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("new candidate work");

    assert_eq!(mutated.locus.name, "Checkout");
    assert_eq!(mutated.allele.state, AlleleState::Mutating);
    assert_eq!(mutated.gene_fqn, "FRS-checkout-0001");
    assert!(mutated.transposon.is_some());
    assert_eq!(
        dnap.project_allele(mutated.allele.id)
            .expect("candidate projection"),
        vec![Sequence {
            definition_id: mutated.mutations[0].sequence_definition_id,
            name: "Some Section".to_owned(),
            value: SequenceValue::String("Draft".to_owned()),
        }]
    );
}

#[test]
fn mutation_sequence_matching_is_kebab_case_insensitive_and_type_checked() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Feature Requirements Specification",
        "FRS",
    );

    let mutated = dnap
        .mutate_new(MutateNew {
            insulator_id,
            genome_id,
            grn_id,
            gene_family_abbreviation: "FRS".to_owned(),
            locus_name: "Checkout".to_owned(),
            mutations: vec![mutation(
                "some-section",
                SequenceValue::String("Draft".to_owned()),
            )],
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("kebab matched sequence");

    assert_eq!(
        dnap.mutate_existing(MutateExisting {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: mutated.gene_fqn,
            mutations: vec![mutation("some-section", SequenceValue::Bool(true))],
            causes: Vec::new(),
            created_by: tf_id,
        }),
        Err(DnapError::SequenceValueTypeMismatch)
    );
}

#[test]
fn active_allele_can_be_resolved_by_locus_name_without_fuzzy_matching() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Feature Requirements Specification",
        "FRS",
    );
    dnap.mutate_new(MutateNew {
        insulator_id,
        genome_id,
        grn_id,
        gene_family_abbreviation: "FRS".to_owned(),
        locus_name: "Checkout".to_owned(),
        mutations: vec![mutation(
            "Some Section",
            SequenceValue::String("Draft".to_owned()),
        )],
        causes: Vec::new(),
        created_by: tf_id,
    })
    .expect("new candidate work");

    let mutated = dnap
        .mutate_existing(MutateExisting {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: "checkout".to_owned(),
            mutations: vec![mutation("Prob", SequenceValue::String("Pain".to_owned()))],
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("locus name resolves");

    assert_eq!(mutated.gene_fqn, "FRS-checkout-0001");
    assert_eq!(mutated.allele.state, AlleleState::Mutating);
}

#[test]
fn active_allele_fqn_resolution_is_scoped_to_the_grn() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, owner_tf_id) = workspace(&mut dnap);
    let other_tf = dnap
        .create_tf(CreateTf {
            insulator_id,
            display_name: "Reviewer".to_owned(),
            external_subject: None,
            identity_provider: None,
        })
        .expect("valid tf");
    define_gene_family(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        owner_tf_id,
        "Feature Requirements Specification",
        "FRS",
    );
    dnap.mutate_new(MutateNew {
        insulator_id,
        genome_id,
        grn_id,
        gene_family_abbreviation: "FRS".to_owned(),
        locus_name: "Checkout".to_owned(),
        mutations: vec![mutation(
            "Some Section",
            SequenceValue::String("Draft".to_owned()),
        )],
        causes: Vec::new(),
        created_by: owner_tf_id,
    })
    .expect("owner candidate work");

    let mutated = dnap
        .mutate_existing(MutateExisting {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: "FRS-checkout".to_owned(),
            mutations: vec![mutation(
                "Problem",
                SequenceValue::String("Cross-user edit".to_owned()),
            )],
            causes: Vec::new(),
            created_by: other_tf.id,
        })
        .expect("shared grn candidate work");
    assert_eq!(mutated.allele.grn_id, grn_id);
}

#[test]
fn lgtm_expresses_unexpressed_mutations_without_requiring_transcribe() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Feature Requirements Specification",
        "FRS",
    );
    let mutated = dnap
        .mutate_new(MutateNew {
            insulator_id,
            genome_id,
            grn_id,
            gene_family_abbreviation: "FRS".to_owned(),
            locus_name: "Checkout".to_owned(),
            mutations: vec![mutation(
                "Some Section",
                SequenceValue::String("Draft".to_owned()),
            )],
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("new candidate work");

    dnap.splice(SpliceAllele {
        insulator_id,
        genome_id,
        grn_id,
        gene_fqn: mutated.gene_fqn.clone(),
        task_realization_texts: vec!["Build checkout".to_owned()],
        lgtm: false,
        created_by: tf_id,
    })
    .expect("first splice");

    let unexpressed = dnap
        .mutate_existing(MutateExisting {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: "FRS-checkout".to_owned(),
            mutations: vec![mutation(
                "Some Section",
                SequenceValue::String("Updated".to_owned()),
            )],
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("mutate spliced work");
    assert_eq!(unexpressed.allele.state, AlleleState::Mutating);

    let spliced = dnap
        .splice(SpliceAllele {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: "FRS-checkout".to_owned(),
            task_realization_texts: Vec::new(),
            lgtm: true,
            created_by: tf_id,
        })
        .expect("lgtm acknowledgement");
    assert_eq!(spliced.allele.state, AlleleState::Expressing);
    assert!(spliced.task_realizations.is_empty());
    assert_eq!(spliced.untranscribed_unexpressed_mutations, 1);
}

#[test]
fn translate_returns_task_realizations_without_changing_allele_state() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Feature Requirements Specification",
        "FRS",
    );
    let mutated = dnap
        .mutate_new(MutateNew {
            insulator_id,
            genome_id,
            grn_id,
            gene_family_abbreviation: "FRS".to_owned(),
            locus_name: "Checkout".to_owned(),
            mutations: vec![mutation(
                "Some Section",
                SequenceValue::String("Draft".to_owned()),
            )],
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("new candidate work");
    dnap.splice(SpliceAllele {
        insulator_id,
        genome_id,
        grn_id,
        gene_fqn: mutated.gene_fqn.clone(),
        task_realization_texts: vec![
            "Implement checkout API".to_owned(),
            "Add retry tests".to_owned(),
        ],
        lgtm: false,
        created_by: tf_id,
    })
    .expect("splice task_realizations");

    let translated = dnap
        .translate(TranslateAllele {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: "checkout".to_owned(),
            created_by: tf_id,
        })
        .expect("translate task_realizations");

    assert_eq!(translated.allele.state, AlleleState::Expressing);
    assert_eq!(
        translated
            .task_realizations
            .iter()
            .map(|task_realization| task_realization.text.as_str())
            .collect::<Vec<_>>(),
        vec!["Implement checkout API", "Add retry tests"]
    );
    assert_eq!(
        dnap.allele(translated.allele.id).expect("allele").state,
        AlleleState::Expressing
    );
}

#[test]
fn translate_errors_when_the_active_allele_has_no_task_realizations() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Feature Requirements Specification",
        "FRS",
    );
    dnap.mutate_new(MutateNew {
        insulator_id,
        genome_id,
        grn_id,
        gene_family_abbreviation: "FRS".to_owned(),
        locus_name: "Checkout".to_owned(),
        mutations: Vec::new(),
        causes: Vec::new(),
        created_by: tf_id,
    })
    .expect("new candidate work");

    assert_eq!(
        dnap.translate(TranslateAllele {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: "checkout".to_owned(),
            created_by: tf_id,
        }),
        Err(DnapError::TaskRealizationsNotFound)
    );
}

#[test]
fn semantic_narrowings_target_mrna_and_can_chain_follow_ups() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family_with_normalized_artifact(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Requirement",
        "REQ",
        NormalizedArtifact::ManagedRequirement,
    );
    let target = dnap
        .mutate_new(MutateNew {
            insulator_id,
            genome_id,
            grn_id,
            gene_family_abbreviation: "REQ".to_owned(),
            locus_name: "Checkout requirements".to_owned(),
            mutations: Vec::new(),
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("target");

    let semantic_narrowing = dnap
        .create_semantic_narrowing(CreateSemanticNarrowing {
            insulator_id,
            genome_id,
            grn_id,
            target_mrna_fqn: "checkout-requirements".to_owned(),
            target_sequence_name: None,
            title: "Clarify payment retries".to_owned(),
            body: None,
            precursor: None,
            created_by: tf_id,
        })
        .expect("semantic_narrowing");
    let answered = dnap
        .append_semantic_narrowing_sequence(AppendSemanticNarrowingSequence {
            insulator_id,
            genome_id,
            grn_id,
            target_mrna_fqn: None,
            target_sequence_name: None,
            semantic_narrowing_title: "clarify-payment-retries".to_owned(),
            body: Some("Retry twice".to_owned()),
            follow_up_title: Some("Clarify retry ceiling".to_owned()),
            follow_up_body: None,
            created_by: tf_id,
        })
        .expect("follow up");

    assert_eq!(semantic_narrowing.target_mrna_locus_id, target.locus.id);
    assert_eq!(answered.semantic_narrowing.id, semantic_narrowing.id);
    assert_eq!(
        answered.sequence.expect("answer").body,
        "Retry twice".to_owned()
    );
    assert_eq!(
        answered.follow_up.expect("follow up").precursor,
        Some(semantic_narrowing.id)
    );
}

#[test]
fn semantic_narrowings_reject_rrna_targets() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family_with_normalized_artifact(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Design",
        "DSN",
        NormalizedArtifact::ResourceReference,
    );
    dnap.mutate_new(MutateNew {
        insulator_id,
        genome_id,
        grn_id,
        gene_family_abbreviation: "DSN".to_owned(),
        locus_name: "Checkout design".to_owned(),
        mutations: Vec::new(),
        causes: Vec::new(),
        created_by: tf_id,
    })
    .expect("design");

    assert_eq!(
        dnap.create_semantic_narrowing(CreateSemanticNarrowing {
            insulator_id,
            genome_id,
            grn_id,
            target_mrna_fqn: "checkout-design".to_owned(),
            target_sequence_name: None,
            title: "Clarify component boundary".to_owned(),
            body: None,
            precursor: None,
            created_by: tf_id,
        }),
        Err(DnapError::SemanticNarrowingTargetRequired)
    );
}

#[test]
pub(super) fn mutation_context_captures_relevant_semantic_narrowings_and_explicit_causes() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family_with_normalized_artifact(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Requirement",
        "REQ",
        NormalizedArtifact::ManagedRequirement,
    );
    dnap.mutate_new(MutateNew {
        insulator_id,
        genome_id,
        grn_id,
        gene_family_abbreviation: "REQ".to_owned(),
        locus_name: "Checkout requirements".to_owned(),
        mutations: vec![mutation(
            "Some Section",
            SequenceValue::String("Draft".to_owned()),
        )],
        causes: Vec::new(),
        created_by: tf_id,
    })
    .expect("target");
    let cause = dnap
        .create_semantic_narrowing(CreateSemanticNarrowing {
            insulator_id,
            genome_id,
            grn_id,
            target_mrna_fqn: "checkout-requirements".to_owned(),
            target_sequence_name: Some("Some Section".to_owned()),
            title: "How strict is latency".to_owned(),
            body: None,
            precursor: None,
            created_by: tf_id,
        })
        .expect("cause semantic_narrowing");
    let cause_answer = dnap
        .append_semantic_narrowing_sequence(AppendSemanticNarrowingSequence {
            insulator_id,
            genome_id,
            grn_id,
            target_mrna_fqn: None,
            target_sequence_name: None,
            semantic_narrowing_title: "how-strict".to_owned(),
            body: Some("Paid checkout under 100ms".to_owned()),
            follow_up_title: None,
            follow_up_body: None,
            created_by: tf_id,
        })
        .expect("cause answer")
        .sequence
        .expect("sequence");
    let unanswered = dnap
        .create_semantic_narrowing(CreateSemanticNarrowing {
            insulator_id,
            genome_id,
            grn_id,
            target_mrna_fqn: "checkout-requirements".to_owned(),
            target_sequence_name: Some("Some Section".to_owned()),
            title: "Which users are included".to_owned(),
            body: None,
            precursor: None,
            created_by: tf_id,
        })
        .expect("unanswered semantic_narrowing");

    let mutated = dnap
        .mutate_existing(MutateExisting {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: "checkout-requirements".to_owned(),
            mutations: vec![mutation(
                "Some Section",
                SequenceValue::String("Paid checkout latency < 100ms".to_owned()),
            )],
            causes: vec!["how-strict".to_owned()],
            created_by: tf_id,
        })
        .expect("mutate with cause");

    assert!(mutated.mutations[0]
        .context
        .contains(&MutationContext::Cause(cause.id, cause_answer.id)));
    assert!(mutated.mutations[0]
        .context
        .contains(&MutationContext::UnansweredContext(unanswered.id)));
}

#[test]
fn transcriptome_tracks_render_cursor_per_sequence_without_storing_projection() {
    let mut dnap = Dnap::default();
    let (insulator_id, genome_id, grn_id, tf_id) = workspace(&mut dnap);
    define_gene_family(
        &mut dnap,
        insulator_id,
        Some(genome_id),
        tf_id,
        "Feature Requirements Specification",
        "FRS",
    );
    let mutated = dnap
        .mutate_new(MutateNew {
            insulator_id,
            genome_id,
            grn_id,
            gene_family_abbreviation: "FRS".to_owned(),
            locus_name: "Checkout".to_owned(),
            mutations: vec![
                mutation("Some Section", SequenceValue::String("Draft".to_owned())),
                mutation("Problem", SequenceValue::String("Pain".to_owned())),
            ],
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("new candidate work");

    let first = dnap
        .transcribe(TranscribeAllele {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: mutated.gene_fqn.clone(),
            full: false,
            created_by: tf_id,
        })
        .expect("first transcript");
    assert_eq!(first.sequences.len(), 2);
    assert_eq!(first.transcriptome.sequences.len(), 2);

    let second = dnap
        .transcribe(TranscribeAllele {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: mutated.gene_fqn.clone(),
            full: false,
            created_by: tf_id,
        })
        .expect("unchanged transcript");
    assert!(second.sequences.is_empty());

    let changed = dnap
        .mutate_existing(MutateExisting {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: mutated.gene_fqn,
            mutations: vec![mutation(
                "Problem",
                SequenceValue::String("Sharper pain".to_owned()),
            )],
            causes: Vec::new(),
            created_by: tf_id,
        })
        .expect("change one sequence");
    let third = dnap
        .transcribe(TranscribeAllele {
            insulator_id,
            genome_id,
            grn_id,
            gene_fqn: changed.gene_fqn,
            full: false,
            created_by: tf_id,
        })
        .expect("changed transcript");

    assert_eq!(third.sequences.len(), 1);
    assert_eq!(third.sequences[0].name, "Problem");
    assert_eq!(third.transcriptome.sequences.len(), 2);
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

fn workspace(dnap: &mut Dnap) -> (InsulatorId, GenomeId, GrnId, TfId) {
    let provisioned = provision_acme(dnap);
    let genome = create_billing_genome(dnap, provisioned.insulator.id);
    let tf = create_cesar(dnap, provisioned.insulator.id);
    let grn = dnap
        .create_grn(CreateGrn {
            insulator_id: provisioned.insulator.id,
            genome_id: genome.id,
            name: "Checkout".to_owned(),
            activator: tf.id,
        })
        .expect("valid grn");
    (provisioned.insulator.id, genome.id, grn.grn.id, tf.id)
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
        normalized_artifact: Some(prd_normalized_artifact()),
        sequences: vec![sequence("Some Section"), sequence("Problem")],
        created_by,
    })
    .expect("valid gene family")
}

fn define_gene_family_with_normalized_artifact(
    dnap: &mut Dnap,
    insulator_id: InsulatorId,
    genome_id: Option<GenomeId>,
    created_by: TfId,
    name: &str,
    abbreviation: &str,
    normalized_artifact: NormalizedArtifact,
) -> DefinedGeneFamily {
    dnap.define_gene_family(DefineGeneFamily {
        insulator_id,
        genome_id,
        name: name.to_owned(),
        abbreviation: abbreviation.to_owned(),
        normalized_artifact: Some(normalized_artifact),
        sequences: vec![sequence("Some Section")],
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

fn mutation(sequence_name: &str, value: SequenceValue) -> SequenceMutation {
    SequenceMutation {
        sequence_name: sequence_name.to_owned(),
        value,
    }
}

fn prd_normalized_artifact() -> NormalizedArtifact {
    NormalizedArtifact::ManagedRequirement
}
