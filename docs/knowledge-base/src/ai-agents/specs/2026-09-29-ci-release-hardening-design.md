# CI release hardening design

Date: 2026-09-29
Status: draft, pending review

## Context

Security reported a pwn-request path in `.github/workflows/release.yml`.
The `release-crates` job triggers on `workflow_run` of "Test" filtered to branch `main`,
checks out `github.event.workflow_run.head_sha`,
and runs `release-plz release` with `CARGO_REGISTRY_TOKEN` and `contents: write`.

The `branches: main` filter matches the head branch name, including a fork's `main`.
Nothing checks `workflow_run.head_repository` or `workflow_run.event`.
The `github.repository_owner == 'agglayer'` guard is always true in a `workflow_run` context.
A fork PR from a branch named `main`, once approved to run and green on Test,
would execute its `build.rs` (and any dependency build script) next to the crates.io token.

Security reports that no fork code has reached this job so far (54 of 54 runs from our own `main`).
Fork PR approval (`all_external_contributors`) is enabled,
but it does not cover `pull_request_target` or `workflow_run`.

### Observed repository state

- Default `GITHUB_TOKEN` permissions: `read`.
- `can_approve_pull_request_reviews`: `true`.
- `allowed_actions`: `all`; `sha_pinning_required`: `false`.
- Repo secrets: `CARGO_REGISTRY_TOKEN`, `RELEASE_APP_ID`, `RELEASE_APP_PRIVATE_KEY`,
  `RELEASE_PLZ_TOKEN` (not referenced by any workflow), `SLACK_WEBHOOK_URL`.
- Only environment: `github-pages`.
- Default-branch rulesets require squash via merge queue and the "Unit Tests" check.
  Org admins, repo admins, and integration `1203717` have `always` bypass,
  so a push to `main` is not guaranteed to have passed tests.

### Other findings

- `proto-publish.yml` uses the same `workflow_run` + `branches: main` pattern with `secrets: inherit`.
  It checks out `github.ref` (base `main`), so fork code does not run,
  but a fork PR can trigger a BSR push of `main`, and `conclusion` is never checked.
