# Window Display Contract

This document freezes the platform-adapter contract for top-level window
state, native window controls, display-topology changes, restore
placement, fullscreen and snapped or tiled modes, presentation windows,
dialog ownership, focus return, and cross-window continuity.

The contract is normative. Where it disagrees with the PRD, technical
architecture, technical design, UI/UX spec, shell boundary ADR, desktop
platform matrix, workspace layout contract, or multi-window verification
seed, the upstream source wins and this document plus its schema and
fixtures update in the same change. Where a platform adapter, renderer,
or shell surface mints private window-state names that conflict with
this document, this document wins and that adapter is non-conforming.

Companion artifacts:

- [`/schemas/platform/window_state.schema.json`](../../schemas/platform/window_state.schema.json)
  is the boundary schema for `window_state_snapshot_record`,
  `window_restore_history_record`, and
  `window_display_verification_case`.
- [`/fixtures/platform/window_display_cases/`](../../fixtures/platform/window_display_cases/)
  contains worked cases for display detach or redock, mixed DPI,
  fullscreen and snapped restore, off-screen dialog recovery,
  suspend/resume, restart/reopen, missing dependency placeholders, and
  detached auxiliary windows.
- [`/artifacts/qa/window_display_matrix.yaml`](../../artifacts/qa/window_display_matrix.yaml)
  and [`/docs/qa/multi_window_verification.md`](../qa/multi_window_verification.md)
  provide the release verification matrix and drill ids this contract
  projects into.

This contract composes with:

- [`/docs/adr/0016-shell-windowing-input-accessibility-boundary.md`](../adr/0016-shell-windowing-input-accessibility-boundary.md)
  for shell ownership, event-loop ordering, focus-chain rules, and
  platform-adapter boundaries.
- [`/docs/platform/desktop_platform_conformance_matrix.md`](../platform/desktop_platform_conformance_matrix.md)
  for claimed macOS, Windows, and Linux desktop rows.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  for pane-tree, restore-phase, placeholder, monitor-affinity, and
  no-rerun vocabulary.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  for typed focus-return outcomes.
