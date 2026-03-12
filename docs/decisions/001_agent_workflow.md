# 001 Agent Workflow and Decision Authority

Status: Accepted
Date: 2026-06-03
Slices: All

## Context

DNAp work depends on a domain philosophy with meaningful, punchy biological analogies, lean interaction design, and a CLI that is easy for both humans and LLMs to use. Future agents must not silently codify unresolved ambiguity or drift into generic CRUD/SaaS patterns.

## Decision

Agents must follow this workflow:

1. Know the current project scope before proposing or implementing changes.
2. Ask high-value questions to resolve ambiguity before docs or code changes.
3. Update docs when a decision or workflow is resolved.
4. Implement in code only after the relevant decision is 100% resolved and approved.
5. Commit often once implementation starts.
6. Suggest changes that are not aligned with current scope only when explicitly framed as out-of-scope or scope-changing.
7. Suggest architecture or pattern changes when justified by the model, implementation pressure, or future maintainability.
8. Push back when a request conflicts with the approved model, project scope, or DNAp philosophy.
9. Require user approval for all material decisions, especially naming.

## Consequences

- Documentation is the first durable output of approved decisions.
- Code should not be used to settle unresolved product or naming questions.
- Naming, CLI language, and architecture must remain aligned with DNAp's domain philosophy.
- Agents should be proactive about risks and alternatives, but approval remains with the user.
