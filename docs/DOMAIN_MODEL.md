# DNAp Domain Model

## Boundaries

`Insulator` is the customer/account tenant and hard isolation boundary.

```rust
struct Insulator {
    id: InsulatorId,
    name: String,
    created_at: Timestamp,
    updated_at: Timestamp,
}
```

`InsulatorPlacement` records operational infrastructure/storage placement for an Insulator. Placement is provisioning configuration, not a DNAp document schema.

```rust
struct InsulatorPlacement {
    insulator_id: InsulatorId,
    strategy: InsulatorPlacementStrategy,
    region: String,
    active: bool,
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum InsulatorPlacementStrategy {
    SharedCluster,
}
```

`Genome` is the project boundary.

```rust
struct Genome {
    id: GenomeId,
    insulator_id: InsulatorId,
    name: String,
    created_at: Timestamp,
    updated_at: Timestamp,
}
```

`Tf` is user identity inside an Insulator. `display_name` is a tenant-facing label, not login identity.

```rust
struct Tf {
    id: TfId,
    insulator_id: InsulatorId,
    display_name: String,
    external_subject: Option<String>,
    identity_provider: Option<String>,
    created_at: Timestamp,
    updated_at: Timestamp,
}
```

Agents and service actors will likely use the same Tf identity model so audit, Histones, approvals, and workflow provenance stay unified. Do not add `TfKind`, delegation fields, or a separate actor taxonomy until concrete non-human actor use cases define the needed configurable structures, likely through Histone-backed authorization and context.

## Configurable Document Types

`GeneFamily` defines a configurable SDLC document/work type.

```rust
struct GeneFamily {
    id: GeneFamilyId,
    insulator_id: InsulatorId,
    genome_id: Option<GenomeId>,
    name: String,
    abbreviation: String,
    current_generation_id: GeneFamilyGenerationId,
    normalized_artifact: NormalizedArtifact,
    created_at: Timestamp,
    updated_at: Timestamp,
}
```

`NormalizedArtifact` is the system-fixed artifact taxonomy for GeneFamilies. It replaces the older `EncodingType::RNA(...)` and `EncodingType::GRN(...)` split as the canonical product model. All `NormalizedArtifact` variants are Gene-capable artifact types.

NormalizedArtifact enum variants use full enterprise semantic names. Biology-inspired names and backronyms may be used for internal typed artifact-reference wrappers where the role matters.

```rust
struct ArtifactRef {
    locus_id: LocusId,
    normalized_artifact: NormalizedArtifact,
}

struct MRna(ArtifactRef);
struct Ribozyme(ArtifactRef);
struct Enhancer(ArtifactRef);
```

`ArtifactRef` is constructed only through resolvers that verify the referenced Locus belongs to a GeneFamily with the expected NormalizedArtifact. Raw `LocusId` remains storage identity; domain relationships should use `ArtifactRef` or a wrapper when a specific artifact type is required.

`GeneFamilyGeneration` is an immutable schema version for a GeneFamily.

```rust
struct GeneFamilyGeneration {
    id: GeneFamilyGenerationId,
    family_id: GeneFamilyId,
    generation: u32,
    sequences: Vec<SequenceDefinition>,
    created_at: Timestamp,
}
```

`SequenceDefinition` defines one required field in a GeneFamilyGeneration.

```rust
struct SequenceDefinition {
    id: SequenceDefinitionId,
    name: String,
    sequence_type: SequenceType,
}

enum SequenceType {
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
```

All SequenceDefinitions in a GeneFamilyGeneration are required before commit. Empty vectors are missing for commit completeness. SequenceDefinition names are enterprise-native tenant data and unique inside a GeneFamilyGeneration.

`SequenceType::Gene` and `SequenceType::GeneVec` are for embedded and linked Genes. A referenced Gene may be a full document such as a PRD or a controlled document item such as an FRS.

CLI input for `Gene` and `GeneVec` sequence values uses Gene FQNs and resolves them to `GeneId` internally.

GeneFamily definitions may be Insulator-scoped or Genome-scoped. Genome-scoped GeneFamilies are project-level overrides. Lookup from a Genome context resolves Genome-scoped override first, then Insulator-scoped default.

GeneFamily abbreviations are unique in effective scope:

- Insulator-scoped abbreviations are unique within the Insulator.
- Genome-scoped abbreviations are unique within the Genome.
- Genome-scoped GeneFamilies may shadow Insulator-scoped abbreviations.

## Gene FQN Presentation

Gene fully qualified name presentation is configurable at Insulator scope and overridable at Genome scope.

Configuration controls:

