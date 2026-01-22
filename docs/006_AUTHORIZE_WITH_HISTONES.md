# 006 Authorize With Histones

## Capability

Add authorization and contextual evaluation to the working vertical path.

This slice wraps previous actions with the single approved authorization abstraction.

## User-Visible Result

A Tf can perform or be denied actions based on HistoneMarks inherited from class, user, and resource context.

Tenant data still uses enterprise-native keys and values.

## Names Requiring Approval Or Confirmation

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
+ Tf.histones
+ Genome/Gene/Allele histones
```

Resolution:

1. Explicit deny wins.
2. Tf histones override TfClass histones.
3. Resource histones constrain access.
4. Default deny.

## Invariants To Decide

- Permissions must not exist independently from Histones.
- TfClass definitions are flat and non-inheritable.
- Composition occurs through multiple `tf_classes` on `Tf`.
- Histones may exist at Insulator or Genome scope.
- Genome Histones extend Insulator Histones.
- `Histone.key` is unique within an Insulator.
- Only one active HistoneMark may exist per `(target, histone)` pair.
- Multi-valued attributes must use vector value types.
- More specific marks override broader marks unless broader state is `ConstitutiveHeterochromatin`.
- Auditable records are degraded, not hard-deleted.

## Approved For This Slice

- None yet.

## Rejected For This Slice

- None yet.

## Deferred

- None yet.

## Deferred Domain Ledger

- Whether `Tf` stores direct relationship lists for `tf_classes`, active `histones`, `pre_initiation_complex`, `mediator_complex`, `repressors`, and `affinity`, or whether those are derived/query-side relationships.
- Whether `TfClassScope` should be explicit as an enum or represented by `insulator_id` plus optional `genome_id`.
- Whether `HistoneTarget` includes `Exon` now; no earlier slice currently defines `Exon`.
- Whether `HistoneValueType` and `HistoneValue` include only String/Int/Bool vectors as recovered, or need Float/Gene references like SequenceValue.
- Whether `ChromatinState` itself encodes allow/deny, or whether allow/deny is represented by a Histone key/value interpreted through the resolver.
- Whether `valid_from`/`valid_until` are required in the first authorization slice or can be deferred until time-bound access matters.
- Whether `ConstitutiveHeterochromatin` invariants are implemented immediately: non-overridable, `valid_until = None`, and rationale required.
- Whether `Heterochromatin` also requires rationale in the first implementation.
- Whether only one active `HistoneMark` per `(target, histone)` is enforced globally, per Insulator, or by target scope.
- Whether degraded auditable records apply to all records from slice 001 onward or only authorization records in this slice.

## Implementation Contract

- Names: pending approval.
- Structs: pending approval.
- Invariants: pending approval.
- Approved tests: none yet.

## Old Vs New

Current code has no real authorization model, only `Human` and `Llm` mutation authors.

Previous LLD made Histones the authorization and context system. New direction keeps that but applies it after the first vertical work loop is coherent.

## Possible Loss

- Loss of conventional role/permission flags.
- Loss of simple author enum.
- Loss of role inheritance.

## Implementation Gate

Do not implement until Histone naming and allow/deny value semantics are approved.

## Test Gate

No tests are approved yet. Derive tests from approved invariants and high-value externally observable behavior during slice design.
