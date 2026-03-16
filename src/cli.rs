use std::ffi::OsString;
use std::fmt;
use std::time::SystemTime;

use crate::app::{
    AppendIntronSequence, AttachEnhancerPromoter, CreateGenome, CreateGrn, CreateIntron, CreateTf,
    DefineGeneFamily, DefineSequence, Dnap, DnapError, IntronSummary, IntronThread, MutateExisting,
    MutateNew, NormalizedArtifact, ProvisionInsulator, SequenceMutation, SequenceType,
    SequenceValue, SpliceAllele, TranscribeAllele, TranslateAllele,
};
use crate::session::{
    LocalState, LocalStateStore, Session, SessionActor, SessionError, SessionIssuer, SessionScope,
};

pub fn run_from_env() -> Result<(), CliError> {
    run(std::env::args_os())
}

pub fn run<I, T>(args: I) -> Result<(), CliError>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    let mut args = args
        .into_iter()
        .map(|arg| arg.into().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    if args.is_empty() {
        return Err(CliError::Usage("missing argv[0]".to_owned()));
    }
    args.remove(0);

    let store = LocalStateStore::default();
    let mut state = store.load()?;
    let output = dispatch(&mut state, args)?;
    store.save(&state)?;
    println!("{output}");
    Ok(())
}

fn dispatch(state: &mut LocalState, args: Vec<String>) -> Result<String, CliError> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(CliError::Usage("expected a command".to_owned()));
    };

    match command {
        "epigenetics" => epigenetics(state, &args[1..]),
        "mutate" => mutate(state, &args[1..]),
        "transcribe" => transcribe(state, &args[1..]),
        "splice" => splice(state, &args[1..]),
        "translate" => translate(state, &args[1..]),
        "explore" => explore(state, &args[1..]),
        "q" => question(state, &args[1..]),
        "a" => answer(state, &args[1..]),
        _ => Err(CliError::Usage(format!("unknown command `{command}`"))),
    }
}

