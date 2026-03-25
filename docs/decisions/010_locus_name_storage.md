# 010 Locus Name Storage

Status: Accepted
Date: 2026-06-03
Slices: mutation CLI, matching, Locus identity, presentation

## Context

`dna mutate` is target-first. The first positional argument is the Locus title/name target. Existing targets are matched by canonical Locus name, and new targets are created when `--new <GeneFamily abbreviation>` is present. The storage/presentation split needed an explicit decision.

## Decision

Store Locus names in canonical kebab form.

Human title-like names are derived for presentation. User input may be title form, kebab form, or other CLI-friendly forms, but persisted `Locus.name` stores the canonical kebab identity.

Examples:

```sh
dna mutate "Some name for this requirement" --new IDEA
```

stores:

```text
Locus.name = "some-name-for-this-requirement"
```

Later inputs such as these resolve to the same Locus under canonical matching:

```sh
dna mutate some-name-for-this-requirement
dna mutate "Some name for this requirement"
```

Do not add a separate domain field such as `normalized_locus_name`; derived lookup keys remain storage/index details.

## Consequences

- Locus identity is stable and CLI-friendly.
- Title presentation is derived from kebab identity unless/until tenant presentation configuration is approved.
- Creation duplicate checks use exact canonical kebab identity, not fuzzy similarity.
- FQN matching remains deferred and is not part of current active target matching.
