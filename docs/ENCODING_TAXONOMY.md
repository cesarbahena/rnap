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

- `ERna`: flexible typed exploration graph node. Tenant-facing name: `eRNA`.
- `MRna`: requirements analysis document. Tenant-facing name: `mRNA`.
- `RRna`: architecture/design document. Tenant-facing name: `rRNA`.
- `TRna`: agentic skills. Tenant-facing name: `tRNA`.

## Regulatory RNA

Regulatory RNA encodings are DNAp workflow/control documents.

```rust
enum RegulatoryRnaType {
    Intron,
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

- `Intron`: disambiguation item. Introns may be chained.
- `SnRna`: task modification suggestion for an mRNA. Tenant-facing name: `snRNA`.
- `ScaRna`: requirement modification suggestion from implementation reality. Tenant-facing name: `scaRNA`.
- `SiRna`: authoritative out-of-scope order. Tenant-facing name: `siRNA`.
- `TmRna`: unblocker mediation request. Tenant-facing name: `tmRNA`.
- `MiRna`: emergent scope reduction discussion. Tenant-facing name: `miRNA`.
- `PiRna`: explicit out-of-scope discussion. Tenant-facing name: `piRNA`.
- `SnoRna`: ADR. Tenant-facing name: `snoRNA`.
- `CrRna`: incident report, possibly linked to a risk. Tenant-facing name: `crRNA`.
- `TracrRna`: root cause analysis. Tenant-facing name: `tracrRNA`.
- `LncRna`: research document. Tenant-facing name: `lncRNA`.
- `CircRna`: onboarding particularities. Tenant-facing name: `circRNA`.
- `SgRna`: suggested CRISPR action change. Tenant-facing name: `sgRNA`.

## Non-Encoding Workflow Terms

`Exon` is an executable task created by `dna splice`. It is not an `EncodingType`.

`Cas` is an action concept used by CRISPR workflows. It is not an `EncodingType`.
