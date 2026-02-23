# DNAp Encoding Taxonomy

Every GeneFamily has an `EncodingType`. Encoding is required because it controls document handling and is core product value.

Canonical term meanings are defined in [ONTOLOGY.md](ONTOLOGY.md).

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
    ERna,
    MRna,
    RRna,
    TRna,
}
```

- `ERna`: flexible typed exploration graph node.
- `MRna`: requirements analysis document.
- `RRna`: architecture/design document.
- `TRna`: agentic skills.

## Regulatory RNA

Regulatory RNA encodings are DNAp workflow/control documents.

```rust
enum RegulatoryRnaType {
    SnRna,
    ScaRna,
    SiRna,
    TmRna,
    MiRna,
    PiRna,
    SnoRna,
    CrRna,
    TracrRna,
    LncRna,
    CircRna,
    SgRna,
}
```

- `SnRna`: task modification suggestion for an mRNA.
- `ScaRna`: requirement modification suggestion from implementation reality.
- `SiRna`: authoritative out-of-scope order.
- `TmRna`: unblocker mediation request.
- `MiRna`: emergent scope reduction discussion.
- `PiRna`: explicit out-of-scope discussion.
- `SnoRna`: ADR.
- `CrRna`: incident report, possibly linked to a risk.
- `TracrRna`: root cause analysis.
- `LncRna`: research document.
- `CircRna`: onboarding particularities.
- `SgRna`: suggested CRISPR action change.

## Non-Encoding Workflow Terms

`Exon` is an executable task created by `dna splice`. It is not an `EncodingType`.

`Cas` is an action concept used by CRISPR workflows. It is not an `EncodingType`.
