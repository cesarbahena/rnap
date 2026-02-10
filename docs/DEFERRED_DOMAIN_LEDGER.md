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
- PreInitiationComplex is modeled as typed relationships among existing controlled documents, not as a standalone container object.
- PreInitiationComplex roots may be Promoters or eRNA.
- Enhancers associate to a Promoter through an Enhancer property, not a separate link object.
- eRNA is a flexible typed exploration graph node and may link to other eRNA nodes.
- Enhancers are formal research documents, such as business, technology, or market research.
- The Promoter association on an Enhancer points to the stable Promoter Locus, not a specific Allele.
- A Promoter may have many named exploration graphs.
- Each exploration graph belongs to exactly one Promoter.
- ExplorationGraph belongs to the stable Promoter Locus, not a specific Promoter Allele.
- ExplorationGraph is a Promoter-owned workflow artifact, not a controlled Gene.
- `Promoter -> eRNA` is represented through exploration graph containment, not a direct graph edge.
- Each exploration graph contains eRNA nodes, but the underlying eRNA controlled documents may be reused across graphs.
- Reused eRNA keeps one controlled document identity, while each exploration graph has its own node presentation state for that eRNA.
- eRNA work may later be canonized or transformed into other RNA document types.
- Canonizing eRNA creates a new target Transposon and Allele with provenance back to the source eRNA.
- Canonization does not change the source eRNA type or erase exploration history.
- `eRNA -> eRNA` is a directed typed graph edge inside an exploration graph.
- eRNA exploration graphs may be cyclic.
- ExplorationGraph owns graph topology.
- eRNA owns controlled document content.
- ExplorationNode is graph-local placement/presentation for a reusable eRNA Locus.
- ExplorationNode points to the stable eRNA Locus, not directly to an Allele.
- Whiteboard rendering resolves an eRNA Locus to the current active Allele unless a future snapshot/export feature requires historical resolution.
- ExplorationEdge belongs to one ExplorationGraph and connects graph-local ExplorationNodes, not reusable eRNA documents directly.
- Exploration graphs are intended to render as collaborative React whiteboards.
- Exploration graph nodes and edges need presentation metadata such as position, size, labels, and style.
- Exploration graph edges keep semantic relationship data separate from graph-local presentation state such as label, route, color, and stroke.
- Real-time collaboration will need operation-friendly changes such as creating, moving, resizing, labeling, and linking nodes, but CRDT/OT semantics are deferred.
- Do not add direct `Enhancer <-> eRNA` edges until a concrete workflow requires them.
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
