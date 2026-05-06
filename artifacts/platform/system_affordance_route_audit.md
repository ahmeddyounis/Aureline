# System-affordance route audit

Reviewer-side audit packet for the OS-level entry and reopen paths
listed in
[`/docs/ux/desktop_affordance_contract.md`](../../docs/ux/desktop_affordance_contract.md).
The packet exists so that every system open, file association,
open-with, reveal-in-system-shell, native dialog, recent-item
registration, dock / taskbar / jump-list reopen, OS-notification
click-through, badge activation, system share target, copy-path or
permalink action, drag-drop entry, open-from-terminal entry,
default-browser callback, protocol-handler invocation, and companion
handoff return is recorded as one inspectable trail bound to a canonical
command ID, canonical object identity, canonical event lineage, and a
single deep-link-intent vocabulary instead of becoming per-platform
folklore.

The packet is review evidence. It does not mint vocabulary. Every axis
is re-exported from the upstream schemas and ledgers:

- [`/schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json)
  — `deep_link_intent_record`, `system_affordance_case_record`,
  `source_surface_class`, `origin_class`, `route_class`,
  `requested_action_class`, `authority_delta_class`, `replay_posture`,
  `fallback_class`, `handler_ownership_class`,
  `ownership_change_review_state`, `privacy_payload_class`,
  `case_class`, `outcome_class`, `audit_event_id`,
  `lifecycle_state_class`.
- [`/artifacts/release/channel_ownership_audit.yaml`](../release/channel_ownership_audit.yaml)
  — side-by-side relation classes, owning-channel rows for file
  associations, recent items, protocol handlers, and diagnostics
  paths.
- [`/artifacts/release/state_root_map.yaml`](../release/state_root_map.yaml)
  — `recent_item_registration_class`,
  `file_association_registration_class`,
  `protocol_handler_ownership_class`,
  `update_marker_ownership_class`.
- [`/artifacts/release/portable_mode_limitations.yaml`](../release/portable_mode_limitations.yaml)
  — portable-mode forbidden host-mutation rows.
- [`/artifacts/platform/claimed_desktop_profiles.yaml`](./claimed_desktop_profiles.yaml)
  — claimed desktop profiles a route audit row MUST cover.
- [`/artifacts/platform/native_lifecycle_drill_packet.md`](./native_lifecycle_drill_packet.md)
  — closed expected-state-token vocabulary the lifecycle reopen rows
  cite.

If this packet disagrees with
[`/docs/ux/desktop_affordance_contract.md`](../../docs/ux/desktop_affordance_contract.md),
[`/schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json),
[`/artifacts/release/channel_ownership_audit.yaml`](../release/channel_ownership_audit.yaml),
[`/artifacts/release/state_root_map.yaml`](../release/state_root_map.yaml),
[`/artifacts/release/portable_mode_limitations.yaml`](../release/portable_mode_limitations.yaml),
or
[`/artifacts/platform/claimed_desktop_profiles.yaml`](./claimed_desktop_profiles.yaml),
those sources win and this packet plus its companion ledger and
fixtures update in the same change.

## Companion artifacts

| Artifact | Role |
|---|---|
| [`/artifacts/platform/file_association_ownership_matrix.yaml`](./file_association_ownership_matrix.yaml) | Machine-readable ownership matrix binding every file-extension class and recent-item / dock / taskbar reopen surface to one owning channel, one registration class, one user-or-admin override-disclosure rule, and the exact-target reopen rule per side-by-side relation. |
| [`/artifacts/platform/protocol_handler_ownership_matrix.yaml`](./protocol_handler_ownership_matrix.yaml) | Protocol-handler focused view (schemes + shared default) that references the full ownership matrix row ids and enumerates exact-target reopen drills. |
| [`/artifacts/platform/recent_item_and_protocol_ownership_audit.md`](./recent_item_and_protocol_ownership_audit.md) | Narrative audit packet for recent-item registration, protocol handlers, file associations, notification click-through, and dock/taskbar open actions. |
| [`/fixtures/platform/deep_link_replay_cases/`](../../fixtures/platform/deep_link_replay_cases/) | Worked `deep_link_intent_record` fixtures that prove every replay-denied posture, every authority-widening intent, every drifted target, and every origin-mismatch return fails closed with a typed denial reason and a fallback that preserves user intent or denies with explanation. |
| [`/fixtures/platform/exact_target_reopen_cases/`](../../fixtures/platform/exact_target_reopen_cases/) | Worked `deep_link_intent_record` fixtures that prove exact-target reopen succeeds truthfully or fails closed with bounded recovery across local files, workspaces, remote targets, review handoff, and auth callbacks. |
| [`/fixtures/platform/system_affordance_cases/`](../../fixtures/platform/system_affordance_cases/) | Upstream `system_affordance_case_record` fixtures the route-audit rows bind to for file association, native dialog, OS notification, badge, removable-volume return, copy / share, and open-from-terminal cases. |

