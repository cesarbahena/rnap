# 003/004 Mutate Candidate Work

## Capability

Create or change candidate work through mutation.

This contract is implemented together with [004_MUTATE_CANDIDATE_WORK.md](004_MUTATE_CANDIDATE_WORK.md).

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

Combined slice 003/004 implements:

- `Locus`
- `Transposon`
- `Allele`
- `AlleleOrigin`
- `AlleleState`
- `Mutation`
- `Sequence`
- `SequenceValue`

## Behavior

- Locus is Genome-scoped document/work item identity.
- Locus anchors committed versions for one document instance.
- Locus has a document instance name from the first positional CLI argument when created through `dna mutate --new`.
- Locus name is not a Sequence value.
- Gene FQN presentation uses Insulator configuration with Genome override.
- Default Gene FQN presentation is `<gene-family-abbreviation>-<locus-name-slug>-<generation>`, for example `FRS-checkout-0001`.
- CLI Gene FQN matching is case-insensitive even when presentation case is configurable.
- Locus belongs to one GeneFamily.
- Locus belongs to a Genome in the same Insulator as its GeneFamily.
- A new Gene/work item starts from a Transposon during `dna mutate --new`.
- Transposon carries origin metadata only.
- Transposon does not carry Sequence values.
- Allele records whether it originated from an existing Gene or a Transposon.
- Allele is created only when `dna mutate --new` includes at least one Sequence mutation.
- Sequence values arrive through Mutations on the Allele in the same operation that creates the new document.
- A new candidate starts in `Mutating`.
- `dna splice` moves an Allele to `Spliced`.
- Mutating a `Spliced` Allele moves it to `StaleSplice`.
- `dna transcribe` moves a `StaleSplice` Allele to `StaleTranscript`.
- `dna splice --lgtm` acknowledges the current Exon DAG and moves a `StaleTranscript` Allele back to `Spliced`.
- `dna select` moves the Allele to `Selected` and creates the immutable Gene.
- A candidate belongs to exactly one Genome.
- The Genome must belong to the Locus Insulator.
- The creator Tf must belong to the same Insulator.
- One active Allele is allowed per `(Locus, Tf)`.
- Multiple Tfs may each have one active Allele for the same Locus.
- Mutations are append-only.
- Mutations target SequenceDefinitions by `SequenceDefinitionId`.
- Mutation values must match the SequenceDefinition type.
- Current candidate state is projected from Mutations.
- A degraded Allele cannot accept new Mutations.

## CLI

Start a new document with an initial mutation:

```sh
dna mutate --new FRS 'Checkout' --some-section 'Awesome section'
```

- `--new FRS` resolves `FRS` as the effective GeneFamily abbreviation in the current Genome context.
- The first positional argument, `Checkout`, is the Locus document instance name.
- At least one Sequence mutation flag is required.
- DNAp creates Locus, Transposon, first Allele, and initial Mutation records together.

## Implementation Contract

- Implement backend/application behavior for starting a new document through Locus, Transposon, first Allele, and at least one initial Mutation.
- Implement the default Gene FQN format only: `<gene-family-abbreviation>-<locus-name-slug>-<generation>`.
- Keep Gene FQN matching case-insensitive.
- Defer configurable FQN storage and Genome override implementation.

## Approved Tests

Pending.
