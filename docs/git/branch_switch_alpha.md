# Git Branch Switch Alpha

The branch switch alpha gives the local daily Git loop a preview-first path for
switching to an existing branch, creating a branch, or checking out a revision.
It reuses the canonical status snapshot before apply so current work, detached
HEAD, and missing remote state stay visible.

## Contract

- `aureline-git` owns `git_branch_preview`, `git_branch_result`,
  `git_branch_activity_record`, `git_branch_support_export_record`, and
  `git_branch_journal_record`.
- Every preview carries the source `git_status_snapshot` truth ref, current
  shell projection, target identity, remote/upstream state, and current-work
  warning state.
- Staged, unstaged, untracked, and conflicted work is disclosed before apply.
  Conflicted work blocks the operation until conflict review resolves it.
- Checkout of a commit or tag marks the target as `detached_head` before apply.
  Missing upstreams, missing remotes, and missing remote branches are explicit
  remote-state tokens instead of hidden fallback behavior.
- Apply checks that current work still matches the previewed basis. If branch,
  revision, changed-path state, index entries, or changed worktree bytes drift,
  apply is blocked and branch review must be reopened.
- Changed-path evidence is bounded by path count and total bytes. Index entries
  and regular files are retained as length/digest/identity evidence rather than
  raw bodies; regular files are opened no-follow and checked before/open/after.
  Symlinks hash the link payload itself without following its target, directory
  identities are checked, and sockets/FIFOs/devices or oversized evidence block.
- The exact safe repository-config evidence is bound to the preview and read
  again immediately before checkout. Benign or hostile config drift both
  invalidate authority; legacy `[filter.name]` process declarations are treated
  the same as quoted filter subsections.
- Non-UTF-8 porcelain path/ref bytes degrade review before lossy path parsing.
  Apply disables Git target guessing, submodule recursion, and the default
  overwrite of ignored worktree files.
- Raw byte evidence and apply operands remain process-local. A serialized,
  deserialized, cross-service, replayed, or publicly tampered preview is an
  inspection record and cannot authorize a branch mutation.
- Apply uses reviewed object ids wherever Git accepts an immutable revision
  (new-branch and detached checkout) and verifies the resulting HEAD oid/state.
  Named-branch switching and `--track` creation remain Git porcelain operations:
  Git exposes no single compare-and-swap primitive that atomically updates the
  worktree and attaches HEAD. A concurrent external Git writer can still race
  the final check; the postcondition converts detected drift to a failed,
  reflog-recoverable result instead of claiming success.
- Process-local authority expires after ten minutes and is capped by entry
  count, per-entry bytes, and total retained bytes with deterministic oldest
  eviction.
- Results include the after-apply shell status projection and activity row so
  title/context, status, activity, support, and CLI surfaces quote one branch
  identity.

## Records

- `git_branch_preview`: operation, current head, target review, current-work
  warning, source shell projection, activity row, and support-export row.
- `git_branch_result`: outcome state, before/after shell identity, after head,
  journal record, activity row, support-export row, and failure/block reasons.
- `git_branch_activity_record`: durable row for preview, block, failure, or
  completed branch identity change.
- `git_branch_support_export_record`: metadata-safe support/export view of the
  same target and current-work lineage.
- `git_branch_journal_record`: actor, source class, operation, target ref,
  before/after status refs, before/after head refs, recovery class, and
  side-effect summary.

## Inspection

Preview a switch to an existing branch:

```sh
cargo run -p aureline-git --bin aureline_git_branch -- --operation switch --target feature --root .
```

Create a branch after preview inspection:

```sh
cargo run -p aureline-git --bin aureline_git_branch -- --operation create --target feature/work --root . --apply
```

Checkout a revision with detached-head disclosure:

```sh
cargo run -p aureline-git --bin aureline_git_branch -- --operation checkout --target HEAD~1 --root .
```

Create from a remote-tracking start point, with missing remote state surfaced
when the remote is not configured:

```sh
cargo run -p aureline-git --bin aureline_git_branch -- --operation create --target feature/from-upstream --start-point origin/feature --track-remote --root .
```

Protected fixture cases live under `fixtures/git/branch_switch_alpha/` and are
covered by:

```sh
cargo test -p aureline-git --test branch_switch_alpha
```
