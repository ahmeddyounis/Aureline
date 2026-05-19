# History-rewrite and stash beta fixtures

These fixtures pin one shared truth for risky Git mutation flows — rebases,
interactive rebases, cherry-picks, reverts, resets, stash captures /
applies / promotions, and the ref-update proposals that gate every ref move.
Desktop, CLI/headless, and support/export surfaces all project these
records through `aureline_git::history_rewrite` so an in-progress session
keeps operation provenance, recovery posture, and explicit next-safe paths
consistent across surfaces and restarts.

The Rust contract lives at `crates/aureline-git/src/history_rewrite/`.
Schemas live at `schemas/git/{conflict_session,sequence_edit_session,stash_entry,recovery_checkpoint}.schema.json`.
Reviewer guidance lives at `docs/source_control/m3/history_rewrite_beta.md`.

## Cases

| Fixture | Record kind | Lifecycle | What it covers |
| --- | --- | --- | --- |
| `rebase_conflict_paused.json` | conflict session | `active_awaiting_resolution` | Rebase paused on two conflicted paths with checkpoint pinned. |
| `cherry_pick_conflict_continue_after_resolve.json` | conflict session | `continuing_after_resolution` | Cherry-pick continuing after user-driven resolution; destructive gate satisfied. |
| `revert_conflict_abort_rolled_back.json` | conflict session | `aborted_rolled_back` | Revert abort restoring HEAD / index / worktree from the captured checkpoint. |
| `interactive_rebase_sequence_running.json` | sequence-edit session | `running` | Interactive rebase with pick / squash / reword / pick steps and a current-step cursor. |
| `stash_apply_with_conflict.json` | stash entry | `applied_with_conflict` | Stash apply paused on conflict with a pinned conflict-session ref. |
| `branch_from_stash_promoted.json` | stash entry | `promoted_to_branch` | Stash promoted to a temporary branch with audit event quoted. |
| `recovery_checkpoint_captured.json` | recovery checkpoint | `captured_ready_to_restore` | Pre-mutation checkpoint covering HEAD and index snapshot. |
| `ref_update_protected_branch_blocked.json` | ref-update proposal | `blocked_protected_branch` | Force-move denied with `open_alternate_worktree` and `create_temporary_branch` next-safe paths. |
| `reset_hard_reflog_only_acknowledged.json` | ref-update proposal | `ready_to_apply` | Hard reset proceeding with the reflog-only disclosure explicitly acknowledged. |

Replay the corpus with:

```
cargo test -p aureline-git --test history_rewrite_beta
```

Every record must keep `raw_path_export_allowed`,
`raw_branch_name_export_allowed`, `raw_patch_body_export_allowed`,
`raw_reflog_body_export_allowed`, and `raw_stash_body_export_allowed` all
false; raw paths, raw branch names, raw patch bodies, raw reflog bodies,
and raw stash bodies never cross this boundary.
