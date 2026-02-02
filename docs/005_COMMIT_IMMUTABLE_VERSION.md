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
- Empty vector values are missing for commit completeness.
- The selected Allele must be `Expressing`; current `Unexpressed` Mutations must be expressed through `dna splice` or `dna splice --lgtm` before selection.
- The selected Allele no longer accepts Mutations after commit.
- Committed Gene is immutable history; active viewing/transcription of in-progress work uses the current Allele projection.

## Implementation Contract

Pending implementation.

## Approved Tests

Pending.
