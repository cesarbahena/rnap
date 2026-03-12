# 002 Ribozyme Graph Deferral

Status: Accepted
Date: 2026-06-03
Slices: 007 and cross-slice taxonomy/model alignment

## Context

The old eRNA exploration graph behavior is no longer aligned with the target model. eRNA is now reserved for `NormalizedArtifact::Executable`: human-readable executable governance with IAM-like DSL attributes. Ribozyme is the approved normalized artifact for exploration, event storming, draft diagrams, early idea graphs, discovery narratives, and whiteboard collaboration.

The graph capabilities are expected to exist later, but the first implementation does not yet have a clear enough collaborative graph domain to justify persisted graph/node/edge records.

## Decision

For now, model Ribozyme as a normal Gene-capable `NormalizedArtifact` only.

Do not add persisted `ExplorationGraph`, `ExplorationNode`, `ExplorationEdge`, `RibozymeGraph`, `RibozymeNode`, `RibozymeEdge`, or equivalent graph-local records until a concrete Ribozyme graph use case is approved.

Future graph capabilities should be modeled around Ribozyme when the domain is clear.

## Consequences

- The immediate target is to migrate old eRNA exploration behavior toward Ribozyme taxonomy without preserving obsolete graph ownership assumptions.
- Ribozyme content may be represented through normal GeneFamily/SequenceDefinition/Locus/Allele/Gene lifecycle in the first pass.
- Collaborative whiteboard layout, graph-local node/edge presentation state, reuse semantics, and realtime operation semantics remain deferred.
- Code should not introduce a generic graph abstraction or revive Promoter-owned exploration graph behavior.
