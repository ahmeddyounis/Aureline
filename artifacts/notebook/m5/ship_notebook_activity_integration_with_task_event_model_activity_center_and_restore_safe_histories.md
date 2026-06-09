# Artifact: Notebook activity integration with task-event model, activity center, and restore-safe histories

## Lane

M05-020 — Notebook document, kernel, output, and canonical-source foundations.

## Claim

The notebook activity integration with task-event model, activity center, and restore-safe histories is materialized as typed Rust records with closed vocabularies, a boundary JSON schema, a checked-in packet artifact, worked fixtures, and automated validation so that:

1. Notebook cell executions emit canonical task events with notebook-specific provenance (notebook ID, cell ID, kernel/session ID, execution-context ref) so the task-event stream never loses notebook identity.
2. Notebook activities appear as typed chronology rows in the activity center with actor, action, object, outcome, surface class, source class, freshness class, and follow-up state so the activity center never hides notebook work behind generic labels.
3. Notebook execution history survives session restore without silently auto-rerunning cells, so the user always sees an honest posture such as `transcript_restored`, `rerun_required`, or `context_unavailable`.
4. No-kernel editability is preserved: notebook activities remain observable and reviewable even when no kernel is selected.

## Evidence

| Evidence kind | Path | State |
|---|---|---|
| Rust implementation | `crates/aureline-notebook/src/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories/` | Landed |
| Schema | `schemas/notebook/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories.schema.json` | Landed |
| Checked-in packet | `artifacts/notebook/m5/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories.json` | Landed |
| Fixture corpus | `fixtures/notebook/m5/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories/` | Landed |
| Integration tests | `crates/aureline-notebook/tests/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories.rs` | Landed |
| Docs | `docs/notebook/m5/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories.md` | Landed |

## Downgrade rules

- If the checked-in packet JSON is stale, missing, or fails validation, the lane narrows to `Incomplete`.
- If any fixture case fails validation, the lane narrows to `Incomplete`.
- If the schema drifts from the Rust implementation without a version bump, the lane narrows to `Blocked`.
- If any consuming surface hides notebook activity from the activity center or presents generic labels instead of actor/action/object/outcome, the lane narrows to `RollbackMissing`.
- If a notebook kernel is presented as live after restore when only a transcript was recovered, the lane narrows to `RollbackMissing`.
- If notebook cell executions fail to emit canonical task events, the lane narrows to `Blocked`.

## Rollback path

1. Revert the module and tests to the last known-good commit.
2. Restore the previous packet JSON from version control.
3. Notify the notebook subsystem owner to re-qualify the lane before widening.

## Freshness SLO

- Packet must be refreshed when the schema version bumps or when the closed vocabularies change.
- Target max age: 30 days.
- Warn window: 7 days before expiry.

## Owner

Notebook subsystem owner (see `CODEOWNERS`).
