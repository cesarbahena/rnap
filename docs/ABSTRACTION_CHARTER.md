# DNAp Abstraction Charter

This document defines what each core abstraction class is allowed to model. New concepts must fit one of these classes before they become persisted objects, code modules, commands, or slice contracts.

## Boundary

A Boundary separates tenant isolation, project scope, canonical placement, or change context.

Boundaries answer where something lives or where work is happening.

- `Insulator`: customer/account tenant boundary.
- `Genome`: project boundary.
- `Chromosome`: named canonical scope inside a Genome.
- `GRN`: Governance Regulatory Network; bounded change context inside a Genome.
- `Operon`: triage grouping of Promoters inside one GRN.

Do not put artifact content, permissions, dependency graphs, or audit facts directly into Boundary objects.

## Artifact Definition

An Artifact Definition describes what kind of controlled artifact can exist.

- `GeneFamily`: configurable enterprise artifact/document/work-item type.
- `GeneFamilyGeneration`: immutable schema version for a GeneFamily.
- `SequenceDefinition`: one configured field in a GeneFamilyGeneration.
- `NormalizedArtifact`: system-fixed platform category for a GeneFamily.

Artifact Definitions answer what schema/protocol an artifact follows. They do not represent one artifact instance.

## Artifact Identity And Version

Artifact identity and versioning model controlled content across time.

- `Locus`: stable identity for one controlled artifact/document/item.
- `Gene`: immutable selected version of a Locus.
- `Transposon`: origin record for a newly introduced Locus.

These objects answer what the controlled artifact is and what versions became canonical.

## Candidate

A Candidate is proposed artifact state inside a change context.

- `Allele`: shared candidate version of one Locus inside one GRN.

Alleles are shared by the GRN. They are not per-Tf private drafts.

## Change

A Change is an edit to candidate artifact state.

- `Mutation`: Sequence value change on an Allele.

Mutation author/provenance belongs to `Signal`, not to ad hoc actor fields on Mutation.

## Control Fact

A Control Fact governs, constrains, or interprets behavior across artifacts and workflows.

- `Histone`: permission/governance fact.
- `eRNA`: protocol/evaluator/control rule artifact.

Histones answer what governance fact exists. eRNA is `NormalizedArtifact::Executable`: a human-readable executable governance artifact with IAM-like DSL attributes. eRNA mainly regulates Gene lifecycle and command state changes by checking dependency state.

## Relationship

A Relationship models dependency, topology, containment, or graph structure between artifacts or candidates.

- `Ribozyme`: flexible exploration artifact for event storming, draft diagrams, early idea graphs, discovery narratives, and whiteboard collaboration.
- Embedded Genes through `SequenceType::Gene` and `GeneVec`: controlled artifact composition inside another artifact.

Ribozyme is the replacement name for the older eRNA exploration role. Ribozyme may need graph-local node and edge records when whiteboard, layout, reuse, or realtime collaboration use cases require them.

Embedded Genes are content composition. Ribozyme is exploratory collaboration. Do not confuse them.

## Signal

A Signal is an append-only event/provenance fact.

- `Signal`: tenant-scoped event record.

Signals answer what happened, who or what caused it, when, why, and against which target.

Signals do not decide permissions. Histones and eRNA protocols decide behavior using Signals as facts.

## Projection

A Projection is read-side/cache/view state over underlying domain records.

- `Transcriptome`: transcript render cursor for `dna transcribe`.

Transcriptome stores what was last shown, not artifact content. It exists for CLI/LLM efficiency and may be deleted or rebuilt without losing domain state.

## Classification Rule

Before adding a new persisted object, classify it:

- Boundary: scope/context.
- Artifact Definition: schema/type.
- Artifact Identity And Version: controlled artifact identity/version.
- Candidate: proposed artifact state.
- Change: edit to candidate state.
- Control Fact: governance/evaluator fact.
- Relationship: topology/composition/dependency.
- Signal: event/provenance.
- Projection: read-side/cache/view.

If a concept does not fit one class clearly, defer it.

If a concept seems to fit multiple classes, split it. Do not create one object that is simultaneously content, permission, relationship, event, and projection.
