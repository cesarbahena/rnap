# 007 Add Workflow Artifacts

## Capability

Add explicit workflow records around candidate work: proposal, context, and generated reasoning.

This slice should only happen after the basic candidate/mutation/commit loop is working.

## User-Visible Result

A Tf or agent can propose a mutation, attach context, and keep generated reasoning linked to the candidate.

## Names Requiring Approval Or Confirmation

- `SgRna`
- `SnRna`
- `GRna`
- `GRnaGeneration`
- `Mediator`
- `BindingAffinity`
- `SiRna`
- `TfComplex`

## Candidate Structs

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
```

## Invariants To Decide

- SgRna carries a concrete mutation proposal.
- SnRna carries structured workflow context.
- GRna carries a general message document.
- GRnaGeneration is immutable.
- A GRna points to one current generation.
- Workflow artifacts are created by TFs and evaluated through Histones.
- Decide whether mutation proposals must always pass through SgRna.
- Decide whether agents are TFs or a separate actor type.
- Decide whether `TfComplex` replaces direct workflow fields on Tf.

## Approved For This Slice

- None yet.

## Rejected For This Slice

- None yet.

## Deferred

- None yet.

## Deferred Domain Ledger

- Whether agents are modeled as `Tf` records, `TfClass` membership, or a separate actor type.
- Whether recovered pending concepts map directly: `Activator`, `Cofactor`, `Mediator`, `PreInitiationComplex`, `SiRNA`, `TmRNA`, and `BindingAffinity`.
- Whether `Tf.pre_initiation_complex`, `mediator_complex`, `repressors`, and `affinity` are source-of-truth fields or derived from workflow artifacts.
- Whether mutation proposals must pass through `SgRna`, or direct mutation remains valid and `SgRna` is optional workflow metadata.
- Whether `SnRna` represents disambiguation/context only, or also stores reusable instruction/prompt state.
- Whether `GRnaGeneration` needs immutable general-message snapshots, model/config metadata, and generated-by provenance.
- Whether `Mediator` is a first-class artifact in this slice or deferred until multi-actor orchestration requires it.
- Whether `BindingAffinity` belongs to authorization evaluation, workflow ranking, or both.
- Whether `SiRna` represents a stalled implementation document, a repressor/blocking workflow artifact, or both.

## Implementation Contract

- Names: pending approval.
- Structs: pending approval.
- Invariants: pending approval.
- Approved tests: none yet.

## Old Vs New

Previous LLD had pending workflow terms:

- Activator
- Cofactor
- Mediator
- PreInitiationComplex
- SiRNA
- TmRNA
- BindingAffinity

Recovered LLD had a richer `TfComplex`. This slice decides only what is needed after mutation/commit already works.

## Possible Loss

- Loss of direct CLI mutation if proposals must go through SgRna.
- Loss of `TfComplex` if direct fields win.
- Loss of current Translation RNA and Regulatory RNA document distinctions.

## Implementation Gate

Do not implement until RNA names and their SDLC meanings are approved.

## Test Gate

No tests are approved yet. Derive tests from approved invariants and high-value externally observable behavior during slice design.
