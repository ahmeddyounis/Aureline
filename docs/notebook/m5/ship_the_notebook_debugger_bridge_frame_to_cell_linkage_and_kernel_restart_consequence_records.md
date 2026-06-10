# Notebook debugger bridge, frame-to-cell linkage, and kernel restart consequence records

## Overview

This document describes the M05-027 debugger-bridge surface for Aureline notebooks. It covers how the notebook chrome renders explicit frame-to-cell relationships and kernel-restart consequences so the user never assumes a debugger frame belongs to the current cell when it does not, and never assumes the debugger survived a restart when it did not.

## Records

### `DebuggerFrameCellLink`

The explicit mapping between a debugger frame and a notebook cell. Carries:

- `link_id` — stable opaque identifier for the frame-to-cell link.
- `document_id_ref` — opaque ref to the owning notebook document.
- `kernel_session_id_ref` — opaque ref to the kernel session.
- `frame_ref` — opaque ref to the debugger frame.
- `cell_id_ref` — opaque ref to the mapped cell; null when the link class is `no_cell_mapping`.
- `link_class` — the relationship class: `exact_cell_match`, `nearest_cell_heuristic`, `no_cell_mapping`, `in_cell_library_code`, `in_cell_external_dependency`, `frame_stale_after_restart`.
- `link_posture_class` — the posture: `actionable_step_into_cell`, `actionable_step_over_cell`, `view_only_no_step`, `stale_reinitialize_required`, `unsupported_no_source_map`.
- `source_line_ref` — opaque ref to the source-line descriptor; null when unavailable.

Validation enforces that:
- `no_cell_mapping` must not carry a `cell_id_ref`.
- Link classes other than `no_cell_mapping` must carry a `cell_id_ref`.
- `stale_reinitialize_required` posture requires `frame_stale_after_restart` link class.

### `KernelRestartDebuggerConsequence`

The typed consequence of a kernel restart on the debugger bridge. Carries:

- `consequence_id` — stable opaque identifier for the consequence.
- `document_id_ref` — opaque ref to the owning notebook document.
- `prior_kernel_session_id_ref` — opaque ref to the prior kernel session; null when none existed.
- `next_kernel_session_id_ref` — opaque ref to the next kernel session; null when unknown or unavailable.
- `restart_kind` — why the restart occurred: `user_initiated_restart`, `transport_lost_reconnect_attempted`, `trust_downgrade_cancels_in_flight`, `policy_denies_continued_execution`, `managed_workspace_lifecycle_change`, `bridge_cancelled_by_restart`.
- `consequence_class` — the consequence: `bridge_preserved_same_session`, `bridge_reset_fresh_session`, `bridge_cancelled_pending_reconnect`, `bridge_unavailable_no_kernel`, `breakpoints_retained_across_restart`, `breakpoints_lost_on_restart`, `variable_state_lost`, `execution_queue_cleared`.
- `in_flight_debug_cancelled` — whether in-flight debug sessions are cancelled.
- `breakpoints_affected` — number of active breakpoints affected.
- `reattach_action_class` — how reattachment works: `reattach_automatically`, `reattach_on_demand`, `reattach_unavailable`, `reattach_requires_review`.
- `reconnect_review_sheet_ref` — opaque ref to the reconnect-review sheet, when one exists.
- `auto_rerun_forbidden` — must be `true` on every record.

Validation enforces that:
- `auto_rerun_forbidden` must be `true`.
- `bridge_preserved_same_session` must not cancel in-flight debug sessions.
- Several consequence classes must cancel in-flight debug sessions.
- `reattach_unavailable` must not carry a `next_kernel_session_id_ref`.
- Certain restart kinds require `reattach_requires_review` or `reattach_unavailable`.

## Checked-in packet

The canonical packet lives at:

```
artifacts/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.json
```

## Schema

The boundary schema lives at:

```
schemas/notebook/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.schema.json
```

## Fixtures

Worked fixtures live at:

```
fixtures/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/
```
