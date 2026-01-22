# DNAp Vertical Build Index

## Purpose

These documents guide a vertical rebuild of DNAp.

Each numbered document is an end-to-end slice that can be disambiguated, name-approved, implemented, and tested before moving to the next slice. The goal is not to finish all design upfront. The goal is to keep the whole product philosophy visible while making one useful capability real at a time.

## Product Philosophy

DNAp is the ultimate SDLC platform.

DNAp models software delivery as evolvable project work: users create and refine structured work, candidates evolve through explicit changes, committed versions become immutable history, authorization and evaluation are part of the model, and later workflow/implementation artifacts connect the model to real delivery.

Biology is heavy in code. Biology is more mainstream in commands. Biology is nonexistent in tenant data unless a tenant explicitly chooses it. Every biological name must be approved before it becomes permanent code, command, file, module, or type language.

## Approved Core Direction

- `Insulator` is tenant boundary.
- `Genome` is project boundary.
- `Cell` is removed for now.
- `TF` is user identity.
- `Genome` can define project-local `GeneFamily`, `TfClass`, and `Histone` extensions.
- `GeneFamily` defines a configurable work/document type.
- `GeneFamilyGeneration` versions a GeneFamily schema.
- `Locus` anchors identity across committed versions.
- `Allele` is mutable candidate work.
- `Mutation` is append-only change.
- `Gene` is immutable committed version selected from an Allele.
- `Histone` and `HistoneMark` are the only authorization/contextual evaluation abstraction.

## Vertical Slices

1. [001_BOOTSTRAP_TENANT_PROJECT_USER.md](001_BOOTSTRAP_TENANT_PROJECT_USER.md)
   Create the minimum world: tenant, project, user identity.

2. [002_DEFINE_WORK_TYPE.md](002_DEFINE_WORK_TYPE.md)
   Define the first tenant/project work type and its fields.

3. [003_OPEN_CANDIDATE_WORK.md](003_OPEN_CANDIDATE_WORK.md)
   Create stable work identity and open the first mutable candidate.

4. [004_MUTATE_CANDIDATE_WORK.md](004_MUTATE_CANDIDATE_WORK.md)
   Apply append-only changes and project candidate state.

5. [005_COMMIT_IMMUTABLE_VERSION.md](005_COMMIT_IMMUTABLE_VERSION.md)
   Select a candidate and create an immutable committed version.

6. [006_AUTHORIZE_WITH_HISTONES.md](006_AUTHORIZE_WITH_HISTONES.md)
   Add the authorization layer across the already-working vertical path.

7. [007_ADD_WORKFLOW_ARTIFACTS.md](007_ADD_WORKFLOW_ARTIFACTS.md)
   Add proposal/context/reasoning artifacts around candidate work.

8. [008_ADD_IMPLEMENTATION_EVALUATION.md](008_ADD_IMPLEMENTATION_EVALUATION.md)
   Add implementation and evaluation records connected to candidates.

## Slice Document Shape

Every slice includes:

- capability,
- user-visible result,
- names requiring approval or confirmation,
- structs touched,
- invariants to decide,
- approved decisions for this slice,
- rejected alternatives for this slice,
- deferred decisions,
- implementation contract,
- old-vs-new context,
- possible loss,
- implementation gate,
- test gate.

## How We Work

For each slice:

1. Read the slice document.
2. Decide only the names, invariants, implementation contract, and tests needed for that slice.
3. Ask one high-value blocking question at a time.
4. Update the document only with approved decisions.
5. Implement the approved contract.
6. Run only approved tests/checks.
7. Move to the next slice after implementation and verification are coherent.

Cross-cutting design can be noted ahead of time, but it should not block a slice unless the current slice cannot be implemented coherently without it.

## Non-Goals

- Do not design from persistence.
- Do not preserve current code structure.
- Do not preserve old crate/module boundaries.
- Do not build a horizontal domain layer before the first vertical capability works.
- Do not keep old biological names by accident.
- Do not pre-author tests before invariants are approved.
