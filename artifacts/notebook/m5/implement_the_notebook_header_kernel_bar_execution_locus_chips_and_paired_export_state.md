# Artifact: Implement the notebook header, kernel bar, execution-locus chips, and paired-export state

## Lane

M05-013 — Notebook document, kernel, output, and canonical-source foundations.

## Claim

The notebook header, kernel bar, execution-locus chips, and paired-export state are materialized as typed Rust records with closed vocabularies, a boundary JSON schema, a checked-in packet artifact, worked fixtures, and automated validation so that:

1. The notebook header communicates document identity, trust, dirty state, kernel state, execution locus, and paired-export posture without requiring a click.
2. The kernel bar distinguishes selected, pending, narrowed, and unavailable kernel states with explicit action affordances.
3. Execution-locus chips compactly show where code runs (local, container, SSH remote, managed workspace, browser bridge, service plane, or no kernel) and whether the boundary is active, degraded, disconnected, reconnecting, or policy-blocked.
4. Paired-export state never silently promotes a derived script or Markdown form to canonical.

## Evidence

| Evidence kind | Path | State |
|---|---|---|
| Rust implementation | `crates/aureline-notebook/src/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state/` | Landed |
| Schema | `schemas/notebook/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state.schema.json` | Landed |
| Checked-in packet | `artifacts/notebook/m5/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state.json` | Landed |
| Fixture corpus | `fixtures/notebook/m5/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state/` | Landed |
| Integration tests | `crates/aureline-notebook/tests/notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state.rs` | Landed |
| Docs | `docs/notebook/m5/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state.md` | Landed |

## Downgrade rules

- If the checked-in packet JSON is stale, missing, or fails validation, the lane narrows to `Incomplete`.
- If any fixture case fails validation, the lane narrows to `Incomplete`.
- If the schema drifts from the Rust implementation without a version bump, the lane narrows to `Blocked`.
- If any consuming surface hides the local-vs-remote boundary cue for a remote kernel, the lane narrows to `RollbackMissing`.
- If a no-kernel state blocks document editing, search, or review, the lane narrows to `RollbackMissing`.

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
