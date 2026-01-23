# 002 Define Work Type

## Capability

Define the first configurable work/document type inside a tenant or project.

This lets DNAp represent enterprise-native work such as:

```text
Product Requirements Document
Feature
Decision
Risk
```

without tenant data needing biological vocabulary.

## User-Visible Result

A Tf can define a work type with fields.

Example tenant data:

```text
name = "Product Requirements Document"
abbreviation = "PRD"
fields = ["title", "problem", "goals"]
```

## Names Requiring Approval Or Confirmation

- `GeneFamily`
- `GeneFamilyGeneration`
- `SequenceDefinition`
- `SequenceType`
- `EncodingType`
- `RnaType`
- `GrnType`

## Candidate Structs

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

struct GeneFamilyGeneration {
    id: GeneFamilyGenerationId,
    family_id: GeneFamilyId,
    generation: u32,
    sequences: Vec<SequenceDefinition>,
    created_by: TfId,
    created_at: Timestamp,
}

struct SequenceDefinition {
    id: SequenceDefinitionId,
    name: String,
    sequence_type: SequenceType,
    required: bool,
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

enum EncodingType {
    RNA(RnaType),
    GRN(GrnType),
}

enum GrnType {
    Promoter,
    Telomere,
    Centromere,
    Silencer,
}

enum RnaType {
    Translation(TranslationRnaType),
    Regulatory(RegulatoryRnaType),
}

enum TranslationRnaType {
    MRNA,
    RRNA,
    TRNA,
}

enum RegulatoryRnaType {
    SnRNA,
    SiRNA,
    TmRNA,
    GRNA,
    MiRNA,
    PiRNA,
    ERNA,
    SnoRNA,
    CrRNA,
    TracrRNA,
    LncRNA,
    CircRNA,
    SgRNA,
}
```

## Invariants To Decide

- GeneFamily defines a configurable work/document type.
- GeneFamily can be Insulator-scoped or Genome-scoped.
- Genome-scoped GeneFamilies extend Insulator-scoped GeneFamilies.
- GeneFamilyGeneration is immutable after creation.
- A GeneFamily points to one current GeneFamilyGeneration.
- EncodingType is required because it defines document handling/type semantics.
- EncodingTypes are system-fixed.
- RnaType and GrnType variants are system-fixed and recovered from `docs/PrevPrev.md`.
- GRN encodings are SDLC phase documents.
- Translation RNA encodings are core SDLC production documents.
- Regulatory RNA encodings are DNAp modern workflow/control documents.
- SequenceDefinition names are enterprise-native tenant data.
- SequenceDefinition has stable identity through `SequenceDefinitionId`.
- SequenceDefinition names are unique inside a GeneFamilyGeneration.
- CLI sequence-name matching accepts kebab-case, case-insensitive, fuzzy user input for ease of use.
- CLI sequence-name matching must resolve to exactly one SequenceDefinition and return its id internally, or return an error.
- More exact CLI sequence-name input can disambiguate fuzzy matching errors.
- Required fields must be present before commit.
- Decide whether `required: bool` is enough or whether writable/hidden semantics are needed now.
- Decide uniqueness scope for `abbreviation`.

## Approved For This Slice

- CLI sequence-name matching is user-friendly: kebab-case, case-insensitive, and fuzzy.
- CLI sequence-name matching must resolve to exactly one SequenceDefinition and return its id internally, or fail with an error.
- More exact user input is the intended way to resolve ambiguous CLI sequence-name matches.
- `SequenceDefinitionId` is the stable field identity inside an immutable GeneFamilyGeneration; `SequenceDefinition.name` remains the tenant-facing field name.
- `EncodingType` is essential, required work-type metadata because typing drives document handling and is part of DNAp's product value.
- GRN document meanings:
  - `Promoter`: planning document.
  - `Telomere`: testing document.
  - `Centromere`: deployment document.
  - `Silencer`: retirement document.
- Translation RNA document meanings:
  - `MRNA`: requirements analysis document.
  - `RRNA`: design document.
  - `TRNA`: development document.
- Regulatory RNA document meanings:
  - `SnRNA`: disambiguation document.
  - `SiRNA`: stalled implementation document.
  - `TmRNA`: task managed document.
  - `GRNA`: general message.
  - `MiRNA`: modified implementation document.
  - `PiRNA`: discard task document; currently ambiguous and intended to complement `Silencer`.
  - `ERNA`: priority booster document.
  - `SnoRNA`: ADR.
  - `CrRNA`: open issue.
  - `TracrRNA`: solved issue report.
  - `LncRNA`: research document.
  - `CircRNA`: onboarding particularities.
  - `SgRNA`: suggested document modification document.

## Rejected For This Slice

- None yet.

## Deferred

- Implementing mutation CLI flags that use sequence-name matching; revisit in slice 004.

## Deferred Domain Ledger

- Whether `GeneFamily` stores direct `insulator_id`/`genome_id` references, object references like the recovered spec's `insulator: Insulator`, or only IDs for the implementation boundary.
- Whether `GeneFamilyGeneration.id` should be `GeneFamilyGenerationId` or the recovered `GenotypeId`.
- Whether `GeneFamily.generation` should exist alongside `current_generation_id`, or whether generation belongs only to immutable `GeneFamilyGeneration` records.
- Whether `SequenceValue` belongs in this slice as part of field type validation, or waits for slice 004 mutations.
- Whether `SequenceType::Gene`/`GeneVec` should reference `GeneId` instead of raw UUIDs.
- Whether `abbreviation` uniqueness is per Insulator, per Genome, or per GeneFamily scope.
- Whether later schema evolution needs rename tracking across GeneFamilyGenerations.

## Implementation Contract

- Names: pending approval.
- Structs: pending approval.
- Invariants: pending approval.
- Approved tests: none yet.

## Old Vs New

Old current code used:

- `Genotype`
- `Trait`
- `Dominance`

New candidate uses:

- `GeneFamily`
- `GeneFamilyGeneration`
- `SequenceDefinition`
- `required`

## Possible Loss

- Loss of `Genotype` name.
- Loss of `Trait` name.
- Loss of `Dominant`, `Recessive`, and `Vestigial` semantics.
- Loss of fuzzy matching for old trait keys.

## Implementation Gate

Do not implement until work type and field names are approved.

## Test Gate

No tests are approved yet. Derive tests from approved invariants and high-value externally observable behavior during slice design.
