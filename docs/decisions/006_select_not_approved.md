# 006 dna select Is Not Approved

Status: Accepted
Date: 2026-06-03
Slices: 005 commit immutable version, authorization, workflow governance

## Context

Agents have repeatedly tried to implement `dna select` as if its semantics were already settled. They are not. Selection touches immutable Gene creation, lifecycle state, governance, dependency gates, authorization, and future eRNA/Histone behavior.

## Decision

Do not implement `dna select` yet.

Do not infer that `dna select` creates an immutable Gene, approves an Allele, marks implementation readiness, or performs any other lifecycle transition until the user explicitly approves its semantics.

The existence of `Gene`, `AlleleState::Selected`, old slice names, candidate structs, tests, or previous docs is not approval to implement selection behavior.

## Consequences

- Slice `005_COMMIT_IMMUTABLE_VERSION` is blocked pending an explicit decision.
- Any future proposal for `dna select` must first ask what selection means in product terms.
- eRNA/Histone governance may need to be decided before selection can be safely implemented.
