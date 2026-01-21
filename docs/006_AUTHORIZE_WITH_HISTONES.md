# 006 Authorize With Histones

## Capability

Add authorization and contextual evaluation to the working vertical path.

This slice wraps previous actions with the single approved authorization abstraction.

## User-Visible Result

A TF can perform or be denied actions based on HistoneMarks inherited from class, user, and resource context.

Tenant data still uses enterprise-native keys and values.

## Names Requiring Approval

- `TfClass`
- `Histone`
- `HistoneMark`
- `HistoneValueType`
- `HistoneValue`
- `HistoneTarget`
- `ChromatinState`
- `Euchromatin`
- `FacultativeHeterochromatin`
- `Heterochromatin`
- `ConstitutiveHeterochromatin`

## Candidate Structs

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

## Authorization Candidate

```text
TfClass.histones
+ TF.histones
+ Genome/Gene/Allele histones
```

Resolution:

1. Explicit deny wins.
2. TF histones override TfClass histones.
3. Resource histones constrain access.
4. Default deny.

## Invariants To Decide

- Permissions must not exist independently from Histones.
- TfClass definitions are flat and non-inheritable.
- Composition occurs through multiple `tf_classes` on `TF`.
- Histones may exist at Insulator or Genome scope.
- Genome Histones extend Insulator Histones.
- `Histone.key` is unique within an Insulator.
- Only one active HistoneMark may exist per `(target, histone)` pair.
- Multi-valued attributes must use vector value types.
- More specific marks override broader marks unless broader state is `ConstitutiveHeterochromatin`.
- Auditable records are degraded, not hard-deleted.

## Old Vs New

Current code has no real authorization model, only `Human` and `Llm` mutation authors.

Previous LLD made Histones the authorization and context system. New direction keeps that but applies it after the first vertical work loop is coherent.

## Possible Loss

- Loss of conventional role/permission flags.
- Loss of simple author enum.
- Loss of role inheritance.

## Implementation Gate

Do not implement until Histone naming and allow/deny value semantics are approved.

## First Tests

- Default deny.
- TF class mark can allow an action.
- TF-specific mark can override class mark.
- Resource deny constrains access.
- Constitutive deny cannot be overridden.
- Cannot create second active mark for same target/histone.

