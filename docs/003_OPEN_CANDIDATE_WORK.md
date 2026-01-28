# 003 Open Candidate Work

## Capability

Open a Genome-scoped document/work item and its first mutable candidate.

## Contract

Use the model in [DOMAIN_MODEL.md](DOMAIN_MODEL.md).

Slice 003 implements:

- `Locus`
- `Transposon`
- `Allele`
- `AlleleOrigin`
- `AlleleState`

## Behavior

- Locus is Genome-scoped document/work item identity.
- Locus anchors committed versions for one document instance.
- Locus belongs to one GeneFamily.
- Locus belongs to a Genome in the same Insulator as its GeneFamily.
- A new Gene/work item starts from a Transposon.
- Transposon carries origin metadata only.
- Transposon does not carry Sequence values.
- Allele records whether it originated from an existing Gene or a Transposon.
- Sequence values arrive through Mutations on the Allele.
- A new candidate starts in `Mutating`.
- A candidate belongs to exactly one Genome.
- The Genome must belong to the Locus Insulator.
- The creator Tf must belong to the same Insulator.
- One active Allele is allowed per `(Locus, Tf)`.
- Multiple Tfs may each have one active Allele for the same Locus.

## Implementation Contract

Pending implementation.

## Approved Tests

Pending.
