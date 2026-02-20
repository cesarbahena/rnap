# Discussion Model Proposal

This document is a proposal, not approved domain truth.

It summarizes the updated mental model to dissect point by point before changing canonical docs or implementation.

## Core Shift

DNAp should start work from ambiguity, not commitment.

The generic "Intent" idea maps to DNAp's existing `Promoter` concept. We should not add an `Intent` object unless later use cases prove `Promoter` cannot carry that role.

Proposed root model:

- `Promoter`: first durable root for a possible change, opportunity, issue, or initiative.
- `Promoter` starts as raw capture and only later becomes scoped/activated.
- `Promoter` does not initially require delivery team, milestones, implementation plan, final requirements, or task breakdown.
- `Promoter` should be broadly discoverable, with mutation and authority controlled separately.

## Activation Pipeline

Proposed SDLC regulation model:

```text
Enhancers
  -> recruit Tfs
  -> through MediatorComplex
  -> activate Promoter
  -> form collaborative working state
  -> produce RNA documents
  -> refine through planning/execution systems
  -> express into canonical Genome
```

Enterprise translation:

- Evidence and context attract the right people, agents, reviewers, and authorities.
- MediatorComplex is not generic chat; it is the organizational signal-integration layer.
- Promoter activation should be earned through research, discussion, authority, scope, and suppression signals.

## Promoter

Proposed meaning:

- Latent initiative/intention.
- Root of exploration and triage.
- Owns or anchors exploration graphs, Enhancer evidence, scope discussions, risks, requirements, and later implementation artifacts.

Open design questions:

- What exact lifecycle stages should Promoter have?
- What fields are required at raw capture versus activation?
- Does Promoter activation create a separate object, state transition, or Chromosome/chromatid working set?

Possible stages:

```text
RawCapture
Triage
Discovery
Scoped
Activated
Implementation
Validation
Reconciled
Archived
```

## Enhancer

Proposed meaning:

- Formal evidence/research/context that can activate or strengthen a Promoter.
- Initiative formalizer or activation support, not a generic research note only.
- Examples: customer evidence, technical research, market research, incident evidence, benchmark, prototype finding, compliance driver.

Biology alignment:

- Enhancers recruit transcription factors.
- Enhancers influence promoter activation through mediator machinery.

Proposed DNAp rule:

- Enhancer should not directly "turn into work."
- Enhancer should help recruit Tfs and feed MediatorComplex discussions that determine whether/how a Promoter activates.

Existing implementation concern:

- Current branch uses `EnhancerContext` as a property from Enhancer Locus to Promoter Locus.
- This may be acceptable as a first property, but the name and shape should be reviewed against the richer activation model.

## Tf

Proposed meaning:

- Actors that can influence expression: humans, agents, reviewers, architects, product owners, security teams, systems.
- Different Tfs bind to different Enhancer patterns or workflow needs.

Enterprise use:

- Recommend or recruit relevant Tfs based on Enhancer type, affected systems, risk, ownership, and prior decisions.
- Separate broad visibility from mutation permission and decision authority.

## MediatorComplex

Proposed meaning:

- Structured coordination and signal-integration layer between Enhancers/Tfs and Promoter activation.
- Not a monolithic persisted object by default.
- Not merely chat.

MediatorComplex responsibilities:

- Aggregate Enhancer signals.
- Recruit or coordinate Tfs.
- Host disambiguation and negotiation.
- Resolve conflicting signals.
- Surface suppressors and scope boundaries.
- Produce activation or non-activation recommendations.
- Route to requirements, design, task planning, governance, release, or incident workflows.

Possible specializations:

- `RequirementMediator`
- `ArchitectureMediator`
- `SecurityMediator`
- `ReleaseMediator`
- `IncidentMediator`
- `GovernanceMediator`
- `ExecutionMediator`

Open design questions:

- Are mediator specializations code types, configurable mediator profiles, or emergent from RNA workflow artifacts?
- Should mediator state live on Promoter, Chromosome/chromatid, or typed RNA artifacts?

## Shared Alleles

Proposed correction:

- Alleles should generally be shared collaborative candidates, not private per-Tf drafts.
- Tf should define authorship, participation, mutation provenance, permissions, and authority, not separate default reality.

Proposed default invariant:

```text
One active shared Allele per Locus in a collaboration context.
Multiple active Alleles require explicit branching/forking semantics.
```

Implication for eRNA graphs:

- Graph meetings should render shared working Alleles, not each viewer's private draft.
- Canonical Genes remain approved/reference knowledge.
- Live meeting state uses shared candidate Alleles.

Open design questions:

- Is the default active Allele per Locus per Genome, per Promoter, per ExplorationGraph, or per Chromosome/chromatid?
- What explicit command creates a branch/forked Allele?

