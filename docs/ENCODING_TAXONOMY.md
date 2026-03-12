# DNAp Normalized Artifact Taxonomy

Every GeneFamily has one system-fixed `NormalizedArtifact`. The artifact taxonomy controls document handling and is core product value.

`NormalizedArtifact` replaces the older `EncodingType::RNA(...)` and `EncodingType::GRN(...)` split as the canonical taxonomy. Legacy biology names remain useful lineage and implementation vocabulary, but the product model is one normalized artifact enum.

All `NormalizedArtifact` variants are Gene-capable artifact types. Previous exceptions such as Intron, Exon, Cas, and Protein are no longer non-Gene workflow records by default; their lifecycle semantics may differ, but they are still modeled through GeneFamily, Locus, Allele, Gene, and Mutation.

Canonical term meanings are defined in [ONTOLOGY.md](ONTOLOGY.md).

## Naming Convention

`NormalizedArtifact` variants use full enterprise semantic names.

Biology-inspired names and backronyms remain first-class internal domain language where they model relationships clearly. They should be used for typed artifact-reference wrappers, CLI aliases, docs, and internal workflow roles, not as tenant-facing defaults.

Examples:

```rust
NormalizedArtifact::ManagedRequirement
struct MRna(ArtifactRef);

NormalizedArtifact::Ribozyme
struct Ribozyme(ArtifactRef);

NormalizedArtifact::EnterpriseNegotiationHandoverCertificate
struct Enhancer(ArtifactRef);
```

`ArtifactRef` is a resolved reference to a Locus plus the NormalizedArtifact invariant. A raw `LocusId` may be stored, but domain APIs should use `ArtifactRef` or a wrapper when a relationship requires a specific artifact type.

```rust
enum NormalizedArtifact {
    Promoter,
    ProblemAssertionManifest,
    Executable,
    Ribozyme,
    ProjectedIntent,
    Spacer,
    Protospacer,
    Phenotype,
    EnterpriseNegotiationHandoverCertificate,
    Silencer,

    StrategicNote,
    SemanticNarrowing,
    SemanticConstraintAssumption,
    Microalignment,
    StopImplementation,
    DeferredScope,

    Intron,
    ManagedRequirement,
    Exon,
    ResourceReference,
    TaskRealization,
    TaskRealizationFramework,

    TestRegressionCriteria,
    TestObjectiveManifest,
    TestOrchestrationManifest,
    CentralRuntimeManifest,
    CountermeasureAssessmentSystem,
    ProductionTestedImplementation,
    Chaperone,

    TaskMediation,
    CausalResolution,
    TraceReport,
    LongNarrativeContext,
    CircularInstitutionalReferenceContext,
    SuggestedChanges,
}
```

## Artifact Meanings

- `Promoter`: initiative seed or story used to start discussion.
- `ProblemAssertionManifest`: PAM, structured problem assertion.
- `Executable`: eRNA, human-readable executable governance artifact with IAM-like DSL attributes, mainly regulating Gene lifecycle and command state changes by checking dependency state.
- `Ribozyme`: flexible exploration artifact for event storming, draft diagrams, early idea graphs, discovery narratives, and whiteboard collaboration.
- `ProjectedIntent`: piRNA, projected or explicit intent boundary.
- `Spacer`: risk assessment.
- `Protospacer`: unforeseen or emergent risk.
- `Phenotype`: definition of success.
- `EnterpriseNegotiationHandoverCertificate`: Enhancer, formal enterprise negotiation/research handover artifact.
- `Silencer`: initiative rejection.
- `StrategicNote`: snoRNA, strategic note or ADR-like artifact.
- `SemanticNarrowing`: snRNA, semantic narrowing artifact.
- `SemanticConstraintAssumption`: scaRNA, semantic constraint or assumption artifact.
- `Microalignment`: miRNA, small scope/alignment adjustment.
- `StopImplementation`: siRNA, authoritative stop/out-of-scope artifact.
- `DeferredScope`: dsRNA, deferred scope artifact.
- `Intron`: raw requirement.
- `ManagedRequirement`: mRNA, managed requirements document.
- `Exon`: refined requirement.
- `ResourceReference`: rRNA, resource or reference document.
- `TaskRealization`: tRNA, task realization artifact.
- `TaskRealizationFramework`: tRF, task realization framework.
- `TestRegressionCriteria`: TERC, test plan.
- `TestObjectiveManifest`: Telomere, executable test case.
- `TestOrchestrationManifest`: CI configuration.
- `CentralRuntimeManifest`: CD configuration.
- `CountermeasureAssessmentSystem`: Cas, threat model or security document.
- `ProductionTestedImplementation`: Protein, verification pipeline artifact.
- `Chaperone`: SDLC alignment review.
- `TaskMediation`: tmRNA, task mediation artifact.
- `CausalResolution`: crRNA, causal or incident resolution artifact.
- `TraceReport`: tracrRNA, trace or root-cause report.
- `LongNarrativeContext`: lncRNA, long narrative context.
- `CircularInstitutionalReferenceContext`: circRNA, institutional reference context.
- `SuggestedChanges`: sgRNA, suggested changes artifact.

## Deferred Lifecycle Semantics

`NormalizedArtifact` defines what artifact kind a GeneFamily represents. It does not hardcode lifecycle, dependency, authorization, or tenant-specific workflow semantics.

Those semantics are deferred until concrete use cases define the structures needed. eRNA is reserved for human-readable executable governance artifacts with IAM-like DSL attributes, mainly regulating Gene lifecycle and command state changes by checking dependency state.
