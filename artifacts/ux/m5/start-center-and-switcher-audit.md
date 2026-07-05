# Start Center and workspace-switcher parity for M5 entry surfaces

Generated from the seeded packet in
[`crate::m5_start_center_and_switcher`](../../../crates/aureline-shell/src/m5_start_center_and_switcher/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- markdown > \
  artifacts/ux/m5/start-center-and-switcher-audit.md
```

- Packet id: `shell:m5_start_center_and_switcher:v1:default`
- Quick-action cards: 5
- Rows: 13
- Surface classes covered: 8/8
- Diagnostics: 5
- All rows in both surfaces: true
- No target kind collapsed: true
- No trust widened: true
- Full parity: true
- Generated at: `2026-06-11T00:00:00Z`

## Quick-action cards

| Action | Icon | Command | Shortcut disclosure | Badge |
|---|---|---|---|---|
| Open folder | `folder-open` | `cmd:workspace.open_folder` | Shortcut: platform open-folder binding when assigned; otherwise Command Palette | - |
| Open workspace | `layout-panel-left` | `cmd:workspace.open_folder` | Shortcut: shares open-folder command binding with workspace-file scope | - |
| Clone repository | `git-branch` | `cmd:workspace.clone_repository` | Shortcut: unassigned by default; available from Command Palette | Review before trust |
| Restore last session | `history` | `cmd:workspace.restore_from_checkpoint` | Shortcut: restore command binding when assigned; otherwise Command Palette | Restore available |
| Import from… | `import` | `cmd:workspace.import_profile` | Shortcut: unassigned by default; available from Command Palette | Compare before apply |

## Parity rows

| Surface class | Target kind | Last opened | Trust | Restore | Root state | In both | Parity | Diagnostic |
|---|---|---|---|---|---|:---:|:---:|---|
| Local folder | `local_folder` | `mono:0000:00:00:00.0010` | `trusted` | `compatible` | `root_resolved` | yes | full | — |
| Workspace file | `workspace_manifest` | `mono:0000:00:00:00.0009` | `trusted` | `exact` | `root_resolved` | yes | full | — |
| Multi-root workspace | `workset_manifest` | `mono:0000:00:00:00.0008` | `trusted` | `compatible` | `root_resolved` | yes | full | — |
| SSH target | `ssh_workspace` | `mono:0000:00:00:00.0007` | `trusted` | `compatible` | `root_resolved` | yes | full | — |
| Container / dev container | `devcontainer_workspace` | `mono:0000:00:00:00.0006` | `trusted` | `compatible` | `root_resolved` | yes | full | — |
| Managed workspace | `managed_cloud_workspace` | `mono:0000:00:00:00.0005` | `pending_evaluation` | `compatible` | `root_resolved` | yes | full | — |
| Import packet | `portable_state_package` | `mono:0000:00:00:00.0004` | `pending_evaluation` | `none` | `root_resolved` | yes | full | — |
| Bundle-backed entry | `template_or_prebuild_snapshot` | `mono:0000:00:00:00.0003` | `trusted` | `none` | `root_resolved` | yes | full | — |
| Local folder | `local_repo_root` | `mono:0000:00:00:00.0002` | `trusted` | `layout_only` | `missing_root` | yes | full | `missing_root` |
| Workspace file | `workspace_manifest` | `mono:0000:00:00:00.0001` | `trusted` | `layout_only` | `relocated_root` | yes | full | `relocated_workspace` |
| Import packet | `handoff_packet` | `mono:0000:00:00:00.0000` | `pending_evaluation` | `evidence_only` | `stale_root` | yes | full | `stale_target` |
| SSH target | `remote_repository` | `mono:0000:00:00:00.0000` | `pending_evaluation` | `evidence_only` | `remote_host_unreachable` | yes | full | `remote_host_unreachable` |
| Managed workspace | `managed_cloud_workspace` | `mono:0000:00:00:00.0000` | `pending_evaluation` | `layout_only` | `root_resolved` | yes | full | `partial_restore` |

## Export-safe diagnostics

| Diagnostic | Surface class | Redacted location | Trust | Restore | Recovery actions |
|---|---|---|---|---|---|
| Missing root | Local folder | Repository | `trusted` | `layout_only` | `locate_missing_target`, `open_without_restore`, `remove_from_recents`, `unpin` |
| Relocated workspace | Workspace file | Workspace | `trusted` | `layout_only` | `locate_missing_target`, `open_without_restore`, `remove_from_recents`, `pin` |
| Stale target | Import packet | Handoff packet | `pending_evaluation` | `evidence_only` | `open_read_only_cached_view`, `open_without_restore`, `remove_from_recents`, `pin` |
| Remote host unreachable | SSH target | Remote repository | `pending_evaluation` | `evidence_only` | `reconnect`, `retry_later`, `open_without_restore`, `remove_from_recents`, `pin` |
| Partial restore | Managed workspace | Cloud workspace | `pending_evaluation` | `layout_only` | `open`, `pin` |

