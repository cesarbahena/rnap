# 008 Add Implementation Evaluation

## Capability

Connect candidate work to implementation output and evaluation state.

This slice turns DNAp from structured work management into SDLC execution tracking.

## User-Visible Result

An Allele can produce implementation output, link to a commit, and record whether evaluation passed, failed, or needs repair.

## Names Requiring Approval

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

## First Tests

- Cannot create Protein for Allele outside Genome.
- Cannot create Fold without commit reference.
- Passed Fold can mark candidate selectable if required fields are complete.
- Failed Fold can mark candidate repair-needed.
- Degraded Fold cannot be reused as active evaluation.

