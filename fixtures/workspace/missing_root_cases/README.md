# Missing-root and recovery recent-work cases

These fixtures exercise the shared recent-work failure taxonomy for
unavailable project-entry targets. Each case is a
`recent_work_entry_record` that can be projected into Start Center,
workspace switcher, and restore-placeholder surfaces without minting
surface-specific state.

| Fixture | Failure state | Required recovery |
| --- | --- | --- |
| `missing_local_root.json` | `missing_path` | locate, open without restore, remove metadata |
| `moved_local_root.json` | `moved_root` | locate, inspect cached view, open without restore, remove metadata |
| `reconnect_ssh_workspace.json` | `reconnect_required` | reconnect, inspect cached view, retry later, remove metadata |
| `inspect_only_handoff.json` | `inspect_only` | inspect cached/evidence state, compare, open without restore, remove metadata |

Removal in these cases is scoped to recent-work metadata only. It does
not delete local files, remote workspaces, recovery journals, restore
checkpoints, or evidence packets.
