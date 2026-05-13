# Git Service Alpha

The Git service alpha gives launch-wedge surfaces one local source of truth for
repository identity, branch state, and changed paths.

## Contract

- `aureline-git` owns the canonical `git_status_snapshot` record.
- The service wraps the system `git` executable once, parses porcelain status
  output, and emits shared shell, activity-center, and review seed projections.
- Local Git status remains authoritative. Provider and review overlays may add
  metadata later, but they do not overwrite local diff truth.
- Non-repository and unavailable-Git states remain visible as degraded records
  instead of disappearing from shell chrome.

## Records

- `git_status_snapshot`: repository/worktree identity, branch or detached HEAD,
  service state, discovery coverage, change counts, and path-level changes.
- `git_shell_status_record`: compact branch and change summary for shell chrome.
- `git_activity_record`: durable refresh/degraded row for the activity center.
- `git_review_seed_record`: local diff authority seed for review surfaces.

All consumer records carry the same `truth_source_ref`, so support exports and
tests can prove the surfaces came from one snapshot rather than duplicate Git
commands.

## Degraded Behavior

- `not_repository`: the selected root is a plain folder; shell and review
  surfaces show that Git does not apply.
- `git_unavailable`: the Git executable or backend is missing; local editing
  can continue while Git surfaces show the missing dependency.
- `refresh_failed`: Git exists but could not produce current worktree status;
  the stale or failure reason is preserved in shell, activity, and review
  records.

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
