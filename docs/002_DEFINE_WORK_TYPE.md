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

A TF can define a work type with fields.

Example tenant data:

```text
name = "Product Requirements Document"
abbreviation = "PRD"
fields = ["title", "problem", "goals"]
```

## Names Requiring Approval

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

## First Tests

- Cannot create GeneFamily with empty name.
- Cannot create GeneFamily with empty abbreviation.
- Cannot create GeneFamilyGeneration with duplicate SequenceDefinition names.
- Cannot mutate a GeneFamilyGeneration after creation.
- Project-scoped GeneFamily must belong to a Genome in the same Insulator.

