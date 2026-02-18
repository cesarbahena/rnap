use std::ffi::OsString;
use std::fmt;
use std::time::SystemTime;

use crate::app::{
    AddExplorationEdge, AddExplorationNode, AttachEnhancerPromoter, CanonizeErna,
    CreateExplorationGraph, CreateGenome, CreateTf, DefineGeneFamily, DefineSequence, DnapError,
    EncodingType, ExplorationGraphId, ExplorationNodeId, GrnType, MutateExisting, MutateNew,
    ProvisionInsulator, RegulatoryRnaType, RnaType, SequenceMutation, SequenceType, SequenceValue,
    SpliceAllele, TranscribeAllele, TranslateAllele, TranslationRnaType,
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
        "graph" => explore_graph(state, args),
        "node" => explore_node(state, args),
        "edge" => explore_edge(state, args),
        "show" => explore_show(state, args),
        "enhancer" => explore_enhancer(state, args),
        "canonize" => explore_canonize(state, args),
        _ => Err(CliError::Usage(format!(
            "unknown explore subcommand `{command}`"
        ))),
    }
}

fn explore_graph(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let promoter_gene_fqn = positional(args, 1, "promoter gene fqn")?;
    let name = positional(args, 2, "graph name")?;
    let created = state
        .dnap
        .create_exploration_graph(CreateExplorationGraph {
            insulator_id: session.scope.insulator_id,
            genome_id: session.scope.genome_id,
            promoter_gene_fqn,
            name,
            created_by: session.actor.tf_id,
        })?;

    Ok(format!(
        "created exploration graph {} for `{}`",
        created.graph.id.raw(),
        created.promoter_locus.name
    ))
}

fn explore_node(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let graph_id = parse_graph_id(&positional(args, 1, "graph id")?)?;
    let erna_locus_name = positional(args, 2, "erna name")?;
    let erna_family_abbreviation = option_value(args, "--family");
    let label = option_value(args, "--label");
    let position_x = option_value(args, "--x")
        .map(|value| parse_i64(&value, "--x"))
        .transpose()?
        .unwrap_or(0);
    let position_y = option_value(args, "--y")
        .map(|value| parse_i64(&value, "--y"))
        .transpose()?
        .unwrap_or(0);
    let added = state.dnap.add_exploration_node(AddExplorationNode {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        graph_id,
        erna_locus_name,
        erna_family_abbreviation,
        label,
        position_x,
        position_y,
        created_by: session.actor.tf_id,
    })?;
    let created = if added.created_erna.is_some() {
        "created"
    } else {
        "reused"
    };

    Ok(format!(
        "added exploration node {} ({created} eRNA `{}`)",
        added.node.id.raw(),
        added.erna_locus.name
    ))
}

fn explore_edge(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let graph_id = parse_graph_id(&positional(args, 1, "graph id")?)?;
    let from_node_id = parse_node_id(&positional(args, 2, "from node id")?)?;
    let to_node_id = parse_node_id(&positional(args, 3, "to node id")?)?;
    let edge = state.dnap.add_exploration_edge(AddExplorationEdge {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        graph_id,
        from_node_id,
        to_node_id,
        label: option_value(args, "--label"),
        created_by: session.actor.tf_id,
    })?;

    Ok(format!("added exploration edge {}", edge.id.raw()))
}

fn explore_show(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let graph_id = parse_graph_id(&positional(args, 1, "graph id")?)?;
    let graph = state
        .dnap
        .exploration_graph(graph_id)
        .ok_or_else(|| CliError::NotFound(format!("exploration graph {}", graph_id.raw())))?;
    let mut output = format!("exploration graph {}: {}", graph.id.raw(), graph.name);
    for node in state.dnap.exploration_nodes(graph_id) {
        output.push_str(&format!(
            "\nnode {}: {} @ {},{}",
            node.id.raw(),
            node.label,
            node.position_x,
            node.position_y
        ));
    }
    for edge in state.dnap.exploration_edges(graph_id) {
        output.push_str(&format!(
            "\nedge {}: {} -> {}",
            edge.id.raw(),
            edge.from_node_id.raw(),
            edge.to_node_id.raw()
        ));
        if let Some(label) = &edge.label {
            output.push_str(&format!(" ({label})"));
        }
    }
    Ok(output)
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
            enhancer_gene_fqn: enhancer_gene_fqn.clone(),
            promoter_gene_fqn: promoter_gene_fqn.clone(),
            updated_by: session.actor.tf_id,
        })?;

    Ok(format!(
        "attached enhancer `{enhancer_gene_fqn}` to promoter `{promoter_gene_fqn}`"
    ))
}

