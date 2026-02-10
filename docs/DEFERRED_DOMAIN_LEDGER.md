# DNAp Deferred Domain Ledger

The ledger contains product concepts that are intentionally not implemented yet. Each entry names the owning slice or decision point.

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
- Update the Rust `EncodingType` enum and CLI parser to the current taxonomy: `Promoter`, `Enhancer`, `PIWI`, `Spacers`, `Telomere`, `Centromere`, `Silencer`, `eRNA`, `mRNA`, `rRNA`, `tRNA`, `Intron`, `snRNA`, `scaRNA`, `siRNA`, `tmRNA`, `gRNA`, `miRNA`, `piRNA`, `snoRNA`, `crRNA`, `tracrRNA`, `lncRNA`, `circRNA`, and `sgRNA`.
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
- TfComplex discussion/alignment subsystem and its named areas: PreInitiationComplex, RepressorsComplex, MediatorComplex, CRISPR, and structural maintenance.
- TfComplex is not automatically a persisted container object. Persist only concrete discussion records and typed relationships that have a defined workflow purpose.
- TfComplex does not define Gene schemas and does not own workflow rules; application behavior interprets rules over concrete discussion records and relationship edges.
- TfComplex target modeling must be constrained per use case and must support both whole-document targets and internal work-item targets.
- PreInitiationComplex uses Promoters or eRNA as open issues with Enhancers and eRNA follow-ups. eRNA can create a graph.
- RepressorsComplex uses piRNA or miRNA open issues and may emit siRNA. Authoritative siRNA issues may also be roots.
- MediatorComplex uses Intron open issues with Intron follow-ups, snRNA task-modification suggestions for mRNA, scaRNA requirement-modification suggestions from implementation reality, and snoRNA ADR discussions for rRNA.
- CRISPR uses proto-spacer emergent issues or crRNA incident reports, PAM exploratory evidence, tracrRNA root cause analysis, and Cas actions.
- sgRNA is repurposed for suggested CRISPR action changes.
- Structural maintenance is a deployment discussion channel for Centromeres and microtubule tasks.
- Whether mutation proposals must pass through SgRNA or direct Mutation remains valid.
- GRNA generation snapshots, model/config metadata, and provenance.

Reference shape for the named TfComplex concept:

```rust
struct TfComplex {
    pre_initiation_complex: Vec<PreInitiator>,
    repressors_complex: Vec<Repressor>,
    mediators_complex: Vec<Mediator>,
    unaligned_product_response: Vec<Upr>,
    crispr_cas: Vec<Crispr>,
    structural_mantainance_complex: Vec<Smc>,
}
```

This shape preserves the named conceptual groups. It is not approval to persist a monolithic `TfComplex` object.

## Implementation Evaluation

- Whether Protein represents output for an Allele, a Gene, or both.
- Fold metadata beyond commit SHA: repository, branch, PR, workflow run, artifact URI, environment, and evaluator provenance.
- Ribosome, RRNA, Chaperone, and Chiasma product roles.
- Phenotype and Phenome as reporting/read-model concepts.
