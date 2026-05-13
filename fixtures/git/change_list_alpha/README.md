# Git Change List Alpha Fixtures

Protected fixtures for the shell source-control change-list projection.

The cases are intentionally data-driven:

- `mixed_groups.yaml` covers staged rows, unstaged rows, untracked files,
  rename metadata, and a path that appears in both groups.
- `large_virtualized.yaml` generates a large synthetic status snapshot and
  verifies that each group exposes a bounded visible window while preserving
  full counts and hidden-row disclosure.

These fixtures exercise shell-facing projections only. The underlying local
Git discovery contract lives in `fixtures/git/status_alpha/`.
