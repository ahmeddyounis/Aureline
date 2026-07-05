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
- Rows: 14
- Workspace-switcher entries: 14
- Restore-prompt cards: 12
- Surface classes covered: 8/8
- Diagnostics: 5
- All rows in both surfaces: true
- No target kind collapsed: true
- No trust widened: true
- Full parity: true
- Object identity model: `docs/workspace/entry_restore_object_model.md`
- Generated at: `2026-06-11T00:00:00Z`

## Quick-action cards

| Action | Icon | Command | Shortcut disclosure | Badge |
|---|---|---|---|---|
| Open folder | `folder-open` | `cmd:workspace.open_folder` | Shortcut: platform open-folder binding when assigned; otherwise Command Palette | - |
| Open workspace | `layout-panel-left` | `cmd:workspace.open_folder` | Shortcut: shares open-folder command binding with workspace-file scope | - |
| Clone repository | `git-branch` | `cmd:workspace.clone_repository` | Shortcut: unassigned by default; available from Command Palette | Review before trust |
| Restore last session | `history` | `cmd:workspace.restore_from_checkpoint` | Shortcut: restore command binding when assigned; otherwise Command Palette | Restore available |
| Import from… | `import` | `cmd:workspace.import_profile` | Shortcut: unassigned by default; available from Command Palette | Compare before apply |

## Workspace-switcher entries

