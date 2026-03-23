# 005 Exon Renamed to tRNA

Status: Accepted
Date: 2026-06-03
Slices: Splicing, artifact taxonomy/model alignment

## Context

Current code used `Exon` for the records created by `dna splice`, but the intended semantic role is task realization work, not a refined-requirement artifact called Exon.

## Decision

Rename the current `Exon` bridge concept to tRNA / `TaskRealization` semantics.

Use `TaskRealization` as the enterprise semantic code name where a full type name is needed. Keep `tRNA` as the biology/backronym alias and CLI lineage. Do not use `Exon` for the current splice/task-realization records.

## Consequences

- `dna splice` creates TaskRealization records.
- The current task DAG belongs to TaskRealization semantics.
- `NormalizedArtifact::Exon` remains only a taxonomy value until a concrete refined-requirement artifact lifecycle is approved or reconsidered.
