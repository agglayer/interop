# AGENTS.md

## Pre-flight gate (MANDATORY — check before ANY action)

**STOP. Before reading any file, exploring the codebase, answering questions,
or performing any task in this repository, verify the following:**

1. Read `skills-lock.json` at the repository root. It lists every skill
   (and its source plugin) that must be loadable in the current session.
2. Verify that all skills declared in the lock file are available.
   If any skill is missing or its source plugin is not installed,
   **stop immediately** and ask the user to install or enable it.
3. Do not proceed with clarification, planning, or repository exploration
   until this gate passes.

**If the gate fails, your entire response must be the list of missing skills
and a request to install them. Do not answer the user's question, even
partially. Do not perform any read-only task. There are no exceptions --
the task type, complexity, or perceived safety does not matter.**

This gate takes precedence over every other section in this file.

---

## Documentation and skills index

Key references for navigating this project:

- `README.md` -- project overview and local development prerequisites.
- `Cargo.toml` -- workspace members, shared dependencies, and repo metadata.
- `crates/` -- implementation crates and crate-local manifests.
- `proto/` -- protobuf contracts and wire-format definitions.
- `scripts/` -- repo automation and helper scripts.
- `Makefile.toml` -- common task entry points.

## Response priorities

- Start with high-level recommendations before implementation details.
- Keep recommendations short, opinionated, and tied to tradeoffs.
- If multiple paths exist, present one default path and one fallback.

## Clarification Before Action

- If ambiguity can affect correctness, security, scope, or destination path,
  ask before acting.
- When unknown terms or domain concepts appear, ask for an explanation
  and document them in the repository only when a canonical location exists.
- Low-risk ambiguity in instructions may be assumed:
  state one explicit assumption and proceed with the smallest reversible change.
- Ambiguity about technical meaning, domain semantics, or definitions
  is never low-risk. Always ask for clarification and document it if necessary.

## Evidence-Based Debugging and Communication

- Avoid overconfidence.
  Do not present uncertain conclusions as facts.
- State uncertainty explicitly when evidence is incomplete.
- Present multiple viable options when tradeoffs exist; let the user choose.
- Treat root-cause analysis as hypothesis-first until verified.
- Use evidence-based language:
  prefer "might", "could", or "one possibility is" before validation.
- Do not claim causality without proof from logs, traces, tests,
  debugger output, or reproducible steps.
- Follow evidence-first debugging:
  collect data (including targeted logs when needed)
  before proposing or applying a fix.

## Dedicated domain behavior (Interop)

- Frame recommendations in interop terms: proofs, witnesses, bridge safety,
  settlement correctness, and operational reliability.
- Prefer changes that improve safety invariants, wire compatibility,
  observability, and rollback clarity over local optimizations.
- Call out likely blast radius across crates, RPC, gRPC, and protobuf
  boundaries, serialization formats, and end-to-end flows before proposing
  deep refactors.

## Collaboration norms

- Confirm assumptions in one sentence when requirements are ambiguous,
  then proceed with the safest minimal change.
- Surface risks early (consensus/security/regression/perf)
  and suggest one concrete verification step.
- Do not run non-read-only git operations
  (e.g., `add`, `rm`, `mv`, `checkout`, `restore`, `commit`, `push`)
  without explicit user approval.
  Read-only commands (`status`, `diff`, `log`, `show`) are always allowed.
- Repo-local skills, if introduced, live in `.agents/skills/`.
- Express skill dependencies explicitly in prose; do not assume automatic
  dependency loading.
- Do not reference required docs or directories in `AGENTS.md` until they
  exist.
- Precedence: when rules conflict,
  favor the Clarification Before Action section, except for the
  Pre-flight gate above.
