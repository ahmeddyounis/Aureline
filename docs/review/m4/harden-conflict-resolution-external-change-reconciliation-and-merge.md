# Hardened conflict resolution, external-change reconciliation, and merge-editor recovery on stable rows

**Scope:** M04-100 — Harden conflict resolution, external-change reconciliation, and merge-editor recovery on stable rows.

**Status:** Stable Git lane — implemented in `crates/aureline-git`.

## Goal

Every merge, rebase, cherry-pick, revert, and external-change reconciliation flow must carry a durable conflict-session object that survives restart, preserves provenance of competing inputs, and can downgrade honestly between structured and raw resolution modes without implying the conflict is resolved when only the UI surface changed.

## Design principles

1. **Durable session object** — A [`StableConflictSessionRecord`] carries stable identity, repo/worktree refs, operation kind, base/ours/theirs refs, affected path set, unresolved count, resolution mode, and started-at/updated-at lineage. It is the single source of truth for restart, support export, and migration recovery.
2. **Provenance preservation** — The `base_ref`, `ours_ref`, and `theirs_ref` fields plus [`ConflictProvenanceRecord`] source classes keep competing inputs attributable. No resolution mode change erases where the inputs came from.
3. **Honest downgrade** — Switching from structured resolver to raw editor is recorded as `StructuredDowngradedToRaw`. This mode explicitly does NOT coexist with `completed_committed` or `completed_handed_off`, and the unresolved count must remain positive.
4. **Recovery checkpoint required for continue** — The `continuing_after_resolution` lifecycle state requires a non-null `recovery_checkpoint_ref`. This ensures destructive continuation is never attempted without a rollback path.
5. **Restart continuity** — The `previous_session_ref` field preserves lineage across IDE or CLI restarts. The support-export packet embeds a [`StableConflictSessionRestartSnapshot`] so the same structured truth can be reopened.
6. **Separable inspectable truths** — Session state, resolution mode, unresolved count, checkpoint state, provenance, and command actionability are all independent fields. No single "status" column hides the underlying truth.
7. **Redaction-safe support export** — Raw paths, branch names, and patch bodies are explicitly forbidden from crossing the support boundary.

## Record kinds

| Record kind | Purpose |
|---|---|
| `git_stable_conflict_session_record` | Durable conflict session with provenance, refs, and lineage. |
| `git_stable_conflict_session_packet` | Top-level packet consumed by editor, Git, CLI, and support surfaces. |
| `git_stable_conflict_session_command_record` | Command-graph operations (open resolver, downgrade, capture checkpoint, continue, abort). |
| `git_stable_conflict_session_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |
| `git_stable_conflict_session_support_export_packet` | Redaction-safe export with restart snapshot. |

## Closed vocabularies

### Operation kinds
- `merge`, `rebase`, `interactive_rebase`, `cherry_pick`, `revert`, `external_change_reconcile`

### Lifecycle states
- `draft_pending_admit`, `active_awaiting_resolution`, `paused_awaiting_user_input`, `paused_awaiting_external_tool`, `continuing_after_resolution`, `aborted_rolled_back`, `completed_committed`, `completed_handed_off`, `failed_no_changes_made`, `downgraded_structured_to_raw`

### Resolution modes
- `structured`, `raw`, `structured_downgraded_to_raw`

### Provenance source classes
- `git_index_stage`, `git_head`, `git_remote_tracking`, `vfs_external_change`, `provider_import`, `user_edited`, `unknown`

### Input freshness classes
- `fresh_observed`, `stale_within_window`, `stale_beyond_window`, `revoked_or_disconnected`, `never_observed`

## Key invariants

- `resolution_mode = structured_downgraded_to_raw` cannot coexist with `lifecycle_state = completed_committed` or `completed_handed_off`.
- `lifecycle_state = continuing_after_resolution` requires `recovery_checkpoint_ref` to be non-null.
- `affected_path_tokens` must be non-empty and unique.
- `consumer_surfaces` must include `support_export` and `audit_lane`.
- All `raw_*_export_allowed` flags in support export must be `false`.
- `base_ref`, `ours_ref`, and `theirs_ref` must all be non-empty.
- Provenance source classes must be from the closed vocabulary.
- Command actionability is computed from session state; `continue_after_resolve` is only actionable when `unresolved_count == 0`.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-git/src/harden_conflict_resolution_external_change_reconciliation_and_merge/mod.rs` |
| Schema | `schemas/git/stable_conflict_session.schema.json` |
| Fixtures | `fixtures/git/m4/harden_conflict_resolution_external_change_reconciliation_and_merge/` |
| Tests | `crates/aureline-git/tests/harden_conflict_resolution_external_change_reconciliation_and_merge_alpha.rs` |
| CLI mirror | `crates/aureline-git/src/bin/aureline_git_stable_conflict_session.rs` |

## Integration with existing lanes

- Consumes [`GitConflictHandoffPacket`] from the `conflicts` module.
- References [`aureline_git::history_rewrite`] records (recovery checkpoints, conflict sessions) via opaque refs.
- Projects into the same inspector/CLI/support-export surfaces as the `history_rewrite` and `harden_merge_rebase_cherry_pick_revert_and_reset` modules.
- Supports `migration_recovery` consumer surface for migration continuity.

## Verification

```bash
cargo test -p aureline-git --test harden_conflict_resolution_external_change_reconciliation_and_merge_alpha
```
