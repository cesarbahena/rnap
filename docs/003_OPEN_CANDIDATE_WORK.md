# 003 Open Candidate Work

## Capability

Open a new mutable candidate for a work item.

This slice introduces stable identity and draft/candidate work without yet solving the full mutation or commit flow.

## User-Visible Result

A Tf can start work of a configured type inside a project.

Example:

```text
Open PRD "Awesome To Do App"
```

The system creates stable identity plus a mutable candidate.

## Names Requiring Approval Or Confirmation

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
- The creator Tf must belong to the same Insulator.
- Decide whether multiple active Alleles can exist for one Locus.
- Decide whether `Transposon` is needed now or can be deferred.

## Approved For This Slice

- None yet.

## Rejected For This Slice

- None yet.

## Deferred

- None yet.

## Deferred Domain Ledger

- Whether `Locus` should include `genome_id`; recovered spec anchored Locus to family and Insulator, while Gene later binds to Genome.
- Whether `Locus` should store `insulator_id` or an object reference like the recovered spec's `insulator: Insulator`.
- Whether opening a candidate requires a display identifier derived from GeneFamily abbreviation, name sequence, and counter, or whether identity remains opaque until commit.
- Whether `Transposon` is the right origin record for a brand-new item, or whether an `Allele` with no base is enough for the first implementation.
- Whether multiple active `Allele` records per `Locus` are allowed for parallel work, branch-like review, or agent exploration.
- Whether `Allele` should carry initial sequence values when opened, or whether all content must arrive through slice 004 `Mutation` records.
- Whether degraded candidate behavior is enough before authorization, or whether audit/degrade rules should wait for slice 006.

## Implementation Contract

- Names: pending approval.
- Structs: pending approval.
- Invariants: pending approval.
- Approved tests: none yet.

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

## Test Gate

No tests are approved yet. Derive tests from approved invariants and high-value externally observable behavior during slice design.
