# 003/004 Mutate Candidate Work

## Capability

Create or change an Allele through mutation.

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

- Locus is stable document/work item identity inside a Chromosome.
- Locus anchors committed versions for one document instance.
- Locus names are unique within the containing Genome under DNAp canonical name matching.
- Locus may move between Chromosomes with Signal audit.
- Locus has a document instance name from the first positional CLI argument when created through `dna mutate --new`.
- Locus name is not a Sequence value.
- Gene FQN presentation uses Insulator configuration with Genome override.
- Default Gene FQN presentation is `<gene-family-abbreviation>-<locus-name-slug>-<generation>`, for example `FRS-checkout-0001`.
- CLI Gene FQN matching is case-insensitive even when presentation case is configurable.
- Locus belongs to one GeneFamily.
- Locus belongs to one current Chromosome.
- A new Gene/work item starts from a Transposon during `dna mutate --new`.
- Transposon carries origin metadata only.
- Transposon does not carry Sequence values.
- Allele records whether it originated from an existing Gene or a Transposon.
- Allele may be created by `dna mutate --new` with zero Sequence mutations.
- Sequence values arrive through later or same-command Mutations on the Allele.
- A new candidate starts in `Mutating`.
- `dna splice` moves an Allele to `Expressing`.
- Mutating an `Expressing` Allele moves it back to `Mutating` when it creates or updates `Unexpressed` Mutations.
- `dna splice --lgtm` is an escape hatch that expresses current `Unexpressed` Mutations without changing Exons.
- `dna select` moves the Allele to `Selected` and creates the immutable Gene.
- A candidate Allele belongs to exactly one GRN.
- The GRN must belong to the same Genome as the Locus's Chromosome.
- One active shared Allele is allowed per `(Locus, GRN)`.
- Multiple GRNs may each have one active Allele for the same Locus.
- Mutations are editable while `Unexpressed`.
- Mutations target SequenceDefinitions by `SequenceDefinitionId`.
- Mutation values must match the SequenceDefinition type.
- Current Allele state is projected from Mutations.
- A degraded Allele cannot accept new Mutations.

## CLI

Start a new document:

```sh
dna mutate 'Checkout' --new FRS
dna mutate 'Checkout' --new FRS --some-section 'Awesome section'
```

- `--new FRS` resolves `FRS` as the effective GeneFamily abbreviation in the current Genome context.
- The first positional argument, `Checkout`, is the Locus document instance name.
- Sequence mutation flags are optional for `dna mutate --new`.
- DNAp creates Locus, Transposon, and first Allele together. If Sequence mutation flags are present, it also creates or updates `Unexpressed` Mutations.

## Implementation Contract

- Implement backend/application behavior for starting a new document through Locus, Transposon, and first Allele.
- Implement the default Gene FQN format only: `<gene-family-abbreviation>-<locus-name-slug>-<generation>`.
- Keep Gene FQN matching case-insensitive.
- Defer configurable FQN storage and Genome override implementation.

## Approved Tests

- `dna mutate --new` can create Locus, Transposon, Allele, zero Mutations, and default Gene FQN.
- `dna mutate --new` can also create initial `Unexpressed` Mutation records.
- Active Allele target matching allows Locus name or omitted generation when it resolves exactly one active Allele in the current GRN context.
- Mutating an `Expressing` Allele with Sequence flags moves it to `Mutating`.
- `dna splice --lgtm` moves the Allele back to `Expressing` by expressing current `Unexpressed` Mutations without changing Exons.
- Active Allele FQN resolution is scoped to the current GRN.
