# Git Diff View Alpha Fixtures

Protected fixtures for the local Git diff-view projection.

- `rust_suspicious_safe_copy.yaml` covers a launch-wedge Rust diff with a
  suspicious bidi control, syntax labeling, exact path/target row labels, and
  raw/plain/context/escaped copy choices.
- `reopen_closed_diff.yaml` covers reopening a closed staged diff with the same
  compare target, path-truth ref, scroll anchor, and selected hunk.

The source-control change-list projection provides public diff-open targets;
the diff viewer consumes those targets instead of hidden shell state.
