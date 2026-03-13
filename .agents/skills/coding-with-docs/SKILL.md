---
name: coding-with-docs
description: Use while implementing an approved DNAp slice so code, tests, and docs stay aligned.
---

# Coding With Docs

## Goal

Implement only the approved slice contract and keep documentation accurate without creating noise.

## Workflow

1. Read `AGENTS.md`.
2. Read the active slice doc.
3. Verify the slice has approved names, invariants, implementation contract, and approved tests.
4. Implement the smallest backend + CLI vertical path for the contract.
5. Keep CLI logic thin over backend/application behavior.
6. Add only explicitly approved tests.
7. Run the approved verification only if code or executable configuration changed.
8. For docs/context-only edits, do not run code tests; review the changed text for consistency instead.
9. Update docs only when implementation changes a real decision or contract.

## Rules

- Do not code unapproved biological names.
- Do not smuggle deferred decisions into code.
- Do not preserve old architecture accidentally.
- Do not add persistence, authorization, workflows, or future-slice concepts unless approved.
- Stop and ask one question if ambiguity appears.
- Tests should validate behavior or invariants, not private implementation.
- Do not run `cargo test`, `cargo fmt`, or other code verification for docs/context-only edits.
- Run `cargo fmt --check && cargo test` after Rust code changes unless a narrower approved verification is provided.

## Completion Checklist

```markdown
- [ ] Approved names used exactly
- [ ] Approved invariants implemented
- [ ] Backend/application behavior is source of truth
- [ ] CLI is thin over backend/application behavior
- [ ] Only approved tests added
- [ ] Code verification passed/reported, or skipped because the change was docs/context-only
- [ ] Docs updated only where reality changed
```
