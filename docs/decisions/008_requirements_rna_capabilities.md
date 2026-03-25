# 008 Requirements RNA Capabilities

Status: Accepted
Date: 2026-06-03
Slices: requirements modeling, artifact taxonomy, mutation/splice workflow

## Context

DNAp needs the natural capabilities of each RNA/NA artifact before eRNA workflow use cases can be safely defined. The requirements cluster has caused semantic confusion, especially old code using `Intron` for what is now `SemanticNarrowing` behavior and old `Exon` records that are now `TaskRealization`.

## Decision

The requirements-cluster capabilities are:

- `Intron`: raw requirement input. It captures messy, partial, stakeholder/customer/business statements. Its job is capture, not precision.
- `ManagedRequirement` / mRNA: managed requirement document. It is the controlled requirement artifact that organizes accepted requirement content and tracks it through workflow.
- `Exon`: refined atomic requirement. It is precise and testable, extracted/refined from raw Intron material and/or mRNA discussion. Exons may compose into an mRNA or be referenced by it.
- `SemanticNarrowing` / snRNA: clarification question/answer that narrows ambiguous meaning.
- `SemanticConstraintAssumption` / scaRNA: explicit assumption or constraint that stabilizes interpretation when certainty is impossible.
- `TaskRealization` / tRNA: task/work realization of requirement content.

## Consequences

- Do not use `Intron` for clarification threads; that is `SemanticNarrowing`.
- Do not use `Exon` for current `dna splice` task records; that is `TaskRealization`.
- Future eRNA workflows for requirements should be based on these capabilities rather than hardcoded command assumptions.
