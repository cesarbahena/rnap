# DNAp Workflow Model

This document defines approved interaction rules among DNAp concepts. It does not define build order.

## General Rule

Code should persist only concrete records and typed relationships with a defined workflow purpose.

`TfComplex` does not define Gene schemas, does not own workflow rules, and does not replace GeneFamily or NormalizedArtifact. Application behavior interprets workflow rules over concrete records and relationship edges.

TfComplex targets must be constrained per use case. DNAp must not add unrestricted links between arbitrary Genes.

## GRN And Operons

Genome is the project boundary. GRN is the active initiative/work context inside a Genome.

A GRN has one or more Operons. An Operon groups Promoter artifacts and represents higher-level intake structure such as an epic.

Promoter is a NormalizedArtifact on a GeneFamily. A concrete Promoter reference is represented by the internal `Promoter(ArtifactRef)` wrapper, not by a raw `LocusId` in domain APIs.

A Promoter may be assigned to only one active Operon at a time.

If the same intake artifact appears relevant to multiple GRNs or Operons, model that as an explicit dependency, duplication, split, conflict, or another approved relationship. Do not model it as multiple active Operon membership.

## PreInitiationComplex

PreInitiationComplex uses existing Promoter, EnterpriseNegotiationHandoverCertificate, and ExploratoryNarrative controlled documents.

### Promoter And Enterprise Negotiation Handover

EnterpriseNegotiationHandoverCertificate artifacts associate to a Promoter through a Promoter property, not a separate link object.

The Promoter association points to the stable Promoter Locus, not a specific Promoter Allele.

EnterpriseNegotiationHandoverCertificate is a NormalizedArtifact, so the exact research/negotiation schema comes from the GeneFamily.

### Promoter And Exploration Graphs

A Promoter may have many named exploration graphs.

Each ExplorationGraph belongs to exactly one Promoter and is owned by the stable Promoter Locus, not a specific Promoter Allele.

ExplorationGraph is a Promoter-owned workflow artifact, not a controlled Gene.

`Promoter -> ExploratoryNarrative` is represented through exploration graph containment, not a direct graph edge.

### Exploratory Narrative Graphs

ExploratoryNarrative owns controlled document content.

ExplorationGraph owns graph topology.

An ExploratoryNarrative may serve as a root node inside an exploration graph.

ExplorationNode is graph-local placement/presentation for a reusable ExploratoryNarrative Locus.

ExplorationNode points to the stable ExploratoryNarrative Locus, not directly to an Allele.

Whiteboard rendering resolves an ExploratoryNarrative Locus to the current active Allele unless a future snapshot/export feature requires historical resolution.

ExplorationEdge belongs to one ExplorationGraph and connects graph-local ExplorationNodes, not reusable ExploratoryNarrative documents directly.

ExploratoryNarrative exploration graphs may be cyclic.

Exploration graphs are intended to render as collaborative React whiteboards.

Exploration graph nodes and edges need presentation metadata such as position, size, labels, and style.

Exploration graph edges keep semantic relationship data separate from graph-local presentation state such as label, route, color, and stroke.

Real-time collaboration will need operation-friendly changes such as creating, moving, resizing, labeling, and linking nodes. CRDT/OT semantics are deferred.

Do not add direct `EnterpriseNegotiationHandoverCertificate <-> ExploratoryNarrative` edges until a concrete workflow requires them.

### Exploratory Narrative Reuse

The same ExploratoryNarrative controlled document may be reused across graphs.

Reused ExploratoryNarrative keeps one controlled document identity, while each graph node has graph-local presentation state.

## RepressorsComplex

RepressorsComplex uses piRNA or miRNA open issues and may emit siRNA.

Authoritative siRNA issues may also be roots.

## MediatorComplex

MediatorComplex uses Intron open issues with Intron follow-ups.

MediatorComplex may use snRNA follow-ups as task modification suggestions for mRNA.

MediatorComplex may use scaRNA as requirement modification suggestions from implementation reality.

snoRNA is an ADR and can be used as an issue discussion about rRNA.

These items may be follow-ups to Introns or root issues linking to another mRNA or rRNA Gene as appropriate.

tmRNA is an unblocker mediation request. It is no longer required by default and is not tied to siRNA.

## CRISPR

CRISPR uses proto-spacer emergent issues or crRNA incident reports.

crRNA may be linked to a risk.

CRISPR may follow up with PAM exploratory evidence and tracrRNA root cause analysis.

Cas actions are analogous to Exons and can be suggested changes through sgRNA.

sgRNA is repurposed for suggested CRISPR action changes.

## Structural Maintenance

Structural maintenance is a deployment discussion channel for Centromeres and microtubule tasks.

More structural maintenance granularity is intentionally open.
