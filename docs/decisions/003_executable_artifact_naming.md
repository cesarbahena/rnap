# 003 Executable Artifact Naming

Status: Accepted
Date: 2026-06-03
Slices: Cross-slice taxonomy/model alignment

## Context

`NormalizedArtifact` enum variants already complete their meaning with the normalized-artifact context. Repeating `NormalizedArtifact` or adding heavy suffixes inside variant names makes the model less lean and less punchy.

The executable regulatory artifact currently carries the eRNA lineage, but future taxonomy may split regulatory artifacts under a nested shape such as `NormalizedArtifact::Regulatory(...)`. That subdivision is not approved yet.

## Decision

Use `Executable` as the canonical `NormalizedArtifact` variant name for the eRNA artifact.

Do not use `ExecutableRegulatoryNormalizedArtifact` as a canonical enum variant or primary model name. Treat eRNA as the biology/backronym alias and lineage term for `NormalizedArtifact::Executable`.

A future `NormalizedArtifact::Regulatory` sub-enum may be considered later, but is not part of the current implementation contract.

## Consequences

- `NormalizedArtifact::Executable` is the current target taxonomy value.
- Docs and code should avoid repeated `NormalizedArtifact` suffixes inside normalized artifact names.
- eRNA remains valid shorthand/lineage language for docs, CLI aliases, and internal roles where useful.
- Regulatory taxonomy nesting remains deferred.
