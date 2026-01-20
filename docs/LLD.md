
# DNAp Core Domain Specification (Recovered Working Draft)

## Platform Boundaries

```rust
struct Insulator {
    id: InsulatorId,

    name: String,

    gene_families: Vec<GeneFamilyId>,
    histones: Vec<HistoneId>,
    tf_classes: Vec<TfClassId>,

    created_at: Timestamp,
    updated_at: Timestamp,
}

struct Genome {
    id: GenomeId,

    insulator: InsulatorId,

    name: String,

    cells: Vec<CellId>,
    gene_families: Vec<GeneFamilyId>,
    tf_classes: Vec<TfClassId>,
    histones: Vec<HistoneId>,

    created_at: Timestamp,
    updated_at: Timestamp,
}

struct Cell {
    id: CellId,

    insulator: InsulatorId,
    genome_id: Option<GenomeId>,

    name: String,

    created_at: Timestamp,
    updated_at: Timestamp,
}
```

## Identity and Access Management

```rust
struct TF {
    id: TfId,

    insulator: InsulatorId,

    tf_classes: Vec<TfClassId>,
    histones: Vec<HistoneMarkId>,

    affinity: Vec<BindingAffinityId>,

    complex: TfComplex,

    created_at: Timestamp,
    updated_at: Timestamp,
}

struct TfComplex {
    pre_initiation_complex: Vec<PreInitiators>,
    repressor_complex: Vec<Repressors>,
    unfolded_protein_response: Vec<Upr>,
    enhancers: Vec<Enhancers>,
    mediator_complex: Vec<Mediators>,
}

enum PreInitiators {
    SnRna(SnRnaId),
    SgRna(SgRnaId),
    GRna(GRnaId),
}

enum Repressors {
    SiRna(SiRnaId),
    GRna(GRnaId),
}

enum Upr {
    Chaperone(ChaperoneId),
    GRna(GRnaId),
}

enum Enhancers {
    ERna(ERnaId),
    GRna(GRnaId),
}

enum Mediators {
    Mediator(MediatorId),
    GRna(GRnaId),
}

struct TfClass {
    id: TfClassId,

    insulator: InsulatorId,
    genome_id: Option<GenomeId>,

    name: String,

    histones: Vec<HistoneMarkId>,

    created_at: Timestamp,
    updated_at: Timestamp,
}
```

## Histones

```rust
struct Histone {
    id: HistoneId,

    insulator: InsulatorId,
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

enum HistoneTarget {
    Tf(TfId),
    TfClass(TfClassId),
    Genome(GenomeId),
    Gene(GeneId),
    Allele(AlleleId),
    Exon(ExonId),
    Protein(ProteinId),
    Fold(FoldId),
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
```

## Schema System

```rust
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

struct SequenceDefinition {
    name: String,
    sequence_type: SequenceType,
}

struct Sequence {
    name: String,
    value: SequenceValue,
}

type SequenceKey = String;

enum SequenceValue {
    String(String),
    StringVec(Vec<String>),
    Int(i64),
    IntVec(Vec<i64>),
    Float(f64),
    FloatVec(Vec<f64>),
    Bool(bool),
    BoolVec(Vec<bool>),
    GeneRef(Uuid),
    GeneRefVec(Vec<Uuid>),
}
```

## Gene Identity and Evolution

```rust
struct Locus {
    id: LocusId,

    family_id: GeneFamilyId,

    insulator: InsulatorId,

    created_at: Timestamp,
}

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

struct Gene {
    id: GeneId,

    genome_id: GenomeId,

    locus_id: LocusId,

    gene_family_generation_id: GeneFamilyGenerationId,

    generation: u32,

    sequences: Vec<Sequence>,

    selected_from: AlleleId,

    insulator: InsulatorId,

    created_at: Timestamp,
}

enum AlleleBase {
    Gene(GeneId),
    Transposon(TransposonId),
}

struct Allele {
    id: AlleleId,

    genome_id: GenomeId,

    locus_id: LocusId,

    base: AlleleBase,

    mutations: Vec<Mutation>,

    state: AlleleState,

    created_by: TfId,

    degraded_at: Option<Timestamp>,
    degraded_by: Option<TfId>,

    created_at: Timestamp,
    updated_at: Timestamp,
}

enum AlleleState {
    Mutating,
    Transcribed,
    Spliced,
    Translating,
    Implementing,
    Blocked,
    Evaluating,
    Repairing,
    Selected,
    Degraded,
}

struct Mutation {
    sequence: SequenceKey,

    value: SequenceValue,

    created_by: TfId,

    created_at: Timestamp,
}
```

## Fold

```rust
struct Protein {
    id: ProteinId,

    genome_id: GenomeId,
    allele_id: AlleleId,

    folds: Vec<FoldId>,

    created_at: Timestamp,
    updated_at: Timestamp,
}

struct Fold {
    id: FoldId,

    protein_id: ProteinId,

    commit_sha: String,
    state: FoldState,

    degraded_at: Option<Timestamp>,
    degraded_by: Option<TfId>,

    created_at: Timestamp,
    updated_at: Timestamp,
}

enum FoldState {
    Pending,
    Running,
    Passed,
    Failed,
    NeedsChaperone,
    Degraded,
}
```

## RNA

```rust
struct SgRna {
    id: SgRnaId,

    gene: GeneId,
    allele: AlleleId,

    activator: TfId,
    cofactors: Vec<TfId>,

    created_by: TfId,

    disambiguation_of: Option<SnRnaId>,

    mutation: Mutation,

    messages: Vec<GRnaId>,

    state: SgRnaState,

    created_at: Timestamp,
    updated_at: Timestamp,
}

struct SnRna {
    id: SnRnaId,

    gene: GeneId,
    allele: AlleleId,

    parent: Option<SnRnaId>,

    activator: TfId,
    cofactors: Vec<TfId>,

    created_by: TfId,

    messages: Vec<GRnaId>,

    state: SnRnaState,

    created_at: Timestamp,
    updated_at: Timestamp,
}

struct GRna {
    id: GRnaId,

    activator: TfId,
    cofactors: Vec<TfId>,

    created_by: TfId,

    current_generation_id: GRnaGenerationId,
    generations: Vec<GRnaGenerationId>,

    degraded_at: Option<Timestamp>,
    degraded_by: Option<TfId>,

    created_at: Timestamp,
    updated_at: Timestamp,
}

struct GRnaGeneration {
    id: GRnaGenerationId,

    grna_id: GRnaId,

    body: String,

    created_by: TfId,
    created_at: Timestamp,
}
```
