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

`GeneFamily` is not a single document schema hard-coded by DNAp. Many enterprise schemas can share one `NormalizedArtifact`.

Example: multiple GeneFamilies may use `EnterpriseNegotiationHandoverCertificate`, such as business research, technology research, market research, or security research.

## Normalized Artifacts

Every GeneFamily has a system-fixed `NormalizedArtifact`. The normalized artifact taxonomy controls document handling and is core product value.

`NormalizedArtifact` replaces the older `EncodingType::RNA(...)` and `EncodingType::GRN(...)` split as the canonical taxonomy.

All `NormalizedArtifact` variants are Gene-capable artifact types. Lifecycle semantics may differ by artifact type and tenant workflow policy, but the artifact itself is modeled through GeneFamily, Locus, Allele, Gene, and Mutation.

- `Promoter`: user story or idea used to start discussion.
- `ProblemAssertionManifest`: PAM, structured problem assertion.
- `ExploratoryNarrative`: eRNA, flexible typed exploration document. Examples include event storming, draft diagrams, and follow-up exploration.
- `ProjectedIntent`: piRNA, projected or explicit intent boundary.
- `Spacer`: documented risk or risk assessment.
- `Protospacer`: unforeseen or emergent risk.
- `Phenotype`: definition of success.
- `EnterpriseNegotiationHandoverCertificate`: Enhancer, formal enterprise negotiation/research handover artifact.
- `Silencer`: retirement document.
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

Lifecycle, dependency, authorization, and tenant-specific workflow semantics are deferred until concrete use cases define the structures needed. Likely direction: configurable workflow policy analogous to Histone-backed configuration, but no ChromatinRemodeler design is approved yet.
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
