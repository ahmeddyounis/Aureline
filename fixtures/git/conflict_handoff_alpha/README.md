# Git Conflict Handoff Alpha Fixtures

These fixtures validate the first handoff packet shared by editor, Git,
CLI, and support/export surfaces when Git merge conflict state or VFS
external-change compare state blocks a silent mutation.

Covered cases:

- `git_merge_conflict.yaml`: a live Git merge conflict stays visible in
  both editor and Git projections with an abort/recovery path.
- `external_change_compare.yaml`: a VFS compare-before-write mismatch
  preserves shared filesystem identity and routes compare/merge/reload/save-as
  actions into both editor and Git projections.

Run with:

```sh
cargo test -p aureline-git --test conflict_handoff_alpha
```
