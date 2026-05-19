# History-rewrite, conflict-session, stash-entry, sequence-edit, and recovery-checkpoint beta contract

This document pins the beta truth for risky Git mutation flows — rebase,
interactive rebase, cherry-pick, revert, reset, patch apply, stash
apply / pop / drop, branch-from-stash, and the ref-update proposals that
gate every ref move. Desktop, CLI / headless, and support / export
surfaces all project the same record family so an in-progress session
keeps operation provenance, recovery posture, and explicit next-safe
paths consistent across surfaces and across editor restarts.

The Rust contract lives at `crates/aureline-git/src/history_rewrite/`.
The boundary schemas live at:

- `schemas/git/conflict_session.schema.json`
- `schemas/git/sequence_edit_session.schema.json`
- `schemas/git/stash_entry.schema.json`
- `schemas/git/recovery_checkpoint.schema.json` (also covers the
  ref-update-proposal record)

Canonical fixtures live under
`fixtures/git/m3/history_rewrite_and_stash/`.

## Record family

| Record kind | Owns | Lifecycle states |
| --- | --- | --- |
| `history_rewrite_conflict_session_record` | one paused merge / rebase / cherry-pick / revert / patch-apply / stash-apply / merge conflict | `draft_pending_admit`, `active_awaiting_resolution`, `paused_awaiting_user_input`, `paused_awaiting_external_tool`, `continuing_after_resolution`, `skipped_conflicted_step`, `aborted_rolled_back`, `completed_committed`, `completed_handed_off`, `failed_no_changes_made` |
| `history_rewrite_sequence_edit_session_record` | one interactive rebase todo / cherry-pick list / patch sequence | `draft_unsaved`, `saved_ready_to_run`, `running`, `paused_for_conflict`, `paused_for_user_edit`, `completed_admitted`, `aborted_rolled_back`, `failed_no_changes_made` |
| `history_rewrite_stash_entry_record` | one stash or shelf object across capture / apply / pop / drop / promote-to-branch | `captured_unapplied`, `applied_kept`, `applied_popped`, `dropped`, `promoted_to_branch`, `applied_with_conflict` |
| `history_rewrite_recovery_checkpoint_record` | one rollback-safe pre-mutation snapshot | `captured_ready_to_restore`, `captured_pending_admit`, `restored`, `expired_pending_prune`, `missing_pending_review` |
| `history_rewrite_ref_update_proposal_record` | one proposed ref move (rebase head, branch delete, force-push, hard reset) | `drafted_pending_review`, `blocked_protected_branch`, `blocked_policy`, `blocked_collaboration`, `ready_to_apply`, `applied`, `withdrawn` |

Each record carries a `worktree_context` block, a `recovery_posture`
block, a non-empty `consumer_surfaces` list that always includes
`support_export` and `audit_lane`, a `support_export` envelope that
forbids raw-path / raw-branch-name / raw-patch-body / raw-reflog-body /
raw-stash-body export, and an `audit_events` projection drawn from the
closed audit-event vocabulary.

## Restart-safe persistence

Every record is intended to outlive an editor restart. The stable IDs
on each row (`conflict_session_id`, `sequence_edit_session_id`,
`stash_entry_id`, `recovery_checkpoint_id`, `ref_update_proposal_id`)
are the persistence keys. A sequence-edit session pins both the
structured `steps` block and the opaque `raw_todo_text_ref` so the raw
todo text and the structured cards stay bound to the same underlying
object; surfaces never compute one without the other. A conflict session
pins the same `raw_todo_text_ref` from its parent sequence-edit session
when one is active.

## Recovery posture and the destructive-action gate

Before a destructive history-rewrite or branch-moving step proceeds, the
session's `recovery_posture.posture_class` must be one of:

- `recovery_checkpoint_captured` — a `recovery_checkpoint_record` was
  captured beforehand and `checkpoint_captured` is true with a non-null
  `recovery_checkpoint_ref`. Restore is offline and deterministic.
- `reflog_only_disclosure_acknowledged` — the user accepted the
  reflog-only fallback disclosure explicitly. The session may proceed,
  but the surface MUST surface that recovery depends on the local
  reflog only.

