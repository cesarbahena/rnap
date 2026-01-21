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

## Names Requiring Approval

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

## First Tests

- Cannot commit Allele missing required sequences.
- Committed Gene contains projected sequence snapshot.
- Committed Gene cannot be mutated.
- First Gene for a Locus has generation 1.
- Next committed Gene for same Locus increments generation.
- Gene records `selected_from`.

