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

## Names Requiring Approval

- `Insulator`
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
- Genome is the project boundary.
- A Genome belongs to exactly one Insulator.
- A TF belongs to exactly one Insulator.
- A TF may later act across multiple Genomes in that Insulator when authorized.
- `name` fields must not be empty.
- Decide whether Genome names must be unique inside an Insulator.
- Decide whether TF names must be unique inside an Insulator.

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

## First Tests

- Cannot create Insulator with empty name.
- Cannot create Genome with empty name.
- Cannot create Genome without Insulator id.
- Cannot create TF with empty name.
- Cannot create TF without Insulator id.

