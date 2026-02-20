# Discussion Model Proposal

This document is a proposal, not approved domain truth.

It summarizes the updated mental model to dissect point by point before changing canonical docs or implementation.

## Core Shift

DNAp should start work from ambiguity, not commitment.

The generic "Intent" idea maps to DNAp's existing `Promoter` concept. We should not add an `Intent` object unless later use cases prove `Promoter` cannot carry that role.

Proposed root model revision:

- `PreInitiationComplex`: root assembly for a possible change, opportunity, issue, or initiative.
- `PreInitiationComplex` contains exactly one `Promoter`.
- `Promoter` is a Gene and carries the initiating idea/story/intent.
- `Promoter` starts as raw capture and only later becomes scoped/activated through the PreInitiationComplex.
- `Promoter` does not initially require delivery team, milestones, implementation plan, final requirements, or task breakdown.
- `PreInitiationComplex` should be broadly discoverable, with mutation and authority controlled separately.

Proposed shape:

```rust
struct PreInitiationComplex {
    id: PreInitiationComplexId,
    promoter_locus_id: LocusId,
    state: PreInitiationComplexState,
    grn_id: Option<GrnId>,
}
```

`promoter_locus_id` must point to a GeneFamily whose EncodingType is `Promoter`.

`grn_id` is empty during early capture/triage and is set once the initiative is approved.

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

## GRN, PreInitiationComplex, And Promoter

Approved direction to explore:

- Use the same Gene/Locus/Allele/Gene systems for Promoter instead of introducing a special non-Gene record type.
- Genes are created and edited through the unified `dna mutate` path.
- Promoters are created with `dna mutate`, not a special `dna promote` command.
- `dna select <promoter> <args>` commits the Promoter and creates or resolves the `PreInitiationComplex`.
- `PreInitiationComplex` points to exactly one Promoter Locus.
- A Promoter Locus may have exactly one active PreInitiationComplex.
- `GRN` is created only after the initiative is approved.
- `GRN` is created with the resolved `PreInitiationComplex`, a `MediatorComplex`, and required Tfs.
- Alleles for downstream initiative work belong to a `GRN`.
- Mutations record `promoted_by: GeneId`.
- Each Gene likely has exactly one `Activator`, meaning owner. This still needs explicit confirmation.
- `Cofactors` are the Tfs involved in the PreInitiationComplex and declared in `dna select <promoter> <args>`.
- Signers are separate from cofactors. Signers are Tfs with the Histones required to review and approve Enhancer documents.
- Every Gene may have optional fixed authorization/signature metadata.
- Each GeneFamilyGeneration may define required authorizers/signers.
- Enhancer schemas commonly require explicit authorizations, but the mechanism should be generic to Genes.
- Final authorizations/signatures attach only to selected immutable Genes, not mutable Alleles.
- Detailed Histone evidence shape is deferred until Histones are better defined.
- All required Enhancer documents must be selected Genes and have all required authorizations before GRN creation.
- Missing required Enhancer authorizations block GRN creation.
- Authorizations are optional for non-Enhancer Genes unless their GeneFamilyGeneration requires them.
- Required Enhancer investigation and authorization policy is tenant-enforced through the Enhancer GeneFamilyGeneration.
- Required Enhancers are not ad hoc PreInitiationComplex fields.
- A PreInitiationComplex may use multiple Enhancer GeneFamilies.
- Enhancer GeneFamilyGenerations have `required_for_grn: bool`.
- To create a GRN, every active GeneFamilyGeneration in scope with `EncodingType::GRN(Enhancer)` and `required_for_grn = true` must have a corresponding Enhancer Gene created, selected, and authorized.
- Each required Enhancer Gene must be authorized by all Tf ids defined by that Enhancer GeneFamilyGeneration.
- Enhancer GeneFamilyGenerations with `required_for_grn = false` can create contextual research documents without blocking GRN creation.
- Enhancer Genes are scoped to the PreInitiationComplex they support.
- Enhancers are documents used to validate the initiating idea.
- If `required_for_grn = true`, there should be at most one selected Enhancer Gene per Promoter/PreInitiationComplex for that Enhancer GeneFamilyGeneration.

Proposed PreInitiationComplex meaning:

- Early assembly inside a GRN around the initiating Promoter.
- Coordinates early activation before mRNA/rRNA/task production.
- Can hold product-side Tfs and route into MediatorComplex.
- References the Promoter Locus, while committed Promoter Genes provide the visible/selected versions most of the team sees.

Proposed Promoter meaning:

- Latent initiative/intention.
- GeneFamily/Locus/Allele/Gene document that carries the initiating idea/story/intent.
- Anchors the "why" inside the PreInitiationComplex.
- Does not itself become the whole container.

Proposed shape:

```rust
struct Grn {
    id: GrnId,
    preinitiation_complex_id: PreInitiationComplexId,
    mediator_complex_id: MediatorComplexId,
}

struct PreInitiationComplex {
    id: PreInitiationComplexId,
    promoter_locus_id: LocusId,
    selected_promoter_gene_id: GeneId,
    activator: TfId,
    cofactors: Vec<TfId>,
}

struct Allele {
    grn_id: GrnId,
    // existing Allele fields
}

struct Mutation {
    promoted_by: GeneId,
    // existing Mutation fields
}

struct GeneAuthorization {
    gene_id: GeneId,
    authorized_by: TfId,
    authorized_at: Timestamp,
}
```

