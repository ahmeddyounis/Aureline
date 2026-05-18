# Notebook Runtime-Truth Report

Status: seeded
Schema version: 1
As of: 2026-05-18

## Scope

This report covers the retained notebook preview runtime-truth record set
that keeps a notebook preview row honest about notebook identity, kernel
/ session state, output rendering trust, variable-explorer freshness,
restart / reconnect consequences, and debugger-bridge support. The model
is the same one quoted by the kernel-bar header chip, the cell-execution
detail row, the variable explorer, the rich-output viewer, the debugger
status strip, audit events, support exports, and evidence packets — the
record ids and closed-vocabulary tokens are stable across all of those
surfaces.

The records do not redefine the four orthogonal trust axes — document,
kernel, output (lineage), widget — that are already frozen by
[`/schemas/notebook/notebook_metadata_aureline.schema.json`](../../../schemas/notebook/notebook_metadata_aureline.schema.json)
and the kernels-and-trust matrix at
[`/artifacts/notebook/kernels_and_trust_matrix.yaml`](../../kernels_and_trust_matrix.yaml).
They add the runtime-bearing surface records the preview row needs to
render without implying JupyterLab-class maturity through silence.

Six record kinds are governed:

- `notebook_kernel_session_summary_record` — the kernel-bar / notebook
  header projection. Names the notebook identity, document trust class,
  dirty state, selected kernel or `No kernel`, kernel origin, local-vs-
  remote boundary cue, target-identity witness, paired-export posture,
  last-successful-run summary, and the kernel-bar actions exposed
  (`Select kernel`, `Change kernel`, `Restart`, `Restart and run all`,
  `Interrupt`, `Reconnect`, `Shutdown`). The boundary schema is at
  [`/schemas/notebook/kernel_session_summary.schema.json`](../../../schemas/notebook/kernel_session_summary.schema.json).
- `notebook_cell_execution_detail_row_record` — one row per cell-execution
  attempt. Names cell id, display index, run scope, started/finished
  timestamps, duration, outcome class, output count, and the opaque refs
  that hand trace/log/diagnostic surfaces off to the run-history / log
  surfaces.
- `notebook_variable_explorer_entry_record` — variable-explorer rows
  labelled `live_from_current_session`, `snapshot_from_prior_session`,
  `stale_after_restart`, `imported_snapshot`, or
  `no_live_variables_no_kernel`, with a controlled truncation class and
  closed action vocabulary.
- `notebook_output_trust_record` — rich-output rendering trust. Pins
  the four explicit classes `sanitized`, `sandboxed`, `trusted_active`,
  and `stale`; cites a stale_reason_class whenever the class is `stale`;
  carries an explicit `hidden_escalation_posture`; and pins the
  raw/compatible-viewer/export/review-before-trust fallback actions in a
  stable order. The boundary schema is at
  [`/schemas/notebook/output_trust_record.schema.json`](../../../schemas/notebook/output_trust_record.schema.json).
- `notebook_debugger_bridge_state_record` — debugger-bridge support
  posture for the active kernel. Names support class
  (`supported`, `supported_partial`, `unsupported`,
  `unsupported_by_policy`, `unsupported_no_kernel`,
  `unsupported_remote_parity_unverified`), unsupported reason class,
  adapter class, kernel class, the relationship between the focused cell
  and the current frame, and the breakpoint posture the runtime can
  honour.
- `notebook_reconnect_review_sheet_record` — generated on any restart,
  reconnect, shutdown, or runtime drift. Names what kernel session was
  in force, what session (if any) is on the other side, why the sheet was
  opened, what the runtime is actually reopening (transcript / fresh
  session / same-identity / identity-changed / degraded preview /
  quarantined), and what happens to in-flight executions, queued cells,
  and live variable state.

The Rust implementation lives at
[`/crates/aureline-notebook/src/runtime_truth/`](../../../crates/aureline-notebook/src/runtime_truth/).
The fixture corpus lives at
[`/fixtures/notebook/m3/kernel_output_and_reconnect/`](../../../fixtures/notebook/m3/kernel_output_and_reconnect/).