A posture of `no_recovery_available_blocks_apply` denies the
destructive-action gate; a posture of `external_handoff_pending` denies
until the external handoff returns. Continuing, skipping, or completing
a conflict session, running or completing a sequence-edit session,
applied / popped stash apply, and `ready_to_apply` / `applied`
ref-update proposals all require a posture that satisfies the gate. An
`applied` proposal with `force_move_required=true` requires
`recovery_checkpoint_captured` specifically; a reflog-only disclosure
is not enough for a force move.

## Protected-branch, policy, and collaboration blocks

A ref-update-proposal record records its active blocks on `blocks[]`
using the closed `block_class` vocabulary:

- `no_block`
- `protected_branch_no_force_push`
- `protected_branch_no_deletion`
- `policy_admin_lock`
- `policy_required_review`
- `collaboration_active_session`
- `missing_recovery_disclosure`
- `block_class_unknown_requires_review`

Each non-`no_block` block MUST list at least one `next_safe_paths[]`
entry. Blocked proposals MUST also list at least one top-level
`next_safe_paths[]` entry. The closed `next_safe_path` vocabulary is:

- `open_alternate_worktree` — open the same history-rewrite in an
  alternate worktree so the protected ref stays put.
- `create_temporary_branch` — produce a temporary branch from the
  rewrite result; review before promoting.
- `export_history_plan` — export a redaction-safe history-rewrite plan
  for offline review or a maintainer.
- `review_only_mode` — open the proposal in review-only mode; no ref
  moves.
- `request_approval` — request an approval ticket from the policy
  surface; the proposal stays blocked until the ticket lands.
- `abort_operation` — abort the in-progress operation safely.
- `restore_checkpoint` — restore the captured recovery checkpoint.
- `switch_to_reflog_disclosure` — accept the reflog-only fallback in
  place of a captured checkpoint.
- `no_safe_path_blocked_requires_human` — there is no admissible safe
  path; a human must be involved. `preserves_protected_refs` MUST be
  false on this path.

A blocked proposal MUST emit a `next_safe_path_offered` audit event for
each path. A `next_safe_path_accepted` audit event records which path
the user took.

## Audit events

Every record may attach audit events drawn from the closed vocabulary
in `HISTORY_REWRITE_AUDIT_EVENTS`. The events covered are:

- session lifecycle: `session_admitted`, `session_resumed_after_restart`
- conflict actions: `conflict_continue_requested`,
  `conflict_skip_requested`, `conflict_abort_requested`
- sequence-edit lifecycle: `sequence_edit_saved`,
  `sequence_edit_step_started`, `sequence_edit_step_completed`
- stash actions: `stash_captured`, `stash_applied`, `stash_dropped`,
  `stash_promoted_to_branch`
- recovery: `recovery_checkpoint_captured`,
  `recovery_checkpoint_restored`
- ref-update proposal: `ref_update_proposal_drafted`,
  `ref_update_proposal_blocked`, `ref_update_proposal_applied`
- safe-path offers: `next_safe_path_offered`,
  `next_safe_path_accepted`

`next_safe_path_*` events MUST cite a `next_safe_path_class`.

## Cross-surface projection

The shared projection produced by `HistoryRewriteRecord::project` lets
desktop, CLI / headless, and support / export surfaces read the same
truth without computing the operation kind, lifecycle, recovery
posture, blocks, or next-safe paths independently:

- desktop history / conflict / sequence / stash panels render the
  projection's `display_label`, `summary`, `lifecycle_state`,
  `recovery_posture_class`, `blocks_summary`, and
  `next_safe_path_classes` directly;
- CLI / headless entry points print the same fields in plain text;
- support / export surfaces quote `support_export_refs`,
  `redaction_class`, and the `raw_*_export_allowed` flags (all `false`
  in the beta) to confirm no raw bodies cross the export boundary;
- the audit lane reads `audit_event_ids` to confirm restart-safe
  session provenance, action lineage, and next-safe-path offers all
  survived restart.

## Out of scope

This contract does not introduce full time-travel debugging and does
not broaden the M3 deliverable into hosted code-review automation
beyond local history-rewrite safety. Hosted-provider integration with
protected-branch policy data is consumed only as block reasons and
next-safe-path hints; provider-side enforcement remains the provider's
responsibility.
