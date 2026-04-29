# Cross-Window Transfer Contract

This document freezes the UX contract for moving, copying, opening, and
restoring work across top-level Aureline windows. It exists so tabs,
editor groups, compare views, inspectors, review surfaces,
presentation windows, and shared-session panes use one visible transfer
model instead of drifting into surface-local drag/drop behavior.

The contract is normative. Where it disagrees with the PRD, technical
architecture, technical design, UI/UX spec, window-display contract,
tab/editor-group contract, drag/drop contract, shell close/reopen
contract, or workspace layout contract, the upstream source wins and
this document plus its schema and fixtures update in the same change.
Where a shell surface, extension view, platform adapter, or command
route mints private cross-window transfer semantics that conflict with
this document, this document wins and that surface is non-conforming.

Companion artifacts:

- [`/schemas/ux/window_transfer_action.schema.json`](../../schemas/ux/window_transfer_action.schema.json)
  is the boundary schema for `window_transfer_action_record`,
  `secondary_window_continuation_record`,
  `window_transfer_restore_record`, and
  `cross_window_transfer_case_record`.
- [`/fixtures/ux/cross_window_transfer_cases/`](../../fixtures/ux/cross_window_transfer_cases/)
  contains worked cases for move, copy, create-window, crash reopen,
  and degraded fallback when transfer prerequisites disappear.

This contract composes with:

- [`/docs/ux/window_display_contract.md`](./window_display_contract.md)
  for window roles, display-topology restore, safe bounds, native
  control projection, restore history, and secondary-window continuity.
- [`/docs/ux/tabs_editor_groups_contract.md`](./tabs_editor_groups_contract.md)
  for tab, pane, group, dirty-state, compare, shared/followed, and
  restored-placeholder identity.
- [`/docs/ux/clipboard_history_contract.md`](./clipboard_history_contract.md)
  for drag/drop result verbs, insertion preview, modifier cues,
  lineage, undo class, and large-transfer feedback.
- [`/docs/ux/shell_close_reopen_contract.md`](./shell_close_reopen_contract.md)
  for close, collapse, reopen, placeholder, and focus-return rules.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  for skeleton-first restore, pane-tree identity, no-rerun behavior,
  and missing-dependency placeholders.