- case,
- component order,
- version/generation formatting semantics.

Default presentation:

```text
<gene-family-abbreviation>-<locus-name-slug>-<generation>
```

Example:

```text
FRS-checkout-0001
```

CLI matching for Gene FQNs is insensitive to presentation case. Presentation is configurable; command matching is forgiving.

```rust
struct GeneFqnFormat {
    case: GeneFqnCase,
    order: Vec<GeneFqnComponent>,
    version: GeneFqnVersionFormat,
}

enum GeneFqnCase {
    KebabLower,
    KebabUpper,
    Preserve,
}

enum GeneFqnComponent {
    GeneFamilyAbbreviation,
    LocusName,
    Generation,
}

enum GeneFqnVersionFormat {
    ZeroPadded { width: u8 },
}
```

## Canonical Scopes

`Chromosome` is a named canonical scope inside a Genome.

```rust
struct Chromosome {
    id: ChromosomeId,
    genome_id: GenomeId,
    name: String,
    created_at: Timestamp,
    updated_at: Timestamp,
    degraded_at: Option<Timestamp>,
}
```

`Chromosome.name` is unique within a Genome under DNAp canonical name matching. Chromosome has no type/kind in the stable foundation.

## Document Instances

`Locus` is the stable identity for one controlled document or controlled document item. It anchors committed versions for one document instance and belongs to a Chromosome.

```rust
struct Locus {
    id: LocusId,
    chromosome_id: ChromosomeId,
    family_id: GeneFamilyId,
    name: String,
    activator: TfId,
    created_at: Timestamp,
    updated_at: Timestamp,
    degraded_at: Option<Timestamp>,
}
```

`Locus.name` is the document instance name used for identity and Gene fully qualified names. It is not a Sequence value.

Invariants:

- A Locus belongs to exactly one current Chromosome.
- A Locus may move between Chromosomes.
- Locus movement is a domain transition and must be recorded through Signal.
- `Locus.name` is unique within the containing Genome under DNAp canonical name matching.
- Implementations may use derived lookup keys or indexes to enforce name matching and uniqueness. Those keys are not domain fields.
- `Locus.activator` is the accountable owner of the controlled artifact line.

## Active Work Context

`Grn` is the active initiative/work context inside a Genome. `Genome` remains the project boundary; `Grn` organizes active SDLC work within that project.

```rust
struct Grn {
    id: GrnId,
    genome_id: GenomeId,
    name: String,
    activator: TfId,
    state: GrnState,
    operons: Vec<OperonId>,
    created_at: Timestamp,
    updated_at: Timestamp,
    degraded_at: Option<Timestamp>,
}

enum GrnState {
    Triage,
    Active,
    Blocked,
    Closed,
}
```

`Operon` groups Promoters within one GRN. Operons represent higher-level intake groupings such as epics.

```rust
struct Operon {
    id: OperonId,
    grn_id: GrnId,
    name: String,
    created_at: Timestamp,
    updated_at: Timestamp,
    degraded_at: Option<Timestamp>,
}

struct OperonPromoter {
    operon_id: OperonId,
    promoter: Promoter,
    triage_tf: Option<TfId>,
    degraded_at: Option<Timestamp>,
}
```

`Promoter` is an internal typed artifact-reference wrapper around `ArtifactRef`. It references a Locus whose GeneFamily has `NormalizedArtifact::Promoter`.

```rust
struct Promoter(ArtifactRef);
```

Invariants:

- A GRN belongs to exactly one Genome.
- `Grn.activator` is the accountable owner of the change context.
- GRN owns operational lifecycle state through `GrnState`.
- GRN exists from triage onward. It is the work container before and after activation.
- A GRN may work across multiple Chromosomes through its Alleles.
- Multiple GRNs may touch the same Chromosome or Locus. Conflict detection/resolution is deferred.
- `GrnState` is not a rule engine; detailed readiness, authorization, dependency, and transition gates are derived from artifacts and future configurable workflow policy.
- An Operon belongs to exactly one GRN.
- Operon does not own lifecycle state. GRN owns operational lifecycle state.
- Operon-specific readiness or blockage is derived from Promoter memberships, artifacts, dependencies, and future configurable workflow policy until concrete use cases prove independent Operon state is needed.
- A Promoter may be assigned to at most one active Operon.
- Intake triage assigns Promoter artifacts to one active Operon in one active GRN.
- Triage responsibility belongs to the Promoter-in-Operon membership, not directly to the Promoter artifact.
- `OperonPromoter.triage_tf` is accountability, not authorization. It does not grant permissions by itself.
- `GrnState::Triage` allows unassigned OperonPromoter membership.
- `GrnState::Active` requires every active OperonPromoter membership to have exactly one triage Tf.
- Only one active OperonPromoter may exist per Promoter.
- Cross-GRN relationships must be modeled explicitly as dependency, duplication, split, conflict, or another approved relationship, not by assigning the same Promoter to multiple active Operons.