## Kernel-bar header rules

| Origin                                  | Boundary cue | Witness/session refs | `Restart`/`Interrupt` exposed | Auto-rerun on restore |
|-----------------------------------------|:------------:|:--------------------:|:-----------------------------:|:--------------------:|
| `no_kernel`                             | n/a          | not carried          | no                            | forbidden            |
| `local_managed_toolchain_kernel`        | hidden       | not carried          | yes                           | forbidden            |
| `local_provisioned_kernel`              | hidden       | not carried          | yes                           | forbidden            |
| `remote_agent_primary_kernel`           | visible      | required             | yes                           | forbidden            |
| `managed_workspace_agent_kernel`        | visible      | required             | yes                           | forbidden            |
| `provider_side_remote_kernel`           | visible      | required             | yes                           | forbidden            |
| `compatibility_bridge_remote_kernel`    | visible      | required             | yes                           | forbidden            |

`no_kernel` rows MUST expose `Select kernel` and MUST NOT silently expose
`Restart` or `Interrupt`. Any retained header row MUST declare
`auto_rerun_forbidden=true`; reopening, reconnecting, or restoring a
notebook MUST NOT re-execute a prior cell.

## Rich-output trust classes

| Class           | Admits active behaviour | Requires explicit review to escalate | Hidden escalation default              |
|-----------------|:-----------------------:|:------------------------------------:|----------------------------------------|
| `sanitized`     | no                      | yes                                  | `no_hidden_escalation_allowed`         |
| `sandboxed`     | no                      | yes                                  | `no_hidden_escalation_allowed`         |
| `trusted_active`| yes                     | n/a                                  | `explicit_review_required` to enter    |
| `stale`         | no                      | yes                                  | `no_hidden_escalation_allowed`         |

Stale outputs MUST cite a `stale_reason_class` such as
`kernel_restarted_since_produce`, `kernel_lost_transport`,
`document_trust_downgraded_since_produce`,
`output_captured_from_prior_session`,
`source_cell_edited_since_produce`, or `orphaned_no_kernel_binding`. Non-
stale outputs MUST NOT carry a stale-reason class. `trusted_active`
outputs MUST advertise a compatible viewer; sanitized / sandboxed
outputs under `no_hidden_escalation_allowed` MUST expose
`review_before_trust` so the user can never be silently escalated.

## Variable-explorer freshness

| Freshness                              | Kernel session ref | `open_live_viewer` admitted | Notes                                                          |
|----------------------------------------|:------------------:|:---------------------------:|----------------------------------------------------------------|
| `live_from_current_session`            | required           | yes                         | Live kernel-bound value.                                       |
| `snapshot_from_prior_session`          | recorded           | no                          | Snapshot drawn from the prior session; never `open_live_viewer`.|
| `stale_after_restart`                  | recorded           | no                          | Retained across restart; redaction/review actions only.        |
| `imported_snapshot`                    | recorded           | no                          | Drawn from imported evidence.                                  |
| `no_live_variables_no_kernel`          | absent             | no                          | No kernel; explorer carries no live values.                    |

Every truncation MUST cite a typed truncation class
(`no_truncation`, `truncated_for_display`, `truncated_for_redaction`,
`truncated_for_size`, `unsupported_type_no_preview`). Exports run through
`export_with_redaction` / `review_before_export`; the explorer never
implies that a truncated view is the whole value.

## Debugger bridge state

| Support class                              | Kernel ref | Unsupported reason            | Breakpoint posture                                |
|--------------------------------------------|:----------:|-------------------------------|---------------------------------------------------|
| `supported`                                | required   | `not_applicable_supported`    | `breakpoints_honoured`                             |
| `supported_partial`                        | required   | typed reason                  | typed posture (often `_source_map_only`)           |
| `unsupported`                              | optional   | typed reason                  | typed posture                                      |
| `unsupported_by_policy`                    | optional   | `adapter_capability_narrowed_by_policy` | `breakpoints_blocked_by_policy`           |
| `unsupported_no_kernel`                    | absent     | `no_kernel_session`           | `breakpoints_not_supported_by_kernel`              |
| `unsupported_remote_parity_unverified`     | recorded   | `remote_adapter_round_trip_unverified` | `breakpoints_cancelled_by_restart` etc.   |

