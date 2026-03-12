# DNAp Ontology

DNAp means Development Network Alignment Platform.

This document defines product terms. It does not define build order or storage implementation.

Modeling rules for these terms are defined in [ABSTRACTION_CHARTER.md](ABSTRACTION_CHARTER.md).

## Platform

- `Insulator`: customer/account tenant and hard isolation boundary.
- `Genome`: project boundary inside an Insulator.
- `Chromosome`: named canonical scope inside a Genome.
- `GRN`: Governance Regulatory Network; work context inside a Genome from triage onward. A GRN owns operational lifecycle state.
- `Operon`: grouping of Promoter artifacts inside one GRN, such as an epic.
- `Tf`: user identity. Tf means Team Factor.
- `Histone`: permission/governance fact.

## Document Identity

- `GeneFamily`: configurable enterprise document or work-item type.
- `GeneFamilyGeneration`: immutable schema version for a GeneFamily.
- `SequenceDefinition`: one required configurable field in a GeneFamilyGeneration.
- `Locus`: stable identity for one controlled document or controlled document item. A Locus belongs to one current Chromosome and may move between Chromosomes.
- `Gene`: canonical controlled document or controlled document item.
- `Transposon`: new non-canonical Gene origin.
- `Allele`: shared work-in-progress Gene candidate for a Locus inside one GRN.
- `Mutation`: Sequence value change on an Allele.
- `Signal`: append-only tenant-scoped audit/provenance event for domain transitions.

The stable workflow foundation is Chromosome, GRN, Operon, Promoter membership, shared Alleles, and Signal audit. A Promoter may be assigned to only one active Operon at a time. Triage responsibility belongs to Promoter-in-Operon membership and is not authorization. Cross-GRN or cross-Operon relationships use explicit relationships such as dependency, duplication, split, or conflict rather than multiple active membership.

`GeneFamily` is not a single document schema hard-coded by DNAp. Many enterprise schemas can share one `NormalizedArtifact`.

Example: multiple GeneFamilies may use `EnterpriseNegotiationHandoverCertificate`, such as business research, technology research, market research, or security research.

## Normalized Artifacts

Every GeneFamily has a system-fixed `NormalizedArtifact`. The normalized artifact taxonomy controls document handling and is core product value.

`NormalizedArtifact` replaces the older `EncodingType::RNA(...)` and `EncodingType::GRN(...)` split as the canonical taxonomy.

All `NormalizedArtifact` variants are Gene-capable artifact types. Lifecycle semantics may differ by artifact type and tenant workflow policy, but the artifact itself is modeled through GeneFamily, Locus, Allele, Gene, and Mutation.

`NormalizedArtifact` variants use full enterprise semantic names. Biology-inspired names and backronyms remain first-class internal domain language for typed artifact-reference wrappers, CLI aliases, docs, and workflow roles.

Example: `NormalizedArtifact::ManagedRequirement` is the artifact taxonomy value, while `MRna(ArtifactRef)` may be used internally when a relationship specifically needs a managed requirement reference.

- `Promoter`: user story or idea used to start discussion.
- `ProblemAssertionManifest`: PAM, structured problem assertion.
- `Executable`: eRNA, human-readable executable governance artifact with IAM-like DSL attributes, mainly regulating Gene lifecycle and command state changes by checking dependency state.
- `Ribozyme`: flexible exploration artifact for event storming, draft diagrams, early idea graphs, discovery narratives, and whiteboard collaboration.
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

Lifecycle, dependency, authorization, and tenant-specific workflow semantics are deferred until concrete use cases define the structures needed. eRNA is reserved for protocol/evaluator/control rule artifacts. No ChromatinRemodeler design is approved.

## Workflow Concepts

- `TfComplex`: discussion and alignment subsystem. It names the communication model but is not approved as a persisted container object.
- `PreInitiationComplex`, `MediatorComplex`, `RepressorsComplex`, `CRISPR`, and `StructuralMaintenance`: named workflow-channel concepts. They are not part of the stable persisted foundation until concrete workflows define their records and relationships.

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