fn epigenetics(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(CliError::Usage(
            "expected epigenetics subcommand".to_owned(),
        ));
    };

    match command {
        "init-insulator" => {
            let name = positional(args, 1, "insulator name")?;
            let region = option_value(args, "--region").unwrap_or("local".to_owned());
            let provisioned = state.dnap.provision_insulator(ProvisionInsulator {
                name,
                placement_region: region,
                placement_strategy: None,
            })?;
            Ok(format!(
                "initialized insulator `{}` ({:?})",
                provisioned.insulator.name, provisioned.insulator.id
            ))
        }
        "init-genome" => {
            let name = positional(args, 1, "genome name")?;
            let insulator_name = required_option(args, "--insulator")?;
            let insulator = state
                .dnap
                .find_insulator_by_name(&insulator_name)
                .ok_or_else(|| CliError::NotFound(format!("insulator `{insulator_name}`")))?;
            let genome = state.dnap.create_genome(CreateGenome {
                insulator_id: insulator.id,
                name,
            })?;
            Ok(format!(
                "initialized genome `{}` ({:?})",
                genome.name, genome.id
            ))
        }
        "init-tf" => {
            let display_name = positional(args, 1, "tf display name")?;
            let insulator_name = required_option(args, "--insulator")?;
            let insulator = state
                .dnap
                .find_insulator_by_name(&insulator_name)
                .ok_or_else(|| CliError::NotFound(format!("insulator `{insulator_name}`")))?;
            let tf = state.dnap.create_tf(CreateTf {
                insulator_id: insulator.id,
                display_name,
                external_subject: None,
                identity_provider: None,
            })?;
            Ok(format!(
                "initialized tf `{}` ({:?})",
                tf.display_name, tf.id
            ))
        }
        "init-grn" => {
            let name = positional(args, 1, "grn name")?;
            let insulator_name = required_option(args, "--insulator")?;
            let genome_name = required_option(args, "--genome")?;
            let tf_name = required_option(args, "--tf")?;
            let insulator = state
                .dnap
                .find_insulator_by_name(&insulator_name)
                .ok_or_else(|| CliError::NotFound(format!("insulator `{insulator_name}`")))?;
            let genome = state
                .dnap
                .find_genome_by_name(insulator.id, &genome_name)
                .ok_or_else(|| CliError::NotFound(format!("genome `{genome_name}`")))?;
            let tf = state
                .dnap
                .find_tf_by_display_name(insulator.id, &tf_name)
                .ok_or_else(|| CliError::NotFound(format!("tf `{tf_name}`")))?;
            let created = state.dnap.create_grn(CreateGrn {
                insulator_id: insulator.id,
                genome_id: genome.id,
                name,
                activator: tf.id,
            })?;
            Ok(format!(
                "initialized grn `{}` ({:?})",
                created.grn.name, created.grn.id
            ))
        }
        "use" => {
            let insulator_name = required_option(args, "--insulator")?;
            let genome_name = required_option(args, "--genome")?;
            let tf_name = required_option(args, "--tf")?;
            let grn_name = required_option(args, "--grn")?;
            let insulator = state
                .dnap
                .find_insulator_by_name(&insulator_name)
                .ok_or_else(|| CliError::NotFound(format!("insulator `{insulator_name}`")))?;
            let genome = state
                .dnap
                .find_genome_by_name(insulator.id, &genome_name)
                .ok_or_else(|| CliError::NotFound(format!("genome `{genome_name}`")))?;
            let tf = state
                .dnap
                .find_tf_by_display_name(insulator.id, &tf_name)
                .ok_or_else(|| CliError::NotFound(format!("tf `{tf_name}`")))?;
            let grn = state
                .dnap
                .find_grn_by_name(genome.id, &grn_name)
                .ok_or_else(|| CliError::NotFound(format!("grn `{grn_name}`")))?;
            state.session = Some(Session {
                schema_version: 1,
                profile: "local".to_owned(),
                actor: SessionActor {
                    tf_id: tf.id,
                    subject: format!("local:{}", tf.display_name),
                },
                scope: SessionScope {
                    insulator_id: insulator.id,
                    genome_id: genome.id,
                    grn_id: grn.id,
                },
                issued_by: SessionIssuer::EpigeneticsLocal,
                issued_at: SystemTime::now(),
                expires_at: None,
            });
            Ok(format!(
                "using insulator `{}`, genome `{}`, grn `{}`, tf `{}`",
                insulator.name, genome.name, grn.name, tf.display_name
            ))
        }
        "define-family" => {
            let session = current_session(state)?;
            let abbreviation = positional(args, 1, "gene family abbreviation")?;
            let name = positional(args, 2, "gene family name")?;
            let normalized_artifact = option_value(args, "--artifact")
                .map(|value| parse_normalized_artifact(&value))
                .transpose()?
                .unwrap_or(NormalizedArtifact::ManagedRequirement);
            let sequence_names = repeated_option(args, "--sequence");
            if sequence_names.is_empty() {
                return Err(CliError::Usage(
                    "define-family requires at least one --sequence".to_owned(),
                ));
            }
            let defined = state.dnap.define_gene_family(DefineGeneFamily {
                insulator_id: session.scope.insulator_id,
                genome_id: Some(session.scope.genome_id),
                name,
                abbreviation,
                normalized_artifact: Some(normalized_artifact),
                sequences: sequence_names
                    .into_iter()
                    .map(|name| DefineSequence {
                        name,
                        sequence_type: SequenceType::String,
                    })
                    .collect(),
                created_by: session.actor.tf_id,
            })?;
            Ok(format!(
                "defined gene family `{}` ({})",
                defined.family.name, defined.family.abbreviation
            ))
        }
        _ => Err(CliError::Usage(format!(
            "unknown epigenetics subcommand `{command}`"
        ))),
    }
}

fn mutate(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    if args.first().map(String::as_str) == Some("--new") {
        let family = positional(args, 1, "gene family abbreviation")?;
        let locus_name = positional(args, 2, "document name")?;
        let mutations = parse_sequence_mutations(&args[3..])?;
        let causes = repeated_option(args, "--because");
        let mutated = state.dnap.mutate_new(MutateNew {
            insulator_id: session.scope.insulator_id,
            genome_id: session.scope.genome_id,
            grn_id: session.scope.grn_id,
            gene_family_abbreviation: family,
            locus_name,
            mutations,
            causes,
            created_by: session.actor.tf_id,
        })?;
        return Ok(format!(
            "mutated new `{}` with {} mutation(s)",
            mutated.gene_fqn,
            mutated.mutations.len()
        ));
    }

    let gene_fqn = positional(args, 0, "gene fqn")?;
    let mutations = parse_sequence_mutations(&args[1..])?;
    let causes = repeated_option(args, "--because");
    let mutated = state.dnap.mutate_existing(MutateExisting {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        grn_id: session.scope.grn_id,
        gene_fqn,
        mutations,
        causes,
        created_by: session.actor.tf_id,
    })?;
    Ok(format!(
        "mutated `{}` with {} mutation(s)",
        mutated.gene_fqn,
        mutated.mutations.len()
    ))
}