## eRNA

Proposed meaning:

- Flexible typed exploration graph node content.
- Useful for event storming, draft diagrams, idea graphs, discovery maps, and exploratory reasoning.

Proposed correction:

- eRNA canonization is removed from the active model.
- Do not design more eRNA transformation mechanics until use cases are clearer.
- eRNA should not become the universal model for all discussions.

Open design questions:

- Is eRNA content versioned as controlled knowledge, or is it mainly collaborative whiteboard state?
- If versioned, does graph meeting state bind eRNA nodes to shared active Alleles?
- Which eRNA changes are content mutations versus graph presentation operations?

## Intron

Approved correction:

- `Intron` is a requirement ambiguity question.
- `Intron` targets `mRNA` only.
- rRNA/design discussions should not use Intron.

Current implementation concern:

- Current code adds `IntronMediation` and `IntronFollowUp` relationship records.
- This may be the wrong domain shape if the RNA artifact itself should own target and chain semantics.

Alternative shape to explore:

```rust
struct Intron {
    locus_id: LocusId,
    target_mrna_locus_id: LocusId,
    parent_intron_locus_id: Option<LocusId>,
}
```

Open design questions:

- Should `Intron` be an actual typed workflow object wrapping a Locus?
- Should target/parent live as fields on `Intron` rather than separate "mediation/follow-up" records?

## snoRNA

Proposed meaning to explore:

- rRNA/design discussion counterpart to Intron.
- Likely ADR/design clarification around architecture/design documents.

Biology alignment:

- snoRNA participates in RNA modification and maturation, but current SDLC mapping treats it as ADR/design discussion.

Open design questions:

- Is snoRNA the right artifact for design/ADR discussion against rRNA?
- Should it mirror Intron's target/chain shape but target rRNA only?
- Or should snoRNA be a broader architectural decision artifact independent of rRNA target?

## snRNA, scaRNA, tRNA, tRF

Proposed updated mapping:

- `snRNA`: planning/refinement/orchestration agent skills.
- `scaRNA`: emergent planning orchestration workflows or patterns.
- `tRNA`: execution agent skills.
- `tRF`: emergent execution workflows or patterns.

Rationale:

- snRNA participates in splicing, so it fits planning decomposition/refinement.
- scaRNA modifies snRNA machinery, so it fits emergent planning-process evolution.
- tRNA participates in translation, so it fits execution capabilities.
- tRFs are derived regulatory fragments, so they fit emergent execution patterns.

Open design questions:

- Should `snRNA` and `tRNA` remain GeneFamily EncodingTypes for documents, or become capability/agent-skill definitions?
- Are `scaRNA` and `tRF` controlled documents, learned workflow policies, or generated operational patterns?
- Does `tRF` need to be added to the taxonomy, or stay deferred?

## Suppression And Scope Control

Current concepts:

- `piRNA`: explicit out-of-scope discussion.
- `miRNA`: emergent scope reduction discussion.
- `siRNA`: authoritative out-of-scope order.
- `Silencer`: retirement document.

Open design questions:

- How do suppressors apply to Promoter activation?
- Does siRNA require authority Histones?
- Can piRNA/miRNA escalate into siRNA?
- How are suppressor effects shown in the Promoter lifecycle?

## Enterprise Operating Model

Proposed default:

- Broad visibility.
- Narrow authority.
- Controlled mutation.
- Explicit classification for sensitive work.

Possible rings:

- Read/discover.
- Participate/comment.
- Contribute/mutate.
- Review.
- Decide/approve/suppress.

Open design questions:

- Which of these are Histones versus workflow roles?
- What is the minimum access model before full Histone authorization?

## Collision Detection

Potential high-value capability:

DNAp can detect regulatory collision before implementation, not only artifact overlap.

Examples:

- Two Promoters recruit the same scarce Tfs.
- Two Promoters depend on the same Enhancer evidence.
- Two active efforts mutate the same Locus or Chromosome area.
- Two mRNAs produce incompatible Exons.
- A suppressor blocks a Promoter that still has active implementation work.

Open design questions:

- What is the first collision type worth implementing?
- Should collision be a report, workflow artifact, HistoneMark, or mediator event?

## Current Implementation Risk

The current autonomous branch implemented some relationship records from a too-generic pattern:

- `EnhancerContext`
- `IntronMediation`
- `IntronFollowUp`

These may be serviceable storage structures, but the domain language may be wrong.

Review direction:

- Prefer typed RNA workflow objects when the relationship is intrinsic to the RNA's meaning.
- Keep generic relationship records only for graph topology or truly generic edges.
- Avoid making eRNA/exploration graph mechanics the default model for all discussion artifacts.
