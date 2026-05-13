# Git Conflict Handoff Alpha

This alpha lane defines the first shared packet for conflicts that must stay
visible across editor and Git surfaces.

## Contract

- `aureline-git` owns `git_conflict_handoff_packet`,
  `git_conflict_surface_record`, and `git_conflict_support_export_record`.
- Git merge conflicts and VFS external-change compares stay distinct through
  `divergence_source`: `git_merge_conflict` versus `vfs_external_change`.
- Both editor and Git projections carry the same `handoff_ref`,
  `path_truth_ref`, safe action set, source detail, and recovery detail.
- VFS compares reuse the VFS-owned `ExternalChangeCompareRecord` and shared
  filesystem identity refs instead of deriving identity from path strings.
- Git conflicts preserve repo-relative path identity, conflict class,
  unresolved count, detected history operation, and an abort/reflog recovery
  path when available.
- Support exports omit raw patch bodies, raw file bodies, and raw absolute
  paths while retaining source, identity refs, status/compare outcome, safe
  action tokens, and checkpoint refs.

## Inspection

Inspect a selected Git conflict path:

```sh
cargo run -p aureline-git --bin aureline_git_conflict_handoff -- --root . --path src/lib.rs
```

Protected fixture coverage:

```sh
cargo test -p aureline-git --test conflict_handoff_alpha
```

Fixtures live under `fixtures/git/conflict_handoff_alpha/`.