fn transcribe(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let gene_fqn = positional(args, 0, "gene fqn")?;
    let full = args.iter().any(|arg| arg == "--full");
    let transcribed = state.dnap.transcribe(TranscribeAllele {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        grn_id: session.scope.grn_id,
        gene_fqn,
        full,
        created_by: session.actor.tf_id,
    })?;

    let mut output = if full {
        format!(
            "transcribed {:?}; showing full transcript",
            transcribed.allele.state
        )
    } else if transcribed.sequences.is_empty() {
        format!(
            "transcribed {:?}; no new Sequence changes since last transcription",
            transcribed.allele.state
        )
    } else {
        format!(
            "transcribed {:?}; showing Sequence changes since last transcription",
            transcribed.allele.state
        )
    };
    for sequence in transcribed.sequences {
        output.push_str(&format!(
            "\n{}: {}",
            sequence.name,
            display_value(&sequence.value)
        ));
        output.push_str("\n  approval: visible");
    }
    Ok(output)
}

fn splice(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let gene_fqn = positional(args, 0, "gene fqn")?;
    let lgtm = args.iter().any(|arg| arg == "--lgtm");
    let exon_texts = args[1..]
        .iter()
        .filter(|arg| !arg.starts_with("--"))
        .cloned()
        .collect::<Vec<_>>();
    if exon_texts.is_empty() && !lgtm {
        return Err(CliError::Usage(
            "splice requires at least one Exon text or --lgtm".to_owned(),
        ));
    }
    let spliced = state.dnap.splice(SpliceAllele {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        grn_id: session.scope.grn_id,
        gene_fqn,
        exon_texts,
        lgtm,
        created_by: session.actor.tf_id,
    })?;
    Ok(format!(
        "spliced {:?} with {} new exon(s); {} untranscribed unexpressed mutation(s)",
        spliced.allele.state,
        spliced.exons.len(),
        spliced.untranscribed_unexpressed_mutations
    ))
}

fn translate(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let gene_fqn = positional(args, 0, "gene fqn")?;
    let translated = state.dnap.translate(TranslateAllele {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        grn_id: session.scope.grn_id,
        gene_fqn,
        created_by: session.actor.tf_id,
    })?;

    let mut output = format!("translated {:?}", translated.allele.state);
    for (index, exon) in translated.exons.iter().enumerate() {
        output.push_str(&format!("\n{}. {}", index + 1, exon.text));
        let dependencies = exon
            .depends_on
            .iter()
            .filter_map(|dependency_id| {
                translated
                    .exons
                    .iter()
                    .find(|candidate| candidate.id == *dependency_id)
            })
            .map(|dependency| dependency.text.as_str())
            .collect::<Vec<_>>();
        if !dependencies.is_empty() {
            output.push_str(&format!("\n   depends on: {}", dependencies.join(", ")));
        }
    }
    Ok(output)
}

fn explore(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(CliError::Usage("expected explore subcommand".to_owned()));
    };

    match command {
        "enhancer" => explore_enhancer(state, args),
        _ => Err(CliError::Usage(format!(
            "unknown explore subcommand `{command}`"
        ))),
    }
}

fn explore_enhancer(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let enhancer_gene_fqn = positional(args, 1, "enhancer gene fqn")?;
    let promoter_gene_fqn = required_option(args, "--promoter")?;
    state
        .dnap
        .attach_enhancer_promoter(AttachEnhancerPromoter {
            insulator_id: session.scope.insulator_id,
            genome_id: session.scope.genome_id,
            grn_id: session.scope.grn_id,
            enhancer_gene_fqn: enhancer_gene_fqn.clone(),
            promoter_gene_fqn: promoter_gene_fqn.clone(),
            updated_by: session.actor.tf_id,
        })?;

    Ok(format!(
        "attached enhancer `{enhancer_gene_fqn}` to promoter `{promoter_gene_fqn}`"
    ))
}

