# DNAp Encoding Taxonomy

DNAp means Document Normalization and Alignment Platform.

Every GeneFamily has an `EncodingType`. Encoding is required because it controls document handling and is core product value.

RNA means Request for Normalization and Alignment.

```rust
enum EncodingType {
    RNA(RnaType),
    GRN(GrnType),
}
```

## GRN

GRN encodings are discussion and delivery-control documents.

```rust
enum GrnType {
    Promoter,
    Enhancer,
    PIWI,
    Spacers,
    Telomere,
    Centromere,
    Silencer,
}
```

- `Promoter`: user story or idea used to start discussion.
- `Enhancer`: formal research document. Examples include business, technology, and market research.
- `PIWI`: documented scope.
- `Spacers`: documented risk.
- `Telomere`: testing document.
- `Centromere`: deployment document.
- `Silencer`: retirement document.

## RNA

RNA encodings are SDLC production and DNAp workflow/control documents.

```rust
enum RnaType {
    Translation(TranslationRnaType),
    Regulatory(RegulatoryRnaType),
}
```

## Translation RNA

Translation RNA encodings are core production and implementation documents.

```rust
enum TranslationRnaType {
    ERNA,
    MRNA,
    RRNA,
    TRNA,
}
```

- `ERNA`: flexible typed exploration graph node. Examples include event storming, draft diagrams, and follow-up exploration.
- `MRNA`: requirements analysis document.
- `RRNA`: architecture/design document.
- `TRNA`: agentic skills.

## Regulatory RNA

Regulatory RNA encodings are DNAp workflow/control documents.

```rust
enum RegulatoryRnaType {
    Intron,
    SnRNA,
    ScaRNA,
    SiRNA,
    TmRNA,
    GRNA,
    MiRNA,
    PiRNA,
    SnoRNA,
    CrRNA,
    TracrRNA,
    LncRNA,
    CircRNA,
    SgRNA,
}
```

- `Intron`: disambiguation item. Introns may be chained.
- `SnRNA`: task modification suggestion for an mRNA.
- `ScaRNA`: requirement modification suggestion from implementation reality.
- `SiRNA`: authoritative out-of-scope order.
- `TmRNA`: unblocker mediation request.
- `GRNA`: general message.
- `MiRNA`: emergent scope reduction discussion.
- `PiRNA`: explicit out-of-scope discussion.
- `SnoRNA`: ADR.
- `CrRNA`: incident report, possibly linked to a risk.
- `TracrRNA`: root cause analysis.
- `LncRNA`: research document.
- `CircRNA`: onboarding particularities.
- `SgRNA`: suggested CRISPR action change.

## Non-Encoding Workflow Terms

`Exon` is an executable task created by `dna splice`. It is not an `EncodingType`.

`Cas` is an action concept used by CRISPR workflows. It is not an `EncodingType`.
