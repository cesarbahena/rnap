use std::ffi::OsString;
use std::fmt;
use std::time::SystemTime;

use crate::app::{
    CreateGenome, CreateTf, DefineGeneFamily, DefineSequence, DnapError, EncodingType, GrnType,
    MutateExisting, MutateNew, ProvisionInsulator, RegulatoryRnaType, RnaType, SequenceMutation,
    SequenceType, SequenceValue, SpliceAllele, TranscribeAllele, TranslationRnaType,
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
        "use" => {
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
                },
                issued_by: SessionIssuer::EpigeneticsLocal,
                issued_at: SystemTime::now(),
                expires_at: None,
            });
            Ok(format!(
                "using insulator `{}`, genome `{}`, tf `{}`",
                insulator.name, genome.name, tf.display_name
            ))
        }
        "define-family" => {
            let session = current_session(state)?;
            let abbreviation = positional(args, 1, "gene family abbreviation")?;
            let name = positional(args, 2, "gene family name")?;
            let encoding = option_value(args, "--encoding")
                .map(|value| parse_encoding(&value))
                .transpose()?
                .unwrap_or(EncodingType::RNA(RnaType::Translation(
                    TranslationRnaType::MRNA,
                )));
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
                encodes: Some(encoding),
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
        let mutated = state.dnap.mutate_new(MutateNew {
            insulator_id: session.scope.insulator_id,
            genome_id: session.scope.genome_id,
            gene_family_abbreviation: family,
            locus_name,
            mutations,
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
    let mutated = state.dnap.mutate_existing(MutateExisting {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        gene_fqn,
        mutations,
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
        gene_fqn,
        full,
        created_by: session.actor.tf_id,
    })?;

    let mut output = format!(
        "transcribed {:?}; approval comments shown",
        transcribed.allele.state
    );
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
    let spliced = state.dnap.splice(SpliceAllele {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        gene_fqn,
        exon_texts,
        lgtm,
        created_by: session.actor.tf_id,
    })?;
    Ok(format!(
        "spliced {:?} with {} new exon(s)",
        spliced.allele.state,
        spliced.exons.len()
    ))
}

fn parse_sequence_mutations(args: &[String]) -> Result<Vec<SequenceMutation>, CliError> {
    let mut mutations = Vec::new();
    let mut index = 0;
    while index < args.len() {
        let flag = &args[index];
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

fn parse_encoding(value: &str) -> Result<EncodingType, CliError> {
    match normalize(value).as_str() {
        "mrna" => Ok(EncodingType::RNA(RnaType::Translation(
            TranslationRnaType::MRNA,
        ))),
        "rrna" => Ok(EncodingType::RNA(RnaType::Translation(
            TranslationRnaType::RRNA,
        ))),
        "trna" => Ok(EncodingType::RNA(RnaType::Translation(
            TranslationRnaType::TRNA,
        ))),
        "sgrna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::SgRNA,
        ))),
        "promoter" => Ok(EncodingType::GRN(GrnType::Promoter)),
        "telomere" => Ok(EncodingType::GRN(GrnType::Telomere)),
        "centromere" => Ok(EncodingType::GRN(GrnType::Centromere)),
        "silencer" => Ok(EncodingType::GRN(GrnType::Silencer)),
        _ => Err(CliError::Usage(format!("unsupported encoding `{value}`"))),
    }
}

fn current_session(state: &LocalState) -> Result<Session, CliError> {
    state
        .session
        .clone()
        .ok_or_else(|| CliError::Session(SessionError::MissingSession))
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
mod tests {
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
            words("epigenetics use --insulator Acme --genome Billing --tf Cesar"),
        )
        .expect("use session");
        dispatch(
            &mut state,
            words(
                "epigenetics define-family FRS FeatureRequirements --encoding mRNA --sequence Summary --sequence Risk",
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
        assert!(transcript.contains("Summary: Draft"));
        assert!(transcript.contains("approval: visible"));

        let spliced =
            dispatch(&mut state, words("splice FRS-checkout BuildCheckout")).expect("splice");
        assert!(spliced.contains("Spliced"));
    }

    #[test]
    fn stale_splice_cli_flow_requires_transcribe_before_lgtm() {
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

        let blocked = dispatch(&mut state, words("splice FRS-checkout --lgtm"))
            .expect_err("lgtm blocked before transcribe");
        assert!(matches!(
            blocked,
            CliError::Dnap(DnapError::StaleSpliceRequiresTranscribe)
        ));

        let transcript =
            dispatch(&mut state, words("transcribe FRS-checkout")).expect("stale transcript");
        assert!(transcript.contains("StaleTranscript"));

        let spliced = dispatch(&mut state, words("splice FRS-checkout --lgtm"))
            .expect("lgtm after transcript");
        assert!(spliced.contains("Spliced"));
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
            words("epigenetics use --insulator Acme --genome Billing --tf Cesar"),
        )
        .expect("use session");
        dispatch(
            &mut state,
            words(
                "epigenetics define-family FRS FeatureRequirements --encoding mRNA --sequence Summary --sequence Risk",
            ),
        )
        .expect("define family");
        state
    }

    fn words(input: &str) -> Vec<String> {
        input.split_whitespace().map(str::to_owned).collect()
    }
}