fn explore_canonize(state: &mut LocalState, args: &[String]) -> Result<String, CliError> {
    let session = current_session(state)?;
    let source_erna_gene_fqn = positional(args, 1, "source erna gene fqn")?;
    let target_gene_family_abbreviation = required_option(args, "--family")?;
    let target_locus_name = positional(args, 2, "target document name")?;
    let canonized = state.dnap.canonize_erna(CanonizeErna {
        insulator_id: session.scope.insulator_id,
        genome_id: session.scope.genome_id,
        source_erna_gene_fqn,
        target_gene_family_abbreviation,
        target_locus_name,
        created_by: session.actor.tf_id,
    })?;

    Ok(format!(
        "canonized eRNA into `{}`",
        canonized.target.gene_fqn
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
        "erna" => Ok(EncodingType::RNA(RnaType::Translation(
            TranslationRnaType::ERNA,
        ))),
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
        "enhancer" => Ok(EncodingType::GRN(GrnType::Enhancer)),
        "piwi" => Ok(EncodingType::GRN(GrnType::PIWI)),
        "spacers" => Ok(EncodingType::GRN(GrnType::Spacers)),
        "telomere" => Ok(EncodingType::GRN(GrnType::Telomere)),
        "centromere" => Ok(EncodingType::GRN(GrnType::Centromere)),
        "silencer" => Ok(EncodingType::GRN(GrnType::Silencer)),
        "intron" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::Intron,
        ))),
        "snrna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::SnRNA,
        ))),
        "scarna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::ScaRNA,
        ))),
        "sirna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::SiRNA,
        ))),
        "tmrna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::TmRNA,
        ))),
        "grna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::GRNA,
        ))),
        "mirna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::MiRNA,
        ))),
        "pirna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::PiRNA,
        ))),
        "snorna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::SnoRNA,
        ))),
        "crrna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::CrRNA,
        ))),
        "tracrrna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::TracrRNA,
        ))),
        "lncrna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::LncRNA,
        ))),
        "circrna" => Ok(EncodingType::RNA(RnaType::Regulatory(
            RegulatoryRnaType::CircRNA,
        ))),
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

fn parse_graph_id(value: &str) -> Result<ExplorationGraphId, CliError> {
    value
        .parse::<u64>()
        .map(ExplorationGraphId::from_raw)
        .map_err(|_| CliError::Usage(format!("invalid graph id `{value}`")))
}

fn parse_node_id(value: &str) -> Result<ExplorationNodeId, CliError> {
    value
        .parse::<u64>()
        .map(ExplorationNodeId::from_raw)
        .map_err(|_| CliError::Usage(format!("invalid node id `{value}`")))
}

