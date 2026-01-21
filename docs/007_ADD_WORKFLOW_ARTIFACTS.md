# 007 Add Workflow Artifacts

## Capability

Add explicit workflow records around candidate work: proposal, context, and generated reasoning.

This slice should only happen after the basic candidate/mutation/commit loop is working.

## User-Visible Result

A TF or agent can propose a mutation, attach context, and keep generated reasoning linked to the candidate.

## Names Requiring Approval

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
- GRna carries conversational or generated reasoning.
- GRnaGeneration is immutable.
- A GRna points to one current generation.
- Workflow artifacts are created by TFs and evaluated through Histones.
- Decide whether mutation proposals must always pass through SgRna.
- Decide whether agents are TFs or a separate actor type.
- Decide whether `TfComplex` replaces direct workflow fields on TF.

## Old Vs New

Previous LLD had pending workflow terms:

- Activator
- Cofactor
- Mediator
- PreInitiationComplex
- siRNA
- tmRNA
- BindingAffinity

Recovered LLD had a richer `TfComplex`. This slice decides only what is needed after mutation/commit already works.

## Possible Loss

- Loss of direct CLI mutation if proposals must go through SgRna.
- Loss of `TfComplex` if direct fields win.
- Loss of current mRNA/tRNA/sRNA module distinctions.

## Implementation Gate

Do not implement until RNA names and their SDLC meanings are approved.

## First Tests

- Cannot create workflow artifact for Allele outside actor Insulator.
- SgRna proposal contains a valid Mutation.
- GRnaGeneration is immutable.
- Degraded GRna cannot receive a new current generation.

