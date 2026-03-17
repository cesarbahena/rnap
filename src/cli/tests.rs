use super::*;

#[test]
fn epigenetics_bootstraps_session_then_normal_workflow_commands_use_it() {
    let mut state = LocalState::default();

    assert!(dispatch(
        &mut state,
        words("epigenetics init-insulator Acme --region us-east-1")
    )
    .expect("init insulator")
    .contains("initialized insulator"));
    dispatch(
        &mut state,
        words("epigenetics init-genome Billing --insulator Acme"),
    )
    .expect("init genome");
    dispatch(
        &mut state,
        words("epigenetics init-tf Cesar --insulator Acme"),
    )
    .expect("init tf");
    dispatch(
        &mut state,
        words("epigenetics init-grn Checkout --insulator Acme --genome Billing --tf Cesar"),
    )
    .expect("init grn");
    dispatch(
        &mut state,
        words("epigenetics use --insulator Acme --genome Billing --grn Checkout --tf Cesar"),
    )
    .expect("use session");
    dispatch(
            &mut state,
            words(
                "epigenetics define-family FRS FeatureRequirements --artifact mRNA --sequence Summary --sequence Risk",
            ),
        )
        .expect("define family");

    let mutated = dispatch(
        &mut state,
        words("mutate --new FRS Checkout --summary Draft --risk Unknown"),
    )
    .expect("mutate new");
    assert!(mutated.contains("FRS-checkout-0001"));

    let transcript =
        dispatch(&mut state, words("transcribe FRS-checkout")).expect("transcribe latest");
    assert!(transcript.contains("showing Sequence changes since last transcription"));
    assert!(transcript.contains("Summary: Draft"));
    assert!(transcript.contains("approval: visible"));

    let full_transcript =
        dispatch(&mut state, words("transcribe FRS-checkout --full")).expect("full transcript");
    assert!(full_transcript.contains("showing full transcript"));
    assert!(full_transcript.contains("Summary: Draft"));

    let spliced = dispatch(&mut state, words("splice FRS-checkout BuildCheckout")).expect("splice");
    assert!(spliced.contains("Expressing"));

    let translated =
        dispatch(&mut state, words("translate FRS-checkout")).expect("translate exons");
    assert!(translated.contains("translated Expressing"));
    assert!(translated.contains("1. BuildCheckout"));

    let unchanged_transcript =
        dispatch(&mut state, words("transcribe FRS-checkout")).expect("transcribe unchanged");
    assert!(unchanged_transcript.contains("no new Sequence changes since last transcription"));
    assert!(!unchanged_transcript.contains("approval comments shown"));
}

#[test]
fn splice_requires_exon_text_or_lgtm_escape_hatch() {
    let mut state = bootstrapped_state();
    dispatch(&mut state, words("mutate --new FRS Checkout")).expect("mutate new");

    let error =
        dispatch(&mut state, words("splice checkout")).expect_err("empty splice is invalid");
    assert!(matches!(error, CliError::Usage(message) if message.contains("Exon text or --lgtm")));
}

#[test]
fn lgtm_cli_flow_expresses_without_requiring_transcribe() {
    let mut state = bootstrapped_state();
    dispatch(
        &mut state,
        words("mutate --new FRS Checkout --summary Draft --risk Unknown"),
    )
    .expect("mutate new");
    dispatch(&mut state, words("splice FRS-checkout BuildCheckout")).expect("splice");
    dispatch(
        &mut state,
        words("mutate FRS-checkout --summary UpdatedDraft"),
    )
    .expect("stale mutation");

    let spliced = dispatch(&mut state, words("splice FRS-checkout --lgtm"))
        .expect("lgtm expresses latest mutation");
    assert!(spliced.contains("Expressing"));
    assert!(spliced.contains("1 untranscribed unexpressed mutation"));
}

#[test]
fn translate_errors_when_no_exons_exist() {
    let mut state = bootstrapped_state();
    dispatch(&mut state, words("mutate --new FRS Checkout")).expect("mutate new");

    let error =
        dispatch(&mut state, words("translate checkout")).expect_err("translate requires exons");
    assert!(matches!(error, CliError::Dnap(DnapError::ExonsNotFound)));
}

#[test]
fn explore_cli_attaches_enhancer_to_promoter_property() {
    let mut state = bootstrapped_state();
    dispatch(
        &mut state,
        words("epigenetics define-family STR Story --artifact promoter --sequence Summary"),
    )
    .expect("promoter family");
    dispatch(
        &mut state,
        words("epigenetics define-family RSH Research --artifact enhancer --sequence Summary"),
    )
    .expect("enhancer family");
    dispatch(&mut state, words("mutate --new STR Checkout")).expect("promoter");
    dispatch(&mut state, words("mutate --new RSH PaymentResearch")).expect("enhancer");

    let output = dispatch(
        &mut state,
        words("explore enhancer PaymentResearch --promoter Checkout"),
    )
    .expect("attach enhancer");

    assert!(output.contains("attached enhancer `PaymentResearch` to promoter `Checkout`"));
}