- `agglayer/reusable-workflows/.github/workflows/proto-publish.yml@main`
  interpolates `${{ github.event.workflow_run.head_branch }}` directly into a `run:` script.
  This is a script-injection sink for any caller without a branch filter.
  Out of scope here; see [Out of scope](#out-of-scope).
- `pr-labeler.yml` and `pr-checking-title.yml` use `pull_request_target`
  but do not check out PR code; no change needed.
- Actions are pinned by tag (including `dtolnay/rust-toolchain@master`),
  reusable workflows by `@main`,
  several workflows lack a top-level `permissions:` block,
  and `automated-release-pr` runs `cargo install` without `--locked`.

## Goals

- No fork-controlled code runs in any job that can publish crates, push to BSR, or write to the repo.
- The crates.io credential is scoped to a `main`-only environment now,
  and replaced by short-lived OIDC tokens in a follow-up.
- Releases still only happen after tests pass on the exact commit being released.
- Regressions to this class of bug are caught in CI.

## Non-goals

- Changes to crate code, RPC, gRPC, or protobuf wire formats.
- Changing the release-plz versioning or changelog flow.
- Fixing `agglayer/reusable-workflows` (tracked separately).

## Design

Delivered as three independent PRs, each revertible on its own.

### PR 1: close the pwn request (urgent)

#### `test.yml`

Add `workflow_call:` to `on:`, keeping existing triggers.
No other change.
The `concurrency` group already keys on `github.run_id` for `main`,
so a nested call from Release will not cancel the standalone Test run.
Verify this during implementation;
if the called workflow inherits the caller's `github.workflow`,
the groups may collide and need a distinct key.

#### `release.yml`

- Remove the `workflow_run` trigger. Keep `push` to `main` and `workflow_dispatch`.
- Replace top-level `permissions` with `permissions: {}`; grant per job.
- Add a `tests` job:

  ```yaml
  tests:
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    uses: ./.github/workflows/test.yml
    permissions:
      contents: read
  ```

- `release-crates`:
  - `needs: tests`
  - `if: github.repository == 'agglayer/interop' && github.event_name == 'push' && github.ref == 'refs/heads/main'`
  - `environment: crates-io`
  - `permissions: contents: write`, plus `pull-requests: write` only if release-plz `release` needs it
    (confirm against release-plz docs during implementation).
  - `concurrency: { group: release-crates, cancel-in-progress: false }`
  - Checkout without an explicit `ref`, so it uses `github.sha`.
  - Drop the `git switch -c release-${{ github.run_id }}` step if release-plz does not need it;
    otherwise keep it unchanged.
- `automated-release-pr`: unchanged in PR 1 apart from inheriting `permissions: {}` at the top level
  (its job-level permissions already exist).

#### `proto-publish.yml`

- Remove the `workflow_run` trigger. Keep `push` on `proto/**` and `workflow_dispatch`.
- Replace `secrets: inherit` with `secrets: { BUF_TOKEN: ${{ secrets.BUF_TOKEN }} }`.

Behavior change: BSR publishes happen on proto changes and manual dispatch,
no longer after every successful Test run on `main`.

#### Admin checklist (applied alongside PR 1)

Order matters so releases are never broken:

1. Create environment `crates-io` with a deployment branch policy of `main` only, no required reviewers.
2. Rotate the crates.io token and store the new one as environment secret `CARGO_REGISTRY_TOKEN`.
3. Merge PR 1.
4. Confirm the next `main` push runs `tests` then `release-crates` in the `crates-io` environment.
5. Delete the repo-level `CARGO_REGISTRY_TOKEN` and revoke the old token on crates.io.
6. Set `can_approve_pull_request_reviews` to `false`.
7. Delete `RELEASE_PLZ_TOKEN` after confirming no external consumer.
8. Raise with security whether the `always` bypass actors on default-branch rulesets are still needed.

### PR 2: crates.io trusted publishing

- A crates.io owner configures a trusted publisher on every publishable crate:
  repository `agglayer/interop`, workflow `release.yml`, environment `crates-io`.
  Crates:
  `agglayer-bincode`, `agglayer-elf-build`, `agglayer-errors`, `agglayer-evm-client`,
  `agglayer-interop`, `agglayer-interop-grpc-types`, `agglayer-interop-types`,
  `agglayer-primitives`, `agglayer-tries`, `unified-bridge`.
  Confirm the list against `publish` settings during implementation.
- `release-crates` gains `id-token: write`
  and a `rust-lang/crates-io-auth-action` step whose token is passed to release-plz
  as `CARGO_REGISTRY_TOKEN`.
- After one successful OIDC release, delete the environment secret and revoke the token.

Risk: any crate without a trusted publisher fails mid-release.
Mitigation: verify all crates are configured before merging.

### PR 3: hygiene and regression guard

- Add a `zizmor` job on changes to `.github/**`.
- Pin third-party actions by commit SHA with a version comment;
  Dependabot's existing `github-actions` ecosystem keeps them current.
- Pin `agglayer/reusable-workflows` calls to a commit SHA.
- Add top-level `permissions: contents: read` to workflows lacking one.
- Add `--locked` to `cargo install` calls in `automated-release-pr`.
- After pinning lands, an admin enables `sha_pinning_required`.

## Out of scope

- `agglayer/reusable-workflows` `proto-publish.yml` `head_branch` injection:
  file an issue for security and the owners of that repo, including an audit of all callers.

## Verification

- `actionlint` and `zizmor` pass on each PR's diff.
- PR 1: after merge, a `main` push shows `Release / tests` followed by `Release / release-crates`
  running under the `crates-io` environment;
  a fork PR from a branch named `main` produces no Release or Protobuf publish run.
- PR 2: the first release after merge publishes via OIDC,
  visible as a trusted-publishing token in crates.io audit logs.
- PR 3: `zizmor` job runs green; Dependabot opens SHA-bump PRs for pinned actions.

## Rollback

Each PR is a workflow-only change and can be reverted independently.
Reverting PR 1 after step 5 of the admin checklist requires restoring a repo-level
`CARGO_REGISTRY_TOKEN`, since the reverted workflow does not use the environment.
