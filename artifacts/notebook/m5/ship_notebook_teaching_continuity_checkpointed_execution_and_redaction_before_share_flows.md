# Notebook teaching continuity, checkpointed execution, and redaction-before-share flows — Artifact

## Packet reference

- **Packet file**: `ship_notebook_teaching_continuity_checkpointed_execution_and_redaction_before_share_flows.json`
- **Schema file**: `schemas/notebook/ship_notebook_teaching_continuity_checkpointed_execution_and_redaction_before_share_flows.schema.json`
- **Crate module**: `aureline-notebook::ship_notebook_teaching_continuity_checkpointed_execution_and_redaction_before_share_flows`
- **Schema version**: `1`
- **Record kind**: `notebook_teaching_continuity_checkpointed_redaction_packet`

## Coverage

This packet covers the closed vocabularies and worked examples for:

- `NotebookTeachingContinuity` — teaching flow mode, checkpoint preference, step state, and sandbox requirement
- `NotebookCheckpointedExecution` — execution checkpoint class, sandbox state, rollback posture, and honest replay label
- `NotebookRedactionBeforeShare` — redaction class, trigger, scope, and explanation before sharing
- `NotebookTeachingMode` — guided_exercise, demo, solo_exploration, classroom, mentor_session
- `NotebookCheckpointPreference` — auto_checkpoint, manual_checkpoint, no_checkpoint, sandbox_only
- `NotebookCheckpointClass` — auto_checkpoint, manual_checkpoint, pre_execution, pre_destructive, sandbox_boundary
- `NotebookSandboxState` — sandboxed, unsandboxed, sandbox_pending, sandbox_failed
- `NotebookRollbackPosture` — rollback_available, rollback_expired, checkpoint_orphaned, exact_replay_available, compensating_replay_only
- `NotebookRedactionClass` — output_redacted, cell_source_redacted, metadata_redacted, variable_redacted, none
- `NotebookRedactionTrigger` — manual_review, policy_auto, sensitivity_scan, recipient_mismatch, teaching_safety

## Downgrade and truth invariants enforced

- Teaching flows requiring sandbox must not silently execute unsandboxed.
- Checkpointed execution with `sandbox_failed` must not claim `rollback_available` or `exact_replay_available`.
- Redaction-before-share with a non-`none` class requires at least one redacted cell or output ref.
- Redaction-before-share with a non-`none` class requires a non-empty explanation.
- All records require non-empty document refs and summaries.

## Consumer contract

Downstream docs, help, support exports, and CI surfaces MUST ingest this packet
instead of cloning status text. The packet is embedded in the crate via
`include_str!` and parsed at runtime by
`current_notebook_teaching_continuity_checkpointed_redaction_packet()`.
