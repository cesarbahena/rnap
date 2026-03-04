# 002 Define Work Type

## Capability

Define configurable SDLC document types through GeneFamily and GeneFamilyGeneration.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md) and normalized artifact taxonomy in [ENCODING_TAXONOMY.md](ENCODING_TAXONOMY.md).

Slice 002 implements:

- `GeneFamily`
- `GeneFamilyGeneration`
- `SequenceDefinition`
- `SequenceType`
- `NormalizedArtifact`

## Behavior

- GeneFamily defines a configurable SDLC document/work type.
- GeneFamily definitions may be Insulator-scoped or Genome-scoped.
- Genome-scoped GeneFamilies are project-level overrides.
- Lookup from a Genome context resolves Genome-scoped override first, then Insulator-scoped default.
- GeneFamily abbreviations are unique in effective scope.
- Genome-scoped GeneFamilies may shadow Insulator-scoped abbreviations.
- GeneFamilyGeneration is immutable after creation.
- A GeneFamily points to one current GeneFamilyGeneration.
- Generation numbers live on GeneFamilyGeneration.
- GeneFamilyGeneration id type is `GeneFamilyGenerationId`.
- NormalizedArtifact is required.
- NormalizedArtifact values are system-fixed.
- NormalizedArtifact replaces the older EncodingType/RNA/GRN split as the canonical taxonomy.
- All NormalizedArtifact variants are Gene-capable artifact types.
- NormalizedArtifact variants use full enterprise semantic names.
- Internal typed artifact-reference wrappers may use biology-inspired names/backronyms when the relationship specifically needs that role.
- SequenceDefinition has stable identity through `SequenceDefinitionId`.
- SequenceDefinition names are enterprise-native tenant data.
- SequenceDefinition names are unique inside a GeneFamilyGeneration.
- All SequenceDefinitions in a GeneFamilyGeneration are required before commit.

## CLI Name Matching

CLI sequence-name matching:

- accepts kebab-case user input,
- is case-insensitive,
- supports fuzzy matching,
- resolves to exactly one SequenceDefinition and returns its id internally,
- errors on zero or multiple matches,
- is disambiguated by more exact input.

Implementing mutation CLI flags that use this matcher belongs to slice 004.

## Implementation Contract

- Implement backend/application behavior for defining `GeneFamily` and `GeneFamilyGeneration`.
- Do not add tenant-user CLI commands in this slice.
- Keep CLI sequence-name matching specified here and implement it when slice 004 adds mutation commands.

## Approved Tests

- Reject blank `GeneFamily.name`.
- Reject blank `GeneFamily.abbreviation`.
- Reject blank `SequenceDefinition.name`.
- Reject duplicate `SequenceDefinition.name` inside one GeneFamilyGeneration.
- Require `NormalizedArtifact`.
- Allow Genome-scoped GeneFamily to shadow an Insulator-scoped abbreviation.
- Reject duplicate abbreviations in the same effective scope.
- Resolve GeneFamily lookup from Genome context as Genome override first, then Insulator default.