PreInitiationComplex is a named discussion concept. It is not part of the stable domain foundation. No persisted PreInitiationComplex object, GRN field, participant model, topic model, or generic channel abstraction is approved until concrete use cases define it.

`Transposon` is the origin path for a new Gene/work item. It carries origin metadata only; Sequence values arrive through Mutations on the Allele.

```rust
struct Transposon {
    id: TransposonId,
    locus_id: LocusId,
    gene_family_generation_id: GeneFamilyGenerationId,
    degraded_at: Option<Timestamp>,
    created_at: Timestamp,
}
```

`Allele` is the mutable working version for a Locus.

```rust
enum AlleleOrigin {
    Gene(GeneId),
    Transposon(TransposonId),
}

struct Allele {
    id: AlleleId,
    grn_id: GrnId,
    locus_id: LocusId,
    origin: AlleleOrigin,
    state: AlleleState,
    degraded_at: Option<Timestamp>,
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum AlleleState {
    Mutating,
    Expressing,
    Selected,
}
```

One active shared Allele is allowed per `(Locus, GRN)`. Alleles are team-visible candidate work inside a GRN, not per-Tf private drafts.

`Mutation` is a Sequence value change on an Allele.

```rust
type SequenceKey = String;

struct Sequence {
    definition_id: SequenceDefinitionId,
    name: SequenceKey,
    value: SequenceValue,
}

enum SequenceValue {
    String(String),
    StringVec(Vec<String>),
    Int(i64),
    IntVec(Vec<i64>),
    Float(f64),
    FloatVec(Vec<f64>),
    Bool(bool),
    BoolVec(Vec<bool>),
    GeneRef(GeneId),
    GeneRefVec(Vec<GeneId>),
}

struct Mutation {
    id: MutationId,
    allele_id: AlleleId,
    sequence_definition_id: SequenceDefinitionId,
    value: SequenceValue,
    state: MutationState,
    degraded_at: Option<Timestamp>,
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum MutationState {
    Unexpressed,
    Expressing,
}
```

Mutations are individual and composable. Before expression, mutating the same Sequence updates the same `Unexpressed` Mutation row. `dna splice` expresses all current `Unexpressed` Mutations, changing them to `Expressing`. Mutating a Sequence that already has an `Expressing` Mutation creates a new `Unexpressed` Mutation for that Sequence.

`dna splice` moves the Allele to `Expressing`. `dna splice --lgtm` is an escape hatch: it expresses current `Unexpressed` Mutations without changing TaskRealizations when the current TaskRealization DAG is still acceptable.

`dna transcribe` is always allowed in every Allele state. It renders the latest Mutation projection for the current Allele against the committed Genes and candidate Alleles in the Chromosome of the Gene being worked on, including unapproved workflow-driven changes.

Approval-status comments for mutated and workflow-suggested Sequences are always shown. They are part of the transcript output, not an optional render mode.

`Transcriptome` is render/access cursor metadata for token-saving transcript output. It tracks what was last shown so later transcriptions can avoid re-outputting unchanged Sequences unless a full render flag is provided. It does not store the projected document content.

```rust
struct Transcriptome {
    id: TranscriptomeId,
    chromosome_id: ChromosomeId,
    allele_id: AlleleId,
    sequences: Vec<TranscriptSequenceCursor>,
    created_at: Timestamp,
    updated_at: Timestamp,
    degraded_at: Option<Timestamp>,
}

struct TranscriptSequenceCursor {
    sequence_definition_id: SequenceDefinitionId,
    last_rendered_mutation_id: Option<MutationId>,
    last_rendered_sequence_hash: Option<SequenceHash>,
}
```

`Gene` is an immutable committed version selected from an Allele.

```rust
struct Gene {
    id: GeneId,
    locus_id: LocusId,
    gene_family_generation_id: GeneFamilyGenerationId,
    generation: u32,
    sequences: Vec<Sequence>,
    selected_from: AlleleId,
    created_at: Timestamp,
}
```

`TaskRealization` is the tRNA task-realization artifact. The current `dna splice` implementation creates TaskRealization records attached to the working Allele. This is an implementation bridge, not the final Gene-capable TaskRealization lifecycle.

