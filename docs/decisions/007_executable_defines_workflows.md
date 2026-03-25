# 007 Executable Defines Workflows

Status: Accepted
Date: 2026-06-03
Slices: workflow governance, authorization, future selection, GRN lifecycle

## Context

eRNA / `NormalizedArtifact::Executable` was previously described as executable governance, but its first regulated transition was unresolved. The approved direction is broader: Executable artifacts define DNAp workflows rather than only gating one command.

## Decision

`NormalizedArtifact::Executable` is the workflow-definition artifact.

Executable/eRNA artifacts define workflows across DNAp, including command/state transitions, lifecycle gates, dependency requirements, and governance checks as those workflows are approved.

Do not reduce eRNA to a single transition such as `dna select`, GRN activation, mutation acceptance, or splice readiness. Those may become workflow steps under Executable definitions, but eRNA's role is to define the workflow model that governs them.

## Consequences

- Future workflow behavior should be designed as Executable-defined protocol, not hardcoded lifecycle lists on individual domain records.
- `dna select` remains unapproved until its product meaning is decided inside the broader workflow model.
- Histones remain governance/context facts; Signals remain provenance facts; Executable workflows may read those facts when approved.
- The first implementation must still choose a narrow Executable workflow slice before code is written.
