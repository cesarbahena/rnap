# 004 Mutate Candidate Work

## Capability

Apply mutable `Unexpressed` Mutations to an Allele and project current Allele state.

`dna mutate --new` also creates the initial Locus, Transposon, and Allele. Sequence mutation flags are optional for `--new`.

This slice is implemented together with [003_OPEN_CANDIDATE_WORK.md](003_OPEN_CANDIDATE_WORK.md). Keep this file as the mutation-specific part of the combined contract.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

Slice 004 implements:

- `Mutation`
- `Sequence`
- `SequenceValue`

## Behavior

- Mutations are mutable while `Unexpressed`.
- Mutations are individual and composable.
- One command versus many commands does not change Mutation semantics.
- Mutations apply to the shared active Allele for `(Locus, GRN)`.
- Mutations target SequenceDefinitions by `SequenceDefinitionId`.
- A Mutation must target an existing SequenceDefinition in the Allele's GeneFamilyGeneration.
- Mutation values must match the SequenceDefinition type.
- `Gene` and `GeneVec` values represent embedded or linked Gene references.
- CLI input for `Gene` and `GeneVec` values uses Gene FQNs and resolves them to `GeneId` internally.
- Current Allele state is projected from Mutations.
- A degraded Allele cannot accept new Mutations.
- Mutation actor provenance is recorded through Signal.
- CLI mutation flags use the approved sequence-name matcher from slice 002.
- `dna mutate --new` may create an Allele with zero Mutations.
- Mutating an `Expressing` Allele is allowed and changes `Allele.state` to `Mutating` when it creates or updates `Unexpressed` Mutations.
- `dna splice` changes `Allele.state` to `Expressing` and changes current `Unexpressed` Mutations to `Expressing`.
- `dna splice --lgtm` is an escape hatch that expresses current `Unexpressed` Mutations without editing the existing TaskRealization DAG.
- `dna transcribe` is always allowed in every Allele state.
- `dna transcribe` renders the latest Mutation projection, including unapproved mutations such as sgRNA suggested document modifications.
- `dna transcribe` always shows approval-status comments for mutated and sgRNA Sequences.
- `Transcriptome` stores render/access cursor metadata for token-saving transcript output, not projected document content.
- `dna translate` renders TaskRealizations attached to the active Allele.
- `dna translate` does not render Sequences and does not mutate state.

## CLI

Render current Allele:

```sh
dna transcribe FRS-checkout-0001
```

- `dna transcribe` shows the current Allele projection.
- It renders latest Allele state, not only the committed/canonical Gene.
- It always includes approval-status comments for mutated and sgRNA Sequences.

Create or acknowledge TaskRealizations on the active mRNA Allele:

```sh
dna splice FRS-checkout-0001 "Some hard task" "An even harder one"
dna splice FRS-checkout "Buy soap" --before-go-laundry
dna splice FRS-checkout --pick-laundry --after-go-laundry
dna splice FRS-checkout --set-buy-soap "Buy hypoallergenic soap"
dna splice FRS-checkout --lgtm
```

- `dna splice` takes an mRNA Gene FQN as the first positional argument and resolves the active Allele for that work item in the current GRN context.
- Positional target matching is case-insensitive and kebab-insensitive, not fuzzy. The generation may be omitted when the matcher resolves exactly one active Allele in the current GRN context.
- Quoted positional arguments create new TaskRealizations attached to that mRNA Allele.
- `--before-<task-realization-name>` places the new or selected TaskRealization before an existing TaskRealization by making the existing TaskRealization depend on it.
- `--after-<task-realization-name>` places the new or selected TaskRealization after an existing TaskRealization by making it depend on the existing TaskRealization.
- `--<task-realization-name>` selects an existing TaskRealization in the mRNA Allele's TaskRealization DAG.
- `--set-<task-realization-name> <text>` replaces the text of an existing TaskRealization.
- `--lgtm` expresses current `Unexpressed` Mutations without editing the existing TaskRealization DAG.
- `dna splice <target>` with neither TaskRealization text nor `--lgtm` is invalid.
- TaskRealizations attached to the Allele organize as a DAG through `depends_on`.
- `dna splice` is not a mutation staging command.

Render TaskRealizations from the active Allele:

```sh
dna translate FRS-checkout-0001
```

- `dna translate` takes a Locus name or Gene FQN as the first positional argument.
- Positional target matching is case-insensitive and kebab-insensitive, not fuzzy. The generation may be omitted when the matcher resolves exactly one active Allele in the current GRN context.
- `dna translate` renders TaskRealizations in dependency order.
- `dna translate` shows dependency text for TaskRealizations that depend on other TaskRealizations.
- `dna translate` errors when the active Allele has no TaskRealizations.
- `dna translate` does not change Allele, Mutation, TaskRealization, or Transcriptome state.

Mutate existing work:

```sh
dna mutate FRS-checkout-0001
```

- Without `--new`, the first positional argument is the Locus name or Gene fully qualified name.
- Positional target matching is case-insensitive and kebab-insensitive, not fuzzy. The generation may be omitted when the matcher resolves exactly one active Allele in the current GRN context.
- Mutation flags set Sequence values on the active Allele.
- Sequence flag names use the approved sequence-name matcher.
- Scalar Sequence flags use `--<sequence-name> <value>`.
- `--set-<sequence-name>` is not scalar syntax.
- Plain `--<sequence-name> <value>` is invalid for vector Sequences.
- Vector Sequence flags use operation prefixes: `--add-<sequence-name>`, `--set-<sequence-name>`, `--set-<sequence-name>-<n>`, and `--remove-<sequence-name>-<n>`.
- `--add-<sequence-name>` appends one or more values.
- `--set-<sequence-name>` replaces the whole vector.
- `--set-<sequence-name>-<n>` replaces only index `n`.
- `--remove-<sequence-name>-<n>` removes index `n` and shifts following elements.
- CLI vector indexes are one-based.
- Indexed vector operations error when `n` is out of bounds.
- DNAp does not expose a command for emptying a sequence.

## Implementation Contract

- Implement backend/application behavior for starting a new Allele through `dna mutate --new`.
- Implement backend/application behavior for mutating existing active Alleles by Gene FQN in the current GRN context.
- Implement SequenceDefinition matching with case/kebab-insensitive command input.
- Implement SequenceValue type checks against the matched SequenceDefinition.
- Implement current Allele projection from latest Mutations.
- Implement Transcriptome render cursor metadata per Sequence.
- Implement expression transitions through `dna mutate`, `dna splice`, and `dna splice --lgtm`.
- Implement `dna translate` as the TaskRealization read side for the active Allele.

## Approved Tests

- Sequence mutation names match case/kebab-insensitively.
- Mutation values must match SequenceDefinition type.
- `dna transcribe` returns latest Allele projection.
- Transcriptome stores per-Sequence render cursor metadata, not projected document content.
- Unchanged transcriptions can omit unchanged Sequences through the Transcriptome cursor.
- Changed Sequences are shown again after later Mutations.
- Repeated edits to the same `Unexpressed` Mutation update the same row and are detected by `SequenceHash` cursor changes.
- `dna splice --lgtm` expresses `Unexpressed` Mutations without requiring prior transcription.
- `dna splice <target>` errors when it has neither TaskRealization text nor `--lgtm`.
- `dna translate` renders TaskRealizations and errors when the active Allele has no TaskRealizations.
