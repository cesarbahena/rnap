# DNAp Build Index

DNAp is an enterprise multitenant SDLC platform for configurable document types and delivery workflows.

## Core Documents

- [ONTOLOGY.md](ONTOLOGY.md): canonical DNAp terms and meanings.
- [WORKFLOW_MODEL.md](WORKFLOW_MODEL.md): approved workflow interactions among DNAp concepts.
- [DOMAIN_MODEL.md](DOMAIN_MODEL.md): current product/domain model.
- [ENCODING_TAXONOMY.md](ENCODING_TAXONOMY.md): system-fixed SDLC document encodings.
- [DEFERRED_DOMAIN_LEDGER.md](DEFERRED_DOMAIN_LEDGER.md): domain obligations intentionally not implemented yet.

## Build Order

1. [001_BOOTSTRAP_TENANT_PROJECT_USER.md](001_BOOTSTRAP_TENANT_PROJECT_USER.md)
   Provision the minimum operational world: Insulator, placement, Genome, and Tf.

2. [002_DEFINE_WORK_TYPE.md](002_DEFINE_WORK_TYPE.md)
   Define configurable SDLC document types through GeneFamily and GeneFamilyGeneration.

3. [003_OPEN_CANDIDATE_WORK.md](003_OPEN_CANDIDATE_WORK.md) + [004_MUTATE_CANDIDATE_WORK.md](004_MUTATE_CANDIDATE_WORK.md)
   Create or change an Allele through `dna mutate`; `dna mutate --new` can create an Allele before Sequence values are filled.

4. Reserved.

5. [005_CLI_SESSION_AND_EPIGENETICS.md](005_CLI_SESSION_AND_EPIGENETICS.md)
   Run local CLI workflows through a replaceable session boundary and temporary `dna epigenetics` bootstrap namespace.

6. [005_COMMIT_IMMUTABLE_VERSION.md](005_COMMIT_IMMUTABLE_VERSION.md)
   Select an Allele and create an immutable Gene.

7. [006_AUTHORIZE_WITH_HISTONES.md](006_AUTHORIZE_WITH_HISTONES.md)
   Add Histone-based authorization and contextual evaluation.

8. [007_ADD_WORKFLOW_ARTIFACTS.md](007_ADD_WORKFLOW_ARTIFACTS.md)
   Add Regulatory RNA workflow documents around Alleles.

9. [008_ADD_IMPLEMENTATION_EVALUATION.md](008_ADD_IMPLEMENTATION_EVALUATION.md)
   Link Alleles to implementation output and evaluation.

## Build Rules

- Build backend/application behavior first.
- Do not expose normal tenant-user CLI commands for provisioning Insulators.
- CLI commands should use mainstream biology-oriented workflow language and avoid generic CRUD.
- Tenant data remains enterprise-native unless a tenant explicitly configures biology-heavy language.
- Configurable definitions may be Insulator-scoped or Genome-scoped.
- Scoped configurable definitions resolve nearest scope first: Genome override, then Insulator default.
- Do not ask sequencing-only questions when the long-term product model is clear.