fn question(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let target_mrna_fqn = positional(args, 0, "mRNA")?;
    let all = args.iter().any(|arg| arg == "--all");
    let sequence_name = args
        .iter()
        .skip(1)
        .find(|arg| arg.starts_with("--") && *arg != "--all")
        .map(|arg| arg.trim_start_matches("--").to_owned());
    let positional_values = args
        .iter()
        .skip(1)
        .filter(|arg| !arg.starts_with("--"))
        .cloned()
        .collect::<Vec<_>>();

    if positional_values.is_empty() {
        let summaries = state.dnap.intron_summaries_for(
            session.scope.insulator_id,
            session.scope.genome_id,
            session.scope.grn_id,
            session.actor.tf_id,
            &target_mrna_fqn,
            sequence_name.as_deref(),
        )?;
        let mut output = "questions".to_owned();
        for summary in summaries {
            output.push_str(&format!("\n{}", format_intron_summary(&summary)));
            if all {
                let thread = state.dnap.intron_thread_by_id(summary.intron.id)?;
                output.push_str(&format_intron_children_recursive(&state.dnap, &thread, 1)?);
            }
        }
        return Ok(output);
    }

    let title = positional_values[0].clone();
    let body = positional_values.get(1).cloned();
    let intron = state.dnap.create_intron(CreateIntron {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        grn_id: session.scope.grn_id,
        target_mrna_fqn: target_mrna_fqn.clone(),
        target_sequence_name: sequence_name,
        title,
        body,
        precursor: None,
        created_by: session.actor.tf_id,
    })?;

    Ok(format!("asked `{}` for `{target_mrna_fqn}`", intron.title))
}

fn answer(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let all = args.iter().any(|arg| arg == "--all");
    let question_index = args
        .iter()
        .position(|arg| arg.starts_with("--") && arg != "--all")
        .ok_or_else(|| CliError::Usage("missing question flag".to_owned()))?;
    let target_mrna_fqn = (question_index > 0).then(|| args[0].clone());
    let question_title = args[question_index].trim_start_matches("--").to_owned();
    let mut answer_body = None;
    let mut follow_up_title = None;
    let mut follow_up_body = None;
    let mut index = question_index + 1;
    if let Some(value) = args.get(index) {
        if value != "-q" && !value.starts_with("--") {
            answer_body = Some(value.clone());
            index += 1;
        }
    }
    if args.get(index).map(String::as_str) == Some("-q") {
        follow_up_title = args.get(index + 1).cloned();
        follow_up_body = args
            .get(index + 2)
            .filter(|value| !value.starts_with("--"))
            .cloned();
    }

    if answer_body.is_none() && follow_up_title.is_none() {
        let thread = state.dnap.intron_thread(
            session.scope.insulator_id,
            session.scope.genome_id,
            session.scope.grn_id,
            session.actor.tf_id,
            &question_title,
            target_mrna_fqn.as_deref().map(|target| (target, None)),
        )?;
        return format_intron_thread(&state.dnap, &thread, all);
    }

    let answered = state.dnap.append_intron_sequence(AppendIntronSequence {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        grn_id: session.scope.grn_id,
        target_mrna_fqn,
        target_sequence_name: None,
        intron_title: question_title,
        body: answer_body,
        follow_up_title,
        follow_up_body,
        created_by: session.actor.tf_id,
    })?;

    let mut output = format!("answered `{}`", answered.intron.title);
    if let Some(follow_up) = answered.follow_up {
        output.push_str(&format!("\nasked follow-up `{}`", follow_up.title));
    }
    Ok(output)
}

