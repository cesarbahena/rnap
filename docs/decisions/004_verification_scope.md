# 004 Verification Scope

Status: Accepted
Date: 2026-06-03
Slices: All

## Context

Agents have been running the full Rust test suite after documentation-only edits. That creates noise and slows down documentation/modeling work without increasing confidence when no executable behavior changed.

## Decision

Run code verification only when code or executable configuration changes.

Documentation-only changes do not require `cargo test` or other code test suites. For documentation-only changes, verify by reading/reviewing the changed docs and checking for obvious broken references or contradictions relevant to the edit.

If a documentation change updates an implementation contract and code is also changed in the same task, run the relevant code verification after the code change.

## Consequences

- Agents should not run tests for docs-only edits.
- Agents should still run `cargo fmt --check` and `cargo test` after Rust code changes unless a narrower approved verification is provided.
- Verification reports must distinguish docs-only review from code test execution.
