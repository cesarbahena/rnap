# 005 Commit Immutable Version

## Capability

Select an Allele and create an immutable committed Gene.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

Slice 005 implements:

- `Gene`

## Behavior

- Gene is an immutable committed version.
- Selecting an Allele creates a Gene.
- Gene stores the committed sequence snapshot.
- Gene records the source Allele in `selected_from`.
- Gene generation increments within a Locus.
- All sequences required by the GeneFamilyGeneration must be present before selection.
- The selected Allele no longer accepts Mutations after commit.

## Implementation Contract

Pending implementation.

## Approved Tests

Pending.
