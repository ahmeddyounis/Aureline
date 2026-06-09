# Notebook activity integration with task-event model, activity center, and restore-safe histories

## Purpose

This document describes the M05-020 notebook activity integration with task-event model, activity center, and restore-safe histories data model that connects notebook execution to the canonical task-event stream, activity-center chronology, and session-restore history.

## Principles

- Notebook cell executions must emit canonical task events with notebook-specific provenance so the task-event stream never loses notebook identity.
- Notebook activities must appear as typed chronology rows in the activity center with actor, action, object, and outcome so the activity center never hides notebook work behind generic labels.
- Notebook execution history must survive session restore without silently auto-rerunning cells; the chrome must always show an honest posture.
- Restore-safe histories must distinguish `exact_restore`, `compatible_restore`, `layout_only`, `recovered_drafts`, and `evidence_only` with explicit postures such as `transcript_restored`, `session_ended`, `reconnect_available`, `rerun_required`, and `context_unavailable`.
- Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT appear on any record.

## Model overview

### NotebookTaskEvent

The task-event record carries:

- `event_id` — stable opaque event identity.
- `notebook_id_ref` — opaque notebook-document id.
- `cell_id_ref` — opaque cell id (stable across save/diff/merge).
- `kernel_session_id_ref` — opaque kernel-session id; null when no kernel is bound.
- `cell_execution_id_ref` — opaque cell-execution id minted by the execution queue.
- `task_event_kind` — `task_queued`, `task_started`, `output_appended`, `task_completed`, `task_failed`, or `task_cancelled`.
- `task_state_class` — `queued`, `running`, `completed`, `failed`, or `cancelled`.
- `execution_context_ref` — opaque execution-context ref shared with the runtime task-event model.
- `occurred_at` — ISO 8601 UTC timestamp.

### NotebookActivityCenterRow

The activity-center row record carries:

- `row_id` — stable opaque row identity.
- `notebook_id_ref` — opaque notebook-document id.
- `cell_id_ref` — opaque cell id; null for kernel-session-level actions.
- `actor_kind` — `user_actor`, `system_actor`, or `kernel_actor`.
- `action` — `started`, `succeeded`, `failed`, `cancelled`, `blocked`, or `restored`.
- `object_kind` — `notebook_cell_run`, `notebook_kernel_session`, or `notebook_output_block`.
- `outcome` — `pending`, `in_progress`, `succeeded`, `failed`, `cancelled`, or `recovered`.
- `occurred_at` — ISO 8601 UTC timestamp.
- `surface_class` — `activity_center`.
- `source_class` — `first_party_direct_observation`, `first_party_synthesized_summary`, or `recovery_reconstructed`.
- `freshness_class` — `current`, `fresh`, `cached`, `stale`, `expired`, or `unknown`.
- `follow_up_state` — `none`, `open`, `acknowledged`, `resolved`, `dismissed`, `snoozed`, or `muted`.

### NotebookRestoreSafeHistory

The restore-safe history record carries:

- `history_id` — stable opaque history identity.
- `notebook_id_ref` — opaque notebook-document id.
- `restore_class` — `exact_restore`, `compatible_restore`, `layout_only`, `recovered_drafts`, or `evidence_only`.
- `restore_posture` — `transcript_restored`, `session_ended`, `reconnect_available`, `rerun_required`, or `context_unavailable`.
- `kernel_session_id_ref` — opaque kernel-session id; null when no session is recoverable.
- `cell_execution_id_refs` — opaque cell-execution ids present at restore time.
- `document_restored_at` — ISO 8601 UTC timestamp.
- `honest_state_label` — human-readable honest-state label rendered after restore.

## Closed vocabularies

| Vocabulary | Variants | Location |
|---|---|---|
| `NotebookTaskEventKind` | `task_queued`, `task_started`, `output_appended`, `task_completed`, `task_failed`, `task_cancelled` | `aureline-notebook` crate |
| `NotebookTaskStateClass` | `queued`, `running`, `completed`, `failed`, `cancelled` | `aureline-notebook` crate |
| `NotebookActivityActorKind` | `user_actor`, `system_actor`, `kernel_actor` | `aureline-notebook` crate |
| `NotebookActivityAction` | `started`, `succeeded`, `failed`, `cancelled`, `blocked`, `restored` | `aureline-notebook` crate |
| `NotebookActivityObjectKind` | `notebook_cell_run`, `notebook_kernel_session`, `notebook_output_block` | `aureline-notebook` crate |
| `NotebookActivityOutcome` | `pending`, `in_progress`, `succeeded`, `failed`, `cancelled`, `recovered` | `aureline-notebook` crate |
| `NotebookActivitySurfaceClass` | `activity_center` | `aureline-notebook` crate |
| `NotebookActivitySourceClass` | `first_party_direct_observation`, `first_party_synthesized_summary`, `recovery_reconstructed` | `aureline-notebook` crate |
| `NotebookActivityFreshnessClass` | `current`, `fresh`, `cached`, `stale`, `expired`, `unknown` | `aureline-notebook` crate |
| `NotebookActivityFollowUpState` | `none`, `open`, `acknowledged`, `resolved`, `dismissed`, `snoozed`, `muted` | `aureline-notebook` crate |
| `NotebookRestoreClass` | `exact_restore`, `compatible_restore`, `layout_only`, `recovered_drafts`, `evidence_only` | `aureline-notebook` crate |
| `NotebookRestorePosture` | `transcript_restored`, `session_ended`, `reconnect_available`, `rerun_required`, `context_unavailable` | `aureline-notebook` crate |

## Schema

The boundary schema lives at:

```
/schemas/notebook/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories.schema.json
```

## Checked-in artifact

The typed activity-integration packet is checked in at:

```
artifacts/notebook/m5/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories.json
```

It is embedded in the `aureline-notebook` crate via `include_str!` so consumers and CI agree on every row without a cargo build in CI.

## Fixtures

Worked fixture cases live under:

```
fixtures/notebook/m5/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories/
```

Cases cover:
- `task_event_lifecycle` — queued, started, output_appended, completed task events.
- `task_event_failure` — failed and cancelled task events.
- `activity_center_cell_run` — user-started and kernel-succeeded cell run rows.
- `activity_center_failure_and_cancel` — failed and cancelled cell run rows.
- `activity_center_restore` — system-restored kernel session row.
- `restore_exact` — exact restore with reconnectable kernel session.
- `restore_transcript` — compatible restore with transcript-only recovery.
- `restore_rerun_required` — layout-only restore with explicit rerun requirement.
- `restore_unavailable` — evidence-only restore with no recoverable context.

## Integration

The `aureline-notebook` crate exposes:

- `NotebookTaskEvent`, `NotebookActivityCenterRow`, `NotebookRestoreSafeHistory`
- `NotebookActivityIntegrationPacket` and `current_notebook_activity_integration_packet()`
- Validation methods on every record that return typed `ActivityIntegrationFinding` findings
- Closed vocabularies with `as_str()` tokens and `ALL` arrays

## Risks and downgrade behavior

- If the embedded JSON artifact is missing or malformed, `current_notebook_activity_integration_packet()` fails at compile time and CI must treat the lane as underqualified.
- If a fixture case fails validation, the corpus is incomplete and promotion should narrow the claim.
- Terminal task-event kinds must pair with terminal task-state classes; mismatches are rejected by validation.
- Activity-center rows must pair outcomes with correct actions; mismatches are rejected by validation.
- Restore-safe histories must not cite a live kernel session when the posture is `transcript_restored` or `session_ended`; violations are rejected by validation.
- Notebook activities must remain observable in the activity center without requiring a selected kernel.
