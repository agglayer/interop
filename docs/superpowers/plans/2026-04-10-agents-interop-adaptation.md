# AGENTS.md Interop Adaptation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rewrite `AGENTS.md` so it keeps the `agglayer/agglayer` operating style while becoming fully truthful and repo-specific for `agglayer/interop`.

**Architecture:** This is a documentation-only change centered on one file, `AGENTS.md`. The work is split by section so each edit has a narrow scope and a matching red/green verification loop using `rg` to prove the old wording is gone and the new wording is present.

**Tech Stack:** Markdown, `rg`, git (approval-gated by current repo rules)

---

### Task 1: Rewrite The Documentation Index

**Files:**
- Modify: `AGENTS.md:3-13`
- Test: `AGENTS.md`

- [ ] **Step 1: Write the failing verification**

```bash
rg -n 'CONTRIBUTING\.md|docs/knowledge-base|Cargo\.toml|crates/|proto/|scripts/' AGENTS.md
```

Expected: matches `CONTRIBUTING.md`, but does not show the new `Cargo.toml`, `crates/`, `proto/`, and `scripts/` anchors in the index section.

- [ ] **Step 2: Run the verification to confirm the current file fails the target state**

Run: `rg -n 'CONTRIBUTING\.md|docs/knowledge-base|Cargo\.toml|crates/|proto/|scripts/' AGENTS.md`
Expected: output includes the stale `CONTRIBUTING.md` reference and misses some or all required interop-specific references.

- [ ] **Step 3: Write the minimal implementation**

Replace the `Documentation and skills index` section with:

```markdown
## Documentation and skills index

Key references for navigating this project:

- `README.md` -- project overview and local development prerequisites.
- `Cargo.toml` -- workspace members, shared dependencies, and repo metadata.
- `crates/` -- implementation crates and crate-local manifests.
- `proto/` -- protobuf contracts and wire-format definitions.
- `scripts/` -- repo automation and helper scripts.
- `Makefile.toml` -- common task entry points.
```

- [ ] **Step 4: Run the verification to confirm the update passes**

Run: `rg -n 'README\.md|Cargo\.toml|crates/|proto/|scripts/|Makefile\.toml' AGENTS.md`
Expected: all six anchors are present.

Run: `rg -n 'CONTRIBUTING\.md' AGENTS.md`
Expected: no matches, exit code `1`.

- [ ] **Step 5: Commit the change if the user approves git writes**

```bash
git add AGENTS.md
git commit -m "docs: adapt AGENTS references for interop"
```


### Task 2: Rewrite Domain Guidance For Interop Risks

**Files:**
- Modify: `AGENTS.md:46-53`
- Test: `AGENTS.md`

- [ ] **Step 1: Write the failing verification**

```bash
rg -n 'Dedicated domain behavior \(Interop\)|proofs, witnesses, bridge safety|wire compatibility|RPC, gRPC, and protobuf boundaries|serialization formats|end-to-end flows' AGENTS.md
```

Expected: no matches, because the file still uses the inherited Agglayer-wide domain wording.

- [ ] **Step 2: Run the verification to confirm it fails for the expected reason**

Run: `rg -n 'Dedicated domain behavior \(Interop\)|proofs, witnesses, bridge safety|wire compatibility|RPC, gRPC, and protobuf boundaries|serialization formats|end-to-end flows' AGENTS.md`
Expected: no output and exit code `1`.

- [ ] **Step 3: Write the minimal implementation**

Replace the `Dedicated domain behavior (Agglayer)` section with:

```markdown
## Dedicated domain behavior (Interop)

- Frame recommendations in interop terms: proofs, witnesses, bridge safety, settlement correctness, and operational reliability.
- Prefer changes that improve safety invariants, wire compatibility, observability, and rollback clarity over local optimizations.
- Call out likely blast radius across crates, RPC, gRPC, and protobuf boundaries, serialization formats, and end-to-end flows before proposing deep refactors.
```

- [ ] **Step 4: Run the verification to confirm the rewrite passes**

Run: `rg -n 'Dedicated domain behavior \(Interop\)|proofs, witnesses, bridge safety|wire compatibility|RPC, gRPC, and protobuf boundaries|serialization formats|end-to-end flows' AGENTS.md`
Expected: matches the new section.

Run: `rg -n 'Dedicated domain behavior \(Agglayer\)|cross-chain settlement' AGENTS.md`
Expected: no matches, exit code `1`.

- [ ] **Step 5: Commit the change if the user approves git writes**

```bash
git add AGENTS.md
git commit -m "docs: focus AGENTS domain guidance on interop"
```


### Task 3: Replace Aspirational Docs Rules With Truthful Skill Rules

**Files:**
- Modify: `AGENTS.md:55-68`
- Test: `AGENTS.md`

- [ ] **Step 1: Write the failing verification**

```bash
rg -n 'All other documentation belongs in `docs/knowledge-base/`|Repo-local skills live in `\.agents/skills/`|automatic dependency loading|Do not reference required docs or directories in `AGENTS\.md` until they exist' AGENTS.md
```

Expected: matches the old `docs/knowledge-base` rule, but does not match the new truthful skill and documentation rules.

- [ ] **Step 2: Run the verification to confirm the gap**

Run: `rg -n 'All other documentation belongs in `docs/knowledge-base/`|Repo-local skills live in `\.agents/skills/`|automatic dependency loading|Do not reference required docs or directories in `AGENTS\.md` until they exist' AGENTS.md`
Expected: output shows the stale documentation rule and misses the replacement wording.

- [ ] **Step 3: Write the minimal implementation**

Update the clarification sentence and replace the bottom of `Collaboration norms` with:

```markdown
## Clarification Before Action

- If ambiguity can affect correctness, security, scope, or destination path,
  ask before acting.
- When unknown terms or domain concepts appear, ask for an explanation
  and document them in the repository only when a canonical location exists.
- Low-risk ambiguity in instructions may be assumed:
  state one explicit assumption and proceed with the smallest reversible change.
- Ambiguity about technical meaning, domain semantics, or definitions
  is never low-risk. Always ask for clarification and document it if necessary.

## Collaboration norms

- Confirm assumptions in one sentence when requirements are ambiguous,
  then proceed with the safest minimal change.
- Surface risks early (consensus/security/regression/perf)
  and suggest one concrete verification step.
- Do not run non-read-only git operations
  (e.g., `add`, `rm`, `mv`, `checkout`, `restore`, `commit`, `push`)
  without explicit user approval.
  Read-only commands (`status`, `diff`, `log`, `show`) are always allowed.
- Repo-local skills live in `.agents/skills/`.
- Express skill dependencies explicitly in prose; do not assume automatic dependency loading.
- Do not reference required docs or directories in `AGENTS.md` until they exist.
- Precedence: when rules conflict,
  favor the Clarification Before Action section.
```

- [ ] **Step 4: Run the verification to confirm the new rules pass**

Run: `rg -n 'Repo-local skills, if introduced, live in `\.agents/skills/`|canonical location exists|dependency loading|Do not reference required docs or directories in `AGENTS\.md` until they exist' AGENTS.md`
Expected: matches the updated clarification rule and all three new collaboration rules.

Run: `rg -n 'All other documentation belongs in `docs/knowledge-base/`' AGENTS.md`
Expected: no matches, exit code `1`.

- [ ] **Step 5: Commit the change if the user approves git writes**

```bash
git add AGENTS.md
git commit -m "docs: clarify local skills and docs rules"
```
