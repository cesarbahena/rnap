# 011 Genome-Wide Locus Names

Status: Accepted
Date: 2026-06-03
Slices: mutation CLI, matching, Locus identity

## Context

Target-first `dna mutate` depends on a clean first positional target. If Locus names were unique only per GeneFamily, users and LLMs would need to carry artifact type or family disambiguators for common commands.

## Decision

Locus names are unique within the containing Genome, regardless of GeneFamily.

The first positional target resolves by canonical kebab Locus name in the current Genome. DNAp does not allow two active Loci with the same canonical name in one Genome even if they belong to different GeneFamilies.

## Consequences

- `dna mutate checkout` can resolve without requiring a GeneFamily qualifier.
- `IDEA checkout` and `FRS checkout` cannot both be active Loci in the same Genome.
- If tenants need related artifacts, they should use distinct names or explicit relationships/workflow, not duplicate Locus identity.
- No `--family` or `FRS/checkout` disambiguation is approved for normal target matching.