| Entry | Object identity | Window | Profile | Keymap | Badges | Dirty | Actions |
|---|---|---|---|---|---|---:|---|
| checkout | `fs:checkout` | `current_window` | profile:local-default | keymap:default | `local` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| platform | `fs:platform_workspace` | `reopen_available` | profile:local-default | keymap:default | `local` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| monorepo bundle | `fs:monorepo_workset` | `reopen_available` | profile:local-default | keymap:default | `local` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| edge-build | `remote:edge_build` | `reopen_available` | profile:remote-work | keymap:default | `remote` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| api sandbox | `remote:api_sandbox` | `reopen_available` | profile:remote-work | keymap:vscode-compatible | `remote` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| research cloud | `remote:research_cloud` | `reopen_available` | profile:remote-work | keymap:profile-default | `remote`, `managed` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| imported settings | `artifact:imported_settings` | `reopen_available` | profile:import-review | keymap:default | `imported` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| typescript web app | `artifact:ts_web_app_template` | `reopen_available` | profile:starter-template | keymap:default | `local` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| release branch | `fs:release_branch` | `open_in_other_window` | profile:local-default | keymap:default | `local` | 1 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `transfer_window` |
| payments | `fs:payments` | `blocked_or_unavailable` | profile:local-default | keymap:default | `local` | 2 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| design system | `fs:design_system` | `blocked_or_unavailable` | profile:local-default | keymap:default | `local` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| vendored docs | `artifact:vendored_docs` | `blocked_or_unavailable` | profile:import-review | keymap:default | `imported`, `cached_only` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |
| staging cluster | `remote:staging_cluster` | `blocked_or_unavailable` | profile:remote-work | keymap:default | `remote`, `cached_only` | 0 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window`, `reconnect` |
| training run | `remote:training_run` | `reopen_available` | profile:remote-work | keymap:profile-default | `remote`, `managed` | 1 | `close_window`, `reopen_previous_workspace`, `move_to_new_window`, `cancel_switch`, `open_in_new_window` |

## Restore-prompt cards

| Prompt | Object identity | Restore | Dirty buffers | Safest action | Actions |
|---|---|---|---:|---|---|
| checkout: Compatible restore with 0 dirty buffer(s) | `fs:checkout` | `compatible_restore` | 0 | `restore_now` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| platform: Exact restore with 0 dirty buffer(s) | `fs:platform_workspace` | `exact_restore` | 0 | `restore_now` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| monorepo bundle: Compatible restore with 0 dirty buffer(s) | `fs:monorepo_workset` | `compatible_restore` | 0 | `restore_now` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| edge-build: Compatible restore with 0 dirty buffer(s) | `remote:edge_build` | `compatible_restore` | 0 | `restore_now` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| api sandbox: Compatible restore with 0 dirty buffer(s) | `remote:api_sandbox` | `compatible_restore` | 0 | `restore_now` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| research cloud: Compatible restore with 0 dirty buffer(s) | `remote:research_cloud` | `compatible_restore` | 0 | `restore_now` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| release branch: Recovered drafts with 1 dirty buffer(s) | `fs:release_branch` | `recovered_drafts` | 1 | `restore_now` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| payments: Recovered drafts with 2 dirty buffer(s) | `fs:payments` | `recovered_drafts` | 2 | `safe_mode` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| design system: Layout only with 0 dirty buffer(s) | `fs:design_system` | `layout_only` | 0 | `open_without_restore` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| vendored docs: Evidence only with 0 dirty buffer(s) | `artifact:vendored_docs` | `evidence_only` | 0 | `open_without_restore` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| staging cluster: Evidence only with 0 dirty buffer(s) | `remote:staging_cluster` | `evidence_only` | 0 | `open_without_restore` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |
| training run: Recovered drafts with 1 dirty buffer(s) | `remote:training_run` | `recovered_drafts` | 1 | `restore_now` | `restore_now`, `safe_mode`, `open_without_restore`, `clear_journal`, `export_evidence` |

## Restore vocabulary

| Token | Label | Definition |
|---|---|---|
| `exact_restore` | Exact restore | The same object identity and session state can be restored without translation. |
| `compatible_restore` | Compatible restore | The same object identity can be restored after a declared compatible translation or rebind. |
| `layout_only` | Layout only | Window, pane, or editor layout can be restored, but live session state cannot. |
| `recovered_drafts` | Recovered drafts | Dirty buffers or drafts can be recovered without claiming a full session restore. |
| `evidence_only` | Evidence only | Evidence can be exported or inspected, but not replayed as active state. |
| `no_restore` | No restore | No restorable state is available for this entry. |

## Parity rows

| Surface class | Target kind | Last opened | Trust | Restore | Root state | In both | Parity | Diagnostic |
|---|---|---|---|---|---|:---:|:---:|---|
| Local folder | `local_folder` | `mono:0000:00:00:00.0010` | `trusted` | `compatible_restore` | `root_resolved` | yes | full | — |
| Workspace file | `workspace_manifest` | `mono:0000:00:00:00.0009` | `trusted` | `exact_restore` | `root_resolved` | yes | full | — |
| Multi-root workspace | `workset_manifest` | `mono:0000:00:00:00.0008` | `trusted` | `compatible_restore` | `root_resolved` | yes | full | — |
| SSH target | `ssh_workspace` | `mono:0000:00:00:00.0007` | `trusted` | `compatible_restore` | `root_resolved` | yes | full | — |
| Container / dev container | `devcontainer_workspace` | `mono:0000:00:00:00.0006` | `trusted` | `compatible_restore` | `root_resolved` | yes | full | — |
| Managed workspace | `managed_cloud_workspace` | `mono:0000:00:00:00.0005` | `pending_evaluation` | `compatible_restore` | `root_resolved` | yes | full | — |
| Import packet | `portable_state_package` | `mono:0000:00:00:00.0004` | `pending_evaluation` | `no_restore` | `root_resolved` | yes | full | — |
| Bundle-backed entry | `template_or_prebuild_snapshot` | `mono:0000:00:00:00.0003` | `trusted` | `no_restore` | `root_resolved` | yes | full | — |
| Local folder | `local_repo_root` | `mono:0000:00:00:00.0002` | `trusted` | `recovered_drafts` | `root_resolved` | yes | full | — |
| Local folder | `local_repo_root` | `mono:0000:00:00:00.0002` | `trusted` | `recovered_drafts` | `missing_root` | yes | full | `missing_root` |
| Workspace file | `workspace_manifest` | `mono:0000:00:00:00.0001` | `trusted` | `layout_only` | `relocated_root` | yes | full | `relocated_workspace` |
| Import packet | `handoff_packet` | `mono:0000:00:00:00.0000` | `pending_evaluation` | `evidence_only` | `stale_root` | yes | full | `stale_target` |
| SSH target | `remote_repository` | `mono:0000:00:00:00.0000` | `pending_evaluation` | `evidence_only` | `remote_host_unreachable` | yes | full | `remote_host_unreachable` |
| Managed workspace | `managed_cloud_workspace` | `mono:0000:00:00:00.0000` | `pending_evaluation` | `recovered_drafts` | `root_resolved` | yes | full | `partial_restore` |

## Export-safe diagnostics

| Diagnostic | Surface class | Redacted location | Trust | Restore | Recovery actions |
|---|---|---|---|---|---|
| Missing root | Local folder | Repository | `trusted` | `layout_only` | `locate_missing_target`, `open_without_restore`, `remove_from_recents`, `unpin` |
| Relocated workspace | Workspace file | Workspace | `trusted` | `layout_only` | `locate_missing_target`, `open_without_restore`, `remove_from_recents`, `pin` |
| Stale target | Import packet | Handoff packet | `pending_evaluation` | `evidence_only` | `open_read_only_cached_view`, `open_without_restore`, `remove_from_recents`, `pin` |
| Remote host unreachable | SSH target | Remote repository | `pending_evaluation` | `evidence_only` | `reconnect`, `retry_later`, `open_without_restore`, `remove_from_recents`, `pin` |
| Partial restore | Managed workspace | Cloud workspace | `pending_evaluation` | `layout_only` | `open`, `pin` |

