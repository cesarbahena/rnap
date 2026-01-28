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

All SequenceDefinitions in a GeneFamilyGeneration are required before commit. SequenceDefinition names are enterprise-native tenant data and unique inside a GeneFamilyGeneration.

GeneFamily definitions may be Insulator-scoped or Genome-scoped. Genome-scoped GeneFamilies are project-level overrides. Lookup from a Genome context resolves Genome-scoped override first, then Insulator-scoped default.

GeneFamily abbreviations are unique in effective scope:

- Insulator-scoped abbreviations are unique within the Insulator.
- Genome-scoped abbreviations are unique within the Genome.
- Genome-scoped GeneFamilies may shadow Insulator-scoped abbreviations.

## Document Instances

`Locus` is Genome-scoped document/work item identity. It anchors committed versions for one document instance.

```rust
struct Locus {
    id: LocusId,
    family_id: GeneFamilyId,
    insulator_id: InsulatorId,
    genome_id: GenomeId,
    created_at: Timestamp,
}
```

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

`Allele` is mutable candidate work.

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
    Degraded,
}
```

One active Allele is allowed per `(Locus, Tf)`. Multiple Tfs may each have one active Allele for the same Locus.

`Mutation` is an append-only field change on an Allele.

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
    sequence_definition_id: SequenceDefinitionId,
    value: SequenceValue,
    created_by: TfId,
    created_at: Timestamp,
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
    Sequence(SequenceDefinitionId),
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