```rust
struct TaskRealization {
    id: TaskRealizationId,
    allele_id: AlleleId,
    text: String,
    depends_on: Vec<TaskRealizationId>,
    created_at: Timestamp,
    degraded_at: Option<Timestamp>,
}
```

TaskRealizations attached to an Allele organize as a DAG through `depends_on`, not a positional list. If TaskRealization A depends on TaskRealization B, B must precede A in the work graph.

`Intron` is a raw requirement artifact type. Fixed Intron discussion records are not part of the stable foundation. The long-term Intron-as-Gene lifecycle is unresolved until concrete requirement-discussion workflows are approved.

Workflow interactions among Intron, Ribozyme, TfComplex, CountermeasureAssessmentSystem, and related communication concepts are defined in [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md). Canonical term meanings are defined in [ONTOLOGY.md](ONTOLOGY.md).

## Signals And Degradation

Domain records store current business state and fields needed for normal queries.

`degraded_at` is the stable soft-delete/deactivation field for records that can become inactive while remaining auditable. Active queries filter on `degraded_at = None`. Degradation is not a business-state enum variant unless the workflow itself needs a degraded state.

Actor provenance, reasons, before/after details, and transition payloads belong in append-only Signals.

```rust
struct Signal {
    id: SignalId,
    insulator_id: InsulatorId,
    tf_id: Option<TfId>,
    signal_type: SignalType,
    target: SignalTarget,
    occurred_at: Timestamp,
    reason: Option<String>,
    payload: SignalPayload,
}

enum SignalTarget {
    Insulator(InsulatorId),
    Genome(GenomeId),
    Chromosome(ChromosomeId),
    Grn(GrnId),
    Operon(OperonId),
    Locus(LocusId),
    Allele(AlleleId),
    Gene(GeneId),
    Mutation(MutationId),
    Tf(TfId),
    Histone(HistoneId),
    HistoneMark(HistoneMarkId),
}
```

Invariants:

- Signal is append-only.
- Signal carries `insulator_id` directly for tenant-scoped audit export and filtering.
- `tf_id` is the Tf responsible for the event when a Tf exists.
- Signal payloads are typed in application code even if a storage adapter persists them as JSONB.
- Do not add `created_by`, `updated_by`, `assigned_by`, `moved_by`, or equivalent per-record audit fields unless a current product query needs that value without consulting Signal.
- Keep domain validity fields such as `HistoneMark.valid_from` and `valid_until` when the timestamps affect behavior, not merely audit.

## Authorization And Context

`Histone` and `HistoneMark` are the single authorization/contextual evaluation abstraction. Permissions do not exist independently from Histones.

```rust
struct TfClass {
    id: TfClassId,
    insulator_id: InsulatorId,
    genome_id: Option<GenomeId>,
    name: String,
    histones: Vec<HistoneMarkId>,
    created_at: Timestamp,
    updated_at: Timestamp,
}

struct Histone {
    id: HistoneId,
    insulator_id: InsulatorId,
    genome_id: Option<GenomeId>,
    name: String,
    key: String,
    value_type: HistoneValueType,
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum HistoneValueType {
    String,
    StringVec,
    Int,
    IntVec,
    Bool,
    BoolVec,
}

enum ChromatinState {
    Euchromatin,
    FacultativeHeterochromatin,
    Heterochromatin,
    ConstitutiveHeterochromatin,
}

struct HistoneMark {
    id: HistoneMarkId,
    histone_id: HistoneId,
    target: HistoneTarget,
    chromatin_state: ChromatinState,
    value: HistoneValue,
    rationale: Option<String>,
    valid_from: Timestamp,
    valid_until: Option<Timestamp>,
    degraded_at: Option<Timestamp>,
    created_at: Timestamp,
}

enum HistoneTarget {
    Tf(TfId),
    TfClass(TfClassId),
    Genome(GenomeId),
    Gene(GeneId),
    Allele(AlleleId),
    Exon(ExonId),
}
```

Histones may exist at Insulator or Genome scope. Genome Histones extend Insulator Histones. TfClass definitions are flat and non-inheritable; composition occurs through multiple classes on a Tf.

Only one active HistoneMark may exist per `(target, histone)` pair. Multi-valued attributes use vector value types instead of multiple active marks. More specific marks override broader inherited marks unless the broader mark is `ConstitutiveHeterochromatin`.

`ConstitutiveHeterochromatin` cannot be overridden by Tf, TfClass, candidate workflow, Mediator, or conditional evaluation. `FacultativeHeterochromatin` requires evaluator resolution. `Heterochromatin` and `ConstitutiveHeterochromatin` require rationale. Auditable records degrade rather than hard-delete.

