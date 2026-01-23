# 001 Provision Tenant, Project, User

## Capability

Provision the minimum operational DNAp world:

- one Insulator,
- one active InsulatorPlacement,
- one Genome,
- one Tf.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

Slice 001 implements:

- `Insulator`
- `InsulatorPlacement`
- `InsulatorPlacementStrategy::SharedCluster`
- `Genome`
- `Tf`

## Behavior

- Insulator creation is provisioning/admin behavior, not a normal tenant-user CLI workflow.
- Provisioning callers must provide an explicit placement region.
- If no placement strategy is specified, DNAp assigns `SharedCluster`.
- Each operational Insulator has exactly one active placement.
- `Insulator.name`, `Genome.name`, `Tf.display_name`, and placement `region` must not be blank.
- `Tf.display_name` is a tenant-facing label, not login identity.
- `Tf.external_subject` and `Tf.identity_provider` reserve a future external identity binding.
- A Genome belongs to exactly one Insulator.
- A Tf belongs to exactly one Insulator.

## CLI

No normal tenant-user CLI is implemented in this slice. Production/admin provisioning command shape is deferred.

## Approved Tests

- Reject blank human-entered labels: `Insulator.name`, `Genome.name`, and `Tf.display_name`.
- Require explicit non-blank placement region during Insulator provisioning.
- Default omitted placement strategy to `SharedCluster`.
- Reject missing-Insulator ownership for `Genome` and `Tf`.

## Status

Implemented in `src/lib.rs`.
