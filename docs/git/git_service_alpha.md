<!--
SPDX-FileCopyrightText: 2026 Aureline contributors
SPDX-License-Identifier: Apache-2.0
-->

# Git Service Alpha

The Git service alpha gives launch-wedge surfaces one local source of truth for
repository identity, branch state, and changed paths.

Reviewed network/local clone subprocesses are owned separately by
[`clone_execution_contract.md`](./clone_execution_contract.md); they reuse the
same fail-closed process posture but carry source-acquisition, transport, and
credential authority that passive local status never receives.

## Contract

- `aureline-git` owns the canonical `git_status_snapshot` record.
- The service wraps the system `git` executable once, parses porcelain status
  output, and emits shared shell, activity-center, and review seed projections.
- Local Git status remains authoritative. Provider and review overlays may add
  metadata later, but they do not overwrite local diff truth.
- Non-repository and unavailable-Git states remain visible as degraded records
  instead of disappearing from shell chrome.
- Passive status starts Git with a minimal environment, system/global config
  disabled, optional locks disabled, submodule recursion disabled, and
  repository-triggered helpers such as file-system monitors disabled. Before
  status runs, repository config files are bounded and inspected; includes,
  hooks, filters, external diff/merge drivers, credential helpers, and other
  process-capable declarations fail closed as `refresh_failed`.
- Every Git subprocess receives closed standard input, a 60-second deadline,
  and independent 16 MiB stdout/stderr limits. Crossing a deadline or output
  bound terminates the child process group and fails closed instead of waiting
  for a helper or grandchild to finish draining pipes.
- Local status, branch, and commit commands receive no SSH agent socket or SSH
  command authority. Protocol defaults deny unknown transports and remote
  helpers; only the direct publish lane can add a reviewed SSH socket.
- Repository discovery, porcelain paths, branch/ref names, object ids, remote
  names, URLs, and divergence counts must be valid UTF-8 and structurally
  complete. Identity bytes are never repaired with lossy conversion, and
  repository-supplied path names are always passed back to Git as literal
  pathspecs rather than pathspec-magic expressions.
- Config inspection recognizes both quoted subsections and legacy dotted
  subsection syntax. Includes, URL rewrites, protocol overrides, SSH/proxy
  routing, remote receive/upload-pack overrides, shell submodule updaters, and
  process-capable filter/diff/merge/credential settings all fail closed.
- Degraded records carry stable, export-safe failure summaries. Raw Git stderr,
  repository paths, and malformed porcelain records are never copied into
  shell, activity, review, or support projections.

## Records

- `git_status_snapshot`: repository/worktree identity, branch or detached HEAD,
  service state, discovery coverage, change counts, and path-level changes.
- `git_shell_status_record`: compact branch and change summary for shell chrome.
- `git_activity_record`: durable refresh/degraded row for the activity center.
- `git_review_seed_record`: local diff authority seed for review surfaces.

All consumer records carry the same `truth_source_ref`, so support exports and
tests can prove the surfaces came from one snapshot rather than duplicate Git
commands.

The shell change-list projection in `docs/git/change_list_alpha.md` consumes
this snapshot directly for staged/unstaged grouping and file-state chips.
The diff-view projection in `docs/git/diff_view_alpha.md` then consumes public
change-list diff-open targets while preserving the same `truth_source_ref`.
The mutation review projection in `docs/git/mutation_review_alpha.md` uses the
same status truth before stage, unstage, discard, and checkpoint restore
commands can apply.
The commit projection in `docs/git/commit_alpha.md` uses the same status truth
to disclose author identity, staged scope, amend/squash guardrails, and
publish-later readiness before a local commit can apply.
The branch switch projection in `docs/git/branch_switch_alpha.md` uses the same
status truth to disclose current work, detached-head posture, missing remote or
upstream state, and after-apply shell identity before a branch operation can
apply.
The publish review projection in `docs/git/publish_review_alpha.md` uses the
same status truth to disclose origin, route, remote, target ref, divergence, and
failure recovery before a push can run.

## Degraded Behavior

- `not_repository`: the selected root is a plain folder; shell and review
  surfaces show that Git does not apply.
- `git_unavailable`: the Git executable or backend is missing; local editing
  can continue while Git surfaces show the missing dependency.
- `refresh_failed`: Git exists but could not produce current worktree status;
  an export-safe failure class is preserved in shell, activity, and review
  records without exposing raw command output.

## Inspection

Use the CLI mirror to inspect the canonical snapshot:

```sh
cargo run -p aureline-git --bin aureline_git_status -- .
```

Use `--bundle` to inspect the shared consumer projections:

```sh
cargo run -p aureline-git --bin aureline_git_status -- --bundle .
```

Protected fixture cases live under `fixtures/git/status_alpha/` and are covered
by `cargo test -p aureline-git`.

Mutation review fixtures live under `fixtures/git/mutation_review_alpha/` and
are covered by `cargo test -p aureline-git --test mutation_review_alpha`.

Commit fixtures live under `fixtures/git/commit_alpha/` and are covered by
`cargo test -p aureline-git --test commit_alpha`.

Branch switch fixtures live under `fixtures/git/branch_switch_alpha/` and are
covered by `cargo test -p aureline-git --test branch_switch_alpha`.

Publish review fixtures live under `fixtures/git/publish_review_alpha/` and are
covered by `cargo test -p aureline-git --test publish_review_alpha`.