- [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md)
  and
  [`/docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md)
  for shared-session control and presenter handoff authority.

## Who Reads This Contract

- Shell authors implementing drag/drop, tear-off, dock, secondary
  window, close, and restore routes.
- Editor, compare, inspector, review, notebook, terminal, runtime, and
  collaboration owners deciding which state can move, copy, degrade, or
  remain in place.
- Command, palette, menu, and accessibility authors ensuring every
  transfer has a command-backed route and a keyboard-reachable preview.
- QA, support, diagnostics, and release tooling that need one
  inspectable packet for transfer outcome, continuity, restore truth,
  and orphan prevention.

## 1. Scope

This contract freezes:

- the cross-window action classes for `move_tab`, `copy_editor`,
  `open_compare_here`, `move_inspector`, and `create_window`;
- how a pre-drop verb preview names the resulting action before the
  window layout mutates;
- how command palette, menu, context menu, and keyboard routes mirror
  the same action when drag/drop is unavailable or refused;
- which workspace, trust, host or remote, profile, collaboration, and
  recovery cues must remain visible after a transfer;
- close/collapse continuation rules for secondary windows that contain
  dirty buffers, active approvals, shared control, evidence review, or
  pinned restore state;
- restore and degraded fallback behavior when a specialized window
  reopens after crash, monitor change, missing extension host, missing
  feature pack, or missing remote target.

Out of scope:

- final drag/drop rendering polish, cursor art, animation, or
  OS-specific window-manager quirks beyond the shared contract;
- a production implementation of platform-native drag/drop;
- new undo, approval, collaboration, restore, or command vocabularies.

## 2. Non-Negotiable Invariants

1. **The verb is visible before drop.** A cross-window drop may not
   mutate layout, move ownership, or create a window until the target
   renders the resulting action class, target window, source posture,
   consequence, and command fallback.
2. **Workspace authority is not forked.** Transfer changes window
   topology or view identity. It never clones dirty-buffer authority,
   save tokens, trust state, remote grants, active approvals, or shared
   control.
3. **Move and copy are different packets.** A move preserves the
   object/view identity and removes it from the origin. A copy mints a
   new view identity pointing at the same canonical object and leaves
   the origin intact.
4. **Every drag path has a command path.** If drag/drop is unavailable,
   blocked, or not keyboard-accessible, the same action class remains
   reachable through a command-backed route with the same preview and
   disabled reason.
5. **Secondary windows cannot close into data loss.** Closing,
   collapsing, or restoring a secondary window either transfers
   critical state to a visible owner or records an explicit
   continuation the user can inspect and resume.
6. **Restore is honest about missing prerequisites.** Crash reopen,
   topology drift, missing extension hosts, missing remote targets, and
   unsupported feature packs may preserve layout and evidence, but they
   must not pretend live authority or specialized UI survived.
7. **Focus remains explainable.** A transfer may activate the target
   window only through a shell-approved route. The source focus anchor,
   target focus owner, and any fallback recovery card stay inspectable.

## 3. Frozen Vocabulary

The schema exports four record kinds:

| Record kind | Purpose |
|---|---|
| `window_transfer_action_record` | One preview, command fallback, commit, or denial for a transfer action. |
| `secondary_window_continuation_record` | One close/collapse review for a secondary window with state that must not become orphaned. |
| `window_transfer_restore_record` | One reopen or restore event for a transferred or specialized window. |
| `cross_window_transfer_case_record` | Reviewable fixture shape binding action, continuity, close, restore, and acceptance assertions. |

### 3.1 Transfer Action Classes

| Action class | User-visible intent | Source after commit | Target after commit |
|---|---|---|---|
| `move_tab` | Move an existing tab or editor pane to another window. | Source tab/group membership is removed; buffer authority remains shared. | Existing tab/pane identity appears in the target window. |
| `copy_editor` | Open a second editor view of the same canonical object. | Source tab remains unchanged. | New tab/view identity points at the same object and authority refs. |
| `open_compare_here` | Open a compare/review surface in the target window. | Source object and selected basis remain unchanged. | Target receives a compare surface with role labels and fallback mode. |
| `move_inspector` | Move or re-dock an inspector for the same target object. | Source inspector slot closes or becomes a placeholder with reopen route. | Target inspector owns focus and shows the same target/trust state. |
| `create_window` | Create a new secondary, auxiliary, presentation, or review window from the current drag/command context. | Source action-specific posture is preserved or removed according to the resolved subaction. | New window joins the same window family and inherits continuity cues. |

The lower-level window-display contract keeps broader topology values
such as `dock_window`, `collapse_to_primary`, and
`promote_to_presentation`. A cross-window transfer surface may use
those values internally only when it also emits one of the action
classes above at the user-facing boundary.

### 3.2 Transfer Phases

`transfer_phase` values:

- `drag_enter_preview`
- `drag_update_preview`
- `drop_committed`
- `drop_denied`
- `command_fallback_preview`
- `command_fallback_committed`
- `restore_reopened`
- `secondary_close_reviewed`

Rules:

- `drag_enter_preview`, `drag_update_preview`, and
  `command_fallback_preview` carry the same required preview fields.
- `drop_committed` and `command_fallback_committed` cite the preview
  packet they committed against.
- `drop_denied` cites a disabled reason and leaves source and target
  topology unchanged.
- Restore and close records cite the transfer action or window family
  they are continuing rather than inventing a separate lineage.

### 3.3 Pre-Drop Verb Preview

Every pre-drop preview must render and export:

- transfer action class and display verb;
- source window, target window, target slot, and whether a new window
  will be created;
- source-after-drop posture: source retained, source removed,
  source placeholder, or source unchanged because the drop is denied;
- target-after-drop posture: existing identity moved, new view created,
  compare opened, inspector docked, or window created;
- consequence class and recovery class from the interaction-safety and
  undo/recovery contracts;
- modifier-key resolution when the action originated from drag/drop;
- command fallback id and disabled reason, if any;
- continuity cues that will remain visible after commit;
- orphan-prevention consequences for dirty buffers, approvals, shared
  control, evidence review, pinned restore state, and active prompts.

A preview that cannot show the action class, target identity,
consequence class, recovery class, and command fallback denies with a
visible disabled reason instead of committing.

### 3.4 Command-Backed Fallbacks

Every action class has at least one command-backed route:

| Action class | Required command route |
|---|---|
| `move_tab` | move active tab or selected tabs to a named window/group. |
| `copy_editor` | open a second editor view in a named window/group. |
| `open_compare_here` | open compare in the current, selected, or new window. |
| `move_inspector` | move inspector to left/right/current/new window target. |
| `create_window` | create secondary, auxiliary, presentation, or review window from the selected context. |

Rules:

- Command routes use the same preview packet and denial vocabulary as
  drag/drop.
- A route disabled in drag/drop is disabled in command surfaces for the
  same reason unless the command path can show the missing preview
  fields safely.
- Keyboard-only users can choose source, target window, and target slot
  without pointer-only state.
- Context menus, window menus, palette rows, and shortcuts cite the
  same command id. Labels may vary by locale; command identity does not.

## 4. Transfer Semantics

### 4.1 Move Tab

Moving a tab changes only window topology:

- `tab_id`, `pane_id_ref`, `stable_content_ref`, dirty authority,
  save target, read-only state, generated state, and restore posture
  remain unchanged.
- Source overflow, focus return, and reopen history record that the tab
  intentionally moved rather than closed.
- Target window shows workspace identity, trust state, host or remote
  state, profile, and dirty/recovery cues before the moved tab becomes
  active.
- If the target window cannot show required cues due to width or
  missing dependencies, the move denies or uses the command fallback
  to choose a safer target slot.

### 4.2 Copy Editor

Copying an editor creates a second view identity over the same
canonical object:

- A new `tab_id` or view id is minted for the target view.
- Buffer text, dirty state, save tokens, and undo authority remain
  owned by the workspace/buffer authority.
- Per-window cursor, scroll, selection, preview, and focus state may
  diverge and remain window-local.
- The copied view must not look like an independent file clone or a
  separate dirty buffer.

### 4.3 Open Compare Here

Opening compare in another window is a presentation transfer:

- The compare basis, source/target/base/result roles, and freshness
  class are visible before commit.
- The target window may choose split compare, tabbed compare, staged
  peek, explicit choice, or deny-until-resize according to the
  tab/editor-group contract.
- Source and target role labels remain textual or structural, not
  icon-only.
- If a required source object, snapshot, generated artifact, remote
  target, or extension provider disappears before commit, the action
  denies or opens an evidence-only placeholder with restore history.

### 4.4 Move Inspector

Moving an inspector transfers contextual focus without moving
workspace authority:

- The inspector target object, trust state, provider/ref freshness, and
  disabled reasons follow the inspector into the target window.
- The source slot records whether it closed intentionally, collapsed to
  a placeholder, or remains available through reopen-last-inspector.
- Active prompts and destructive confirmations owned by the inspector
  remain attached to the owning window or move only through an explicit
  continuation.
- A missing provider produces an attributed placeholder instead of an
  ownerless blank inspector.

### 4.5 Create Window

Creating a window is both a transfer and a topology mutation:

- The new window receives a fresh `window_id_ref`, joins the same
  `window_family_ref`, and carries the same
  `workspace_authority_ref`.
- The chosen `window_role` is explicit: `secondary_workspace`,
  `auxiliary`, `presentation`, or `review`.
- Workspace identity, trust state, host or remote state, profile,
  collaboration/follow/presentation role, and recovery cues are visible
  in the new window before the source state changes.
- If creation fails after preview but before commit, source topology
  remains valid and focus returns to the invoker.

## 5. Continuity Rules

Every successful transfer preserves or explicitly rebinds:

| Cue | Rule |
|---|---|
| Workspace identity | `workspace_authority_ref` remains the same unless an explicit workspace-switch command is used. |
| Trust state | Restricted, untrusted, policy-blocked, or trusted posture follows every affected window. |
| Host or remote state | Local, remote, managed, disconnected, reconnecting, read-only, and expired authority states stay visible. |
| Profile | Profile and policy epoch remain visible and never silently change because of a window move. |
| Recovery-critical cues | Dirty buffers, recovered drafts, restore fidelity, missing dependencies, and evidence-only state remain visible. |
| Collaboration/control | Shared, followed, presenting, observing, and driver/control grants remain separate from local window focus. |
| Command lineage | Drag/drop, command fallback, undo/reopen history, restore history, and support export cite the same action lineage. |

Rules:

- Moving work between windows never upgrades trust, admits a previously
  blocked command, reconnects a remote target, or reacquires shared
  control.
- A target window that cannot display required continuity cues must use
  a safer presentation, open an explicit choice surface, or deny.
- When a transfer spans local and remote contexts, the host/remote
  boundary label stays in the title/context, status, tab/inspector, or
  placeholder surface.
- Support export can reconstruct the source window, target window,
  action class, preview packet, continuity cues, restore history, and
  close continuation without raw paths, raw payloads, or capability
  tickets.

## 6. Secondary-Window Close And Orphan Prevention

Closing, collapsing, or restoring away a secondary window enters review
when it contains any critical state:

- dirty buffers, recovered drafts, save conflicts, or unsaved notebook
  changes;
- active approvals, permission prompts, trust prompts, or destructive
  confirmations;
- shared-session control, presenter/driver authority, or pending
  control transfer;
- long-running evidence review, support export, migration review,
  incident review, or compare review;
- active terminal/debug/notebook/runtime evidence that must not rerun;
- pinned restore state, crash-loop recovery, or placeholder surfaces
  with user-visible recovery actions.

The continuation choices are closed:

- `transfer_to_primary_window`
- `transfer_to_named_window`
- `keep_secondary_window_open`
- `pause_or_archive_review`
- `export_evidence_then_close`
- `discard_after_explicit_confirmation`
- `close_blocked_until_state_resolved`

Rules:

- A close review names every at-risk state class and the visible owner
  that will receive it.
- `discard_after_explicit_confirmation` is only valid for state whose
  owning contract allows discard; dirty buffers and active approvals
  require their own save/discard or deny path.
- Shared-session control cannot move to another window as active
  control unless the collaboration authority records an explicit
  grant/acceptance. Otherwise it continues as view-only or ends with an
  audit event.
- Long-running evidence review can close only after a durable route,
  export, pause/archive, or named destination owns the continuation.
- A secondary window that carries pinned restore state may close only
  after the restore card or provenance row is reachable from another
  visible window or support export.

## 7. Restore And Degraded Fallback

Transfer-related windows restore skeleton first and hydrate second.
Restore may recreate window topology, panes, tabs, inspectors, compare
roles, titles, scroll, selection, and evidence refs. It must not rerun
commands, reattach debuggers, restart notebook kernels, reopen shared
control, reuse expired approvals, or reconnect remote targets without a
fresh authority path.

`restore_trigger_class` values:

- `crash_reopen`
- `display_topology_changed`
- `monitor_unavailable`
- `extension_host_missing`
- `remote_target_missing`
- `feature_pack_missing`
- `schema_version_changed`
- `policy_or_trust_changed`
- `user_reopen_command`

`degraded_fallback_class` values:

- `no_degradation`
- `safe_bounds_repositioned`
- `collapse_to_primary_with_history`
- `same_slot_placeholder`
- `evidence_only_surface`
- `command_fallback_required`
- `manual_rebind_required`
- `restore_denied_until_prerequisite_returns`

Rules:

- A specialized window may collapse back to the primary workspace only
  with a restore-history event and a visible explanation.
- Missing extension hosts, remote targets, feature packs, and revoked
  authority reopen as placeholders or evidence-only surfaces in the
  original role/slot where possible.
- Monitor and safe-bounds changes preserve role, focus, and recovery
  actions before preserving stale coordinates.
- A restored compare/review window that lost its live provider keeps
  basis/evidence refs and role labels, but does not claim current live
  data.
- Restore summaries state what restored exactly, what degraded, what
  requires rebind, and which command can continue.

## 8. Accessibility, Focus, And Support Export

- The pre-drop preview, command fallback, disabled reason, continuation
  choices, and restore summary are keyboard reachable and announced.
- Pointer-only drag affordances are additive. They are not the only
  route to move, copy, compare, inspect, create, close, or continue.
- Focus returns to the source invoker when a transfer denies or fails.
  On success, focus lands on the target surface only when the shell
  explicitly activated that window.
- Support export includes the action class, source/target refs,
  preview packet ref, command fallback ref, continuity cues,
  orphan-prevention summary, restore/degraded fallback class, and
  acceptance assertions.
- Raw file bodies, raw absolute paths, raw terminal output, raw
  notebook output, raw URLs, raw credentials, approval-ticket bodies,
  and provider payloads do not cross this boundary.

## 9. Fixture And Evidence Rules

- Every fixture validates against
  [`/schemas/ux/window_transfer_action.schema.json`](../../schemas/ux/window_transfer_action.schema.json).
- The corpus covers at least one move, one copy, one create-window,
  one reopen-after-crash, and one degraded-fallback case.
- Every fixture states whether the result was visible before drop,
  which command fallback mirrors the action, which continuity cues stay
  visible, and which orphan-prevention states were resolved or absent.
- Fixtures that include terminal, debug, notebook, remote,
  collaboration, review, or evidence surfaces distinguish restorable
  context from live authority.
- Future release packets, support exports, and QA suites cite
  `transfer_action_class`, `transfer_phase`, `restore_trigger_class`,
  `degraded_fallback_class`, and `secondary_window_continuation_class`
  instead of inventing new names.

## 10. Acceptance Checklist

A fixture, implementation, or support-export packet conforms when:

1. The cross-window result is visible before drop or command commit.
2. Drag/drop and command-backed routes share action class, preview,
   denial reason, and lineage.
3. Move, copy, compare-open, inspector-move, and create-window
   semantics are distinct and inspectable.
4. Workspace identity, trust state, host/remote state, profile, and
   recovery-critical cues stay visible in every affected window.
5. Closing or collapsing a secondary window cannot orphan dirty
   buffers, approvals, prompts, shared control, evidence review, or
   pinned restore state.
6. Crash reopen and dependency drift preserve truthful layout/evidence
   without hidden rerun or silent authority reuse.
7. Restore/degraded fallback explains what changed and provides a
   command-backed continuation when continuation is possible.
8. Focus and accessibility paths are not pointer-only.
9. Support export can reconstruct the transfer without raw sensitive
   payloads or private platform handles.
