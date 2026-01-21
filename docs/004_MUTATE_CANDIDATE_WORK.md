# 004 Mutate Candidate Work

## Capability

Apply append-only changes to a candidate and project its current state.

This makes the first useful work loop possible: open candidate, change fields, inspect current values.

## User-Visible Result

A TF can set or change fields on an Allele.

Example:

```text
title = "Awesome To Do App"
problem = "Tasks are scattered"
```

The current candidate state is computed from its mutations.

## Names Requiring Approval

- `Mutation`
- `Sequence`
- `SequenceKey`
- `SequenceValue`

## Candidate Structs

```rust
type SequenceKey = String;

struct Sequence {
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
    GeneRef(Uuid),
    GeneRefVec(Vec<Uuid>),
}

struct Mutation {
    sequence: SequenceKey,
    value: SequenceValue,
    created_by: TfId,
    created_at: Timestamp,
}

struct Allele {
    mutations: Vec<Mutation>,
}
```

## Invariants To Decide

- Mutations are append-only.
- A Mutation must target an existing SequenceDefinition in the Allele GeneFamilyGeneration.
- A Mutation value must match the SequenceDefinition type.
- Current state is projected from the last Mutation per SequenceKey unless a different rule is approved.
- A degraded Allele cannot accept new Mutations.
- Mutations are created by TFs.
- Decide whether vectors append by default or replace by default.
- Decide whether a Mutation needs rationale/context.

## Old Vs New

Current code has Mutation on Gene:

```rust
gene_id
trait_key
value
by
context
```

New direction moves Mutation to Allele and replaces `trait_key` with `sequence`.

## Possible Loss

- Loss of mutation `context` unless added here.
- Loss of `by = Human | Llm`.
- Loss of fuzzy trait matching.
- Loss of old append/replace CLI flags unless vector mutation semantics preserve them.

## Implementation Gate

Do not implement until Mutation shape and vector semantics are approved.

## First Tests

- Cannot mutate unknown SequenceKey.
- Cannot mutate with wrong SequenceValue type.
- Cannot mutate degraded Allele.
- Projection returns latest value for scalar fields.
- Required fields can be detected as missing.

