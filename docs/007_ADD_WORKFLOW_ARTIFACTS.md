# 007 Add Workflow Artifacts

## Capability

Add Regulatory RNA workflow documents around candidate work.

## Contract

Use [DOMAIN_MODEL.md](DOMAIN_MODEL.md) and [ENCODING_TAXONOMY.md](ENCODING_TAXONOMY.md).

Slice 007 implements workflow document records for selected Regulatory RNA encodings and concrete TfComplex discussion relationships.

## Behavior

- `Intron` is a chainable disambiguation item.
- `SnRNA` is a task modification suggestion for an mRNA.
- `ScaRNA` is a requirement modification suggestion from implementation reality.
- `SiRNA` is an authoritative out-of-scope order.
- `TmRNA` is an unblocker mediation request.
- `GRNA` is a general message.
- `MiRNA` is an emergent scope reduction discussion.
- `PiRNA` is an explicit out-of-scope discussion.
- `SnoRNA` is an ADR.
- `CrRNA` is an incident report.
- `TracrRNA` is a root cause analysis.
- `LncRNA` is a research document.
- `CircRNA` is onboarding particularities.
- `SgRNA` is a suggested CRISPR action change.
- Workflow artifacts are created by Tfs and evaluated through Histones.
- `Exon` remains the executable task created by `dna splice`; it is not an EncodingType.
- `Cas` actions belong to CRISPR workflows; `Cas` is not an EncodingType.
- `TfComplex` names the discussion/alignment subsystem. It is not automatically a persisted container object.
- Code should persist only concrete discussion records and typed relationships with a defined workflow purpose.
- `TfComplex` does not define Gene schemas and does not own workflow rules.
- TfComplex relationships may target whole documents or work items inside those documents, but allowed targets are constrained by each use case.
- PreInitiationComplex uses properties and typed exploration relationships among existing Promoter, Enhancer, and eRNA documents.
- eRNA is a flexible typed exploration graph node for work such as event storming, draft diagrams, and follow-up exploration.
- Enhancers are formal research documents, such as business, technology, or market research.
- Enhancers associate to a Promoter through an Enhancer property, not a separate link object.
- The Promoter association on an Enhancer points to the stable Promoter Locus, not a specific Allele.
- A Promoter may have many named exploration graphs.
- Each exploration graph belongs to exactly one Promoter.
- ExplorationGraph belongs to the stable Promoter Locus, not a specific Promoter Allele.
- ExplorationGraph is a Promoter-owned workflow artifact, not a controlled Gene.
- Exploration graphs contain eRNA nodes and directed typed eRNA edges.
- eRNA controlled documents may be reused across graphs.
- Reused eRNA keeps one controlled document identity, while each graph node has graph-local presentation state.
- Exploration graph edges keep semantic relationship data separate from graph-local presentation state such as label, route, color, and stroke.
- eRNA work may later be canonized or transformed into other RNA document types.
- Canonizing eRNA creates a new target Transposon and Allele with provenance back to the source eRNA, leaving the source eRNA unchanged.
- eRNA exploration graphs may be cyclic.
- ExplorationGraph owns graph topology.
- eRNA owns controlled document content.
- ExplorationNode is graph-local placement/presentation for a reusable eRNA Locus.
- ExplorationNode points to the stable eRNA Locus, not directly to an Allele.
- Whiteboard rendering resolves an eRNA Locus to the current active Allele unless a future snapshot/export feature requires historical resolution.
- ExplorationEdge belongs to one ExplorationGraph and connects graph-local ExplorationNodes, not reusable eRNA documents directly.
- Exploration graphs are intended to render as collaborative React whiteboards.
- Exploration graph nodes and edges need presentation metadata such as position, size, labels, and style.
- Direct `Enhancer <-> eRNA` edges are not part of the approved model.

## Implementation Contract

Pending implementation.

## Approved Tests

Pending.