fn parse_sequence_mutations(args: &[String]) -> Result<Vec<SequenceMutation>, CliError> {
    let mut mutations = Vec::new();
    let mut index = 0;
    while index < args.len() {
        let flag = &args[index];
        if flag == "--because" {
            index += 2;
            continue;
        }
        if !flag.starts_with("--") {
            return Err(CliError::Usage(format!(
                "expected sequence flag, got `{flag}`"
            )));
        }
        let Some(value) = args.get(index + 1) else {
            return Err(CliError::Usage(format!("missing value for `{flag}`")));
        };
        if value.starts_with("--") {
            return Err(CliError::Usage(format!("missing value for `{flag}`")));
        }
        mutations.push(SequenceMutation {
            sequence_name: flag.trim_start_matches("--").to_owned(),
            value: SequenceValue::String(value.clone()),
        });
        index += 2;
    }
    Ok(mutations)
}

fn parse_normalized_artifact(value: &str) -> Result<NormalizedArtifact, CliError> {
    match normalize(value).as_str() {
        "promoter" => Ok(NormalizedArtifact::Promoter),
        "pam" | "problemassertionmanifest" => Ok(NormalizedArtifact::ProblemAssertionManifest),
        "erna" | "executable" => Ok(NormalizedArtifact::Executable),
        "ribozyme" => Ok(NormalizedArtifact::Ribozyme),
        "pirna" | "projectedintent" => Ok(NormalizedArtifact::ProjectedIntent),
        "spacer" | "spacers" => Ok(NormalizedArtifact::Spacer),
        "protospacer" => Ok(NormalizedArtifact::Protospacer),
        "phenotype" => Ok(NormalizedArtifact::Phenotype),
        "enhancer" | "enterprisenegotiationhandovercertificate" => {
            Ok(NormalizedArtifact::EnterpriseNegotiationHandoverCertificate)
        }
        "silencer" => Ok(NormalizedArtifact::Silencer),
        "snorna" | "strategicnote" => Ok(NormalizedArtifact::StrategicNote),
        "snrna" | "semanticnarrowing" => Ok(NormalizedArtifact::SemanticNarrowing),
        "scarna" | "semanticconstraintassumption" => {
            Ok(NormalizedArtifact::SemanticConstraintAssumption)
        }
        "mirna" | "microalignment" => Ok(NormalizedArtifact::Microalignment),
        "sirna" | "stopimplementation" => Ok(NormalizedArtifact::StopImplementation),
        "dsrna" | "deferredscope" => Ok(NormalizedArtifact::DeferredScope),
        "intron" => Ok(NormalizedArtifact::Intron),
        "mrna" | "managedrequirement" => Ok(NormalizedArtifact::ManagedRequirement),
        "exon" => Ok(NormalizedArtifact::Exon),
        "rrna" | "resourcereference" => Ok(NormalizedArtifact::ResourceReference),
        "trna" | "taskrealization" => Ok(NormalizedArtifact::TaskRealization),
        "trf" | "taskrealizationframework" => Ok(NormalizedArtifact::TaskRealizationFramework),
        "terc" | "testregressioncriteria" => Ok(NormalizedArtifact::TestRegressionCriteria),
        "telomere" | "testobjectivemanifest" => Ok(NormalizedArtifact::TestObjectiveManifest),
        "testorchestrationmanifest" => Ok(NormalizedArtifact::TestOrchestrationManifest),
        "centralruntimemanifest" => Ok(NormalizedArtifact::CentralRuntimeManifest),
        "cas" | "countermeasureassessmentsystem" => {
            Ok(NormalizedArtifact::CountermeasureAssessmentSystem)
        }
        "protein" | "productiontestedimplementation" => {
            Ok(NormalizedArtifact::ProductionTestedImplementation)
        }
        "chaperone" => Ok(NormalizedArtifact::Chaperone),
        "tmrna" | "taskmediation" => Ok(NormalizedArtifact::TaskMediation),
        "crrna" | "causalresolution" => Ok(NormalizedArtifact::CausalResolution),
        "tracrrna" | "tracereport" => Ok(NormalizedArtifact::TraceReport),
        "lncrna" | "longnarrativecontext" => Ok(NormalizedArtifact::LongNarrativeContext),
        "circrna" | "circularinstitutionalreferencecontext" => {
            Ok(NormalizedArtifact::CircularInstitutionalReferenceContext)
        }
        "sgrna" | "suggestedchanges" => Ok(NormalizedArtifact::SuggestedChanges),
        _ => Err(CliError::Usage(format!(
            "unsupported normalized artifact `{value}`"
        ))),
    }
}

fn current_session(state: &LocalState) -> Result<Session, CliError> {
    state
        .session
        .clone()
        .ok_or(CliError::Session(SessionError::MissingSession))
}