The bridge MUST name the relationship between the focused cell and the
current frame
(`no_active_frame`, `current_cell_matches_current_frame`,
`different_cell_paused`, `frame_in_imported_library`,
`frame_in_prior_cell_attempt`, `frame_stale_after_restart`). When kernel
restart / reconnect / runtime drift occurs, the bridge MUST declare
`reconnect_review_required=true` and cite a `reconnect_review_sheet_ref`.

## Reconnect / restart review sheet

Every restart, reconnect, shutdown, identity rotation, trust downgrade,
managed-workspace lifecycle change, policy-deny event, or window-exceeded
fresh-session event generates a sheet. The sheet pins:

- the reason (`user_initiated_restart`, `user_initiated_reconnect`,
  `user_initiated_shutdown`, `transport_lost_reconnect_attempted`,
  `identity_rotation_requires_renegotiation`,
  `trust_downgrade_cancels_in_flight`,
  `managed_workspace_lifecycle_change`,
  `policy_denies_continued_execution`,
  `window_exceeded_fresh_session_required`);
- the consequence — what Aureline is actually reopening
  (`reopening_transcript_no_live_kernel`,
  `reopening_live_kernel_fresh_session`,
  `reopening_live_kernel_same_identity`,
  `reopening_live_kernel_identity_changed`,
  `reopening_degraded_preview_no_execution`,
  `quarantined_awaiting_operator_review`);
- whether in-flight executions are cancelled, how many queued cells are
  affected, whether live variable state is lost, and whether explicit
  user confirmation is required.

Every consequence other than `reopening_live_kernel_same_identity` MUST
cancel in-flight executions and MUST declare
`live_variable_state_lost=true`. Trust downgrades, identity rotations,
user restarts, user shutdowns, and window-exceeded fresh sessions MUST
require user confirmation. `auto_rerun_forbidden` is always `true`.

## Acceptance claims preserved

The fixtures and validators in this seed jointly enforce:

- Notebook document state, kernel session state, and output trust /
  rendering state remain distinguishable on every row. Collapsing them
  into a single "notebook trust" chip is non-conforming.
- Restart / reconnect flows say what runtime state will be lost, what
  queued work is affected, and whether the row is reopening a transcript,
  a live kernel, or a degraded preview — never silence.
- Rich outputs, variable views, and debugger bridges preserve boundary,
  freshness, and trust labels in UI and exported evidence; the four
  output trust classes never silently escalate; the debugger never
  claims parity it cannot back.
- Claimed notebook preview rows do not imply hidden broad authority,
  silent reruns, or stable debugger parity when the runtime cannot
  actually support it. A `no_kernel` document remains editable,
  reviewable, and diffable.

## Source-of-truth crosswalk

| Surface                                      | Record carried                                                                 |
|----------------------------------------------|--------------------------------------------------------------------------------|
| Notebook header / kernel bar                 | `notebook_kernel_session_summary_record`                                       |
| Cell-execution row (status strip, run gutter)| `notebook_cell_execution_detail_row_record`                                    |
| Variable explorer                            | `notebook_variable_explorer_entry_record`                                      |
| Output viewer (rich output cards)            | `notebook_output_trust_record`                                                 |
| Debugger status strip / step bar             | `notebook_debugger_bridge_state_record`                                        |
| Restart / reconnect review sheet             | `notebook_reconnect_review_sheet_record`                                       |
| Activity timeline / audit row                | The above records cited by opaque ref; raw payloads MUST NOT appear.            |
| Support export / evidence packet             | The above records carried as bounded ref bundles; redaction class `metadata_safe_default`. |