#[test]
fn q_and_a_cli_create_answer_and_show_semantic_narrowings() {
    let mut state = bootstrapped_state();
    dispatch(&mut state, words("mutate --new FRS Checkout")).expect("target");

    let asked = dispatch(&mut state, words("q Checkout ClarifyRetries")).expect("ask");
    assert!(asked.contains("asked `ClarifyRetries`"));

    let answered = dispatch(
        &mut state,
        words("a --clarifyretries RetryTwice -q ClarifyCeiling"),
    )
    .expect("answer");
    assert!(answered.contains("answered `ClarifyRetries`"));
    assert!(answered.contains("asked follow-up `ClarifyCeiling`"));

    let questions = dispatch(&mut state, words("q Checkout")).expect("list questions");
    assert!(questions.contains("ClarifyRetries -> RetryTwice"));
    assert!(questions.contains("1 follow-up"));

    let thread = dispatch(&mut state, words("a --clarifyretries")).expect("show thread");
    assert!(thread.contains("question: ClarifyRetries"));
    assert!(thread.contains("answer: RetryTwice"));
    assert!(thread.contains("follow-up: ClarifyCeiling"));
}

#[test]
fn parses_current_normalized_artifact_taxonomy_aliases() {
    let cases = [
        ("promoter", NormalizedArtifact::Promoter),
        (
            "enhancer",
            NormalizedArtifact::EnterpriseNegotiationHandoverCertificate,
        ),
        ("spacers", NormalizedArtifact::Spacer),
        ("telomere", NormalizedArtifact::TestObjectiveManifest),
        ("silencer", NormalizedArtifact::Silencer),
        ("eRNA", NormalizedArtifact::Executable),
        ("mRNA", NormalizedArtifact::ManagedRequirement),
        ("rRNA", NormalizedArtifact::ResourceReference),
        ("tRNA", NormalizedArtifact::TaskRealization),
        ("snRNA", NormalizedArtifact::SemanticNarrowing),
        ("scaRNA", NormalizedArtifact::SemanticConstraintAssumption),
        ("siRNA", NormalizedArtifact::StopImplementation),
        ("tmRNA", NormalizedArtifact::TaskMediation),
        ("miRNA", NormalizedArtifact::Microalignment),
        ("piRNA", NormalizedArtifact::ProjectedIntent),
        ("snoRNA", NormalizedArtifact::StrategicNote),
        ("crRNA", NormalizedArtifact::CausalResolution),
        ("tracrRNA", NormalizedArtifact::TraceReport),
        ("lncRNA", NormalizedArtifact::LongNarrativeContext),
        (
            "circRNA",
            NormalizedArtifact::CircularInstitutionalReferenceContext,
        ),
        ("sgRNA", NormalizedArtifact::SuggestedChanges),
    ];

    for (alias, expected) in cases {
        assert_eq!(
            parse_normalized_artifact(alias).expect("supported normalized_artifact"),
            expected
        );
    }
}

#[test]
fn rejects_unknown_normalized_artifact_aliases() {
    let error = parse_normalized_artifact("not-a-real-normalized_artifact")
        .expect_err("unknown normalized_artifact");

    assert!(
        matches!(error, CliError::Usage(message) if message.contains("unsupported normalized artifact"))
    );
}

fn bootstrapped_state() -> LocalState {
    let mut state = LocalState::default();
    dispatch(
        &mut state,
        words("epigenetics init-insulator Acme --region us-east-1"),
    )
    .expect("init insulator");
    dispatch(
        &mut state,
        words("epigenetics init-genome Billing --insulator Acme"),
    )
    .expect("init genome");
    dispatch(
        &mut state,
        words("epigenetics init-tf Cesar --insulator Acme"),
    )
    .expect("init tf");
    dispatch(
        &mut state,
        words("epigenetics init-grn Checkout --insulator Acme --genome Billing --tf Cesar"),
    )
    .expect("init grn");
    dispatch(
        &mut state,
        words("epigenetics use --insulator Acme --genome Billing --grn Checkout --tf Cesar"),
    )
    .expect("use session");
    dispatch(
            &mut state,
            words(
                "epigenetics define-family FRS FeatureRequirements --artifact mRNA --sequence Summary --sequence Risk",
            ),
        )
        .expect("define family");
    state
}

fn words(input: &str) -> Vec<String> {
    input.split_whitespace().map(str::to_owned).collect()
}