## 1. Scope

This audit freezes:

- the route-audit table that maps every `source_surface_class` (system
  open, file association, open-with, reveal-in-system-shell, native
  open / save dialog, dock / taskbar recent, dock / taskbar jump action,
  OS notification click, OS badge activation, system share target,
  copy path or permalink, drag-drop open, open-from-terminal,
  default-browser callback, protocol handler, companion handoff return)
  to its required canonical anchors and audit columns;
- the recent-item registration audit per side-by-side relation, so
  reviewers can detect when a channel or build ownership change would
  strand users on the wrong recent-item list, the wrong file
  association, or the wrong deep-link handler;
- the summary-only OS entry rules for dock, taskbar, jump list, and
  notification quick actions, so a privileged or mutating action is
  never invoked from a summary-only OS surface;
- the exact-target reopen contract for every claimed desktop profile,
  so notification click-through, dock / taskbar reopen, recent-item
  reopen, removable-volume return, and protocol-handler entry resolve
  to the durable object via `exact_reopen_linkage` rather than
  reopening a generic shell;
- the replay-deny matrix that binds every `replay_posture` value in
  the `replay_denied_*` family to its required `policy_resolution_class`
  (`denied_replay`), `fallback_class`, `degraded_reason`, and
  `audit_event_id`;
- the conformance assertions a release / support / accessibility
  reviewer applies before any change to a system affordance is
  considered conforming.

## 2. Out of scope

- OS-registration code, installer behavior, notification daemons,
  Launch Services adapters, Windows toast pipelines, Linux
  desktop-file generation, or shell-recent integration. This packet
  is review evidence; OS-adapter code lands in the eventual platform
  -integration crate.
- Final user-facing microcopy. This audit pins fields, postures, and
  audit columns; product writing chooses final strings inside those
  limits.
- New `source_surface_class`, new `route_class`, new
  `requested_action_class`, new `authority_delta_class`, new
  `replay_posture`, new `fallback_class`, new `handler_ownership_class`,
  new `case_class`, new `outcome_class`, or new `audit_event_id`
  values. Every column in this audit is re-exported; widening requires
  a decision row in the owning schema.

## 3. Re-exported vocabularies

Each list below is re-exported verbatim from the upstream schema or
ledger. If a value diverges, this audit MUST update in the same change
as the upstream.

### 3.1 Source surface classes

