# Docs Reset Recovery Notes

This document records what changed during the docs reset that produced the current model documents:

- `DOMAIN_MODEL.md`
- `ENCODING_TAXONOMY.md`
- `DEFERRED_DOMAIN_LEDGER.md`
- concise slice contracts `001` through `008`

Use this file if a future agent suspects useful information was lost while removing outdated or meta-heavy slice text.

## Preserved In Current Docs

The reset preserved these product concepts in active docs:

- `Insulator` as the customer/account tenant and hard isolation boundary.
- `InsulatorPlacement` as provisioning/infrastructure placement, distinct from DNAp document schemas.
- Explicit placement `region`.
- `SharedCluster` default placement strategy.
- `Genome` as project boundary.
- `Tf` as user identity inside an Insulator.
- `Tf.display_name` as a label, separate from optional external identity binding fields.
- `GeneFamily` as configurable SDLC document/work type.
- Insulator-scoped and Genome-scoped GeneFamilies.
- Genome-scoped GeneFamilies as project-level overrides.
- Nearest-scope lookup: Genome override, then Insulator default.
- GeneFamily abbreviation uniqueness in effective scope.
- `GeneFamilyGeneration` as immutable schema version.
- `GeneFamilyGenerationId` replacing old `GenotypeId`.
- `SequenceDefinitionId` as stable field identity.
- All SequenceDefinitions required before commit.
- `EncodingType` as required, product-critical document typing.
- GRN / Translation RNA / Regulatory RNA taxonomy and document meanings.
- CLI sequence-name matcher: kebab-case, case-insensitive, fuzzy, exactly one match or error.
- `Locus` as Genome-scoped document/work item identity.
- `Transposon` as origin metadata for new Genes/work items.
- `Allele` as the mutable working version for a Locus.
- One active Allele per `(Locus, Tf)`.
- `Mutation` as a mutable `Unexpressed` Sequence value change until expression.
- `Gene` as immutable committed version selected from an Allele.
- `Histone` / `HistoneMark` as the single authorization/contextual evaluation abstraction.
- `Protein` / `Fold` / `FoldState` as implementation and evaluation concepts.

## Intentionally Removed From Active Slice Flow

These were removed from slice docs because they made the active product definition harder to use:

- Old-vs-new commentary.
- Possible-loss sections.
- Repeated per-slice deferred ledgers.
- Approval-gate scaffolding.
- Historical candidate names that have already been replaced.
- `Dominance`, `Dominant`, `Recessive`, and `Vestigial`.
- Invented `writable` / `hidden` SequenceDefinition behavior.
- Temporary `Prev.md` and `PrevPrev.md` recovery files.

If one of these turns out to be product-relevant, restore it as a current product concept, not as historical commentary.

## Areas That May Be Under-Preserved

These details were compressed during the reset and should be checked if work reaches the relevant slice.

### Product Philosophy

The old build index stated that DNAp models software delivery as evolvable project work:

- users create and refine structured work,
- candidates evolve through explicit changes,
- committed versions become immutable history,
- authorization and evaluation are part of the model,
- workflow and implementation artifacts connect the model to real delivery.

This philosophy is still reflected in the model, but the exact prose is not currently prominent.

### Histone Resolution

The earlier docs included a clearer authorization resolution order:

```text
TfClass histones
+ Tf histones
+ Genome/Gene/Allele/Sequence context histones
```

Resolution rules to preserve or re-check:

- Explicit deny wins.
- Tf histones override TfClass histones.
- Resource/context histones constrain access.
- Default deny.
- More specific HistoneMarks override broader inherited marks unless the broader mark is `ConstitutiveHeterochromatin`.

### ChromatinState Invariants

The reset preserved these only in compressed form. Re-check before implementing slice 006:

- `ConstitutiveHeterochromatin` cannot be overridden by Tf, TfClass, candidate workflow, Mediator, or conditional evaluation.
- `ConstitutiveHeterochromatin` requires `valid_until = None`.
- `FacultativeHeterochromatin` requires evaluator resolution.
- `Heterochromatin` requires rationale.
- `ConstitutiveHeterochromatin` requires rationale.
- Only one active `HistoneMark` may exist per `(target, histone)` pair.
- Multi-valued attributes must use vector value types instead of multiple active marks.
- Auditable records degrade rather than hard-delete.

### Histone Scope And Targets

Check these before slice 006:

