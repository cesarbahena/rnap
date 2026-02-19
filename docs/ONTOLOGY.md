# DNAp Ontology

DNAp means Document Normalization and Alignment Platform.

This document defines product terms. It does not define build order or storage implementation.

## Platform

- `Insulator`: customer/account tenant and hard isolation boundary.
- `Genome`: project boundary inside an Insulator.
- `Tf`: user identity. Tf means Team Factor.
- `Histone`: IAM-like system permission and contextual evaluation attribute.

## Document Identity

- `GeneFamily`: configurable enterprise document or work-item type.
- `GeneFamilyGeneration`: immutable schema version for a GeneFamily.
- `SequenceDefinition`: one required configurable field in a GeneFamilyGeneration.
- `Locus`: stable identity for one controlled document or controlled document item.
- `Gene`: canonical controlled document or controlled document item.
- `Transposon`: new non-canonical Gene origin.
- `Allele`: work-in-progress Gene candidate for a Locus.
- `Mutation`: Sequence value change on an Allele.

`GeneFamily` is not a single document schema hard-coded by DNAp. Many enterprise schemas can share one EncodingType.

Example: multiple GeneFamilies may encode `Enhancer`, such as business research, technology research, market research, or security research.

## Encoding

Every GeneFamily has a system-fixed `EncodingType`. Encoding controls document handling and is core product value.

RNA means Request for Normalization and Alignment.

### GRN

- `Promoter`: user story or idea used to start discussion.
- `Enhancer`: formal research document. Examples include business, technology, and market research.
- `PIWI`: documented scope.
- `Spacers`: documented risk.
- `Telomere`: testing document.
- `Centromere`: deployment document.
- `Silencer`: retirement document.

### Translation RNA

- `eRNA`: flexible typed exploration graph node. Examples include event storming, draft diagrams, and follow-up exploration.
- `mRNA`: requirements analysis document.
- `rRNA`: architecture/design document.
- `tRNA`: agentic skills.

### Regulatory RNA

- `Intron`: disambiguation item. Introns may be chained.
- `snRNA`: task modification suggestion for an mRNA.
- `scaRNA`: requirement modification suggestion from implementation reality.
- `siRNA`: authoritative out-of-scope order.
- `tmRNA`: unblocker mediation request.
- `miRNA`: emergent scope reduction discussion.
- `piRNA`: explicit out-of-scope discussion.
- `snoRNA`: ADR.
- `crRNA`: incident report, possibly linked to a risk.
- `tracrRNA`: root cause analysis.
- `lncRNA`: research document.
- `circRNA`: onboarding particularities.
- `sgRNA`: suggested CRISPR action change.

## Non-Encoding Workflow Terms

- `Exon`: executable task created by `dna splice`. Exon is not an EncodingType.
- `Cas`: CRISPR action concept. Cas is not an EncodingType.
- `TfComplex`: discussion and alignment subsystem. It names the communication model but is not automatically a persisted container object.

## TfComplex Areas

`TfComplex` preserves these named conceptual groups:

```rust
struct TfComplex {
    pre_initiation_complex: Vec<PreInitiator>,
    repressors_complex: Vec<Repressor>,
    mediators_complex: Vec<Mediator>,
    unaligned_product_response: Vec<Upr>,
    crispr_cas: Vec<Crispr>,
    structural_mantainance_complex: Vec<Smc>,
}
```

This shape preserves the named conceptual groups. It is not approval to persist a monolithic `TfComplex` object.
