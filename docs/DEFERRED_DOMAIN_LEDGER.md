# DNAp Deferred Domain Ledger

The ledger contains implementation obligations and open decisions that are intentionally not implemented yet. Product terms live in [ONTOLOGY.md](ONTOLOGY.md). Approved workflow interactions live in [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).

## Platform Boundaries

- `Cell`: software-system boundary. Deferred until system boundaries and `@Cell` references are needed.
- `@Cell` references: resolve eagerly or lazily. Deferred to the mutation/reference model.

## Tenant And Identity

- SSO/SCIM login, provisioning, deprovisioning, group sync, and lifecycle handling. Deferred beyond slice 001.
- External identity bindings becoming required for production tenants. Deferred to identity configuration.
- Tf relationship fields: `tf_classes`, `histones`, `pre_initiation_complex`, `mediator_complex`, `repressors`, and `affinity`. Deferred to authorization and workflow slices.

## Document Types

- Implement `NormalizedArtifact` as the canonical GeneFamily taxonomy and replace the older `EncodingType`/RNA/GRN split in code.
- Schema evolution and rename tracking across GeneFamilyGenerations.
- `SequenceValue` validation during work-type definition versus mutation application.
- Exact Gene FQN configuration storage and override implementation.
- Artifact-specific lifecycle, dependency, authorization, and tenant workflow semantics. Deferred until concrete use cases define the needed configurable structures; no ChromatinRemodeler design is approved yet.

## Candidate Work

- Allele initial sequence values versus all values arriving through Mutations.
- Degraded candidate behavior before full authorization.

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

## Workflow

- Non-human actor identity for agents and services. Likely direction: keep a unified Tf identity model, but defer `TfKind`, delegation fields, ownership, and control relationships until concrete use cases define the configurable structures needed, likely alongside Histone-backed authorization and context.
- Implement PreInitiationComplex behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md), including Promoter-owned exploration graphs and EnterpriseNegotiationHandoverCertificate Promoter properties.
- Implement ExplorationGraph, ExplorationNode, and ExplorationEdge as workflow artifacts for collaborative whiteboards.
- Decide and implement operation/revision semantics for real-time graph collaboration. CRDT/OT semantics are deferred.
- Implement RepressorsComplex behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).
- Implement MediatorComplex behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).
- Implement CRISPR behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).
- Implement structural maintenance behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).
- Whether mutation proposals must pass through SgRNA or direct Mutation remains valid.

## Implementation Evaluation

- Whether Protein represents output for an Allele, a Gene, or both.
- Fold metadata beyond commit SHA: repository, branch, PR, workflow run, artifact URI, environment, and evaluator provenance.
- Ribosome, RRNA, Chaperone, and Chiasma product roles.
- Phenotype and Phenome as reporting/read-model concepts.
