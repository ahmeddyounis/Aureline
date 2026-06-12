# Start Center and workspace-switcher parity for M5 entry surfaces

Generated from the seeded packet in
[`crate::m5_start_center_and_switcher`](../../../crates/aureline-shell/src/m5_start_center_and_switcher/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher -- markdown > \
  artifacts/ux/m5/start-center-and-switcher-audit.md
```

- Packet id: `shell:m5_start_center_and_switcher:v1:default`
- Rows: 13
- Surface classes covered: 8/8
- Diagnostics: 5
- All rows in both surfaces: true
- No target kind collapsed: true
- No trust widened: true
- Full parity: true
- Generated at: `2026-06-11T00:00:00Z`

## Parity rows

| Surface class | Target kind | Trust | Restore | Root state | In both | Parity | Diagnostic |
|---|---|---|---|---|:---:|:---:|---|
| Local folder | `local_folder` | `trusted` | `compatible` | `root_resolved` | yes | full | — |
| Workspace file | `workspace_manifest` | `trusted` | `exact` | `root_resolved` | yes | full | — |
| Multi-root workspace | `workset_manifest` | `trusted` | `compatible` | `root_resolved` | yes | full | — |
| SSH target | `ssh_workspace` | `trusted` | `compatible` | `root_resolved` | yes | full | — |
| Container / dev container | `devcontainer_workspace` | `trusted` | `compatible` | `root_resolved` | yes | full | — |
| Managed workspace | `managed_cloud_workspace` | `pending_evaluation` | `compatible` | `root_resolved` | yes | full | — |
| Import packet | `portable_state_package` | `pending_evaluation` | `none` | `root_resolved` | yes | full | — |
| Bundle-backed entry | `template_or_prebuild_snapshot` | `trusted` | `none` | `root_resolved` | yes | full | — |
| Local folder | `local_repo_root` | `trusted` | `layout_only` | `missing_root` | yes | full | `missing_root` |
| Workspace file | `workspace_manifest` | `trusted` | `layout_only` | `relocated_root` | yes | full | `relocated_workspace` |
| Import packet | `handoff_packet` | `pending_evaluation` | `evidence_only` | `stale_root` | yes | full | `stale_target` |
| SSH target | `remote_repository` | `pending_evaluation` | `evidence_only` | `remote_host_unreachable` | yes | full | `remote_host_unreachable` |
| Managed workspace | `managed_cloud_workspace` | `pending_evaluation` | `layout_only` | `root_resolved` | yes | full | `partial_restore` |

## Export-safe diagnostics

| Diagnostic | Surface class | Redacted location | Trust | Restore | Recovery actions |
|---|---|---|---|---|---|
| Missing root | Local folder | Repository | `trusted` | `layout_only` | `locate_missing_target`, `open_without_restore`, `remove_from_recents`, `unpin` |
| Relocated workspace | Workspace file | Workspace | `trusted` | `layout_only` | `locate_missing_target`, `open_without_restore`, `remove_from_recents`, `pin` |
| Stale target | Import packet | Handoff packet | `pending_evaluation` | `evidence_only` | `open_read_only_cached_view`, `open_without_restore`, `remove_from_recents`, `pin` |
| Remote host unreachable | SSH target | Remote repository | `pending_evaluation` | `evidence_only` | `reconnect`, `retry_later`, `open_without_restore`, `remove_from_recents`, `pin` |
| Partial restore | Managed workspace | Cloud workspace | `pending_evaluation` | `layout_only` | `open`, `pin` |