- [`/docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md)
  for dialog and sheet origin, ownership, dismissal, and durable
  handoff rules.

## Who Reads This Contract

- Platform-adapter authors translating AppKit, Win32/DWM, Wayland/X11,
  portal, window-manager, display, DPI, and native-chrome facts into
  Aureline events.
- Shell and renderer authors preserving window topology, pane layout,
  active focus, native titlebar affordances, and presentation state.
- QA, release, accessibility, support, and diagnostics tooling that
  need one inspectable explanation for display-topology drift and
  restore placement fallback.

## 1. Scope

This contract freezes:

- platform-neutral names for window modes, placement modes, native
  window controls, display topology, virtual desktop or Space affinity,
  snapped or tiled placement, fullscreen, presentation, and restore
  fallback;
- which state is durable window topology versus best-effort display
  hint versus live authority that must revalidate;
- how windows move across displays, scaling buckets, display detach or
  return, virtual desktops, snapped or tiled arrangements, fullscreen,
  and presentation targets;
- how owned dialogs, sheets, permission prompts, trust prompts, and
  destructive confirmations attach to their owning window and return
  focus;
- when restore history must record that layout intent was preserved but
  placement, mode, role, focus, or authority had to degrade safely;
- continuity requirements for secondary windows and cross-window
  transfers so workspace identity, trust state, host or remote state,
  profile, and recovery-critical cues stay visible.

Out of scope:

- OS-specific compositor internals or every possible window-manager
  nuance.
- Final pixel-perfect visual polish for native controls or titlebar
  styling.
- A production windowing implementation.

## 2. Non-Negotiable Invariants

1. **Layout intent outranks stale geometry.** Window role, pane order,
   active focus, dialog ownership, presentation intent, and recovery
   actions matter more than exact pixel replay.
2. **Display metadata is a hint.** Display ids, coordinates, safe
   bounds, scale buckets, Space or virtual desktop ids, and snapped
   groups are machine-local and best effort. They never become
   workspace authority.
3. **Native chrome is not alternate product logic.** Titlebar controls,
   system menus, traffic lights, caption buttons, CSD controls, and
   fullscreen buttons project canonical commands and disabled reasons.
   They do not bypass preview, approval, trust, or restore rules.
4. **Focus is window-local unless the user switches windows.** A
   display change may remap bounds, scale, or native window-manager
   state; it may not silently change the owning window or focus owner.
5. **Owned prompts move with the owner.** Dialogs, sheets, permission
   prompts, trust prompts, and destructive confirmations remain attached
   to the originating window and return focus there on dismiss.
6. **Fallback is visible.** If Aureline preserves layout intent but
   clears fullscreen, breaks a snapped layout, moves a window to safe
   bounds, recenters a dialog, collapses a secondary role, or requires
   live-authority rebind, restore history records that fact.
7. **No hidden rerun or authority reuse.** Restore, wake, display
   reconnect, presentation recovery, and reopen may restore context and
   evidence. They must not replay commands, input, debug control,
   terminal authority, notebook kernels, remote grants, or approvals.

## 3. Frozen Vocabulary

The schema exports three record kinds:

| Record kind | Purpose |
|---|---|
| `window_state_snapshot_record` | Current or remembered state for one top-level Aureline window. |
| `window_restore_history_record` | One restore, wake, reopen, or display-topology adjustment event. |
| `window_display_verification_case` | Reviewable fixture shape used by the multi-window verification corpus. |

### 3.1 Window Roles

`window_role` is window-topology state. It never grants workspace
authority by itself.

| Value | Meaning |
|---|---|
| `primary_workspace` | Main workspace shell for one workspace authority. |
| `secondary_workspace` | Additional work window sharing the same workspace authority. |
| `auxiliary` | Detached inspector, preview, compare, docs, or task-support window. |
| `presentation` | Window optimized for presenting or following while preserving truth cues. |
| `review` | Review, diff, migration, incident, or support-focused window. |
| `companion` | Desktop-owned companion surface tied to the same command and trust graph. |

Rules:

- A `window_family_ref` binds sibling windows that share a workspace
  authority.
- Every top-level window carries `workspace_authority_ref`,
  `profile_ref`, `window_role`, and an inspectable `active_window_token`.
- A secondary or auxiliary window must keep workspace identity, trust
  state, host or remote state, profile, and recovery-critical cues
  visible even when its layout differs from the primary workspace.

### 3.2 Window Modes and Placement

Platform adapters map native window-manager state into these values
only. They may not create OS-specific parallel mode names.

| Field | Values | Notes |
|---|---|---|
| `window_mode` | `normal`, `maximized_or_zoomed`, `fullscreen`, `presentation_fullscreen`, `minimized` | Top-level mode visible to the shell. macOS zoom and Windows/Linux maximize both map to `maximized_or_zoomed`. |
| `placement_kind` | `freeform`, `maximized_or_zoomed`, `fullscreen_space_or_desktop`, `snapped_or_tiled`, `presentation_target`, `safe_bounds_fallback` | Describes why the bounds are shaped the way they are. Snapped and tiled states share one product name even when OS APIs differ. |
| `virtual_desktop_affinity` | `none`, `prefer_original_virtual_desktop`, `current_virtual_desktop`, `fallback_current_virtual_desktop`, `unknown_best_effort` | Space or virtual desktop ids are hints. Fallback to the current visible desktop is valid when the old one is unavailable. |
| `scale_bucket` | `1x`, `1_25x`, `1_5x`, `2x`, `other` | Scale affects rendering and hit testing. It is not a layout identity. |

Rules:

- `fullscreen`, `presentation_fullscreen`, and `snapped_or_tiled` are
  restorable intents, not guarantees that the OS will accept exact
  replay later.
- If a stale fullscreen, snapped, tiled, or virtual-desktop state cannot
  be restored safely, Aureline falls back to `safe_bounds_fallback` or
  `maximized_or_zoomed` and records the adjustment.
- A minimized window may restore minimized only when doing so does not
  hide recovery-critical UI, pending prompts, trust review, or unsaved
  consequences.

### 3.3 Native Titlebar and Control Affordances

The product vocabulary is platform-neutral:

| Control | Required mapping |
|---|---|
| `close` | Close or request close through the shell close/reopen contract. |
| `minimize` | Minimize/hide where the claimed profile supports it; otherwise disabled with a reason. |
| `maximize_or_zoom` | macOS zoom, Windows maximize, and Linux maximize all map here. |
| `fullscreen_toggle` | Native fullscreen or shell presentation fullscreen; platform support decides projection. |
| `system_menu` | Window/system menu projection of canonical commands and disabled reasons. |
| `window_drag_region` | Native hit-test or drag region that never steals focus from active controls. |
| `resize_border` | Resize affordance honoring minimum safe bounds and adaptive collapse. |

`titlebar_mode` values:

- `native_titlebar`
- `custom_titlebar_native_controls`
- `custom_chrome_projected_controls`
- `fullscreen_chrome_hidden_by_os`
- `platform_constrained_degraded`

Rules:

- Every claimed row must expose close, minimize where supported,
  maximize or zoom, fullscreen toggle where supported, drag region, and
  resize behavior through a predictable native or host-rendered route.
- Native controls project canonical command outcomes. A disabled close,
  blocked fullscreen, or unavailable minimize path must expose the same
  disabled reason as the host-rendered command path.
- Fullscreen or presentation mode may hide native chrome by OS rule, but
  critical workspace identity, trust, host/remote, and recovery cues
  must remain available inside the shell.

## 4. Topology Change Handling

`topology_change_class` is the closed vocabulary for display events that
can affect placement:

- `display_added`
- `display_removed`
- `display_reordered`
- `display_moved`
- `safe_bounds_changed`
- `scale_changed`
- `virtual_desktop_changed`
- `fullscreen_state_rewritten`
- `snap_or_tile_state_rewritten`
- `presentation_target_changed`
- `wake_display_reconnect`
- `app_reopen`
- `window_detached`
- `window_docked`
- `dependency_missing`
- `unsupported_display_topology`
- `unknown_topology_change`

Handling rules:

1. The adapter reports typed topology facts before the shell decides
   restore or focus outcome. Generic "resize" callbacks are not enough
   for topology drift.
2. The shell recomputes safe bounds for every top-level window and every
   owned transient before restoring focus.
3. When a display disappears, every window and owned transient that
   would be unreachable moves to the nearest safe visible bounds. If no
   nearest bounds can be proven, the fallback is the primary display.
4. When a display returns, Aureline does not silently jump windows back
   to the old display. It may offer a restore-layout action using the
   remembered intent and must keep current focus visible.
5. When scale changes, the renderer updates scale and hit testing before
   command routing resumes. The active pane, owner dialog, and recovery
   actions must remain keyboard reachable.
6. When snapped, tiled, fullscreen, Space, or virtual-desktop state is
   rewritten by the OS, Aureline preserves the working intent and
   records any fallback. It must not treat stale snapped coordinates as
   durable truth.

## 5. Restore History

A `window_restore_history_record` is required whenever a restore,
reopen, wake, display change, or topology drift changes what the user
would reasonably expect from remembered placement.

`restore_history_event_class` values:

- `display_topology_changed`
- `scale_bucket_changed`
- `window_moved_to_safe_bounds`
- `fullscreen_or_space_unsupported`
- `snap_or_tile_unsupported`
- `presentation_target_unavailable`
- `dialog_owner_recentered`
- `focus_target_fallback_applied`
- `window_role_changed`
- `window_role_collapsed`
- `missing_surface_placeholder_inserted`
- `authority_rebind_required`
- `live_surface_no_rerun_enforced`
- `no_restore_history_required`

`restore_adjustment_class` values:

- `snapped_to_safe_bounds`
- `moved_to_primary_display`
- `scale_normalized`
- `fullscreen_cleared`
- `snapped_layout_cleared`
- `tile_group_broken`
- `virtual_desktop_fallback`
- `presentation_mode_cleared`
- `dialog_recentered_to_owner`
- `native_chrome_reprojected`
- `stacking_repaired`
- `window_role_collapsed_to_primary`
- `placeholder_inserted`
- `authority_rebind_deferred`
- `none`

History records must include:

- the source snapshot or restore candidate ref;
- prior and resulting `window_mode`, `placement_kind`,
  `virtual_desktop_affinity`, `scale_bucket`, and safe-bounds posture;
- the event classes and adjustment classes applied;
- continuity level: `exact_restore`, `compatible_restore`,
  `layout_only`, `evidence_only`, or `no_restore`;
- whether focus returned to the exact target, owner, adjacent visible
  pane, main workspace, or a recovery card;
- whether live authority remained valid, requires manual rebind, or is
  evidence-only until reauthorized;
- visible recovery actions.

Rules:

- Restore history is visible through diagnostics, support export, and
  restore-provenance surfaces. Logs alone are not sufficient.
- If Aureline cannot explain why it moved, resized, unsnapped,
  unfullscreened, recentered, or role-collapsed a window, it must not
  claim an exact restore.
- A restore can preserve intent and still be truthful as
  `compatible_restore` or `layout_only`.

## 6. Focus, Active Window, and Owned Prompts

The shell owns the logical focus chain. Platform adapters report native
activation and focus events, but they do not decide product focus truth.

Focus-return order:

1. recorded exact focus target when visible and valid;
2. owning surface in the same window;
3. adjacent visible pane in the same window;
4. main workspace in the same window;
5. visible recovery card or restore details in the same window.

Rules:

- `active_window_token` changes only on explicit user activation,
  command-dispatch activation, or a shell-approved recovery route.
  Display topology changes do not by themselves transfer active-window
  identity.
- Owned dialogs and sheets carry `owning_window_ref` and
  `owning_surface_ref`. They recenter with the owner when safe bounds
  change and return focus to the owner when dismissed.
- A prompt whose owner is unavailable falls back to a visible recovery
  card in the same window family. It must not reopen as a global,
  ownerless dialog.
- Reopen and wake may restore focus history, but hidden, filtered,
  blocked, collapsed, off-screen, or unreachable targets are invalid.

## 7. Secondary Windows and Cross-Window Transfers

Cross-window movement is a transfer of window topology, not a fork of
workspace authority.

`transfer_action_class` values:

- `move_pane`
- `copy_view`
- `detach_window`
- `dock_window`
- `promote_to_presentation`
- `collapse_to_primary`
- `open_secondary_window`

Rules:

- The transfer affordance must advertise the resulting verb before the
  transfer commits.
- Moving a pane preserves pane identity and surrounding restore history
  where safe. Copying a view creates a new view identity that still
  points to the same canonical object.
- Closing or collapsing a secondary window must never orphan dirty
  buffers, active approvals, trust prompts, collaboration control,
  remote host state, or recovery-critical cues. It either transfers
  them to a visible owner or records a visible recovery action.
- Secondary windows restored after display or dependency drift may
  collapse back to a safer window role only with a restore-history event
  and visible explanation.

## 8. Presentation Mode

Presentation mode is window topology plus interruption policy. It is not
workspace authority.

Rules:

- Entering presentation mode preserves workspace identity, path or
  object identity, trust state, profile, host or remote boundary,
  collaboration role badges, and recovery-critical state.
- Exiting presentation mode restores prior layout, panel visibility, and
  selection when valid. Invalid targets follow the focus-return order.
- If the presentation display disappears, Aureline keeps the presenter
  window visible on safe bounds, records
  `presentation_target_unavailable`, and may clear
  `presentation_fullscreen` to a safer mode.
- Presentation, focus mode, quiet hours, or screen sharing may suppress
  decorative interruptions. They may not suppress trust, policy,
  recovery, data-loss, or authority-rebind disclosures.

## 9. Platform Adapter Mapping

Adapters map native state into the frozen vocabulary:

| Native behavior | Product mapping |
|---|---|
| macOS zoom, Windows maximize, Linux maximize | `window_mode = maximized_or_zoomed`; `placement_kind = maximized_or_zoomed` |
| macOS fullscreen / Spaces, Windows fullscreen, GNOME fullscreen | `window_mode = fullscreen`; `placement_kind = fullscreen_space_or_desktop` |
| Windows Snap, GNOME tiling, compositor-managed tiling | `placement_kind = snapped_or_tiled` |
| Projector or external presentation display | `placement_kind = presentation_target`; optional `window_mode = presentation_fullscreen` |
| Missing monitor, unsafe bounds, unsupported display stack | `placement_kind = safe_bounds_fallback` plus restore-history event |
| Native titlebar unavailable or constrained | `titlebar_mode = platform_constrained_degraded` plus disabled reason and recovery path |

Adapters may carry OS-specific diagnostics in adapter-private metadata,
but any field visible to shell, support, QA, or release tooling must use
the schema vocabulary.

## 10. Fixture and Evidence Rules

- Every window/display fixture validates against the window-state schema
  and names one scenario id from the QA matrix.
- Every fixture states which native controls must remain mapped,
  whether restore history is required, which focus-return rule applies,
  and which continuity cues stay visible.
- Fixtures that include terminal, debug, notebook, preview, AI, remote,
  or collaboration surfaces distinguish restorable context from live
  authority.
- Future release packets, support exports, and conformance suites cite
  `scenario_id`, `required_drill`, `restore_history_event_class`, and
  `restore_adjustment_class` values instead of inventing new names.
