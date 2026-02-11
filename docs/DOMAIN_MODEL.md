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
    encodes: EncodingType,
    created_at: Timestamp,
    updated_at: Timestamp,
}
```

`GeneFamilyGeneration` is an immutable schema version for a GeneFamily.

```rust
struct GeneFamilyGeneration {
    id: GeneFamilyGenerationId,
    family_id: GeneFamilyId,
    generation: u32,
    sequences: Vec<SequenceDefinition>,
    created_by: TfId,
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

## Document Instances

`Locus` is Genome-scoped document/work item identity. It anchors committed versions for one document instance.

```rust
struct Locus {
    id: LocusId,
    family_id: GeneFamilyId,
    insulator_id: InsulatorId,
    genome_id: GenomeId,
    name: String,
    created_at: Timestamp,
}
```

`Locus.name` is the document instance name used for identity and Gene fully qualified names. It is not a Sequence value.

`Transposon` is the origin path for a new Gene/work item. It carries origin metadata only; Sequence values arrive through Mutations on the Allele.

```rust
struct Transposon {
    id: TransposonId,
    genome_id: GenomeId,
    locus_id: LocusId,
    gene_family_generation_id: GeneFamilyGenerationId,
    created_by: TfId,
    degraded_at: Option<Timestamp>,
    degraded_by: Option<TfId>,
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
    genome_id: GenomeId,
    locus_id: LocusId,
    origin: AlleleOrigin,
    state: AlleleState,
    created_by: TfId,
    degraded_at: Option<Timestamp>,
    degraded_by: Option<TfId>,
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum AlleleState {
    Mutating,
    Expressing,
    Selected,
    Degraded,
}
```

One active Allele is allowed per `(Locus, Tf)`. Multiple Tfs may each have one active Allele for the same Locus.

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
    created_by: TfId,
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum MutationState {
    Unexpressed,
    Expressing,
}
```

Mutations are individual and composable. Before expression, mutating the same Sequence updates the same `Unexpressed` Mutation row. `dna splice` expresses all current `Unexpressed` Mutations, changing them to `Expressing`. Mutating a Sequence that already has an `Expressing` Mutation creates a new `Unexpressed` Mutation for that Sequence.

`dna splice` moves the Allele to `Expressing`. `dna splice --lgtm` is an escape hatch: it expresses current `Unexpressed` Mutations without changing Exons when the current Exon DAG is still acceptable.

`dna transcribe` is always allowed in every Allele state. It renders the latest Mutation projection for the current Allele against the committed Genes and candidate Alleles in the Chromosome of the Gene being worked on, including unapproved workflow-driven changes.

Approval-status comments for mutated and workflow-suggested Sequences are always shown. They are part of the transcript output, not an optional render mode.

`Transcriptome` is render/access cursor metadata for token-saving transcript output. It tracks what was last shown so later transcriptions can avoid re-outputting unchanged Sequences unless a full render flag is provided. It does not store the projected document content.

```rust
struct Chromosome {
    id: ChromosomeId,
    genome_id: GenomeId,
    locus_id: LocusId,
    genes: Vec<GeneId>,
    alleles: Vec<AlleleId>,
}

struct Transcriptome {
    id: TranscriptomeId,
    chromosome_id: ChromosomeId,
    allele_id: AlleleId,
    sequences: Vec<TranscriptSequenceCursor>,
    created_by: TfId,
    created_at: Timestamp,
    updated_at: Timestamp,
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
    genome_id: GenomeId,
    locus_id: LocusId,
    gene_family_generation_id: GeneFamilyGenerationId,
    generation: u32,
    sequences: Vec<Sequence>,
    selected_from: AlleleId,
    insulator_id: InsulatorId,
    created_at: Timestamp,
}
```

`Exon` is created by `dna splice` from an mRNA Allele. Exons are attached to the working Allele and represent executable tasks derived from that requirements analysis document.

`Exon` is not an `EncodingType`; it is a workflow/task object.

```rust
struct Exon {
    id: ExonId,
    allele_id: AlleleId,
    text: String,
    depends_on: Vec<ExonId>,
    created_by: TfId,
    created_at: Timestamp,
}
```

Exons attached to an Allele organize as a DAG through `depends_on`, not a positional list. If Exon A depends on Exon B, B must precede A in the work graph.

`Intron` is a Regulatory RNA encoding for chainable disambiguation items. Unlike Exons, Introns are controlled document items modeled through normal GeneFamily and GeneFamilyGeneration configuration.

Workflow interactions among Intron, eRNA, TfComplex, Cas, and related communication concepts are defined in [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md). Canonical term meanings are defined in [ONTOLOGY.md](ONTOLOGY.md).

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
    created_by: TfId,
    degraded_at: Option<Timestamp>,
    degraded_by: Option<TfId>,
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
- At least one Sequence mutation flag is required.
- DNAp creates the Locus, Transposon, first Allele, and initial Mutation records in one operation.
- Alleles are created by `dna mutate --new` only when the command actually mutates at least one Sequence.

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

`dna splice` creates or acknowledges Exons attached to the active mRNA Allele.

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
- Quoted positional arguments create new Exons attached to that mRNA Allele.
- `--before-<exon-name>` places the new or selected Exon before an existing Exon by making the existing Exon depend on it.
- `--after-<exon-name>` places the new or selected Exon after an existing Exon by making it depend on the existing Exon.
- `--<exon-name>` selects an existing Exon in the mRNA Allele's Exon DAG.
- `--set-<exon-name> <text>` replaces the text of an existing Exon.
- `--lgtm` is an escape hatch that expresses current `Unexpressed` Mutations without changing Exons when the existing Exon DAG is still acceptable.
- Exons attached to the Allele organize as a DAG through `depends_on`.
- `dna splice` is not a mutation staging command.

After `dna splice`, the Allele remains an Allele with `state = Expressing`. `dna select` is the final command that creates the immutable Gene.
