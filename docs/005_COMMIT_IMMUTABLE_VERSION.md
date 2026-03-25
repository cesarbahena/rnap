# 005 Commit Immutable Version

## Status

Blocked. `dna select` semantics are not approved.

## Capability

Unresolved. This slice name is historical build-order language only; it is not approval to implement selection, immutable Gene creation, approval, implementation readiness, or any lifecycle transition.

## Blocking Decision

Before this slice can be implemented, the user must explicitly approve what `dna select` means in product terms.

Do **not** infer selection semantics from:

- the existence of `Gene`,
- `AlleleState::Selected`,
- this slice name,
- old docs,
- candidate structs,
- tests,
- conventional version-control analogies.

## Known Open Questions

Ask one at a time before proposing code:

- Does `dna select` create an immutable Gene, or is immutable Gene creation a separate transition?
- Is selection a canonical document-version decision, a governance approval checkpoint, implementation readiness, or something else?
- What facts must be checked before selection: TaskRealizations, Signals, Histones, eRNA, required Sequences, review state, dependency state?
- Is selection always side-effect-free until a command commits Signals, or does selection itself emit Signals and create records?

## Implementation Contract

No implementation contract is approved.

## Approved Tests

No tests are approved.
