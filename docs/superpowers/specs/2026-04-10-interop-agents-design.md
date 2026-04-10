## Purpose

Adapt the inherited `AGENTS.md` from `agglayer/agglayer` into a truthful, reusable operating contract for `agglayer/interop`, while preserving the source repo's standard style and engineering discipline.

## Current Repo Context

- `interop` currently has `README.md`, `Cargo.toml`, `Makefile.toml`, `crates/`, `proto/`, and `scripts/`.
- `CONTRIBUTING.md`, `docs/knowledge-base/`, and `.agents/skills/` do not currently exist in this repo.
- `docs/superpowers/specs/` was created for this design doc and should not be treated as proof that a broader repo documentation structure already exists.
- The current `AGENTS.md` references several of those missing paths, so it is partially aspirational rather than fully accurate.

## Goals

- Keep the overall style and operating model inspired by `agglayer/agglayer`.
- Remove or rewrite statements that are false for `interop` today.
- Preserve high-signal collaboration guidance around ambiguity, debugging, safety, and git operations.
- Introduce a standard place for repo-local skills without overpromising tool support that may not exist.

## Non-Goals

- Do not redesign the entire repository workflow.
- Do not create placeholder docs just to satisfy inherited references.
- Do not define automatic skill-loading or dependency metadata unless the tooling is verified to support it.

## Recommended Approach

Use a mirror-with-adaptation strategy:

- Preserve the section structure and tone from `agglayer/agglayer` where it still fits.
- Rewrite repo references and domain language so each rule is true for `interop`.
- Prefer explicit, enforceable instructions over aspirational documentation layout.

This keeps a standard way of working across repos without copying assumptions from the larger `agglayer/agglayer` node-oriented codebase.

## Section-by-Section Decisions

### Keep Mostly As-Is

- `Response priorities`
- `Clarification Before Action`
- `Evidence-Based Debugging and Communication`
- The git-safety parts of `Collaboration norms`

These sections already fit protocol-heavy and safety-sensitive work in `interop`.

### Rewrite

#### `Documentation and skills index`

Replace inherited references with paths that exist and matter in `interop`:

- `README.md` for repo overview
- `Cargo.toml` for workspace layout and shared dependencies
- `crates/` for crate-level implementation work
- `proto/` for protobuf contracts
- `scripts/` for repo automation
- `Makefile.toml` for common task entry points

Optional additions if useful:

- `.github/workflows/` for CI conventions
- crate-specific `README.md` files if they are added later

#### `Dedicated domain behavior`

Rename to `Dedicated domain behavior (Interop)` and refocus guidance around the actual risks in this repo:

- proof and witness correctness
- bridge and settlement safety
- RPC, gRPC, and protobuf boundary compatibility
- serialization compatibility across bincode, serde, and wire formats
- blast radius across shared crates and end-to-end flows

The language should avoid assuming the full `agglayer/agglayer` node, orchestrator, epoch, or prover deployment architecture unless a rule truly applies here.

### Remove

Remove or replace references that are currently false:

- `CONTRIBUTING.md`
- `docs/`
- `docs/knowledge-base/`
- The rule that all non-skill documentation belongs in `docs/knowledge-base/`

These can be reintroduced later if the directories are actually created and adopted.

## Proposed Final Structure

1. `Documentation and skills index`
2. `Response priorities`
3. `Clarification Before Action`
4. `Evidence-Based Debugging and Communication`
5. `Dedicated domain behavior (Interop)`
6. `Collaboration norms`

## Repo-Local Skills Standard

The repo can define local skills inside:

`.agents/skills/<skill-name>/SKILL.md`

Recommended standard:

- `AGENTS.md` may require repo-local skills for repo-specific workflows.
- Repo-local skills may explicitly name required background skills.
- Repo-local skills may explicitly name required next skills.

Example patterns:

- "Before changing protocol behavior, invoke `brainstorming` and any applicable repo-local interop skill."
- "REQUIRED BACKGROUND: use `test-driven-development`."
- "After completing the change, use `verification-before-completion`."

## Skill Dependency Model

The recommended dependency model is explicit composition in prose, not automatic dependency resolution.

Supported by design:

- `AGENTS.md` requiring a skill before a class of tasks
- A local skill requiring another skill before or after execution
- A local skill documenting related skills that should be used together

Not assumed without tool verification:

- declarative dependency metadata that auto-loads other skills
- hidden or transitive dependency resolution
- guaranteed ordered loading from repo-local skill manifests

This keeps the standard portable across tools and avoids encoding behavior the runtime may not implement.

## Recommended Wording Changes

- Keep the source-repo inspiration visible in tone, not in inaccurate repo references.
- Use `interop`-specific crate and protocol examples instead of `agglayer` node/orchestrator examples.
- State only rules that can be followed immediately in this repo.
- Avoid requiring future directories until they exist.

## Risks and Tradeoffs

- Keeping too much inherited wording will make `AGENTS.md` look authoritative while silently pointing to missing structures.
- Making the file too minimal would lose the standardization goal and reduce cross-repo consistency.
- Defining skill dependencies as prose is less automated, but it is safer than relying on unverified tool features.

## Follow-Up Recommendations

- Rewrite `AGENTS.md` now using the structure above.
- Create `.agents/skills/` only when the first repo-local skill is ready to be added.
- If the repo later adopts a documentation tree, update `AGENTS.md` only after the location is created and agreed upon.

## Constraint Note

The brainstorming workflow normally calls for committing the design doc. This repo's current `AGENTS.md` forbids non-read-only git operations without explicit user approval, so the spec should be written first and committed only if the user requests or approves that step.
