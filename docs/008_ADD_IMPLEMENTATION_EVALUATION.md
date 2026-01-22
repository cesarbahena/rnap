# 008 Add Implementation Evaluation

## Capability

Connect candidate work to implementation output and evaluation state.

This slice turns DNAp from structured work management into SDLC execution tracking.

## User-Visible Result

An Allele can produce implementation output, link to a commit, and record whether evaluation passed, failed, or needs repair.

## Names Requiring Approval Or Confirmation

- `Protein`
- `Fold`
- `FoldState`
- `Ribosome`
- `Rrna`
- `Chaperone`
- `Chiasma`

## Candidate Structs

```rust
struct Protein {
    id: ProteinId,
    genome_id: GenomeId,
    allele_id: AlleleId,
    folds: Vec<FoldId>,
    created_at: Timestamp,
    updated_at: Timestamp,
}

struct Fold {
    id: FoldId,
    protein_id: ProteinId,
    commit_sha: String,
    state: FoldState,
    degraded_at: Option<Timestamp>,
    degraded_by: Option<TfId>,
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum FoldState {
    Pending,
    Running,
    Passed,
    Failed,
    NeedsChaperone,
    Degraded,
}
```

## Invariants To Decide

- Protein belongs to one Allele.
- Fold belongs to one Protein.
- Fold records implementation/evaluation commit.
- Failed Fold may move Allele to repair.
- Passed Fold may allow Allele selection.
- Degraded Fold remains auditable.
- Decide whether Fold needs more than `commit_sha`.
- Decide whether Ribosome, Rrna, Chaperone, and Chiasma remain.

## Approved For This Slice

- None yet.

## Rejected For This Slice

- None yet.

## Deferred

- None yet.

## Recovered Spec Gaps To Decide

- Whether `Protein` represents implementation output for an `Allele`, for a committed `Gene`, or both.
- Whether `Fold` needs more than `commit_sha`, such as repository, branch, PR, workflow run, artifact URI, environment, and evaluator provenance.
- Whether `FoldState` should distinguish execution status from evaluation verdict instead of combining both.
- Whether `Ribosome` is the evaluator/execution engine, and whether it should be first-class in this slice.
- Whether `Rrna` is evaluator configuration/policy, model instructions, or something else.
- Whether `Chaperone` is a repair workflow, an actor class, or a generated patch record.
- Whether `Chiasma` is a violation/finding record and whether it replaces or complements failed Fold details.
- Whether old `Phenotype`/`Phenome` concepts are truly removed or deferred as reporting/read-model concepts.
- Whether passed Fold allows commit selection automatically, or only satisfies one precondition for explicit TF selection.

## Implementation Contract

- Names: pending approval.
- Structs: pending approval.
- Invariants: pending approval.
- Approved tests: none yet.

## Old Vs New

Current code has thin modules for Protein, Fold, Ribosome, Rrna, Chaperone, Chiasma, Phenotype, and Phenome.

New direction keeps only the concepts approved in this slice.

## Possible Loss

- Loss of `ProteinResult`.
- Loss of Ribosome as evaluator.
- Loss of Rrna as evaluator configuration.
- Loss of Chaperone as repair mechanism.
- Loss of Chiasma as violation record.
- Loss of Phenotype/Phenome.

## Implementation Gate

Do not implement until implementation/evaluation vocabulary is approved.

## Test Gate

No tests are approved yet. Derive tests from approved invariants and high-value externally observable behavior during slice design.