## CLI Name Matching

CLI sequence-name matching is designed for ease of use:

- accepts kebab-case user input,
- is case-insensitive,
- supports fuzzy matching,
- must resolve to exactly one SequenceDefinition,
- returns an error for zero or multiple matches,
- lets the user disambiguate with more exact input.

## CLI Mutation Entry Point

`dna mutate` is the user-facing command for creating or changing an Allele.

Starting a new document uses `--new` with a GeneFamily abbreviation and a document name as the first positional argument:

```sh
dna mutate --new FRS 'Checkout' --some-section 'Awesome section'
```

In this form:

- `FRS` is resolved as a GeneFamily abbreviation in the current Genome context.
- `Checkout` is the Locus document instance name.
- Sequence mutation flags are optional.
- DNAp creates the Locus, Transposon, and first Allele in one operation. If Sequence mutation flags are present, it also creates or updates `Unexpressed` Mutations.

Mutating existing work omits `--new` and uses the Gene fully qualified name as the first positional argument:

```sh
dna mutate FRS-checkout-0001
```

Positional target matching is case-insensitive and kebab-insensitive, not fuzzy. The generation may be omitted when the matcher resolves exactly one active Allele.

Sequence values are provided through mutation flags. Sequence flag names use the approved sequence-name matcher.

Scalar sequence mutation flags use the sequence name directly:

```sh
--<sequence-name> <value>
```

`--set-<sequence-name>` is not scalar syntax.

Gene reference values use Gene FQNs:

```sh
dna mutate --new PRD 'Checkout PRD' --feature FRS-checkout-0001
```

The CLI resolves `FRS-checkout-0001` to a `GeneId` before storing the Mutation.

Vector sequence mutation flags use explicit add/set/remove operations. The flag shape is:

```sh
--add-<sequence-name> <value-1> ... <value-n>
--set-<sequence-name> <value-1> ... <value-n>
--set-<sequence-name>-<n> <value>
--remove-<sequence-name>-<n>
```

For a vector Sequence named `label`, examples are `--add-label`, `--set-label`, `--set-label-2`, and `--remove-label-2`. `label` is not a keyword; it is the matched Sequence name.

Semantics:

- `--add-<sequence-name>` appends one or more values.
- `--set-<sequence-name>` replaces the whole vector.
- `--set-<sequence-name>-<n>` replaces only index `n`.
- `--remove-<sequence-name>-<n>` removes index `n` and shifts following elements.
- CLI vector indexes are one-based because these indexes are document-facing.
- Indexed vector operations error when `n` is out of bounds.
- DNAp does not expose a command for emptying a sequence. Scalars are overwritten by setting a new value. Vectors are replaced with `--set-<sequence-name>` and one or more values.

Dynamic sequence-name matching still applies to the sequence-name part of these flags.

Plain `--<sequence-name> <value>` is invalid for vector sequences because append, replacement, and indexed edits must be explicit.

## CLI Splice Entry Point

`dna splice` creates or acknowledges TaskRealizations attached to the active mRNA Allele.

```sh
dna splice <mrna-gene> "Some hard task" "An even harder one"
dna splice <mrna-gene> "Buy soap" --before-go-laundry
dna splice <mrna-gene> --pick-laundry --after-go-laundry
dna splice <mrna-gene> --set-buy-soap "Buy hypoallergenic soap"
dna splice <mrna-gene> --lgtm
```

In this form:

- `<mrna-gene>` resolves the active mRNA Allele by Locus name or Gene FQN.
- Positional target matching is case-insensitive and kebab-insensitive, not fuzzy. The generation may be omitted when the matcher resolves exactly one active Allele.
- Quoted positional arguments create new TaskRealizations attached to that mRNA Allele.
- `--before-<task-realization-name>` places the new or selected TaskRealization before an existing TaskRealization by making the existing TaskRealization depend on it.
- `--after-<task-realization-name>` places the new or selected TaskRealization after an existing TaskRealization by making it depend on the existing TaskRealization.
- `--<task-realization-name>` selects an existing TaskRealization in the mRNA Allele's TaskRealization DAG.
- `--set-<task-realization-name> <text>` replaces the text of an existing TaskRealization.
- `--lgtm` is an escape hatch that expresses current `Unexpressed` Mutations without changing TaskRealizations when the existing TaskRealization DAG is still acceptable.
- TaskRealizations attached to the Allele organize as a DAG through `depends_on`.
- `dna splice` is not a mutation staging command.

After `dna splice`, the Allele remains an Allele with `state = Expressing`. `dna select` is the final command that creates the immutable Gene.
