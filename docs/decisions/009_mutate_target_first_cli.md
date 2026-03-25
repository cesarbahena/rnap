# 009 Mutate Target-First CLI

Status: Accepted
Date: 2026-06-03
Slices: mutation CLI, artifact creation, matching

## Context

The intended `dna mutate` UX was decided previously but not durably recorded. Current docs/code may still show `--new` before the artifact type and locus name, which obscures the invariant that the first positional argument is always the artifact identity the user is working on.

## Decision

`dna mutate` is target-first.

The first positional argument is always the Locus title/name target:

```sh
dna mutate "Some name for this requirement" --new IDEA
dna mutate some-name-for-this-requirement --some-field "My awesome idea"
```

Rules:

- The first positional argument is the Locus title/name, either human title form or kebab/CLI form.
- Existing Locus matching is forgiving/fuzzy enough for human and LLM use, subject to exact disambiguation when needed.
- New Locus creation is exact: if the target does not resolve and `--new <gene-family-abbreviation>` is present, DNAp creates a new Locus using that provided title/name.
- If the target does not resolve and `--new` is absent, the command errors.
- `--new` takes the GeneFamily abbreviation/type for creation.
- The older shape `dna mutate --new <family> <name>` is not the canonical CLI contract.

## Consequences

- CLI docs and code should migrate to `dna mutate <target> --new <family>`.
- The target-first invariant should hold across creation and mutation.
- Tenant-configured presentation can control saved/displayed names, but CLI matching remains forgiving for existing targets and exact for new target creation.