Re-exported from
[`schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json)
`#/$defs/source_surface_class`.

`system_open`, `file_association`, `open_with`,
`reveal_in_system_shell`, `native_open_dialog`, `native_save_dialog`,
`dock_taskbar_recent`, `dock_taskbar_jump_action`,
`os_notification_click`, `os_badge_activation`,
`system_share_target`, `copy_path_or_permalink`, `drag_drop_open`,
`open_from_terminal`, `default_browser_callback`, `protocol_handler`,
`companion_handoff_return`.

### 3.2 Route classes

Re-exported from
[`schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json)
`#/$defs/route_class`.

`local_file_open`, `workspace_open`, `review_or_work_item`,
`auth_callback`, `collaboration_session_join`,
`managed_workspace_resume`, `command_invocation`,
`external_browser_return`, `settings_or_policy_review`,
`support_or_incident`, `provider_console_handoff`,
`unavailable_target_recovery`.

### 3.3 Requested-action classes

Re-exported from
[`schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json)
`#/$defs/requested_action_class`.

`inspect_only`, `reveal_only`, `open_existing_context`,
`create_or_add_context`, `join_presence`, `resume_session`,
`auth_return`, `retry_or_reconnect`, `acknowledge_notification`,
`mutating_command_request`, `privileged_authority_widening`.

### 3.4 Authority-delta classes

`none`, `trust_boundary_crossing`, `policy_boundary_crossing`,
`auth_scope_widening`, `remote_authority_rebind`,
`collaboration_presence_widening`, `external_visibility_widening`,
`destructive_or_mutating`, `unknown_requires_review`. Every non-`none`
value requires review before execution.

### 3.5 Replay-posture vocabulary

`single_use`, `bounded_reuse`, `read_only_resumable`,
`replay_denied_consumed`, `replay_denied_expired`,
`replay_denied_policy_epoch_changed`,
`replay_denied_target_drifted`, `replay_denied_origin_mismatch`.

Every `replay_denied_*` value MUST resolve to
`policy_resolution_class = denied_replay` and one of the closed
fallback classes in §3.6.

### 3.6 Fallback classes

`open_intent_review_sheet`, `open_read_only_placeholder`,
`open_cached_context`, `locate_missing_target`,
`continue_local_only`, `open_activity_center`,
`open_default_browser`, `deny_with_explanation`, `export_context`,
`no_fallback_available`. A row that selects `no_fallback_available`
MUST also select `deny_with_explanation` semantics — silent dismissal
is non-conforming.

### 3.7 Handler-ownership classes and review state

Handler-ownership classes (re-exported):
`machine_global_registered`, `current_user_registered`,
`portable_local_only`, `managed_policy_owned`,
`no_handler_ownership`, `conflict_unknown_owner`.

Ownership-change review states (re-exported): `no_change`,
`preview_required_before_change`, `blocked_by_policy`,
`accepted_after_review`, `denied_conflict`.

### 3.8 Audit-event ids

`platform.deep_link_intent_received`,
`platform.deep_link_intent_admitted`,
`platform.deep_link_intent_review_required`,
`platform.deep_link_intent_denied`,
`platform.system_affordance_case_exercised`,
`platform.handler_ownership_reviewed`,
`platform.notification_clickthrough_resolved`,
`platform.lifecycle_recovery_preserved_context`.

## 4. Route-audit rows

Every `source_surface_class` row below pins:

- the canonical command-ID family the surface MUST resolve to (a
  `cmd:*` ref into the eventual command registry);
- the canonical object identity, event lineage, or browser-handoff
  ref the surface MUST cite;
- the default `route_class`, `requested_action_class`, and
  `authority_delta_class`;
- the replay posture admitted by default and the replay-deny posture
  reviewers compare expired / drifted / origin-mismatched intents
  against;
- the default `fallback_class` for missing, moved, blocked, or
  policy-narrowed targets;
- the `audit_event_id` that closes the trail;
- the upstream fixture refs in
  [`/fixtures/platform/system_affordance_cases/`](../../fixtures/platform/system_affordance_cases/)
  and
  [`/fixtures/platform/deep_link_replay_cases/`](../../fixtures/platform/deep_link_replay_cases/)
  that exercise the row.

A row missing any column is non-conforming. Tooling reads this table
top-to-bottom; new rows require a new `source_surface_class` value in
the upstream schema.

| `source_surface_class` | Canonical command-ID family | Canonical anchor (besides `command_id_ref`) | Default `route_class` | Default `requested_action_class` | Default `authority_delta_class` | Default admitted `replay_posture` | Default `fallback_class` when target degrades | Default `audit_event_id` | Upstream fixture refs |
|---|---|---|---|---|---|---|---|---|---|
| `system_open` | `cmd:workspace.open_folder`, `cmd:workspace.open_file`, `cmd:start_center.open_recent` | `object_identity_ref` (workspace manifest, file, or recent-work entry) | `local_file_open` or `workspace_open` | `open_existing_context` | `none` (or `trust_boundary_crossing` when target is outside trusted scope) | `single_use` | `open_intent_review_sheet` | `platform.system_affordance_case_exercised` | `file_association_workspace_review` |
| `file_association` | `cmd:workspace.open_folder`, `cmd:workspace.open_file` | `object_identity_ref` (workspace manifest or file), handler-owner channel | `workspace_open` or `local_file_open` | `open_existing_context` | `trust_boundary_crossing` until trust resolved | `single_use` | `open_intent_review_sheet` | `platform.system_affordance_case_exercised` | `file_association_workspace_review`, `deep_link_replay_cases/workspace_open_replay_denied_expired` |
| `open_with` | `cmd:file.open_with` | `object_identity_ref` (file or generated artifact), canonical-vs-presentation path | `local_file_open` | `open_existing_context` or `inspect_only` | `none` | `single_use` | `open_read_only_placeholder` | `platform.system_affordance_case_exercised` | `file_association_workspace_review` |
| `reveal_in_system_shell` | `cmd:shell.reveal_path` | `object_identity_ref` (canonical path token, never raw absolute path) | `local_file_open` | `reveal_only` | `none` | `read_only_resumable` | `locate_missing_target` | `platform.system_affordance_case_exercised` | `removable_volume_return_cached_context` |
| `native_open_dialog` | `cmd:workspace.open_folder`, `cmd:workspace.open_file`, `cmd:vfs.import_root` | `object_identity_ref` (selected target identity), VFS-token | `local_file_open` or `workspace_open` | `open_existing_context` or `create_or_add_context` | `trust_boundary_crossing` for import | `single_use` | `open_intent_review_sheet` | `platform.system_affordance_case_exercised` | `native_save_dialog_read_only_boundary` |
| `native_save_dialog` | `cmd:vfs.save_as`, `cmd:vfs.checkpoint` | `object_identity_ref` (save-target token), write-posture | `command_invocation` | `mutating_command_request` | `destructive_or_mutating` | `single_use` | `open_intent_review_sheet` | `platform.system_affordance_case_exercised` | `native_save_dialog_read_only_boundary` |
| `dock_taskbar_recent` | `cmd:start_center.open_recent`, `cmd:workspace.open_folder` | `object_identity_ref` (recent-work entry or workspace manifest), owning-channel ref | `workspace_open` | `open_existing_context` | `none` (channel ownership change widens to `trust_boundary_crossing`) | `single_use` | `open_intent_review_sheet` or `locate_missing_target` | `platform.system_affordance_case_exercised` | `removable_volume_return_cached_context`, `deep_link_replay_cases/dock_recent_replay_denied_target_drifted` |
| `dock_taskbar_jump_action` | `cmd:start_center.new_workspace`, `cmd:start_center.open_recent`, `cmd:activity.open_event` | `object_identity_ref` (durable object or activity-center event), owning-channel ref | `workspace_open` or `command_invocation` | `inspect_only` or `open_existing_context` | `none` | `single_use` | `open_activity_center` | `platform.system_affordance_case_exercised` | `removable_volume_return_cached_context` |
| `os_notification_click` | `cmd:review.open_thread`, `cmd:activity.open_event`, `cmd:run.open_run`, `cmd:trust.review_change` | `canonical_event_id_ref` and `event_lineage_ref` (notification-delivery contract) | `review_or_work_item` or `command_invocation` | `open_existing_context` or `acknowledge_notification` | `none` for inspect; `trust_boundary_crossing` for trust-change events | `single_use` | `open_activity_center` | `platform.notification_clickthrough_resolved` | `notification_clickthrough_lock_screen_privacy` |
| `os_badge_activation` | `cmd:activity.open_event`, `cmd:start_center.open` | `event_lineage_ref` and durable-object set; badge counts derive from deduped durable objects | `review_or_work_item` or `unavailable_target_recovery` | `inspect_only` | `none` | `read_only_resumable` | `open_activity_center` | `platform.notification_clickthrough_resolved` | `badge_presence_traceable_count` |
| `system_share_target` | `cmd:share.export_object`, `cmd:share.copy_permalink` | `object_identity_ref` (source object), audience / visibility class | `command_invocation` | `mutating_command_request` | `external_visibility_widening` | `single_use` | `open_intent_review_sheet` | `platform.system_affordance_case_exercised` | `system_share_copy_permalink_review` |
| `copy_path_or_permalink` | `cmd:share.copy_path`, `cmd:share.copy_permalink` | `object_identity_ref` (source object), representation class | `command_invocation` | `inspect_only` | `none` (permalink) or `external_visibility_widening` (publishing scope) | `single_use` | `deny_with_explanation` | `platform.system_affordance_case_exercised` | `system_share_copy_permalink_review` |
| `drag_drop_open` | `cmd:workspace.open_file`, `cmd:vfs.import_root` | `object_identity_ref` (target file or root), drag-drop result verb | `local_file_open` or `workspace_open` | `create_or_add_context` | `trust_boundary_crossing` for import | `single_use` | `open_intent_review_sheet` | `platform.system_affordance_case_exercised` | `file_association_workspace_review` |
| `open_from_terminal` | `cmd:workspace.open_folder`, `cmd:workspace.open_file` | `object_identity_ref` (resolved target), source-process class, cwd token | `local_file_open` or `workspace_open` | `open_existing_context` | `trust_boundary_crossing` until trust resolves | `single_use` | `open_intent_review_sheet` | `platform.system_affordance_case_exercised` | `open_from_terminal_wake_revalidation` |
| `default_browser_callback` | `cmd:auth.complete_callback`, `cmd:browser.return_object` | `browser_handoff_packet_ref` plus `canonical_event_id_ref` | `external_browser_return` or `auth_callback` | `auth_return` or `open_existing_context` | `auth_scope_widening` for auth; `none` for object return | `single_use` | `open_intent_review_sheet` | `platform.deep_link_intent_admitted` | `deep_link_replay_cases/auth_callback_replay_denied_consumed`, `deep_link_replay_cases/browser_return_replay_denied_origin_mismatch` |
| `protocol_handler` | `cmd:review.open_thread`, `cmd:workspace.open_folder`, `cmd:command.invoke`, `cmd:auth.complete_callback` | `object_identity_ref` plus `canonical_event_id_ref`; handler-owner channel | Any of `review_or_work_item`, `workspace_open`, `command_invocation`, `auth_callback`, `provider_console_handoff` | varies | varies (`none` for inspect; `auth_scope_widening`, `remote_authority_rebind`, `destructive_or_mutating`, or `privileged_authority_widening` for review-bearing intents) | `single_use` | `open_intent_review_sheet` | `platform.deep_link_intent_admitted` or `platform.deep_link_intent_denied` | `deep_link_remote_review_replay_denied`, `deep_link_replay_cases/managed_resume_replay_denied_policy_epoch_changed`, `deep_link_replay_cases/review_link_replay_denied_target_drifted`, `deep_link_replay_cases/command_invocation_widened_authority_denied` |
| `companion_handoff_return` | `cmd:companion.return_object`, `cmd:auth.complete_callback` | `browser_handoff_packet_ref` plus `event_lineage_ref` | `external_browser_return` or `auth_callback` | `auth_return` or `open_existing_context` | `auth_scope_widening` or `none` | `single_use` | `open_intent_review_sheet` | `platform.deep_link_intent_admitted` | `deep_link_replay_cases/browser_return_replay_denied_origin_mismatch` |

## 5. Recent-item, dock, taskbar, and jump-list reopen audit

Every claimed desktop profile in
[`/artifacts/platform/claimed_desktop_profiles.yaml`](./claimed_desktop_profiles.yaml)
MUST satisfy every row below. A profile that cannot fill a row records
`not_applicable` with a typed reason rather than omitting it. Side-by
-side relation rows are taken verbatim from
[`/artifacts/release/channel_ownership_audit.yaml`](../release/channel_ownership_audit.yaml).

| Audit axis | Required rule | Forbidden behavior |
|---|---|---|
| Recent-item registration ownership | One owning channel per recent-item list; recent-item entries carry channel/build owner refs and resolve to a durable workspace, file, or recent-work entry. Side-by-side hosts render two recent-item lists, never one merged list. | Merging stable and preview recent items into one OS list; portable mode writing into any machine-global recent-item list; reopening a recent item without the owning-channel ref. |
| Recent-item reopen target binding | `dock_taskbar_recent` reopen MUST resolve to the durable `object_identity_ref` (workspace manifest, file, or recent-work entry) admitted by the owning channel; cross-channel recent items reopen through `open_intent_review_sheet`. | Reopening a stable recent-item entry under preview without review; falling back to a generic Start Center home screen as "reopen". |
| Dock / taskbar jump action ownership | Jump actions are summary-only OS surfaces; they MAY invoke `cmd:start_center.new_workspace`, `cmd:start_center.open_recent`, `cmd:activity.open_event`, or `cmd:settings.open`, but only those. | Triggering a mutating command, a privileged step-up, a remote write, or a collaboration role change directly from a jump action. |
| Notification quick-action bounds | Notification actions are bounded to inspect, open, acknowledge, snooze, retry attach, or open handoff; consequence-bearing commands MUST land on an in-product review surface first. | Marking a review approved, paused, dismissed for the team, or applied from the notification action without opening the in-product review surface. |
| Exact-target reopen rule per profile | Every reopen surface advertises one closed `expected_state_token` from [`/artifacts/platform/native_lifecycle_drill_packet.md`](./native_lifecycle_drill_packet.md) §3 (e.g. `Reopen required`, `Root unavailable`, `Local fallback`) when exact open is unavailable. | Reopening a removable-volume-backed workspace by silently retargeting another mount; reopening an expired callback by silently reauth-ing; reopening a missing target by silently opening Start Center home. |
| Channel-ownership change disclosure | When dock / taskbar / jump-list / recent-item registration would change owning channel or build, the surface advertises `ownership_change_review_state = preview_required_before_change` (or `blocked_by_policy` under managed policy). | Last-writer-wins ownership change; silently re-binding aureline:// to a different channel; promoting a preview build to default file handler without a review sheet. |
| Portable-mode suppression | Portable rows MUST register `recent_item_registration_class = per_channel_under_portable_root` only and MUST NOT register file associations, protocol handlers, dock entries, or notification badges into machine-global stores; this matches the portable forbidden-host-mutation rows in [`/artifacts/release/portable_mode_limitations.yaml`](../release/portable_mode_limitations.yaml). | Portable mode writing into a machine-global recent-items list, registering a default file handler, claiming aureline://, or installing a system service. |
| Side-by-side ownership row coverage | Every side-by-side relation in [`/artifacts/release/channel_ownership_audit.yaml`](../release/channel_ownership_audit.yaml) (`stable_and_preview`, `stable_and_beta`, `stable_and_lts`, `preview_and_beta`, `installed_and_portable`, `three_channel_matrix`, `managed_and_portable`) MUST have at least one route-audit row showing recent-item, file-association, and protocol-handler ownership for the pair. | Auditing only the installed channel and ignoring the side-by-side neighbor; conflating per-channel owned axes with shared-under-user-or-admin-choice axes. |

## 6. Replay-deny matrix

Every `replay_denied_*` posture MUST close with the row below. A
deep-link or browser-return token that takes any other path is
non-conforming. The matrix is the contract reviewers compare expired,
widened, or drifted intents against and the upstream contract for the
fixtures under
[`/fixtures/platform/deep_link_replay_cases/`](../../fixtures/platform/deep_link_replay_cases/).

| `replay_posture` | Required `policy_resolution_class` | Required `degraded_reason` | Required `audit_event_id` | Required `fallback_class` posture | Worked fixture |
|---|---|---|---|---|---|
| `replay_denied_consumed` | `denied_replay` | `replay_denied` | `platform.deep_link_intent_denied` | `open_cached_context` or `deny_with_explanation`; `preserves_user_intent` MUST be `true` | [`deep_link_remote_review_replay_denied`](../../fixtures/platform/system_affordance_cases/deep_link_remote_review_replay_denied.json), [`auth_callback_replay_denied_consumed`](../../fixtures/platform/deep_link_replay_cases/auth_callback_replay_denied_consumed.yaml) |
| `replay_denied_expired` | `denied_replay` | `replay_denied` (and `expired_session` when the session also lapsed) | `platform.deep_link_intent_denied` | `open_intent_review_sheet` or `deny_with_explanation`; reopen routes through revalidation, never silent reauth | [`workspace_open_replay_denied_expired`](../../fixtures/platform/deep_link_replay_cases/workspace_open_replay_denied_expired.yaml) |
| `replay_denied_policy_epoch_changed` | `denied_replay` | `replay_denied` | `platform.deep_link_intent_denied` | `open_intent_review_sheet` (review under the new policy epoch) or `deny_with_explanation` | [`managed_resume_replay_denied_policy_epoch_changed`](../../fixtures/platform/deep_link_replay_cases/managed_resume_replay_denied_policy_epoch_changed.yaml) |
| `replay_denied_target_drifted` | `denied_replay` | `replay_denied` (and `target_moved` or `target_ambiguous` when the target also drifted) | `platform.deep_link_intent_denied` | `open_cached_context` or `locate_missing_target`; `preserves_user_intent` MUST be `true` | [`review_link_replay_denied_target_drifted`](../../fixtures/platform/deep_link_replay_cases/review_link_replay_denied_target_drifted.yaml), [`dock_recent_replay_denied_target_drifted`](../../fixtures/platform/deep_link_replay_cases/dock_recent_replay_denied_target_drifted.yaml) |
| `replay_denied_origin_mismatch` | `denied_replay` | `replay_denied` (and `origin_untrusted` when origin class is below the route's required trust) | `platform.deep_link_intent_denied` | `deny_with_explanation`; replay-deny rows with origin mismatch MUST NOT silently fall back to a permissive route | [`browser_return_replay_denied_origin_mismatch`](../../fixtures/platform/deep_link_replay_cases/browser_return_replay_denied_origin_mismatch.yaml), [`command_invocation_widened_authority_denied`](../../fixtures/platform/deep_link_replay_cases/command_invocation_widened_authority_denied.yaml) |

## 7. Conformance assertions

A change touching an OS-level entry path is conforming only if EVERY
assertion below holds. Tooling and reviewers compare against this
list; new assertions require a decision row.

1. **One canonical anchor per OS entry.** Every fixture cites at
   least one of `command_id_ref`, `canonical_event_id_ref`,
   `event_lineage_ref`, or `object_identity_ref`. A row that says
   only "open this URL" or "show this path" is non-conforming.
2. **No authority widening from the OS.** Every row carries
   `expected_behavior.no_authority_widening = true`. An OS entry path
   never grants more trust, policy authority, remote authority,
   collaboration presence, or target certainty than the in-product
   object carries.
3. **Replay denied by default.** Every external authority-bearing
   intent defaults to `replay_posture = single_use`;
   `read_only_resumable` is admitted only for `inspect_only` or
   `reveal_only` actions. Every `replay_denied_*` posture closes with
   `policy_resolution_class = denied_replay` and one of the
   `audit_event_id` values in §6.
4. **Boundary changes route through review.** Any row whose
   `authority_delta_class` is non-`none` advertises
   `trust_review_requirement` other than `no_review_required` and a
   `fallback` whose `preserves_user_intent` is `true` (or
   `deny_with_explanation` semantics).
5. **Channel-ownership change is reviewable.** Side-by-side relations
   from [`/artifacts/release/channel_ownership_audit.yaml`](../release/channel_ownership_audit.yaml)
   that would change the owning channel or build for file
   associations, recent-item registrations, protocol-handler defaults,
   or notification badge ownership advertise
   `ownership_change_review_state = preview_required_before_change`
   (or `blocked_by_policy` under managed policy). Last-writer-wins
   ownership change is non-conforming.
6. **Exact-target reopen, not generic reopen.** Notification
   click-through, dock / taskbar reopen, recent-item reopen, removable
   -volume return, and protocol-handler entry resolve to the durable
   object via `exact_reopen_linkage`. Reopening a generic Start Center
   home, an empty workspace shell, or a placeholder without
   `preserves_user_intent = true` is non-conforming.
7. **Summary-only OS surfaces stay inspect-only.** Dock / taskbar
   jump actions, notification quick actions, and OS-badge activations
   never invoke a mutating, privileged, remote, or policy-bearing
   command directly. Such commands route through an in-product
   review surface (`open_intent_review_sheet` fallback).
8. **Portable mode never mutates host shell.** Portable rows record
   `handler_ownership.ownership_class = portable_local_only`, suppress
   file-association registration, protocol-handler claims, dock
   entries, and machine-global recent-item writes per
   [`/artifacts/release/portable_mode_limitations.yaml`](../release/portable_mode_limitations.yaml).
9. **Lock-screen privacy preserved.** Lock-screen, companion, and
   OS-notification surfaces use `lock_screen_safe_generic`,
   `metadata_safe_default`, `no_lock_screen_delivery`, or
   `support_export_metadata_only` per the privacy projection on the
   row. Raw paths, raw URLs, raw callback bodies, raw secrets, raw
   prompt text, raw customer-owned identifiers, tenant names, and
   review titles are not exposed on the lock screen.
10. **Audit-event closure.** Every fixture row carries one
    `audit_event_id` from §3.8. A row that elides the audit event id
    is non-conforming.
11. **Reviewer can detect a stranding ownership change.** A reviewer
    reading
    [`/artifacts/platform/file_association_ownership_matrix.yaml`](./file_association_ownership_matrix.yaml)
    plus
    [`/artifacts/release/channel_ownership_audit.yaml`](../release/channel_ownership_audit.yaml)
    can answer, for any channel/build ownership change: which
    extension class, recent-item list, protocol scheme, and
    notification badge would land on the wrong handler, what review
    surface preserves user intent, and which audit event id closes
    the trail.

## 8. Reviewer checklist

A change touching an OS-level entry path is conforming only if a
reviewer can answer:

1. Which `source_surface_class` row from §4 covers the change, which
   canonical command ID and object identity back the surface, and
   which fixture under
   [`/fixtures/platform/system_affordance_cases/`](../../fixtures/platform/system_affordance_cases/)
   or
   [`/fixtures/platform/deep_link_replay_cases/`](../../fixtures/platform/deep_link_replay_cases/)
   exercises the row?
2. What `route_class`, `requested_action_class`, and
   `authority_delta_class` does the surface mint, and where is the
   review sheet that names origin, target identity, command ID,
   handler owner, replay posture, and fallback before execution?
3. What happens when the target is missing, moved, blocked, stale,
   or privacy-narrowed — which `fallback_class` preserves user intent,
   and does it route to a review surface, cached context, locate
   action, or denial with explanation rather than a generic shell?
4. What is the owning channel/build for the file association, recent
   item, protocol handler, or notification badge, and which side-by
   -side relation row in
   [`/artifacts/release/channel_ownership_audit.yaml`](../release/channel_ownership_audit.yaml)
   covers the change? If ownership changes, what review state does the
   surface advertise?
5. Which replay-deny row in §6 covers expired, widened, drifted,
   policy-epoch-changed, consumed, or origin-mismatched intents, and
   which fixture in
   [`/fixtures/platform/deep_link_replay_cases/`](../../fixtures/platform/deep_link_replay_cases/)
   proves the surface fails closed?
6. Does the OS notification, lock-screen summary, dock / taskbar
   summary, system share target, copied path, or permalink imply more
   authority, presence, or certainty than the in-product source object
   carries — and does the privacy projection cover in-product, OS
   notification, lock-screen, and support-export surfaces?
7. Does wake, display reconnect, DPI / topology change, removable
   -volume return, or expired session preserve context without
   destructive cleanup, hidden focus stealing, or silent rerun, per
   [`/artifacts/platform/native_lifecycle_drill_packet.md`](./native_lifecycle_drill_packet.md)?
