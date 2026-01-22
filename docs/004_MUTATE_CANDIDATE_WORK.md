# 004 Mutate Candidate Work

## Capability

Apply append-only changes to a candidate and project its current state.

This makes the first useful work loop possible: open candidate, change fields, inspect current values.

## User-Visible Result

A Tf can set or change fields on an Allele.

Example:

```text
title = "Awesome To Do App"
problem = "Tasks are scattered"
```

The current candidate state is computed from its mutations.

## Names Requiring Approval Or Confirmation

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

## Approved For This Slice

- None yet.

## Rejected For This Slice

- None yet.

## Deferred

- None yet.

## Deferred Domain Ledger

- Whether `Mutation` keeps a `context`/rationale field from the old implementation, or whether workflow context waits for slice 007 artifacts.
- Whether author identity is always `TfId`; recovered/old context had simpler author categories such as human/LLM that may need mapping into TFs later.
- Whether vector `SequenceValue` mutations replace whole vectors or support append/remove operations.
- Whether references in `SequenceValue::GeneRef` use `GeneId`, `LocusId`, or raw UUIDs.
- Whether `@Cell` references return with `Cell` and resolve eagerly or lazily, or remain deferred with `Cell`.
- Whether current-state projection needs deletion/clear semantics for optional fields.
- Whether mutations need optimistic concurrency/version preconditions so concurrent Tf or agent changes do not overwrite unexpectedly.

## Implementation Contract

- Names: pending approval.
- Structs: pending approval.
- Invariants: pending approval.
- Approved tests: none yet.

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

## Test Gate

No tests are approved yet. Derive tests from approved invariants and high-value externally observable behavior during slice design.
