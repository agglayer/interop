# CI release hardening, PR 1: close the pwn request, implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended)
> or superpowers:executing-plans to implement this plan task-by-task.
> Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ensure no fork-controlled code can run in a job holding `CARGO_REGISTRY_TOKEN`, `BUF_TOKEN`,
or a write-scoped `GITHUB_TOKEN`, while still releasing only after tests pass.

**Architecture:** Replace the `workflow_run` triggers in `release.yml` and `proto-publish.yml`
with `push`-only paths.
`release.yml` calls `test.yml` as a reusable workflow and gates `release-crates` on it.
The crates.io token moves to a `main`-only `crates-io` environment.

**Tech Stack:** GitHub Actions YAML, release-plz action, `actionlint`, `zizmor`
(both run via `nix run nixpkgs#<tool>`).

**Spec:** `docs/knowledge-base/src/ai-agents/specs/2026-09-29-ci-release-hardening-design.md`

## Global Constraints

- Only files under `.github/workflows/` change in this PR, plus this plan and the PR description.
- No `workflow_run` or `pull_request_target` trigger may be added.
- `release-crates` runs only when `github.event_name == 'push' && github.ref == 'refs/heads/main'`.
- `release-crates` uses `environment: crates-io`.
- Top-level `permissions: {}` in `release.yml`; every job declares its own permissions.
- No non-read-only git operation without explicit user approval (AGENTS.md).
- Commits are signed and follow Conventional Commits with `ci` type.

## Review Focus

1. Nested `test.yml` run and standalone `Test` run on the same `main` push must not cancel each other
   (concurrency group collision).
2. `workflow_dispatch` of Release must not publish crates (tests and release-crates skipped),
   and `automated-release-pr` must still run.
3. `release-plz release` on a `push` checkout must see a branch, not detached HEAD,
   now that the `git switch` step is removed.
4. `release-crates` must fail loudly, not publish partially, if the `crates-io` environment secret is missing.
5. `proto-publish.yml` must still publish on `push` to `proto/**` with only `BUF_TOKEN` passed.

---

### Task 1: Baseline security lint (failing test)

**Files:**
- None modified. Output saved to scratchpad.

**Interfaces:**
- Produces: baseline `zizmor` findings used as the "failing test" for Tasks 2 to 4.

- [ ] **Step 1: Run zizmor on the current workflows**

Run: `nix run nixpkgs#zizmor -- --offline --min-severity medium .github/workflows/`
Expected: findings including `dangerous-triggers` on `release.yml` and `proto-publish.yml`
(the `workflow_run` triggers).
Record the full output; other findings (unpinned actions, excessive permissions) are PR 3 scope.

- [ ] **Step 2: Run actionlint baseline**

Run: `nix run nixpkgs#actionlint -- -color`
Expected: record any existing errors so new ones can be told apart.

### Task 2: Make `test.yml` callable

**Files:**
- Modify: `.github/workflows/test.yml:3-12`

**Interfaces:**
- Produces: reusable workflow `./.github/workflows/test.yml` with no inputs and no secrets,
  consumed by Task 3.

- [ ] **Step 1: Add `workflow_call` trigger**

```yaml
on:
  push:
    branches:
      - main
  merge_group:
  pull_request:
    branches:
      - "**"
  workflow_dispatch:
  workflow_call:
```

- [ ] **Step 2: Make the concurrency group safe for nested calls**

In a called workflow, `github.workflow` and `github.ref` are the caller's.
On a `main` push the group already resolves to `<workflow>-<run_id>`, which is unique per run,
so the standalone Test run and the nested one cannot collide.
To make this explicit and robust to later edits, append the event name:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.event_name }}-${{ github.ref == 'refs/heads/main' && github.run_id || github.event.pull_request.number || github.ref }}
  cancel-in-progress: true
```

- [ ] **Step 3: Lint**

Run: `nix run nixpkgs#actionlint -- .github/workflows/test.yml`
Expected: no new errors versus the Task 1 baseline.

- [ ] **Step 4: Commit (after user approval)**

```bash
git add .github/workflows/test.yml
git commit -m "ci(test): allow test workflow to be called by other workflows"
```

### Task 3: Harden `release.yml`

**Files:**
- Modify: `.github/workflows/release.yml:1-58`

**Interfaces:**
- Consumes: `./.github/workflows/test.yml` (Task 2).
- Produces: `release-crates` job depending on `tests`, under environment `crates-io`.

- [ ] **Step 1: Replace header, triggers, and `release-crates`**

Replace everything from line 1 through the end of the `release-crates` job with:

