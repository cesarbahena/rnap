# 005 Commit Immutable Version

## Capability

Select a candidate Allele and create an immutable committed Gene.

This completes the first full DNAp loop:

```text
tenant -> project -> work type -> candidate -> mutations -> committed version
```

## User-Visible Result

A TF can commit a complete candidate.

The committed version is immutable and can be referenced later.

## Names Requiring Approval Or Confirmation

- `Gene`
- `GeneId`
- `generation`
- `selected_from`

## Candidate Struct

```rust
struct Gene {
    id: GeneId,
    genome_id: GenomeId,
    locus_id: LocusId,
    gene_family_generation_id: GeneFamilyGenerationId,
    generation: u32,
    sequences: Vec<Sequence>,
    selected_from: AlleleId,
    insulator_id: InsulatorId,
    created_at: Timestamp,
}
```

## Invariants To Decide

- Gene is immutable committed version.
- Selecting an Allele creates a Gene.
- Gene stores the committed sequence snapshot.
- Gene records the source Allele in `selected_from`.
- Gene generation increments within a Locus.
- Required sequences must be present before selection.
- Selected Allele state changes after commit.
- Decide whether selected Allele becomes `Selected`, locked, or degraded.
- Decide whether committed Gene has a display name or only derived identity.

## Approved For This Slice

- None yet.

## Rejected For This Slice

- None yet.

## Deferred

- None yet.

## Deferred Domain Ledger

- Whether committed `Gene` stores `insulator_id` or an object reference like the recovered spec's `insulator: Insulator`.
- Whether `Gene.generation` increments per `Locus`, per `GeneFamily`, or per `(Genome, Locus)`.
- Whether `selected_from` is the permanent name for the source Allele relationship.
- Whether the selected Allele becomes `Selected`, locked/read-only, degraded, or remains active after commit.
- Whether committed Genes need an explicit display name or slug, or whether names are always derived from committed Sequences.
- Whether committed identity such as `PRD-awesome-to-do-app-0001` is required in this slice or deferred to rendering/search.
- Whether committing must snapshot all projected sequences, including optional unset fields, or only present values.

## Implementation Contract

- Names: pending approval.
- Structs: pending approval.
- Invariants: pending approval.
- Approved tests: none yet.

## Old Vs New

Previous LLD had Gene as immutable version but did not always include `sequences` and `selected_from`.

Recovered LLD included both. New candidate keeps them so the committed version is a complete snapshot.

Current code has Gene as mutable and stores mutations directly. New direction moves mutable state to Allele.

## Possible Loss

- Loss of Gene as mutable active work.
- Loss of direct Gene name.
- Loss of direct mutation list on Gene.
- Loss of old `transcribe gene` behavior unless rebuilt as committed/candidate rendering.

## Implementation Gate

Do not implement until Gene immutability, naming, and selected Allele behavior are approved.

## Test Gate

No tests are approved yet. Derive tests from approved invariants and high-value externally observable behavior during slice design.
