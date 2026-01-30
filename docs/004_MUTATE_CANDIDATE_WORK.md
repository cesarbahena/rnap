# 004 Mutate Candidate Work

## Capability

Apply append-only Mutations to an Allele and project candidate state.

`dna mutate --new` also creates the initial Locus, Transposon, and Allele when at least one Sequence mutation flag is provided.

This slice is implemented together with [003_OPEN_CANDIDATE_WORK.md](003_OPEN_CANDIDATE_WORK.md). Keep this file as the mutation-specific part of the combined contract.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

Slice 004 implements:

- `Mutation`
- `Sequence`
- `SequenceValue`

## Behavior

- Mutations are append-only.
- Mutations are individual and composable.
- One command versus many commands does not change Mutation semantics.
- Mutations target SequenceDefinitions by `SequenceDefinitionId`.
- A Mutation must target an existing SequenceDefinition in the Allele's GeneFamilyGeneration.
- Mutation values must match the SequenceDefinition type.
- `Gene` and `GeneVec` values represent embedded or linked Gene references.
- CLI input for `Gene` and `GeneVec` values uses Gene FQNs and resolves them to `GeneId` internally.
- Current candidate state is projected from Mutations.
- A degraded Allele cannot accept new Mutations.
- Mutations are created by Tfs.
- CLI mutation flags use the approved sequence-name matcher from slice 002.
- `dna mutate --new` requires at least one Sequence mutation flag.
- A new Allele is not created without an actual Mutation.
- Mutating a `Spliced` Allele is allowed and changes `Allele.state` to `StaleSplice`.
- `dna transcribe` changes a `StaleSplice` Allele to `StaleTranscript`.
- `dna splice` changes `Allele.state` to `Spliced`.
- `dna splice --lgtm` changes a `StaleTranscript` Allele back to `Spliced` without editing the existing Exon DAG.
- `dna transcribe` is always allowed in every Allele state.
- `dna transcribe` renders the latest Mutation projection, including unapproved mutations such as sgRNA suggested document modifications.
- `dna transcribe` always shows approval-status comments for mutated and sgRNA Sequences.
- `Transcriptome` stores render/access cursor metadata for token-saving transcript output, not projected document content.

## CLI

Render current candidate work:

```sh
dna transcribe FRS-checkout-0001
```

- `dna transcribe` shows the current Allele projection.
- It renders latest candidate state, not only the committed/canonical Gene.
- It always includes approval-status comments for mutated and sgRNA Sequences.

Create or acknowledge Exons on the active mRNA Allele:

```sh
dna splice FRS-checkout-0001 "Some hard task" "An even harder one"
dna splice FRS-checkout "Buy soap" --before-go-laundry
dna splice FRS-checkout --pick-laundry --after-go-laundry
dna splice FRS-checkout --set-buy-soap "Buy hypoallergenic soap"
dna splice FRS-checkout --lgtm
```

- `dna splice` takes an mRNA Gene FQN as the first positional argument and resolves the active Allele for that work item.
- Gene FQN matching is fuzzy and case-insensitive. The generation may be omitted when the matcher resolves exactly one Gene or active Allele.
- Quoted positional arguments create new Exons attached to that mRNA Allele.
- `--before-<exon-name>` places the new or selected Exon before an existing Exon by making the existing Exon depend on it.
- `--after-<exon-name>` places the new or selected Exon after an existing Exon by making it depend on the existing Exon.
- `--<exon-name>` selects an existing Exon in the mRNA Allele's Exon DAG.
- `--set-<exon-name> <text>` replaces the text of an existing Exon.
- `--lgtm` acknowledges that the existing Exon DAG is still up to date after post-splice Mutations.
- Exons attached to the Allele organize as a DAG through `depends_on`.
- `dna splice` is not a mutation staging command.

Mutate existing work:

```sh
dna mutate FRS-checkout-0001
```

- Without `--new`, the first positional argument is the Gene fully qualified name.
- Gene FQN matching is fuzzy and case-insensitive. The generation may be omitted when the matcher resolves exactly one Gene.
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

- Implement backend/application behavior for starting new candidate work through `dna mutate --new`.
- Implement backend/application behavior for mutating existing active Alleles by Gene FQN.
- Implement SequenceDefinition matching with case/kebab-insensitive command input.
- Implement SequenceValue type checks against the matched SequenceDefinition.
- Implement current Allele projection from latest Mutations.
- Implement Transcriptome render cursor metadata per Sequence.
- Implement stale splice lifecycle transitions through `dna mutate`, `dna transcribe`, `dna splice`, and `dna splice --lgtm`.

## Approved Tests

- Sequence mutation names match case/kebab-insensitively.
- Mutation values must match SequenceDefinition type.
- `dna transcribe` returns latest candidate projection.
- Transcriptome stores per-Sequence render cursor metadata, not projected document content.
- Unchanged transcriptions can omit unchanged Sequences through the Transcriptome cursor.
- Changed Sequences are shown again after later Mutations.
- `dna splice --lgtm` is blocked from `StaleSplice` until `dna transcribe` moves the Allele to `StaleTranscript`.
