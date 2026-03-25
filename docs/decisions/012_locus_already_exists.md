# 012 Locus Already Exists on New Mutation

Status: Accepted
Date: 2026-06-03
Slices: mutation CLI, matching, Locus identity

## Context

`dna mutate` is target-first and Locus names are unique Genome-wide. Creation with `--new` must not silently mutate an existing Locus or create a duplicate under a different GeneFamily.

## Decision

When `dna mutate <target> --new <family>` is used and the canonical kebab target already exists anywhere in the current Genome, DNAp errors with `LocusAlreadyExists`.

When `--new` is absent, an existing canonical target mutates the active Allele for that Locus in the current GRN.

## Consequences

- `--new` always means create a new Locus.
- Duplicate target creation fails even if the existing Locus belongs to a different GeneFamily.
- The user should omit `--new` to mutate existing work or choose a different Locus name.
