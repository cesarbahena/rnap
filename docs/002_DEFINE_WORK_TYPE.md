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
```

## Invariants To Decide

- GeneFamily defines a configurable work/document type.
- GeneFamily can be Insulator-scoped or Genome-scoped.
- Genome-scoped GeneFamilies extend Insulator-scoped GeneFamilies.
- GeneFamilyGeneration is immutable after creation.
- A GeneFamily points to one current GeneFamilyGeneration.
- SequenceDefinition names are enterprise-native tenant data.
- SequenceDefinition names are unique inside a GeneFamilyGeneration.
- Required fields must be present before commit.
- Decide whether `required: bool` is enough or whether writable/hidden semantics are needed now.
- Decide uniqueness scope for `abbreviation`.

## Approved For This Slice

- None yet.

## Rejected For This Slice

- None yet.

## Deferred

- None yet.

## Deferred Domain Ledger

- Whether `GeneFamily` stores direct `insulator_id`/`genome_id` references, object references like the recovered spec's `insulator: Insulator`, or only IDs for the implementation boundary.
- Whether `GeneFamilyGeneration.id` should be `GeneFamilyGenerationId` or the recovered `GenotypeId`.
- Whether `GeneFamily.generation` should exist alongside `current_generation_id`, or whether generation belongs only to immutable `GeneFamilyGeneration` records.
- Whether `EncodingType` is required for slice 002 or can be deferred until semantic classification matters.
- Whether `SequenceValue` belongs in this slice as part of field type validation, or waits for slice 004 mutations.
- Whether `SequenceType::Gene`/`GeneVec` should reference `GeneId` instead of raw UUIDs.
- Whether `abbreviation` uniqueness is per Insulator, per Genome, or per GeneFamily scope.
- Whether tenant-configured sequence names need key/display-name separation to keep enterprise-native labels stable while supporting machine-safe keys.

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
