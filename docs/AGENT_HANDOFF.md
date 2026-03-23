# DNAp Agent Handoff

This handoff is for agents continuing DNAp model or implementation work. It is not the authoritative product model. Use it to orient quickly, then rely on the official docs listed below.

## Read First

- [ABSTRACTION_CHARTER.md](ABSTRACTION_CHARTER.md): modeling rules. New persisted concepts must fit an abstraction class before implementation.
- [000_VERTICAL_BUILD_INDEX.md](000_VERTICAL_BUILD_INDEX.md): build order and active slice map.
- [ONTOLOGY.md](ONTOLOGY.md): canonical terms and current meanings.
- [DOMAIN_MODEL.md](DOMAIN_MODEL.md): current target domain structures.
- [ENCODING_TAXONOMY.md](ENCODING_TAXONOMY.md): `NormalizedArtifact` taxonomy.
- [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md): approved workflow interactions and explicit deferrals.
- [DEFERRED_DOMAIN_LEDGER.md](DEFERRED_DOMAIN_LEDGER.md): obligations and known code/model gaps.
- [DOCS_RESET_RECOVERY_NOTES.md](DOCS_RESET_RECOVERY_NOTES.md): recovery notes if something appears lost.
- [decisions/001_agent_workflow.md](decisions/001_agent_workflow.md): accepted agent workflow and decision authority.
- [decisions/002_ribozyme_graph_deferral.md](decisions/002_ribozyme_graph_deferral.md): accepted deferral of Ribozyme graph-local records until concrete use cases are approved.
- [decisions/004_verification_scope.md](decisions/004_verification_scope.md): accepted verification rule; run code tests only when code or executable configuration changes.

Treat [DISCUSSION_MODEL_PROPOSAL.md](DISCUSSION_MODEL_PROPOSAL.md) and [AUTONOMOUS_DECISION_LOG.md](AUTONOMOUS_DECISION_LOG.md) as superseded recovery context only.

## Current Modeling Direction

DNAp is not a generic CRUD/document app. The model is intended to represent enterprise SDLC work as controlled artifacts changing through governed, shared candidate states.

The current core stack is:

```text
Insulator -> Genome -> Chromosome -> Locus -> Gene
                         ^
                         |
                    GRN -> Allele -> Mutation
```

Key meanings:

- `Chromosome` is canonical placement inside a `Genome`.
- `GRN` is change context and may work across multiple Chromosomes.
- `Locus` is stable artifact identity, belongs to one current Chromosome, and may move.
- `Gene` is an immutable selected version of a Locus.
- `Allele` is a shared candidate version for one `(Locus, GRN)`.
- `Mutation` is a field-level edit to an Allele.
- `Signal` is append-only event/provenance.
- `Histone` is permission/governance fact.
- `eRNA` is `NormalizedArtifact::Executable`: a human-readable executable governance artifact with IAM-like DSL attributes.
- `Ribozyme` replaces the old eRNA exploration role: event storming, draft diagrams, early idea graphs, discovery narratives, and whiteboard collaboration.

## Important Recent Decisions

These were clarified during discussion and may not be fully implemented in code yet:

- `DomainEvent` is renamed to `Signal`.
- `Executable` is the eRNA normalized artifact variant; avoid redundant `NormalizedArtifact` wording inside variant names.
- `ExploratoryNarrative` is removed from the active taxonomy and replaced by `Ribozyme`.
- Ribozyme is modeled as a normal Gene-capable `NormalizedArtifact` for now; graph-local records are deferred until a concrete Ribozyme graph domain is approved.
- eRNA regulates Gene lifecycle and command state changes by checking dependency state. It is not mainly a `dna select` policy object.
- eRNA itself declares and evaluates dependency requirements; do not invent a separate dependency graph abstraction unless a concrete use case proves it.
- Enhancer remains `EnterpriseNegotiationHandoverCertificate`: an executive/enterprise handover document, not executable governance.
- Alleles are shared per `(Locus, GRN)`, not per Tf.
- `Locus.name` is unique within the containing Genome under canonical CLI/name matching. Do not add a domain field named `normalized_locus_name`; derived lookup keys are storage/index details.
- `degraded_at` is the soft-delete/current-active filter. `Signal` records actor, reason, and payload. Do not remove `degraded_at` just because audit exists.
- Avoid `created_by`, `degraded_by`, `assigned_by`, and similar per-record actor audit fields unless a concrete current query requires denormalization.
- `GRN.activator` and `Locus.activator` are independent concepts. Do not derive one from the other.
- Do not add `Allele.activator`, `Gene.activator`, `Transposon.activator`, `GRN.cofactors`, `GrnTf`, or generic recruitment/role objects until a concrete workflow requires them.
- Locus activator participation in a GRN is Histone/eRNA governance territory, not a structural invariant.

## Common Failure Modes To Avoid

- Do not create a persisted object from a single workflow example. Classify it with [ABSTRACTION_CHARTER.md](ABSTRACTION_CHARTER.md) first.
- Do not use eRNA as a generic graph, diagram, dependency edge, or whiteboard object. That old role is now Ribozyme.
- Do not use Histones as event history. Signals record what happened; Histones are governance facts.
- Do not use Signals as authorization. eRNA/Histones evaluate behavior using Signals as facts.
- Do not put artifact content into boundaries such as GRN, Chromosome, or Operon.
- Do not put lifecycle policy lists on `GeneFamilyGeneration` just because eRNA controls lifecycle. eRNA is in control and should declare what it applies to.
- Do not revive `PreInitiationComplex`, `MediatorComplex`, `RepressorsComplex`, `CRISPR`, `StructuralMaintenance`, or `TfComplex` as persisted objects without concrete use cases.
- Do not use the old Promoter-owned exploration graph design as current foundation.

## Known Code/Doc Mismatches

The docs currently define the target model ahead of implementation in several areas:

- Code now has the flat `NormalizedArtifact` taxonomy.
- Old eRNA exploration graph and Enhancer-Promoter context code/tests were removed; Ribozyme and Enhancer remain Gene-capable artifacts until concrete relationship/use cases are approved.
- Code now uses a minimal GRN bridge for active Allele resolution and uniqueness: one active Allele per `(Locus, GRN)`.
- Current Q/A clarification code is `SemanticNarrowing` (`snRNA`) behavior, not the future raw-requirement `Intron` artifact.
- `Signal`, real canonical `Chromosome`, raw-requirement `Intron`, and full `Ribozyme` behavior are not fully implemented.

When implementing, update tests only with explicit approval and make sure test names do not preserve obsolete model language.

## Next Useful Questions

Ask one at a time and keep them tied to durable model semantics:

- What exact command/state transitions does eRNA regulate first?
- What facts can eRNA DSL read in the first implementation: Histones, Signals, Gene states, Allele states, Mutation states, Ribozyme artifacts?
- What does an eRNA evaluation return: allow, deny, warn, require, or another result shape?
- Is eRNA evaluation always side-effect-free, with commands creating Signals after evaluation?
- What minimum Ribozyme use case should replace the old exploration graph implementation?
- What GRN lifecycle command should replace the temporary `dna epigenetics init-grn` bootstrap path?

## Verification

Current code verification baseline: `cargo fmt --check && cargo test` passed with 34 tests after commit `f12d652`.

Run code verification only when code or executable configuration changes. Do not run `cargo test` for documentation-only edits; instead review the changed docs for local consistency, broken references, and contradictions relevant to the edit.
