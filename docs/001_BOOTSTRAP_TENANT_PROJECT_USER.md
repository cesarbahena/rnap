# 001 Bootstrap Tenant, Project, User

## Capability

Create the minimum DNAp world:

- one tenant,
- one project,
- one user identity.

This gives every later slice a real scope and actor without designing the whole authorization system yet.

## User-Visible Result

A caller can create an Insulator, create a Genome inside it, and create a TF inside the Insulator.

Tenant data uses enterprise-native names:

```text
tenant = "Acme"
project = "Billing Platform"
user = "Cesar"
```

The code may use biological names after approval.

Each operational Insulator has placement metadata. Placement is infrastructure/provisioning configuration, not a DNAp document schema.

## Names Requiring Approval Or Confirmation

- `Insulator`
- `InsulatorPlacement`
- `InsulatorPlacementStrategy`
- `SharedCluster`
- `Genome`
- `TF`
- `TfId`

## Candidate Structs

```rust
struct Insulator {
    id: InsulatorId,
    name: String,
    created_at: Timestamp,
    updated_at: Timestamp,
}

struct InsulatorPlacement {
    insulator_id: InsulatorId,
    strategy: InsulatorPlacementStrategy,
    region: String,
    active: bool,
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum InsulatorPlacementStrategy {
    SharedCluster,
}

struct Genome {
    id: GenomeId,
    insulator_id: InsulatorId,
    name: String,
    created_at: Timestamp,
    updated_at: Timestamp,
}

struct TF {
    id: TfId,
    insulator_id: InsulatorId,
    name: String,
    created_at: Timestamp,
    updated_at: Timestamp,
}
```

## Invariants To Decide

- Insulator is the tenant boundary.
- InsulatorPlacement records operational infrastructure/storage placement for an Insulator.
- InsulatorPlacement is distinct from DNAp document schemas, which are configurable tenant/project data.
- An operational Insulator has exactly one active InsulatorPlacement.
- If no placement strategy is specified, DNAp assigns SharedCluster.
- InsulatorPlacement records a region.
- Genome is the project boundary.
- A Genome belongs to exactly one Insulator.
- A TF belongs to exactly one Insulator.
- A TF may later act across multiple Genomes in that Insulator when authorized.
- `name` fields must not be empty.
- Decide whether Genome names must be unique inside an Insulator.
- Decide whether TF names must be unique inside an Insulator.

## Approved For This Slice

- `Insulator` is the customer/account tenant and hard isolation boundary.
- `Genome` is the project boundary.
- `TF` is user identity.
- `Cell` is removed for now.
- Backend and CLI are the first delivery surfaces.
- CLI behavior should stay thin over backend/application behavior.
- CLI language should use more mainstream biology concepts, but should not be generic CRUD.
- `InsulatorPlacement` is the tenant infrastructure/storage placement concept.
- `InsulatorPlacement` is distinct from DNAp document schemas.
- `SharedCluster` is the default InsulatorPlacement strategy when no placement is specified.
- InsulatorPlacement includes region.
- Bootstrap callers must provide an explicit InsulatorPlacement region; DNAp does not assign a default region in this slice.

## Rejected For This Slice

- Database schema terminology for tenant placement: rejected because DNAp has configurable document schemas as data.

## Deferred

- Genome name uniqueness inside an Insulator.
- TF name uniqueness inside an Insulator.
- Full authorization for TF access across multiple Genomes; revisit in slice 006.
- Default region selection and detailed storage topology/provisioning automation beyond the minimal InsulatorPlacement contract.

## Recovered Spec Gaps To Decide

- Whether `Cell` remains deferred or returns as the software-system boundary; recovered spec included `Cell`, while the current vertical build removed it for now.
- Whether bootstrap should include collection references on `Insulator` and `Genome` such as `gene_families`, `histones`, and `tf_classes`, or keep slice 001 to identity and ownership only.
- Whether `TF` needs a tenant-facing display name in this slice; recovered spec only modeled TF identity and relationships, but bootstrap examples include a user name.
- Whether `TF` identity should include early relationship fields such as `tf_classes`, `histones`, `pre_initiation_complex`, `mediator_complex`, `repressors`, and `affinity`, or defer them to authorization/workflow slices.
- Where the already-defined CLI subcommands are recorded, so slice 001 can align to the command contract without inventing new command names.

## Implementation Contract

- Names: pending final confirmation before code.
- Structs: pending final confirmation before code.
- Invariants: pending final confirmation before code.
- CLI: use mainstream biology-oriented workflow language rather than generic CRUD commands.
- Approved tests: none yet.

## Old Vs New

Old previous LLD included `Cell` under platform boundaries.

New direction removes `Cell` for now. This slice only has tenant, project, and user.

Current code has `Genome` but uses it as the old tenant-ish boundary. New direction moves tenant responsibility to `Insulator` and makes `Genome` the project.

## Possible Loss

- Loss of `Cell` as software-system boundary.
- Loss of current `Genome` meaning as top-level tenant.
- Loss of current simple user-less author model.

## Implementation Gate

Do not implement until `Insulator`, `Genome`, and `TF` are approved or replaced.

## Test Gate

No tests are approved yet. Derive tests from approved invariants and high-value externally observable behavior during slice design.
