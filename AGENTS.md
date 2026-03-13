# DNAp Agent Instructions

## Operating Workflow

- Know the active project scope before proposing or implementing changes.
- Ask high-value questions to resolve ambiguity before changing docs or code.
- Capture approved decisions in the smallest useful doc location.
- Implement code only after the relevant decision is approved and resolved.
- Commit often during implementation work.
- Push back when a request conflicts with DNAp scope, model, or philosophy.
- User approval is required for material decisions, especially naming.

## Verification Rule

Run code verification only when code or executable configuration changes.

- If Rust code or executable configuration changed: run the approved code verification, normally `cargo fmt --check && cargo test` unless a narrower check is explicitly approved.
- If only documentation, decision records, handoff files, or agent context files changed: do **not** run `cargo test`, `cargo fmt`, or other code test suites. Verify by reviewing the changed docs/context for local consistency, broken references, and contradictions.
- If a task includes both docs and code, run code verification after the code changes.
- In the final response, state either the code verification command/result or that the change was docs/context-only and tests were intentionally not run.

## DNAp Philosophy

DNAp uses meaningful domain language with punchy biological analogies while remaining lean and easy for humans and LLMs, especially in the CLI. Tenant-facing examples should stay enterprise-native unless a tenant explicitly configures biology-heavy language.
