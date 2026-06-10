# Notebook collaboration follow and presenter state with live-versus-captured runtime disclosure

## Overview

This document describes the M05-026 implementation for notebook collaboration
follow state, presenter state, and explicit live-versus-captured runtime
disclosure. The implementation ensures that notebook collaboration never
silently conflates live runtime state with captured output, and that follow
and presenter authority are always visible and bounded.

## Records

### `NotebookCollaborationFollowState`

Per-participant follow posture in a notebook collaboration session.

| Field | Type | Description |
|-------|------|-------------|
| `record_kind` | string | `"notebook_collaboration_follow_state"` |
| `notebook_collaboration_follow_presenter_schema_version` | integer | `1` |
| `follow_state_id` | opaque id | Stable follow-state identifier |
| `document_id_ref` | opaque ref | Owning notebook document |
| `session_envelope_ref` | opaque ref | Collaboration session envelope |
| `participant_ref` | opaque ref | Participant actor |
| `follow_mode` | `NotebookFollowMode` | Follow posture |
| `follow_target_class` | `NotebookFollowTargetClass` | What kind of entity is followed |
| `follow_target_ref` | opaque ref | Target being followed |
| `current_cell_id_ref` | nullable opaque ref | Cell in the participant's view |
| `current_output_handle_ref` | nullable opaque ref | Output in the participant's view |
| `follow_explanation` | nullable string | Required when breakaway or degraded |
| `summary` | string | Export-safe summary |

### `NotebookPresenterState`

Presenter identity, mode, and shared scope for a notebook collaboration session.

| Field | Type | Description |
|-------|------|-------------|
| `record_kind` | string | `"notebook_presenter_state"` |
| `notebook_collaboration_follow_presenter_schema_version` | integer | `1` |
| `presenter_state_id` | opaque id | Stable presenter-state identifier |
| `document_id_ref` | opaque ref | Owning notebook document |
| `session_envelope_ref` | opaque ref | Collaboration session envelope |
| `presenter_mode` | `NotebookPresenterMode` | Presenter lifecycle state |
| `presenter_actor_ref` | opaque ref | Actor holding the presenter role |
| `shared_cell_id_ref` | nullable opaque ref | Cell currently shared |
| `shared_output_handle_ref` | nullable opaque ref | Output currently shared |
| `presenter_actions` | `NotebookPresenterActionClass[]` | Available actions for current mode |
| `summary` | string | Export-safe summary |

### `NotebookRuntimeDisclosure`

Explicit live-versus-captured boundary disclosure for a collaborative view.

| Field | Type | Description |
|-------|------|-------------|
| `record_kind` | string | `"notebook_runtime_disclosure"` |
| `notebook_collaboration_follow_presenter_schema_version` | integer | `1` |
| `runtime_disclosure_id` | opaque id | Stable runtime-disclosure identifier |
| `document_id_ref` | opaque ref | Owning notebook document |
| `disclosure_class` | `NotebookRuntimeDisclosureClass` | Boundary classification |
| `kernel_session_ref` | nullable opaque ref | Required for `live_runtime` and `stale_runtime` |
| `captured_at` | nullable date-time | Required for `captured_output` |
| `disclosure_actions` | `NotebookRuntimeDisclosureActionClass[]` | Available transitions |
| `summary` | string | Export-safe summary |

## Closed vocabularies

### `NotebookFollowMode`

- `following_presenter` — Viewer follows the presenter.
- `independent` — Viewer is browsing independently without breakaway.
- `breakaway` — Viewer intentionally browsed independently from presenter.
- `return_available` — Viewer can return to presenter from a visible affordance.
- `follow_degraded` — Follow is degraded or unavailable on the current client.

### `NotebookFollowTargetClass`

- `presenter` — Following the presenter.
- `specific_cell` — Following a specific cell.
- `specific_output` — Following a specific output.
- `specific_participant` — Following a specific participant.

### `NotebookPresenterMode`

- `active_presenter` — Current active presenter sharing scope.
- `co_presenter` — Explicit co-presenter with shared authority.
- `idle` — No presenter role is actively sharing scope.
- `handoff_pending` — Presenter handoff is in progress.

### `NotebookPresenterActionClass`

- `share_screen` — Share the full screen.
- `share_cell` — Share a specific cell.
- `share_output` — Share a specific output.
- `handoff` — Hand off presenter control.
- `pause` — Pause presenting.
- `resume` — Resume presenting.

### `NotebookRuntimeDisclosureClass`

- `live_runtime` — View shows live runtime state from an active kernel.
- `captured_output` — View shows captured output from a prior session.
- `mixed_state` — Some cells show live runtime, others captured output.
- `stale_runtime` — Kernel session exists but is stale relative to source.
- `no_kernel` — No kernel is selected; only saved document and captured outputs.

### `NotebookRuntimeDisclosureActionClass`

- `refresh_runtime` — Refresh the live runtime connection.
- `acknowledge_captured` — Acknowledge that the view is captured output.
- `switch_to_live` — Switch the view to live runtime.
- `switch_to_captured` — Switch the view to captured output.
- `request_kernel` — Request a kernel to enable live runtime.

## Checked-in artifact

The canonical packet lives at:

```
artifacts/notebook/m5/implement_notebook_collaboration_follow_and_presenter_state_with_live_versus_captured_runtime_disclosure.json
```

It is embedded in the `aureline-notebook` crate via `include_str!` and parsed
by `current_notebook_collaboration_follow_presenter_packet()`.

## Schema

The JSON Schema lives at:

```
schemas/notebook/implement_notebook_collaboration_follow_and_presenter_state_with_live_versus_captured_runtime_disclosure.schema.json
```

## Fixtures

Worked fixtures live at:

```
fixtures/notebook/m5/implement_notebook_collaboration_follow_and_presenter_state_with_live_versus_captured_runtime_disclosure/
```

## Downgrade behavior

- When `follow_mode` is `breakaway` or `follow_degraded`, `follow_explanation`
  MUST be present.
- When `presenter_mode` is `active_presenter`, at least one of
  `shared_cell_id_ref` or `shared_output_handle_ref` MUST be present.
- When `disclosure_class` is `live_runtime` or `stale_runtime`,
  `kernel_session_ref` MUST be present.
- When `disclosure_class` is `captured_output`, `captured_at` MUST be present.

## Integration with existing lanes

This module reuses the session-envelope vocabulary from
`aureline-collab::session_role_admission_and_retention_qualification` via
opaque `session_envelope_ref` pointers. It does not redefine presenter,
co-presenter, or follow semantics; it adds the notebook-specific cell-aware
projection of those roles.
