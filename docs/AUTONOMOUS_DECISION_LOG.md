# Autonomous Decision Log

This branch records implementation decisions made while advancing DNAp without live user review.

Each entry is provisional until reviewed and merged.

## 2026-02-11: Exploration Graph Vertical

- Decision: implement the next autonomous vertical around Promoter-owned eRNA exploration graphs.
- Reason: it exercises the approved encoding taxonomy and implements an approved workflow model without touching deferred `dna select`.
- Decision: add concrete records for `ExplorationGraph`, `ExplorationNode`, and `ExplorationEdge`.
- Reason: the workflow model already approved graph topology as workflow artifact state, while eRNA remains controlled document content.
- Decision: keep graphs owned by a stable Promoter `LocusId`, not a Promoter Allele.
- Reason: graph ownership should survive Promoter Allele changes.
- Decision: graph nodes point to stable eRNA `LocusId`, not directly to eRNA Alleles.
- Reason: whiteboard rendering can resolve current active eRNA Alleles later without freezing graph topology to a draft.
- Decision: allow graph-node creation to auto-create an eRNA Transposon and Allele when requested by command/API.
- Reason: whiteboard workflows should not require pre-creating every eRNA document before placing a node; the auto-created record still uses normal Locus, Transposon, and Allele structures.
- Decision: keep edge semantics as free text for now instead of a fixed enum.
- Reason: the user rejected constraining relationship semantics as product value; structural graph rules matter now, semantic ontology can emerge from tenant schemas and later workflow needs.
- Decision: do not implement realtime CRDT/OT, React UI, canonization, or Promoter/Enhancer relationships in this vertical.
- Reason: those are approved future workflow areas but are larger than the first backend/CLI path.
