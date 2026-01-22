# 001 Bootstrap Tenant, Project, User

## Capability

Create the minimum DNAp world:

- one tenant,
- one project,
- one user identity.

This gives every later slice a real scope and actor without designing the whole authorization system yet.

## User-Visible Result

A caller can create an Insulator, create a Genome inside it, and create a Tf inside the Insulator.

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
- `Tf`
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

struct Tf {
    id: TfId,
    insulator_id: InsulatorId,
    display_name: String,
    external_subject: Option<String>,
    identity_provider: Option<String>,
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
- A Tf belongs to exactly one Insulator.
- A Tf may later act across multiple Genomes in that Insulator when authorized.
- `Insulator.name`, `Genome.name`, `Tf.display_name`, and `InsulatorPlacement.region` must not be blank.
- `Tf.display_name` is a tenant-facing label, not the login identity.
- `Tf.external_subject` and `Tf.identity_provider` reserve a future SSO/SCIM binding without implementing SSO/SCIM in this slice.
- Decide whether Genome names must be unique inside an Insulator.
- Decide whether Tf display names must be unique inside an Insulator.

## Approved For This Slice

- `Insulator` is the customer/account tenant and hard isolation boundary.
- `Genome` is the project boundary.
- `Tf` is user identity.
- `Cell` is removed for now.
- Backend and CLI are the first delivery surfaces.
- CLI behavior should stay thin over backend/application behavior.
- CLI language should use more mainstream biology concepts, but should not be generic CRUD.
- Insulator creation is provisioning/admin behavior, not a normal tenant-user CLI workflow.
- Slice 001 implementation is backend/application first; normal user CLI is deferred until a user-work command exists in a later slice.
- `InsulatorPlacement` is the tenant infrastructure/storage placement concept.
- `InsulatorPlacement` is distinct from DNAp document schemas.
- `SharedCluster` is the default InsulatorPlacement strategy when no placement is specified.
- InsulatorPlacement includes region.
- Provisioning callers must provide an explicit InsulatorPlacement region; DNAp does not assign a default region in this slice.
- `Tf` keeps `display_name` separate from optional external identity binding fields: `external_subject` and `identity_provider`.
- Full SSO/SCIM behavior is deferred; slice 001 only prevents display name from becoming the durable login identity.

## Rejected For This Slice

- Database schema terminology for tenant placement: rejected because DNAp has configurable document schemas as data.

## Deferred

- Genome name uniqueness inside an Insulator.
- Tf display name uniqueness inside an Insulator.
- Full authorization for Tf access across multiple Genomes; revisit in slice 006.
- SSO/SCIM login, provisioning, deprovisioning, group sync, and lifecycle handling.
- Production tenant provisioning CLI shape; slice 001 may expose backend behavior without making Insulator creation a tenant-user command.
- Normal user CLI for this slice; start CLI when a later slice has a tenant-user workflow command.
- Default region selection and detailed storage topology/provisioning automation beyond the minimal InsulatorPlacement contract.

## Deferred Domain Ledger

- Whether `Cell` remains deferred or returns as the software-system boundary; recovered spec included `Cell`, while the current vertical build removed it for now.
- Whether provisioning should include collection references on `Insulator` and `Genome` such as `gene_families`, `histones`, and `tf_classes`, or keep slice 001 to identity and ownership only.
- Whether external identity bindings become required for production tenants or remain optional per Insulator identity configuration.
- Whether `Tf` identity should include early relationship fields such as `tf_classes`, `histones`, `pre_initiation_complex`, `mediator_complex`, `repressors`, and `affinity`, or defer them to authorization/workflow slices.
- Whether local development needs a bootstrap command that creates an Insulator/Genome/Tf fixture without becoming the production tenant provisioning interface.

## Implementation Contract

- Names: `Insulator`, `InsulatorPlacement`, `InsulatorPlacementStrategy`, `SharedCluster`, `Genome`, `Tf`, and `TfId`.
- Structs: implement the slice 001 candidate structs only; do not add deferred relationship collections.
- Invariants: enforce tenant/project/user ownership boundaries, non-blank human-entered labels, explicit placement region, exactly one active placement per operational Insulator, and default placement strategy `SharedCluster`.
- CLI: no normal user CLI in slice 001; do not expose Insulator creation as a tenant-user CRUD workflow. Production/admin provisioning command shape is deferred.
- Approved tests:
  - Reject blank human-entered labels: `Insulator.name`, `Genome.name`, and `Tf.display_name`.
  - Require explicit non-blank placement region during Insulator provisioning.
  - Default omitted placement strategy to `SharedCluster`.
  - Reject cross-insulator or missing-Insulator ownership for `Genome` and `Tf`.

## Security Note

Actors: provisioning/admin caller for slice 001 backend behavior.
Boundaries: `Insulator` is the tenant isolation boundary; `Genome` and `Tf` must belong to exactly one existing Insulator.
Required checks: reject missing or cross-Insulator ownership and keep display names distinct from optional external identity bindings.
Abuse cases: creating project or actor records under the wrong tenant boundary.
Approved tests: cross-insulator or missing-Insulator ownership rejection for `Genome` and `Tf`.

## Old Vs New

Old previous LLD included `Cell` under platform boundaries.

New direction removes `Cell` for now. This slice only has tenant, project, and user.

Current code has `Genome` but uses it as the old tenant-ish boundary. New direction moves tenant responsibility to `Insulator` and makes `Genome` the project.

## Possible Loss

- Loss of `Cell` as software-system boundary.
- Loss of current `Genome` meaning as top-level tenant.
- Loss of current simple user-less author model.

## Implementation Gate

Do not implement until `Insulator`, `Genome`, and `Tf` are approved or replaced.

## Test Gate

No tests are approved yet. Derive tests from approved invariants and high-value externally observable behavior during slice design.
