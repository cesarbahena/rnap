# 008 Add Implementation Evaluation

## Capability

Link candidate work to implementation output and evaluation.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

Slice 008 implements:

- `Protein`
- `Fold`
- `FoldState`

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

## Behavior

- Protein belongs to one Allele.
- Fold belongs to one Protein.
- Fold records implementation/evaluation commit.
- Failed Fold may move Allele to repair.
- Passed Fold may allow Allele selection.
- Degraded Fold remains auditable.

## Implementation Contract

Pending implementation.

## Approved Tests

Pending.
