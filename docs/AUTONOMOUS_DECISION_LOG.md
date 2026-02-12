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

## Autonomous Remaining-Feature Roadmap

This roadmap defines the order and assumptions for continued autonomous work on this branch.

### Guiding Decisions

- Prefer backend/application APIs first, then thin CLI commands.
- Keep tenant/project/user/session boundaries explicit in every command object.
- Do not implement `dna select` until regulatory workflow pressure makes the selection semantics clearer.
- Do not introduce a monolithic persisted `TfComplex`; implement concrete workflow records and typed relationships.
- Keep `GeneFamily` configurable and avoid hard-coding tenant document schemas beyond system-fixed `EncodingType`.
- Use `LocusId` for stable document/work-item relationships unless a workflow specifically needs Allele snapshot semantics.
- Keep graph/whiteboard data operation-friendly without committing to a CRDT/OT library yet.
- Keep local JSON state as an adapter for now, but shape APIs so a real persistence/auth adapter can replace it.

### Implementation Priority

1. CLI for exploration graphs.
   - Add commands to create a Promoter-owned graph, add eRNA nodes, add edges, and list graph topology.
   - Use normal session scope from `dna epigenetics use`.
   - Keep graph ids visible for now because no human-friendly graph FQN has been approved.

2. Enhancer Promoter property.
   - Add a Promoter Locus reference on Enhancer workflow metadata, not a separate link object.
   - Keep the exact Enhancer schema tenant-configurable through GeneFamily Sequences.
   - Reject attaching non-Enhancer documents as Enhancers and non-Promoter documents as Promoter targets.

3. eRNA canonization.
   - Add an application operation that creates a new target Transposon and Allele from an existing eRNA Locus.
   - Preserve source eRNA unchanged and store provenance from source eRNA Locus to target Locus.
   - Do not implement `dna select` as part of canonization.

4. MediatorComplex concrete workflows.
   - Start with `Intron` as chainable disambiguation items.
   - Add targeted follow-up records for `snRNA` and `scaRNA` only after there is a concrete mRNA target.
   - Keep `snoRNA` ADR behavior linked to rRNA.

5. RepressorsComplex concrete workflows.
   - Model piRNA and miRNA as open scope-control discussions.
   - Model siRNA as authoritative out-of-scope order.
   - Do not implement broad arbitrary Gene links; use constrained target types per workflow.

6. CRISPR concrete workflows.
   - Add incident/open-issue flow around `crRNA`, `tracrRNA`, and `sgRNA`.
   - Keep `Cas` as an action concept, not EncodingType.
   - Defer PAM evidence shape until at least one useful CLI workflow exists.

7. Structural maintenance.
   - Keep Centromere deployment documents as current controlled documents.
   - Defer microtubule/task granularity until deployment workflows are clearer.

8. Authorization with Histones.
   - Add Histone and HistoneMark models before production-like authorization.
   - Keep current local session bootstrap as temporary superadmin/demo path.
   - Do not add SSO/SCIM implementation yet; preserve identity adapter boundaries.

9. Persistence hardening.
   - Split current god-file pressure by moving domain areas into modules once another feature lands.
   - Keep serialization compatibility in mind but do not over-design migration before real storage exists.

### Explicit Deferrals

- `dna select` and immutable Gene creation remain deferred.
- Full frontend whiteboard and realtime collaboration remain deferred.
- CRDT/OT semantics remain deferred.
- SSO/SCIM remains deferred.
- Legal hold, retention, export/delete, and regional residency remain documented enterprise needs, not local CLI implementation.
