# Recent Work, Restore Card, and Workspace Switcher Row Contract

This document freezes the row-level anatomy used by recent-work
lists, restore cards, and workspace-switcher entries. It is a
companion to the broader Start Center disclosure contract and to
the workspace entry / restore object model:

- [`/docs/ux/start_center_contract.md`](./start_center_contract.md)
- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
- [`/schemas/ux/recent_work_row.schema.json`](../../schemas/ux/recent_work_row.schema.json)
- [`/fixtures/ux/recent_work_rows/`](../../fixtures/ux/recent_work_rows/)

The goal is simple: a missing folder, stale cache, disconnected
remote, suspended managed workspace, or partial restore can never
look like an ordinary local open. The same unavailable-target and
recovery vocabulary is reused across the Start Center, the
workspace switcher, `Open Recent`, and restore cards.

## 1. Scope

This contract freezes three presentation records:

- `recent_work_row_record` - a concrete row rendered in recent
  work, pinned work, and `Open Recent`.
- `restore_card_summary_record` - a concrete restore card that
  summarizes a pending `restore_prompt_record`.
- `workspace_switcher_row_record` - a concrete row rendered in
  a palette, menu, or dedicated workspace switcher.

These records do not replace the upstream
`project_entry_action_record`, `recent_work_entry_record`, or
`restore_prompt_record`. They wrap those records with the fields a
surface must show before activation.

## 2. One Activation Path

Every activation from Start Center, `Open Recent`, or the in-workspace
switcher resolves through the same workspace object model:

1. The row references the upstream entry or restore object.
2. Activation emits or reuses one `project_entry_action_record`.
3. The entry action resolves target identity, trust, restore
   availability, and authority re-evaluation.
4. Failures route to the shared unavailable-target recovery path.

A switcher row must not invent a faster path that bypasses the
Start Center error handling path. A Start Center row must not invent
copy or actions the switcher cannot show. Both rows quote the same
target state, recovery action ids, and write-safety badge.

## 3. Shared Vocabulary

### 3.1 Root Kind

`root_kind` names the root being opened, not just the icon:

- `local_file`
- `local_folder`
- `local_repo_root`
- `workspace_manifest`
- `multi_root_workspace`
- `workset`
- `remote_repository`
- `ssh_remote_root`
- `container_root`
- `devcontainer_root`
- `managed_cloud_workspace`
- `template_snapshot`
- `imported_or_handoff_root`
- `recovery_checkpoint`
- `recent_session`

Rows may still show a friendly icon, but the root kind is the
machine-readable truth. A remote or managed root must not be rendered
with only a generic folder identity.

### 3.2 Target State

The row-level target state set is:

- `reachable`
- `stale_metadata`
- `missing_target`
- `moved_target_detected`
- `missing_mount`
- `remote_unreachable`
- `authority_expired`
- `suspended_managed_workspace`
- `locked_by_other_instance`
- `policy_blocked`
- `quarantined`
- `mode_downgraded`
- `cached_only`
- `unknown`

Every state except `reachable` requires an unavailable-target block
with at least one recovery action and a user-visible explanation.
`cached_only` means metadata or a read-only cache can be inspected,
but writes are not safe until the canonical owner is revalidated.

### 3.3 Recovery Actions

The shared recovery action set is:

- `open`
- `open_in_new_window`
- `open_restricted`
- `open_read_only_cached_view`
- `locate_missing_target`
- `reconnect`
- `reauth`
- `retry_later`
- `compare_before_restore`
- `open_without_restore`
- `restore_now`
- `skip_once`
- `open_clean`
- `recover_draft`
- `export_evidence`
- `pin`
- `unpin`
- `remove_from_recents`
- `reveal_in_explorer`
- `resume`
- `rebuild`
- `cancel_switch`
- `reopen_previous_workspace`
- `close_other_window`
- `transfer_window`
- `clear_recent_work`
- `exit_privacy_reduced_mode`

`open_read_only_cached_view` is the only action that may inspect a
stale or disconnected target without revalidating write authority.
`retry_later` preserves the row and schedules no destructive cleanup.
`remove_from_recents` is always destructive-scoped metadata cleanup
and requires a confirmation preview when the target is missing,
moved, remote, or policy-blocked.

### 3.4 Write-Safety Badges

Rows and cards use one write-safety badge:

- `writes_allowed`
- `writes_require_revalidation`
- `writes_blocked_cached_view_only`
- `writes_blocked_target_unavailable`
- `writes_blocked_policy`
- `writes_unsafe_stale_or_disconnected`

Only `writes_allowed` may pair with a normal `open` primary action.
All other badges must either disable writes, force a repair action,
or route to `open_read_only_cached_view`.

### 3.5 Time and Privacy Fields

Rows expose at least one of:

- `last_opened_at`
- `last_validated_at`

Remote, cached, and privacy-reduced rows prefer
`last_validated_at` when last-opened time would imply freshness the
target no longer has.