fn positional(args: &[String], index: usize, name: &str) -> Result<String, CliError> {
    args.get(index)
        .filter(|value| !value.starts_with("--"))
        .cloned()
        .ok_or_else(|| CliError::Usage(format!("missing {name}")))
}

fn required_option(args: &[String], flag: &str) -> Result<String, CliError> {
    option_value(args, flag).ok_or_else(|| CliError::Usage(format!("missing {flag}")))
}

fn option_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].clone())
}

fn repeated_option(args: &[String], flag: &str) -> Vec<String> {
    args.windows(2)
        .filter(|window| window[0] == flag)
        .map(|window| window[1].clone())
        .collect()
}

fn display_value(value: &SequenceValue) -> String {
    match value {
        SequenceValue::String(value) => value.clone(),
        SequenceValue::StringVec(value) => value.join(", "),
        SequenceValue::Int(value) => value.to_string(),
        SequenceValue::IntVec(value) => format!("{value:?}"),
        SequenceValue::Float(value) => value.to_string(),
        SequenceValue::FloatVec(value) => format!("{value:?}"),
        SequenceValue::Bool(value) => value.to_string(),
        SequenceValue::BoolVec(value) => format!("{value:?}"),
        SequenceValue::GeneRef(value) => format!("{value:?}"),
        SequenceValue::GeneRefVec(value) => format!("{value:?}"),
    }
}

fn format_intron_summary(summary: &IntronSummary) -> String {
    let mut line = summary.intron.title.clone();
    if let Some(sequence) = &summary.latest_sequence {
        line.push_str(&format!(" -> {}", sequence.body));
    }
    if summary.has_precursor {
        line.push_str(" [has precursor]");
    }
    if summary.child_count > 0 {
        line.push_str(&format!(" [{} follow-up(s)]", summary.child_count));
    }
    line
}

fn format_intron_thread(dnap: &Dnap, thread: &IntronThread, all: bool) -> Result<String, CliError> {
    let mut output = format!("question: {}", thread.intron.title);
    if let Some(body) = &thread.intron.body {
        output.push_str(&format!("\nbody: {body}"));
    }
    for precursor in &thread.precursors {
        output.push_str(&format!(
            "\nprecursor: {}",
            format_intron_summary(precursor)
        ));
    }
    for sequence in &thread.sequences {
        output.push_str(&format!("\nanswer: {}", sequence.body));
    }
    for child in &thread.children {
        output.push_str(&format!("\nfollow-up: {}", format_intron_summary(child)));
        if all {
            let child_thread = dnap.intron_thread_by_id(child.intron.id)?;
            output.push_str(&format_intron_children_recursive(dnap, &child_thread, 1)?);
        }
    }
    Ok(output)
}

fn format_intron_children_recursive(
    dnap: &Dnap,
    thread: &IntronThread,
    depth: usize,
) -> Result<String, CliError> {
    let indent = "  ".repeat(depth);
    let mut output = String::new();
    for sequence in &thread.sequences {
        output.push_str(&format!("\n{indent}answer: {}", sequence.body));
    }
    for child in &thread.children {
        output.push_str(&format!(
            "\n{indent}follow-up: {}",
            format_intron_summary(child)
        ));
        let child_thread = dnap.intron_thread_by_id(child.intron.id)?;
        output.push_str(&format_intron_children_recursive(
            dnap,
            &child_thread,
            depth + 1,
        )?);
    }
    Ok(output)
}

fn normalize(value: &str) -> String {
    value
        .trim()
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect()
}

#[derive(Debug)]
pub enum CliError {
    Usage(String),
    NotFound(String),
    Dnap(DnapError),
    Session(SessionError),
}

impl From<DnapError> for CliError {
    fn from(error: DnapError) -> Self {
        CliError::Dnap(error)
    }
}

impl From<SessionError> for CliError {
    fn from(error: SessionError) -> Self {
        CliError::Session(error)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Usage(message) => write!(formatter, "{message}"),
            CliError::NotFound(message) => write!(formatter, "not found: {message}"),
            CliError::Dnap(error) => write!(formatter, "{error:?}"),
            CliError::Session(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for CliError {}

#[cfg(test)]
mod tests;
