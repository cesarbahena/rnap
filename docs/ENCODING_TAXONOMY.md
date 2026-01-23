# DNAp Encoding Taxonomy

Every GeneFamily has an `EncodingType`. Encoding is required because it controls document handling and is core product value.

```rust
enum EncodingType {
    RNA(RnaType),
    GRN(GrnType),
}
```

## GRN

GRN encodings are SDLC phase documents.

```rust
enum GrnType {
    Promoter,
    Telomere,
    Centromere,
    Silencer,
}
```

- `Promoter`: planning document.
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

Translation RNA encodings are core SDLC production documents.

```rust
enum TranslationRnaType {
    MRNA,
    RRNA,
    TRNA,
}
```

- `MRNA`: requirements analysis document.
- `RRNA`: design document.
- `TRNA`: development document.

## Regulatory RNA

Regulatory RNA encodings are DNAp workflow/control documents.

```rust
enum RegulatoryRnaType {
    SnRNA,
    SiRNA,
    TmRNA,
    GRNA,
    MiRNA,
    PiRNA,
    ERNA,
    SnoRNA,
    CrRNA,
    TracrRNA,
    LncRNA,
    CircRNA,
    SgRNA,
}
```

- `SnRNA`: disambiguation document.
- `SiRNA`: stalled implementation document.
- `TmRNA`: task managed document.
- `GRNA`: general message.
- `MiRNA`: modified implementation document.
- `PiRNA`: discard task document. This complements `Silencer`; exact boundary remains open.
- `ERNA`: priority booster document.
- `SnoRNA`: ADR.
- `CrRNA`: open issue.
- `TracrRNA`: solved issue report.
- `LncRNA`: research document.
- `CircRNA`: onboarding particularities.
- `SgRNA`: suggested document modification document.