Privacy reduction is explicit. A row whose path, account affordance,
or timestamp is hidden must carry `privacy_reduction_mode` and
`privacy_redaction_applied`. Hiding recent-work metadata must not
remove the ability to open a local folder or clone a repository.

## 4. Recent-Work Row Anatomy

Every `recent_work_row_record` includes:

- `primary_label` - the project or workspace name.
- `location_or_target_subtitle` - redaction-aware path, host,
  provider, target, or root class.
- `target_kind` - re-exported from the entry / restore object
  model.
- `root_kind` - from this contract.
- `target_state` and `availability_class`.
- `last_opened_at` or `last_validated_at`.
- `trust_state`.
- `restore_availability`.
- `pinned`.
- `write_safety_badge`.
- `row_actions[]`.
- `activation_contract`.

Required actions:

- Reachable rows expose `open` or `open_in_new_window`, plus
  `pin` or `unpin`, and `remove_from_recents`.
- Missing or moved local rows expose `locate_missing_target` and
  may expose `open_read_only_cached_view` only when a safe cache
  exists.
- Disconnected remote rows expose `reconnect` or `reauth`, plus
  `retry_later`; if a cache is available they may expose
  `open_read_only_cached_view`.
- Removing a missing or moved row must explain that only recent-work
  metadata is removed. It must not delete the target or recovery
  journal.

## 5. Restore-Card Summary Anatomy

Every `restore_card_summary_record` includes:

- `restore_prompt_ref`.
- `restore_class` - one of the upstream restore levels.
- `restorable_counts` with separate counts for windows, editors,
  terminals, remote sessions, dirty buffers, notebooks, tasks, and
  evidence packets.
- `remote_session_summaries[]` for each restorable or blocked
  remote session.
- `dirty_buffer_summary`.
- `missing_dependency_warnings[]`.
- `write_safety_badge`.
- `card_actions[]` containing `restore_now`, `skip_once`, and
  `open_clean` on every non-takeover card.

Restore cards never flatten dirty-buffer recovery, terminal
topology, remote-session reattach, and evidence-only recovery into a
single "reopen" count. Terminals, notebooks, debug sessions, tasks,
remote shells, tunnels, and provider actions are restored as topology
or evidence until the user explicitly reruns or reattaches.

## 6. Workspace-Switcher Row Anatomy

Every `workspace_switcher_row_record` includes:

- `primary_label` and `location_or_target_subtitle`.
- `switcher_entry_classes[]`, drawn from `local`, `remote`,
  `managed`, `pinned`, `recent`, `recently_restored`,
  `open_window`, `template`, and `imported`.
- `searchable_terms[]`, pre-redacted.
- `target_kind`, `root_kind`, `target_state`, `trust_state`,
  `restore_availability`, and `write_safety_badge`.
- `cross_window_consequence`.
- `row_actions[]`.
- `activation_contract`.

Cross-window consequences are explicit. The switcher must say
whether activation focuses an existing window, opens a new window,
replaces the current workspace, adds a root, closes or suspends the
previous workspace, or is blocked by dirty buffers. If the new open
fails, the row must preserve `cancel_switch` and
`reopen_previous_workspace` where those actions are applicable.

## 7. Privacy Reduction and Clearing Controls

Sensitive environments may reduce, hide, or clear recent-work
metadata, but they must not disable local work entry. A conforming
surface:

- keeps `open` for local folder/workspace entry and clone entry
  reachable;
- shows a privacy-reduction notice before hiding paths or rows;
- distinguishes hidden-by-privacy from empty-by-history;
- offers `clear_recent_work` with confirmation and preview;
- offers `exit_privacy_reduced_mode` when the current profile
  allows it;
- never clears dirty-buffer journals, restore prompts, settings,
  profiles, or evidence packets as part of recent-work clearing.

## 8. Unavailable-Target Rules

1. A broken, stale, cached-only, or remote-backed row must show a
   target-state chip and write-safety badge before activation.
2. `open_read_only_cached_view` must be labelled as read-only and
   must not create save tokens for the canonical target.
3. `retry_later` must leave the row in place and must not remove
   cached metadata.
4. `remove_from_recents` must explain the condition first and must
   confirm metadata deletion before it runs.
5. A moved target must prefer `locate_missing_target` over deletion.
6. A disconnected remote target must prefer `reconnect`, `reauth`,
   or `retry_later` over deletion.
7. Restore cards and switcher rows reuse these same action ids and
   badges; surface-specific synonyms are non-conforming.

## 9. Worked Fixtures

The fixture corpus under
[`/fixtures/ux/recent_work_rows/`](../../fixtures/ux/recent_work_rows/)
exercises:

- ordinary reachable local work;
- a moved local repository with read-only cached view and explicit
  write-unsafeness;
- a disconnected remote workspace with reconnect, retry, and cached
  view actions;
- a partial restore card with remote-session and dirty-buffer
  warnings;
- a workspace-switcher row that is pinned, managed, recently
  restored, and has cross-window consequences;
- a privacy-reduced row that hides recent-work metadata while
  preserving local entry and clear controls.
