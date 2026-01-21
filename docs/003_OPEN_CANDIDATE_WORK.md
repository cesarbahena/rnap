# 003 Open Candidate Work

## Capability

Open a new mutable candidate for a work item.

This slice introduces stable identity and draft/candidate work without yet solving the full mutation or commit flow.

## User-Visible Result

A TF can start work of a configured type inside a project.

Example:

```text
Open PRD "Awesome To Do App"
```

The system creates stable identity plus a mutable candidate.

## Names Requiring Approval

- `Locus`
- `Allele`
- `AlleleBase`
- `AlleleState`
- `Transposon`

## Candidate Structs

```rust
struct Locus {
    id: LocusId,
    family_id: GeneFamilyId,
    insulator_id: InsulatorId,
    created_at: Timestamp,
}

enum AlleleBase {
    Gene(GeneId),
    Transposon(TransposonId),
}

struct Transposon {
    id: TransposonId,
    genome_id: GenomeId,
    locus_id: LocusId,
    gene_family_generation_id: GeneFamilyGenerationId,
    created_by: TfId,
    degraded_at: Option<Timestamp>,
    degraded_by: Option<TfId>,
    created_at: Timestamp,
}

struct Allele {
    id: AlleleId,
    genome_id: GenomeId,
    locus_id: LocusId,
    base: AlleleBase,
    state: AlleleState,
    created_by: TfId,
    degraded_at: Option<Timestamp>,
    degraded_by: Option<TfId>,
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum AlleleState {
    Mutating,
    Degraded,
}
```

## Invariants To Decide

- Locus anchors identity across committed versions.
- A Locus belongs to one GeneFamily.
- A new work item starts from a Transposon.
- A new candidate starts in `Mutating`.
- A candidate belongs to exactly one Genome.
- The Genome must belong to the Locus Insulator.
- The creator TF must belong to the same Insulator.
- Decide whether multiple active Alleles can exist for one Locus.
- Decide whether `Transposon` is needed now or can be deferred.

## Old Vs New

Old current code created a mutable `Gene` directly.

New direction creates:

- stable identity: `Locus`,
- origin: `Transposon`,
- mutable candidate: `Allele`.

## Possible Loss

- Loss of simple direct `create <kind> <name>` creating a Gene.
- Loss of Gene as the first mutable object.
- Loss of direct name on Gene unless later represented as a Sequence.

## Implementation Gate

Do not implement until `Locus`, `Allele`, and `Transposon` are approved or replaced.

## First Tests

- Cannot open candidate for GeneFamily outside the Insulator.
- Cannot open candidate in Genome outside the Insulator.
- Cannot open candidate by TF outside the Insulator.
- New candidate starts in `Mutating`.
- New candidate records creator.