- Histones may exist at Insulator or Genome scope.
- Genome Histones extend Insulator Histones.
- `Histone.key` is unique within an Insulator.
- `HistoneTarget` includes `Exon`.
- `dna splice <mrna-gene> "Some hard task" "An even harder one"` creates Exons attached to the active mRNA Allele.
- `dna splice` takes a Locus name or Gene FQN. Positional target matching is case-insensitive and kebab-insensitive, not fuzzy.
- `dna splice <mrna-gene> "Buy soap" --before-go-laundry` creates or selects the Exon and makes `go-laundry` depend on it.
- `dna splice <mrna-gene> --pick-laundry --after-go-laundry` makes `pick-laundry` depend on `go-laundry`.
- `dna splice <mrna-gene> --set-buy-soap "Buy hypoallergenic soap"` replaces the text of the existing Exon.
- `dna splice <mrna-gene> --lgtm` is an escape hatch that expresses current `Unexpressed` Mutations without changing Exons.
- Splicing changes `Allele.state` to `Expressing`; mutating an expressing Allele with Sequence flags changes it to `Mutating`; `dna select` is the final immutable Gene boundary.
- `dna transcribe` is always allowed in every Allele state. It renders the latest Mutation projection, including unapproved mutations such as sgRNA suggested document modifications.
- `dna transcribe` always shows approval-status comments for mutated and sgRNA Sequences.
- `Transcriptome` stores render/access cursor metadata for token-saving transcript output, not projected document content.
- Exons attached to an Allele organize as a DAG through `depends_on`, not an ordinal list and not a separate edge object.
- Exons were lost during the docs reset and must not be collapsed into SequenceDefinition or Mutation.

### Fold And Evaluation Semantics

The old slice had an explicit unresolved question:

- Should `FoldState` distinguish execution status from evaluation verdict instead of combining both?

The current ledger records Fold metadata and related roles, but this exact question should be restored before slice 008 implementation.

Also re-check:

- Whether `Protein` represents output for an Allele, a committed Gene, or both.
- Whether `Fold` needs repository, branch, PR, workflow run, artifact URI, environment, and evaluator provenance.
- Whether `Ribosome` is evaluator/execution engine.
- Whether `RRNA` is evaluator configuration/policy or design document only.
- Whether `Chaperone` is repair workflow, actor class, or generated patch record.
- Whether `Chiasma` is violation/finding record.
- Whether `Phenotype` / `Phenome` survive as reporting/read-model concepts.

### Workflow Artifacts

The current taxonomy defines Regulatory RNA document meanings, but slice 007 still needs concrete workflow structures.

Re-check:

- `Activator`
- `Cofactor`
- `Mediator`
- `PreInitiationComplex`
- `BindingAffinity`
- whether agents are Tfs, TfClass membership, or a separate actor type
- whether mutation proposals must pass through `SgRNA`
- whether direct Mutation remains valid
- whether `GRNA` needs immutable generation snapshots, model/config metadata, and provenance

### Document Identity And Display

Re-check before slice 003 / 005:

- Whether document display identifiers derive from GeneFamily abbreviation, a document name sequence, and a counter.
- Example from prior docs:

```text
name = "Product Requirements Document"
abbreviation = "PRD"
PRD-awesome-to-do-app-0001
```

This is not currently in the active model except as a deferred ledger entry.

### Mutation Semantics

Re-check before slice 004:

- vector replacement vs append/remove behavior,
- deletion/clear semantics,
- mutation rationale/context,
- optimistic concurrency or version preconditions,
- Gene references through `GeneId` versus another identity,
- `@Cell` reference behavior if Cell returns.

## Recovery Sources

The temporary files used during recovery were removed:

- `docs/Prev.md`
- `docs/PrevPrev.md`

The committed history still contains prior slice versions. If recovery is needed, inspect commits before:

- `1fcd3e2 docs: define current domain model`
- `11e5c45 docs: recover encoding document taxonomy`
- `e4107aa feat: implement tenant project user bootstrap`

Useful commands:

```sh
git show 11e5c45^:docs/002_DEFINE_WORK_TYPE.md
git show 11e5c45^:docs/006_AUTHORIZE_WITH_HISTONES.md
git show 11e5c45^:docs/008_ADD_IMPLEMENTATION_EVALUATION.md
git show 1fcd3e2^:docs/000_VERTICAL_BUILD_INDEX.md
```

## Rule For Future Recovery

If a future agent finds a missing concept:

1. Restore it into `DOMAIN_MODEL.md`, `ENCODING_TAXONOMY.md`, or `DEFERRED_DOMAIN_LEDGER.md`.
2. Keep slice docs as build contracts, not archaeology.
3. Do not reintroduce old-vs-new commentary unless it is the only safe way to preserve an unresolved product decision.