```yaml
name: Release

permissions: {}

on:
  push:
    branches:
      - main
  workflow_dispatch:
    inputs:
      version_bump:
        description: "Version bump level"
        required: true
        default: "auto"
        type: choice
        options:
          - auto
          - major
          - minor
          - patch

jobs:
  tests:
    name: Tests
    if: github.repository == 'agglayer/interop' && github.event_name == 'push' && github.ref == 'refs/heads/main'
    uses: ./.github/workflows/test.yml
    permissions:
      contents: read

  release-crates:
    name: Release crates
    runs-on: ubuntu-latest
    needs: tests
    if: github.repository == 'agglayer/interop' && github.event_name == 'push' && github.ref == 'refs/heads/main'
    environment: crates-io
    permissions:
      contents: write
    concurrency:
      group: release-crates
      cancel-in-progress: false
    steps:
      - name: Checkout repository
        uses: actions/checkout@v7
        with:
          fetch-depth: 0

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - uses: taiki-e/install-action@protoc

      - name: Run release-plz
        uses: release-plz/action@v0.5
        with:
          command: release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

Notes for the implementer:
- The `git switch -c release-${{ github.run_id }}` step existed because checking out `head_sha`
  leaves a detached HEAD.
  A default `push` checkout is on branch `main`, so the step is dropped (Review Focus 3).
- `pull-requests: write` is dropped. The release-plz docs show `release` needing only `contents: write`;
  `pull-requests: write` is for `release-pr`.
  Confirm against https://release-plz.dev/docs/github/quickstart before committing.
  If it is needed, add it back with a comment citing the reason.

- [ ] **Step 2: Keep `automated-release-pr` unchanged**

Its `if:` already restricts it to `push` on `main` or `workflow_dispatch`,
and it declares job-level `permissions`, so it is unaffected by top-level `permissions: {}`.
Do not edit it in this PR.

- [ ] **Step 3: Fail loudly on a missing token (Review Focus 4)**

Insert before "Run release-plz":

```yaml
      - name: Check crates.io token is present
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: |
          if [ -z "$CARGO_REGISTRY_TOKEN" ]; then
            echo "::error::CARGO_REGISTRY_TOKEN is not set in the crates-io environment"
            exit 1
          fi
```

- [ ] **Step 4: Lint and security-lint**

Run: `nix run nixpkgs#actionlint -- .github/workflows/release.yml`
Expected: no new errors.

Run: `nix run nixpkgs#zizmor -- --offline --min-severity medium .github/workflows/release.yml`
Expected: no `dangerous-triggers` finding.
Remaining `unpinned-uses` findings are PR 3 scope.

- [ ] **Step 5: Assert trigger invariants**

Run: `grep -nE 'workflow_run|pull_request_target|head_sha' .github/workflows/release.yml`
Expected: no output.

- [ ] **Step 6: Commit (after user approval)**

```bash
git add .github/workflows/release.yml
git commit -m "fix(ci): publish crates only from push to main behind tests"
```

### Task 4: Remove `workflow_run` from `proto-publish.yml`

**Files:**
- Modify: `.github/workflows/proto-publish.yml:3-37`

**Interfaces:**
- Consumes: `agglayer/reusable-workflows/.github/workflows/proto-publish.yml@main`,
  which reads `secrets.BUF_TOKEN`.

- [ ] **Step 1: Replace triggers and secrets**

```yaml
on:
  push:
    branches:
      - main
    paths:
      - "proto/**"
  workflow_dispatch:
    inputs:
      ref:
        description: "Branch or tag to use for the workflow"
        required: false
        default: "main"
        type: string

permissions:
  contents: read

jobs:
  publish:
    name: Publish protobuf to BSR
    uses: agglayer/reusable-workflows/.github/workflows/proto-publish.yml@main
    permissions:
      contents: read
    with:
      ref: ${{ github.event_name == 'workflow_dispatch' && github.event.inputs.ref || github.ref }}
    secrets:
      BUF_TOKEN: ${{ secrets.BUF_TOKEN }}
```

Note: the called workflow does not declare `secrets:` under `workflow_call`.
Passing named secrets to a workflow that declares none is an error.
If actionlint or a dry run reports this, keep `secrets: inherit`
and record in the PR description that narrowing depends on a reusable-workflows change.

- [ ] **Step 2: Lint**

Run: `nix run nixpkgs#actionlint -- .github/workflows/proto-publish.yml`
Run: `nix run nixpkgs#zizmor -- --offline --min-severity medium .github/workflows/proto-publish.yml`
Expected: no `dangerous-triggers` finding.

- [ ] **Step 3: Commit (after user approval)**

```bash
git add .github/workflows/proto-publish.yml
git commit -m "fix(ci): stop triggering protobuf publish from workflow_run"
```

### Task 5: Full-tree verification and PR text

**Files:**
- Create (scratchpad, not committed): `pr1-description.md`, `reusable-workflows-issue.md`

- [ ] **Step 1: Full lint**

Run: `nix run nixpkgs#actionlint -- -color`
Run: `nix run nixpkgs#zizmor -- --offline --min-severity medium .github/workflows/`
Expected: no `dangerous-triggers` findings anywhere; no new actionlint errors versus Task 1.

- [ ] **Step 2: Repo-wide trigger invariant**

Run: `grep -rnE 'workflow_run|head_sha' .github/workflows/`
Expected: no output.

- [ ] **Step 3: Draft PR description**

Include: summary of the vulnerability (no exploit detail beyond what the security notice states),
the three file changes, the admin checklist from the spec (ordered, with "must happen before merge"
on environment creation and secret setup), and verification steps.
End with the attribution line required by the session.

- [ ] **Step 4: Draft reusable-workflows issue**

Describe the class of problem (untrusted `workflow_run.head_branch` interpolated into `run:`),
the affected file, and the fix (pass through `env:` and quote),
plus a request to audit all callers for missing branch filters.
Do not file it; hand it to the user.

- [ ] **Step 5: Push and open PR (only after explicit user approval)**

## Post-merge verification (manual, admin)

1. Before merge: `crates-io` environment exists, policy `main` only, rotated secret set.
2. After merge: the merge commit's Release run shows `Tests / Unit Tests`,
   `Tests / Isolated feature checks`, then `Release crates` under environment `crates-io`.
3. The standalone `Test` run for the same commit completes and is not cancelled (Review Focus 1).
4. Delete the repo-level `CARGO_REGISTRY_TOKEN`; revoke the old crates.io token.
5. Set `can_approve_pull_request_reviews` to `false`; remove `RELEASE_PLZ_TOKEN` once confirmed unused.
