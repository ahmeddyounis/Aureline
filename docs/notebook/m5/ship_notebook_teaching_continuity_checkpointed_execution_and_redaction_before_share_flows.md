# Notebook teaching continuity, checkpointed execution, and redaction-before-share flows

## Purpose

This module provides the typed records, closed vocabularies, and checked-in artifact that keep notebook teaching flows honest about checkpoint preference, sandbox posture, and rollback truth; checkpointed execution honest about save-point identity, sandbox state, and replay posture; and redaction-before-share honest about what was redacted, why, and under what trigger.

## Records

### `NotebookTeachingContinuity`

Carries a teaching flow’s mode, checkpoint preference, guided-exercise step state, and sandbox requirement so teaching surfaces never run destructive cells without an explicit safe path.

| Field | Type | Description |
|---|---|---|
| `record_kind` | string | Always `notebook_teaching_continuity`. |
| `notebook_teaching_continuity_checkpointed_redaction_schema_version` | integer | Schema version (currently `1`). |
| `teaching_continuity_id` | opaque id | Stable id for this teaching-continuity record. |
| `document_id_ref` | opaque ref | Notebook document this flow belongs to. |
| `teaching_mode` | closed vocab | `guided_exercise`, `demo`, `solo_exploration`, `classroom`, `mentor_session`. |
| `checkpoint_preference` | closed vocab | `auto_checkpoint`, `manual_checkpoint`, `no_checkpoint`, `sandbox_only`. |
| `current_step_index` | integer or null | Current step in a guided exercise. |
| `total_steps` | integer or null | Total steps in a guided exercise. |
| `sandbox_required` | boolean | Whether sandboxed execution is required. |
| `sandbox_unavailable_explanation` | string or null | Explanation when sandbox is required but unavailable. |
| `summary` | string | Export-safe summary. |

### `NotebookCheckpointedExecution`

Carries an execution checkpoint’s class, sandbox state, rollback posture, and honest replay label so users always know whether a checkpoint is available, expired, orphaned, or replayable.

| Field | Type | Description |
|---|---|---|
| `record_kind` | string | Always `notebook_checkpointed_execution`. |
| `notebook_teaching_continuity_checkpointed_redaction_schema_version` | integer | Schema version (currently `1`). |
| `checkpointed_execution_id` | opaque id | Stable id for this checkpoint. |
| `document_id_ref` | opaque ref | Notebook document this checkpoint belongs to. |
| `cell_id_ref` | opaque ref | Cell that created or owns this checkpoint. |
| `checkpoint_class` | closed vocab | `auto_checkpoint`, `manual_checkpoint`, `pre_execution`, `pre_destructive`, `sandbox_boundary`. |
| `sandbox_state` | closed vocab | `sandboxed`, `unsandboxed`, `sandbox_pending`, `sandbox_failed`. |
| `rollback_posture` | closed vocab | `rollback_available`, `rollback_expired`, `checkpoint_orphaned`, `exact_replay_available`, `compensating_replay_only`. |
| `checkpointed_at` | string | UTC timestamp when the checkpoint was created. |
| `honest_state_label` | string | Human-readable honest-state label. |
| `summary` | string | Export-safe summary. |

### `NotebookRedactionBeforeShare`

Carries redaction class, trigger, scope, and explanation so collaboration and presentation surfaces expose collapse/redaction controls before broad sharing when outputs may contain sensitive data.

| Field | Type | Description |
|---|---|---|
| `record_kind` | string | Always `notebook_redaction_before_share`. |
| `notebook_teaching_continuity_checkpointed_redaction_schema_version` | integer | Schema version (currently `1`). |
| `redaction_id` | opaque id | Stable id for this redaction record. |
| `document_id_ref` | opaque ref | Notebook document this redaction belongs to. |
| `redaction_class` | closed vocab | `output_redacted`, `cell_source_redacted`, `metadata_redacted`, `variable_redacted`, `none`. |
| `redaction_trigger` | closed vocab | `manual_review`, `policy_auto`, `sensitivity_scan`, `recipient_mismatch`, `teaching_safety`. |
| `redacted_cell_refs` | array of opaque refs | Cells whose content was redacted. |
| `redacted_output_refs` | array of opaque refs | Outputs that were redacted. |
| `redaction_explanation` | string | Export-safe explanation of what was redacted and why. |
| `summary` | string | Export-safe summary. |

## Checked-in artifact

The canonical packet lives at:

```
artifacts/notebook/m5/ship_notebook_teaching_continuity_checkpointed_execution_and_redaction_before_share_flows.json
```

Downstream docs, help, support, and CI surfaces ingest this artifact instead of cloning status text.

## Schema

The boundary schema lives at:

```
schemas/notebook/ship_notebook_teaching_continuity_checkpointed_execution_and_redaction_before_share_flows.schema.json
```

## Fixtures

Worked fixtures live under:

```
fixtures/notebook/m5/ship_notebook_teaching_continuity_checkpointed_execution_and_redaction_before_share_flows/
```

## Design references

- Technical Design Document §7.10.7 — Notebook collaboration, presentation, and teaching continuity architecture
- UX Design System Style Guide §26.11 — Notebook collaboration, presentation, and teaching continuity
- PRD §5.32 — Notebook and structured-artifact obligations
