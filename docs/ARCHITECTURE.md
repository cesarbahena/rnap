# Architecture

DNAp uses a backend-first modular core with adapter boundaries.

This is an explicit boundary commitment, not a commitment to a full Clean Architecture framework, CQRS, event sourcing, microservices, or per-entity repositories. The current goal is to keep the domain model and application behavior clear while the ontology, authorization, workflow, collaboration, and persistence requirements continue to mature.

## Module Direction

- `src/app/`: domain model plus application behavior for the in-process core.
- `src/session.rs`: replaceable session boundary plus local user-level state storage for development workflows.
- `src/cli.rs`: thin `dna` command parser/dispatcher. Dynamic Sequence flags are parsed here and sent to application behavior as structured mutations.
- `src/main.rs`: binary entry point only.
- `src/lib.rs`: public crate boundary and application re-exports.

As `src/app/` grows, split it by durable product boundaries rather than by storage tables:

- identity: Insulator, placement, Genome, Tf.
- artifact taxonomy: fixed NormalizedArtifact taxonomy and internal biology/backronym lineage.
- genes: GeneFamily, GeneFamilyGeneration, SequenceDefinition, Locus, Transposon, Allele, Gene, Mutation.
- workflow: Transcriptome cursor, current Exon bridge records, and SemanticNarrowing clarification threads.
- application use cases: mutation, transcription, splicing, translation, exploration, questions/answers.
- matching: human-oriented normalized/fuzzy command resolution shared by CLI-facing use cases.

## Boundary Rules

- Domain modules own names, invariants, value types, and durable product concepts.
- Application modules own use cases and enforce cross-entity invariants.
- `LocusId` is raw storage identity. Use resolved artifact references when a relationship requires a specific NormalizedArtifact.
- `ArtifactRef` carries a resolved `LocusId` plus its NormalizedArtifact invariant. Internal biological/backronym wrappers such as `MRna(ArtifactRef)` or `Enhancer(ArtifactRef)` may be used where the role matters.
- NormalizedArtifact enum variants use full enterprise semantic names. Biological/backronym names are internal role wrappers, aliases, CLI aliases, or documentation lineage, not tenant-facing defaults.
- CLI parsing must stay thin and must not duplicate backend/application rules.
- Local JSON state is an adapter for development workflows, not product storage architecture.
- Session state carries actor and scope in a way that can later be replaced by real login/auth.
- Do not introduce repository abstractions until a second storage adapter or placement requirement makes the contract real.
- Do not create a god module for new workflow concepts. If a concept has approved behavior, put its types and use case behavior near the owning product boundary.

## Runtime Boundaries

- Tenant identity: `Insulator`.
- Project scope: `Genome`.
- Actor identity: `Tf`.
- Tenant content: GeneFamily, Locus, Allele, Mutation, Transcriptome cursor, current Exon bridge records, and SemanticNarrowing clarification threads.
- Local CLI session: actor and scope only; no real auth secrets.
- Authorization: not implemented yet; reserved for Histone/HistoneMark slices.
- Persistence: local JSON state adapter for CLI workflow testing; not product storage architecture.

`dna epigenetics` is the current superadmin bootstrap namespace. Future real login/auth should replace the session provider, not the normal workflow commands.

## Test Direction

Rust unit tests belong near the code they exercise when private behavior must be checked. Cross-module behavior should be covered through public application APIs and CLI behavior through CLI dispatch/output tests only when CLI semantics are the contract.

Tests should assert externally meaningful behavior and approved invariants, not private counters, file layout, timestamps, or local JSON mechanics.
