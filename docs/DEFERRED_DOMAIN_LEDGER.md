# DNAp Deferred Domain Ledger

The ledger contains implementation obligations and open decisions that are intentionally not implemented yet. Product terms live in [ONTOLOGY.md](ONTOLOGY.md). Approved workflow interactions live in [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).

## Platform Boundaries

- `Cell`: software-system boundary. Deferred until system boundaries and `@Cell` references are needed.
- `@Cell` references: resolve eagerly or lazily. Deferred to the mutation/reference model.

## Tenant And Identity

- SSO/SCIM login, provisioning, deprovisioning, group sync, and lifecycle handling. Deferred beyond slice 001.
- External identity bindings becoming required for production tenants. Deferred to identity configuration.
- Tf relationship fields: `tf_classes`, `histones`, and future workflow participation/provenance fields. Deferred to authorization and workflow slices. Do not add `pre_initiation_complex`, `mediator_complex`, `repressors`, or `affinity` fields to Tf without concrete use cases.

## Document Types

- Implement `NormalizedArtifact` as the canonical GeneFamily taxonomy and replace the older `EncodingType`/RNA/GRN split in code.
- Implement `ArtifactRef` plus narrowly useful biology/backronym wrappers such as `MRna(ArtifactRef)` or `Enhancer(ArtifactRef)` where relationships require a specific NormalizedArtifact.
- Implement Chromosome as the named canonical scope inside a Genome.
- Move Locus identity to Chromosome scope while keeping Locus names unique across the containing Genome.
- Implement Locus movement between Chromosomes with Signal audit.
- Schema evolution and rename tracking across GeneFamilyGenerations.
- `SequenceValue` validation during work-type definition versus mutation application.
- Exact Gene FQN configuration storage and override implementation.
- Artifact-specific lifecycle, dependency, authorization, and tenant workflow semantics. Deferred until concrete use cases define the needed structures. eRNA is reserved for human-readable executable governance artifacts with IAM-like DSL attributes.
- NormalizedArtifact transition from current code's older `EncodingType` split. Docs define the desired model; code migration remains pending.

## Candidate Work

- Allele initial sequence values versus all values arriving through Mutations.
- Degraded candidate behavior before full authorization.
- Migrate active Allele uniqueness from `(Locus, Tf)` to shared `(Locus, GRN)`.

## Audit

- Rename DomainEvent to Signal and implement append-only Signal with tenant-scoped `insulator_id`, optional `tf_id`, typed target, typed payload, reason, and timestamp.
- Keep `degraded_at` as the standard soft-delete/deactivation field for active filtering.
- Remove per-record actor audit fields such as `created_by`, `degraded_by`, `assigned_by`, and `moved_by` unless a concrete query requires denormalization.
- Keep behavior-validity timestamps such as HistoneMark validity windows when they affect behavior.

## Mutations

- Mutation rationale/context before workflow artifacts.
- Delete/clear semantics for sequence values.
- Optimistic concurrency/version preconditions for concurrent Tf or agent edits.
- CLI mutation flags using the approved sequence-name matcher.

## Splicing

- Exact Exon lifecycle after `dna splice` under the NormalizedArtifact model.
- Reconcile current Exon task records with long-term Exon-as-Gene artifact semantics.

## Authorization

- `TfClassScope` enum versus `insulator_id` plus optional `genome_id`.
- Full ChromatinState resolution semantics.
- Time-bound marks through `valid_from` and `valid_until`.
- Degraded auditable records beyond authorization records.
- Whether default Histone policy grants triage owners common routing permissions. Triage assignment itself is accountability, not authorization.

## Workflow

- Non-human actor identity for agents and services. Likely direction: keep a unified Tf identity model, but defer `TfKind`, delegation fields, ownership, and control relationships until concrete use cases define the configurable structures needed, likely alongside Histone-backed authorization and context.
- Implement minimal GRN and Operon active work context. Decision approved: Genome is project boundary, GRN is work context from triage onward, GRN owns operational lifecycle state, Operon groups Promoters, a Promoter may belong to only one active Operon at a time, activation requires one triage Tf for each active Promoter membership, and triage assignment is accountability rather than authorization.
- Allow one GRN to work across multiple Chromosomes through its Alleles.
- Allow multiple GRNs to mutate the same Locus through separate shared Alleles. Conflict detection/resolution is deferred.
- Operon lifecycle state. Deferred because GRN owns operational lifecycle state; revisit only when an Operon needs independent workflow status separate from its GRN.
- PreInitiationComplex structure. Decision approved: keep it as a named discussion concept only; no persisted object, GRN field, participant model, topic model, or generic channel abstraction is approved until concrete use cases define it.
- Generic discussion-channel abstraction. Deferred because PreInitiationComplex, MediatorComplex, RepressorsComplex, CRISPR, and StructuralMaintenance do not yet have enough shared approved behavior to justify a common object.
- EnterpriseNegotiationHandoverCertificate association rules. Deferred; it remains a NormalizedArtifact, but exact Promoter/GRN/Operon attachment semantics are not active foundation.
- Remodel ExplorationGraph, ExplorationNode, and ExplorationEdge ownership around GRN/Operon or another approved scope. Previous Promoter-owned graph behavior is no longer foundation because GRNs contain Operons with many Promoters.
- Replace the old eRNA exploration role with Ribozyme.
- Reserve eRNA for `ExecutableRegulatoryNormalizedArtifact`, a human-readable executable governance artifact with IAM-like DSL attributes.
- Add `ExecutableRegulatoryNormalizedArtifact` and `Ribozyme` to NormalizedArtifact in code.
- Decide and implement operation/revision semantics for real-time graph collaboration. CRDT/OT semantics are deferred.
- Explore RepressorsComplex behavior only through concrete approved use cases.
- Explore MediatorComplex behavior only through concrete approved use cases.
- Explore CRISPR behavior only through concrete approved use cases.
- Explore StructuralMaintenance behavior only through concrete approved use cases.
- Whether mutation proposals must pass through SgRNA or direct Mutation remains valid.

## Implementation Evaluation

- Whether Protein represents output for an Allele, a Gene, or both.
- Fold metadata beyond commit SHA: repository, branch, PR, workflow run, artifact URI, environment, and evaluator provenance.
- Ribosome, RRNA, Chaperone, and Chiasma product roles.
- Phenotype and Phenome as reporting/read-model concepts.
