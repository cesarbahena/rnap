# 007 Add Workflow Artifacts

## Capability

Add Regulatory RNA workflow documents around candidate work.

## Contract

Use [DOMAIN_MODEL.md](DOMAIN_MODEL.md) and [ENCODING_TAXONOMY.md](ENCODING_TAXONOMY.md).

Slice 007 implements workflow document records for selected Regulatory RNA encodings and concrete TfComplex discussion relationships.

## Behavior

- `Intron` is a chainable disambiguation item.
- `SnRNA` is a task modification suggestion for an mRNA.
- `ScaRNA` is a requirement modification suggestion from implementation reality.
- `SiRNA` is an authoritative out-of-scope order.
- `TmRNA` is an unblocker mediation request.
- `GRNA` is a general message.
- `MiRNA` is an emergent scope reduction discussion.
- `PiRNA` is an explicit out-of-scope discussion.
- `SnoRNA` is an ADR.
- `CrRNA` is an incident report.
- `TracrRNA` is a root cause analysis.
- `LncRNA` is a research document.
- `CircRNA` is onboarding particularities.
- `SgRNA` is a suggested CRISPR action change.
- Workflow artifacts are created by Tfs and evaluated through Histones.
- `Exon` remains the executable task created by `dna splice`; it is not an EncodingType.
- `Cas` actions belong to CRISPR workflows; `Cas` is not an EncodingType.
- `TfComplex` names the discussion/alignment subsystem. It is not automatically a persisted container object.
- Code should persist only concrete discussion records and typed relationships with a defined workflow purpose.
- `TfComplex` does not define Gene schemas and does not own workflow rules.
- TfComplex relationships may target whole documents or work items inside those documents, but allowed targets are constrained by each use case.

## Implementation Contract

Pending implementation.

## Approved Tests

Pending.
