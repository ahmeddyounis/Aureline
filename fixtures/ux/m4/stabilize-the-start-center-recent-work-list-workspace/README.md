# Entry-target disclosure stable matrix fixtures

Each `*.json` here is a pinned `entry_target_disclosure_record` (schema:
`schemas/ux/stabilize-the-start-center-recent-work-list-workspace.schema.json`),
minted bit-for-bit from the in-code corpus in
`crates/aureline-shell/src/start_center_stable/corpus.rs`. The corpus projects a
single claimed-stable recent-work registry through the **live** Start Center and
workspace-switcher builders, so these records are a genuine projection of the
shell's row code rather than a parallel model.

These are **generated, not hand-edited**. Regenerate with:

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_start_center_stable -- emit-fixtures \
  fixtures/ux/m4/stabilize-the-start-center-recent-work-list-workspace
```

The replay + invariant gate is
`crates/aureline-shell/tests/start_center_stable_fixtures.rs`; it fails if any
fixture drifts from the corpus or violates a disclosure invariant (claim
ceiling, recovery-before-commit, cross-surface parity, route parity, or
accessibility). The contract narrative is
`docs/ux/m4/stabilize-the-start-center-recent-work-list-workspace.md` and the
release-evidence packet is
`artifacts/ux/m4/stabilize-the-start-center-recent-work-list-workspace.md`.

The eight records cover the claimed stable matrix:

| Fixture | Target kind | Class | Failure state |
| --- | --- | --- | --- |
| `local_folder_reachable.json` | Folder | local | ready |
| `local_file_reachable.json` | File | local | ready |
| `multi_root_workspace_reachable.json` | Workspace | local | ready |
| `local_repo_missing_path.json` | Repository | local | missing_path |
| `workspace_moved_root.json` | Workspace | local | moved_root |
| `ssh_remote_unreachable.json` | SSH | remote_backed | reconnect_required |
| `devcontainer_unreachable.json` | Dev container | remote_backed | reconnect_required |
| `managed_cloud_authority_expired.json` | Cloud workspace | managed | reconnect_required |
