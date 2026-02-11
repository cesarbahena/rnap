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

- Schema evolution and rename tracking across GeneFamilyGenerations.
- `SequenceValue` validation during work-type definition versus mutation application.
- Exact Gene FQN configuration storage and override implementation.
- Keep `Exon` and `Cas` out of `EncodingType`.

## Candidate Work

- Allele initial sequence values versus all values arriving through Mutations.
- Degraded candidate behavior before full authorization.

## Mutations

- Mutation rationale/context before workflow artifacts.
- Delete/clear semantics for sequence values.
- Optimistic concurrency/version preconditions for concurrent Tf or agent edits.
- CLI mutation flags using the approved sequence-name matcher.

## Splicing

- Exact Exon lifecycle after `dna splice`.
- Exons are executable tasks created by `dna splice`, not EncodingTypes.

## Authorization

- `TfClassScope` enum versus `insulator_id` plus optional `genome_id`.
- Full ChromatinState resolution semantics.
- Time-bound marks through `valid_from` and `valid_until`.
- Degraded auditable records beyond authorization records.

## Workflow

- Whether agents are Tfs, TfClass membership, or a separate actor type.
- Implement PreInitiationComplex behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md), including Promoter-owned exploration graphs and Enhancer Promoter properties.
- Implement ExplorationGraph, ExplorationNode, and ExplorationEdge as workflow artifacts for collaborative whiteboards.
- Decide and implement operation/revision semantics for real-time graph collaboration. CRDT/OT semantics are deferred.
- Implement eRNA canonization into target RNA Transposon and Allele with provenance.
- Implement RepressorsComplex behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).
- Implement MediatorComplex behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).
- Implement CRISPR behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).
- Implement structural maintenance behavior from [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md).
- Whether mutation proposals must pass through SgRNA or direct Mutation remains valid.
- GRNA generation snapshots, model/config metadata, and provenance.

## Implementation Evaluation

- Whether Protein represents output for an Allele, a Gene, or both.
- Fold metadata beyond commit SHA: repository, branch, PR, workflow run, artifact URI, environment, and evaluator provenance.
- Ribosome, RRNA, Chaperone, and Chiasma product roles.
- Phenotype and Phenome as reporting/read-model concepts.
