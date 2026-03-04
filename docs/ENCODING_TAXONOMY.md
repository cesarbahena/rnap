# DNAp Normalized Artifact Taxonomy

Every GeneFamily has one system-fixed `NormalizedArtifact`. The artifact taxonomy controls document handling and is core product value.

`NormalizedArtifact` replaces the older `EncodingType::RNA(...)` and `EncodingType::GRN(...)` split as the canonical taxonomy. Legacy biology names remain useful lineage and implementation vocabulary, but the product model is one normalized artifact enum.

All `NormalizedArtifact` variants are Gene-capable artifact types. Previous exceptions such as Intron, Exon, Cas, and Protein are no longer non-Gene workflow records by default; their lifecycle semantics may differ, but they are still modeled through GeneFamily, Locus, Allele, Gene, and Mutation.

Canonical term meanings are defined in [ONTOLOGY.md](ONTOLOGY.md).

```rust
enum NormalizedArtifact {
    Promoter,
    ProblemAssertionManifest,
    ExploratoryNarrative,
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
- `ExploratoryNarrative`: eRNA, flexible exploration document for narratives, diagrams, and early graph work.
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

Those semantics are deferred until concrete use cases define the structures needed. Likely direction: tenant/project configurable workflow policy analogous to Histone-backed configuration, but no ChromatinRemodeler design is approved yet.