fn parse_i64(value: &str, flag: &str) -> Result<i64, CliError> {
    value
        .parse::<i64>()
        .map_err(|_| CliError::Usage(format!("invalid value for {flag}: `{value}`")))
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
        assert!(transcript.contains("showing Sequence changes since last transcription"));
        assert!(transcript.contains("Summary: Draft"));
        assert!(transcript.contains("approval: visible"));

        let full_transcript =
            dispatch(&mut state, words("transcribe FRS-checkout --full")).expect("full transcript");
        assert!(full_transcript.contains("showing full transcript"));
        assert!(full_transcript.contains("Summary: Draft"));

        let spliced =
            dispatch(&mut state, words("splice FRS-checkout BuildCheckout")).expect("splice");
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
        assert!(
            matches!(error, CliError::Usage(message) if message.contains("Exon text or --lgtm"))
        );
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

        let error = dispatch(&mut state, words("translate checkout"))
            .expect_err("translate requires exons");
        assert!(matches!(error, CliError::Dnap(DnapError::ExonsNotFound)));
    }

    #[test]
    fn explore_cli_creates_graph_nodes_and_edges() {
        let mut state = bootstrapped_state();
        dispatch(
            &mut state,
            words("epigenetics define-family STR Story --encoding promoter --sequence Summary"),
        )
        .expect("promoter family");
        dispatch(
            &mut state,
            words("epigenetics define-family EXP Exploration --encoding eRNA --sequence Summary"),
        )
        .expect("erna family");
        dispatch(&mut state, words("mutate --new STR Checkout")).expect("promoter");

        let graph = dispatch(
            &mut state,
            words("explore graph checkout CheckoutDiscovery"),
        )
        .expect("graph");
        assert!(graph.contains("created exploration graph 1"));

        let first = dispatch(
            &mut state,
            words("explore node 1 PaymentAuthorized --family EXP --x 10 --y 20"),
        )
        .expect("first node");
        assert!(first.contains("added exploration node 1"));
        assert!(first.contains("created eRNA"));

        let second = dispatch(
            &mut state,
            words("explore node 1 ReceiptSent --family EXP --label Receipt"),
        )
        .expect("second node");
        assert!(second.contains("added exploration node 2"));

        let edge = dispatch(&mut state, words("explore edge 1 1 2 --label emits")).expect("edge");
        assert!(edge.contains("added exploration edge 1"));

        let shown = dispatch(&mut state, words("explore show 1")).expect("show graph");
        assert!(shown.contains("exploration graph 1: CheckoutDiscovery"));
        assert!(shown.contains("node 1: PaymentAuthorized @ 10,20"));
        assert!(shown.contains("node 2: Receipt @ 0,0"));
        assert!(shown.contains("edge 1: 1 -> 2 (emits)"));
    }

    #[test]
    fn explore_cli_attaches_enhancer_to_promoter_property() {
        let mut state = bootstrapped_state();
        dispatch(
            &mut state,
            words("epigenetics define-family STR Story --encoding promoter --sequence Summary"),
        )
        .expect("promoter family");
        dispatch(
            &mut state,
            words("epigenetics define-family RSH Research --encoding enhancer --sequence Summary"),
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
    fn explore_cli_canonizes_erna_into_target_family() {
        let mut state = bootstrapped_state();
        dispatch(
            &mut state,
            words("epigenetics define-family EXP Exploration --encoding eRNA --sequence Summary"),
        )
        .expect("erna family");
        dispatch(
            &mut state,
            words("epigenetics define-family REQ Requirement --encoding mRNA --sequence Summary"),
        )
        .expect("requirement family");
        dispatch(&mut state, words("mutate --new EXP AccountRecoverySketch")).expect("erna");

        let output = dispatch(
            &mut state,
            words("explore canonize AccountRecoverySketch AccountRecoveryRequirement --family REQ"),
        )
        .expect("canonize");

        assert!(output.contains("canonized eRNA into `REQ-accountrecoveryrequirement-0001`"));
    }

    #[test]
    fn parses_current_encoding_taxonomy_aliases() {
        let cases = [
            ("promoter", EncodingType::GRN(GrnType::Promoter)),
            ("enhancer", EncodingType::GRN(GrnType::Enhancer)),
            ("piwi", EncodingType::GRN(GrnType::PIWI)),
            ("spacers", EncodingType::GRN(GrnType::Spacers)),
            ("telomere", EncodingType::GRN(GrnType::Telomere)),
            ("centromere", EncodingType::GRN(GrnType::Centromere)),
            ("silencer", EncodingType::GRN(GrnType::Silencer)),
            (
                "eRNA",
                EncodingType::RNA(RnaType::Translation(TranslationRnaType::ERNA)),
            ),
            (
                "mRNA",
                EncodingType::RNA(RnaType::Translation(TranslationRnaType::MRNA)),
            ),
            (
                "rRNA",
                EncodingType::RNA(RnaType::Translation(TranslationRnaType::RRNA)),
            ),
            (
                "tRNA",
                EncodingType::RNA(RnaType::Translation(TranslationRnaType::TRNA)),
            ),
            (
                "intron",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::Intron)),
            ),
            (
                "snRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::SnRNA)),
            ),
            (
                "scaRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::ScaRNA)),
            ),
            (
                "siRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::SiRNA)),
            ),
            (
                "tmRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::TmRNA)),
            ),
            (
                "gRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::GRNA)),
            ),
            (
                "miRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::MiRNA)),
            ),
            (
                "piRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::PiRNA)),
            ),
            (
                "snoRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::SnoRNA)),
            ),
            (
                "crRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::CrRNA)),
            ),
            (
                "tracrRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::TracrRNA)),
            ),
            (
                "lncRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::LncRNA)),
            ),
            (
                "circRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::CircRNA)),
            ),
            (
                "sgRNA",
                EncodingType::RNA(RnaType::Regulatory(RegulatoryRnaType::SgRNA)),
            ),
        ];

        for (alias, expected) in cases {
            assert_eq!(parse_encoding(alias).expect("supported encoding"), expected);
        }
    }

    #[test]
    fn rejects_unknown_encoding_aliases() {
        let error = parse_encoding("not-a-real-encoding").expect_err("unknown encoding");

        assert!(
            matches!(error, CliError::Usage(message) if message.contains("unsupported encoding"))
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
