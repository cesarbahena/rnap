# 004 Mutate Candidate Work

## Capability

Apply append-only Mutations to an Allele and project candidate state.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

Slice 004 implements:

- `Mutation`
- `Sequence`
- `SequenceValue`

## Behavior

- Mutations are append-only.
- Mutations target SequenceDefinitions by `SequenceDefinitionId`.
- A Mutation must target an existing SequenceDefinition in the Allele's GeneFamilyGeneration.
- Mutation values must match the SequenceDefinition type.
- Current candidate state is projected from Mutations.
- A degraded Allele cannot accept new Mutations.
- Mutations are created by Tfs.
- CLI mutation flags use the approved sequence-name matcher from slice 002.

## Implementation Contract

Pending implementation.

## Approved Tests

Pending.