`promoted_by` points to the committed Gene version that promoted/justified the Mutation. For Promoter-driven work, that Gene must be a committed Promoter Gene from the GRN's resolved PreInitiationComplex Promoter Locus.

Open design questions:

- What arguments does `dna select <promoter> <args>` need to create or resolve the PreInitiationComplex?
- What exact approval event creates the GRN from a resolved PreInitiationComplex?
- Which Tfs are required when creating the GRN: product Tfs, requirements Tfs, both, or separate recruitment steps?
- Can a GRN ever change its resolved PreInitiationComplex, or is that immutable?
- Does every Gene have exactly one Activator?
- Are cofactors mutable after PreInitiationComplex creation, or do changes require a new selected Promoter/PreInitiationComplex revision?
- Where are required signer sets defined: Histones on Enhancer GeneFamily, Histones on PreInitiationComplex, or GRN creation policy?
- Should GeneFamilyGeneration signer requirements name specific Tf ids, TfClass/role ids, required Histones, or a combination? Deferred until Histones are better defined.
- Should `authorized_by` store only the Tf, or also the HistoneMarks/permission evidence used at signing time? Deferred until Histones are better defined.

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

State note:

- PreInitiationComplex lifecycle state is provisional.
- It is not yet clear which concrete use cases require state beyond existence, selected Promoter, scoped Enhancers, authorizations, and GRN creation.
- Keep state as a possible model tool, but do not implement until a workflow needs it.

Select CLI direction:

Gene creation/mutation CLI direction:

```sh
dna mutate "Title for my idea" --new US --some-user-story-field "Value" --some-other-field "Other thing"
```

- `dna mutate` always requires at least one Sequence mutation flag.
- `--new <gene-family-abbreviation>` appears after the first positional argument and may appear anywhere after it.
- The first positional argument is the candidate Gene title/name when `--new` is present.
- Without `--new`, the first positional argument must fuzzy match an existing current Gene.
- If the first positional argument does not match an existing Gene, the command errors unless `--new` is present.
- This uniform path applies to Promoters, Enhancers, mRNAs, rRNAs, and other Genes.
- Existing Genes are modified with the same command, for example:

```sh
dna mutate my-idea --some-field "Value"
```

- Workflow scoping can use explicit special keyword flags.
- For now, Enhancers are scoped with:

```sh
dna mutate "Payment provider research" --new TECH --promoter my-idea --summary "..."
```

- `--promoter <fuzzy-slug-for-promoter>` resolves the selected Promoter Gene.
- `--promoter` may be omitted when the Tf is assigned to exactly one active PreInitiationComplex.
- If the Tf is assigned to several active PreInitiationComplexes, the user must pass `--promoter` or configure the active PreInitiationComplex in Tf state.
- The Enhancer is scoped to the PreInitiationComplex that Promoter belongs to.
- Mutations created in that scoped operation record `promoted_by: GeneId` using the selected Promoter Gene at that time.
- Enhancer creation/editing is not limited to Activator and Cofactors by default.
- Edit rights should be resolved through the authorization model, not inferred from PreInitiationComplex role.

PreInitiationComplex creation CLI direction:

```sh
dna select <promoter> --tf <cofactor> --tf <cofactor>
```

- The selecting Tf is the default Activator.
- Repeated `--tf` arguments declare Cofactors.
- Explicit `--activator <tf>` may be considered later, but should require authority.

Proposed lifecycle ownership:

- Promoter uses normal Gene/Allele selection mechanics for content.
- PreInitiationComplex owns product-side early recruitment/coordination around the selected Promoter.
- Activator and cofactors create Enhancer documents as official documented investigation.
- Enhancer documents are protocolary investigation artifacts.
- Activator and Cofactors may create Enhancers scoped to the PreInitiationComplex.
- Signers are separate and authorize selected Enhancer Genes.
- Required signatures on Enhancer documents are collected from signer Tfs with appropriate Histones.
- Required authorizers are defined by the Enhancer GeneFamilyGeneration schema.
- Required Enhancer documents/policies are defined by the Enhancer GeneFamilyGeneration so tenants can enforce the activation protocol.
- GRN creation scans active in-scope Enhancer GeneFamilyGenerations and requires one selected, fully authorized Enhancer Gene for each generation marked `required_for_grn = true`.
- Non-required Enhancer generations remain available for contextual research.
- Required Enhancers are resolved through explicit PreInitiationComplex attachment, not inferred globally from all Enhancer Genes.
- Optional Enhancers are also scoped to the PreInitiationComplex; they provide contextual validation but do not block GRN creation.
- The same authorization mechanism should be available to other Gene types, even if Enhancers use it most often.
- Authorization is finalized against selected Genes only.
- GRN creation requires all required Enhancer Genes to be selected and fully authorized.
- GRN owns the approved shared regulatory context after initiative approval.
- An activator can call the GRN creation CLI command only after required signatures are collected.
- GRN creation is allowed for the Activator by default.
- GRN creation can later be delegated through Histones.
- MediatorComplex is introduced when the initiative is approved and the GRN is created.
- Most team visibility can default to committed Promoter Genes, while participants in later GRN work can see working Alleles.

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
- Enhancer should help recruit Tfs and feed PreInitiationComplex/MediatorComplex discussions that determine whether/how a Promoter activates.

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
