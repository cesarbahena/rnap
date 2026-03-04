# DNAp Workflow Model

This document defines approved interaction rules among DNAp concepts. It does not define build order.

## Stable Foundation

Code should persist only concrete records and typed relationships with a defined workflow purpose.

The stable workflow foundation is intentionally small:

- Genome is the project boundary.
- Chromosome is a named canonical scope inside a Genome.
- GRN is the work context inside a Genome from triage onward.
- GRN owns operational lifecycle state.
- GRN may work across multiple Chromosomes.
- Operon groups Promoter artifacts inside one GRN.
- Promoter is a `NormalizedArtifact` on a GeneFamily.
- Alleles are shared per `(Locus, GRN)`.
- Triage assignment is accountability, not authorization.
- Authorization remains Histone-backed.
- Workflow-channel concepts are deferred until concrete use cases prove the records and relationships they need.

## GRN And Operons

Genome is the project boundary. GRN is the active initiative/work context inside a Genome.

GRN exists from triage onward. It is the work container before and after activation.

GRN can coordinate changes across multiple Chromosomes. Chromosome is canonical placement; GRN is change context.

GRN owns operational lifecycle state: Triage, Active, Blocked, or Closed.

GRN lifecycle state is not the workflow rule engine. Detailed readiness, authorization, dependency, and transition gates are derived from artifacts and future configurable workflow policy.

A GRN has one or more Operons. An Operon groups Promoter artifacts and represents higher-level intake structure such as an epic.

Operon does not own lifecycle state. GRN owns operational lifecycle state. Operon-specific readiness or blockage is derived from its Promoter memberships, artifacts, dependencies, and future configurable workflow policy until a concrete use case proves independent Operon state is needed.

Promoter is a NormalizedArtifact on a GeneFamily. A concrete Promoter reference is represented by the internal `Promoter(ArtifactRef)` wrapper, not by a raw `LocusId` in domain APIs.

A Promoter may be assigned to only one active Operon at a time.

Triage responsibility belongs to Promoter-in-Operon membership, not directly to the Promoter artifact.

Triage assignment is accountability, not authorization. `OperonPromoter.triage_tf` identifies who is responsible for routing and coordination, but it does not grant permissions by itself.

Unassigned Promoter membership is allowed while a GRN is in Triage. A GRN cannot become Active unless every active Promoter membership has exactly one triage Tf.

Only one active OperonPromoter may exist per Promoter.

If the same intake artifact appears relevant to multiple GRNs or Operons, model that as an explicit dependency, duplication, split, conflict, or another approved relationship. Do not model it as multiple active Operon membership.

CLI workflows may compose GRN creation, Operon creation, Promoter membership, and triage assignment into one user-facing action for ease of use.

Domain records and audit must keep those operations distinguishable. GRN creation, Operon creation, Promoter membership, and triage assignment are separate domain facts even when one CLI command performs them together.

## Alleles In GRNs

Alleles are shared candidate versions inside a GRN.

One active Allele is allowed per `(Locus, GRN)`.

Multiple GRNs may each have an active Allele for the same Locus. This is allowed because GRNs isolate parallel change contexts. Conflict detection and future resolution commands are deferred until concrete use cases define them.

Mutations record field-level edits on the shared Allele. Tf authorship, selection, degradation, movement, and other transition provenance are recorded through Signal.

## Deferred Workflow Concepts

The following concepts are preserved as product vocabulary, but they are not approved persisted objects, generic channel abstractions, or workflow rule engines:

- PreInitiationComplex
- MediatorComplex
- RepressorsComplex
- CRISPR
- StructuralMaintenance
- TfComplex

Do not abstract these concepts into a generic discussion-channel model before their concrete workflows prove shared structure.

Do not add unrestricted links between arbitrary Genes. Relationship targets must be constrained by approved use cases and should use typed artifact references when a relationship requires a specific `NormalizedArtifact`.

## Deferred Research And Exploration

EnterpriseNegotiationHandoverCertificate artifacts are formal enterprise negotiation/research handover documents. Their exact association rules are deferred.

ExplorationGraph ownership is not part of the active foundation. The old Promoter-owned graph model is outdated because GRNs now contain Operons with many Promoters.

Exploration graph ownership must be remodeled around GRN, Operon, or another concrete scope when the use cases are clear.

ExplorationGraph remains workflow topology/presentation state, not controlled document content.

Do not add new Promoter-owned exploration graph behavior until the ownership scope is redesigned.

Ribozyme owns relationship/dependency/exploration topology. If a whiteboard or diagram use case needs graph-local state, graph/node/edge records belong to Ribozyme modeling.

A Ribozyme may point to Genes, Loci, Alleles, or other approved typed references when concrete graph use cases are approved.

ExplorationNode is graph-local placement/presentation for a reusable artifact reference.

ExplorationNode points to stable artifact identity unless a concrete use case requires candidate or historical resolution.

Whiteboard rendering resolves a Ribozyme-referenced Locus to the current active Allele unless a future snapshot/export feature requires historical resolution.

ExplorationEdge belongs to one ExplorationGraph and connects graph-local ExplorationNodes, not reusable controlled artifacts directly.

Ribozyme exploration graphs may be cyclic.

Exploration graphs are intended to render as collaborative React whiteboards.

Exploration graph nodes and edges need presentation metadata such as position, size, labels, and style.

Exploration graph edges keep semantic relationship data separate from graph-local presentation state such as label, route, color, and stroke.

Real-time collaboration will need operation-friendly changes such as creating, moving, resizing, labeling, and linking nodes. CRDT/OT semantics are deferred.

Do not add direct `EnterpriseNegotiationHandoverCertificate <-> Ribozyme` edges until a concrete workflow requires them.

The same controlled artifact may be reused across Ribozyme graphs.

Reused artifacts keep one controlled document identity, while each graph node has graph-local presentation state.

## Deferred Channel Notes

The old channel notes remain useful as recovery context only:

- RepressorsComplex may eventually use ProjectedIntent, Microalignment, StopImplementation, or DeferredScope artifacts.
- MediatorComplex may eventually involve Intron, SemanticNarrowing, SemanticConstraintAssumption, StrategicNote, and TaskMediation artifacts.
- CRISPR may eventually involve Protospacer, CausalResolution, TraceReport, CountermeasureAssessmentSystem, and SuggestedChanges artifacts.
- StructuralMaintenance may eventually govern deployment and runtime artifacts.

These statements do not approve object shapes, command behavior, lifecycle state, or relationship cardinality.
