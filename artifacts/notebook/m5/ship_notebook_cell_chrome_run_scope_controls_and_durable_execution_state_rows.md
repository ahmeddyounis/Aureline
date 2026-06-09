# Artifact: Ship notebook cell chrome, run-scope controls, and durable execution-state rows

## Lane

M05-014 — Notebook document, kernel, output, and canonical-source foundations.

## Claim

The notebook cell chrome, run-scope controls, and durable execution-state rows are materialized as typed Rust records with closed vocabularies, a boundary JSON schema, a checked-in packet artifact, worked fixtures, and automated validation so that:

1. The cell chrome communicates execution badge, status, run scope, output trust, and available actions per cell without requiring a live kernel.
2. Run-scope controls distinguish current session, prior session, replay, manual action, and queued states with explicit lock reasons so the user never mistakes a locked scope for a free choice.
3. Durable execution-state rows survive kernel loss, restart, and reconnect so the chrome never collapses into an opaque viewer when the runtime disappears.
4. No-kernel editability is preserved: cells remain editable, searchable, and reviewable even when no kernel is selected.

## Evidence

| Evidence kind | Path | State |
|---|---|---|
| Rust implementation | `crates/aureline-notebook/src/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows/` | Landed |
| Schema | `schemas/notebook/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.schema.json` | Landed |
| Checked-in packet | `artifacts/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.json` | Landed |
| Fixture corpus | `fixtures/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows/` | Landed |
| Integration tests | `crates/aureline-notebook/tests/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.rs` | Landed |
| Docs | `docs/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.md` | Landed |

## Downgrade rules

- If the checked-in packet JSON is stale, missing, or fails validation, the lane narrows to `Incomplete`.
- If any fixture case fails validation, the lane narrows to `Incomplete`.
- If the schema drifts from the Rust implementation without a version bump, the lane narrows to `Blocked`.
- If any consuming surface hides the no-kernel editability state or blocks document search/review without a kernel, the lane narrows to `RollbackMissing`.
- If a durable execution-state row is silently discarded on kernel disconnect, the lane narrows to `RollbackMissing`.

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
