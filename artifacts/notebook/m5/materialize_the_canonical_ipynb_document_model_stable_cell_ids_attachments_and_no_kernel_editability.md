# Artifact: Materialize the canonical .ipynb document model, stable cell IDs, attachments, and no-kernel editability

## Lane

M05-012 — Notebook document, kernel, output, and canonical-source foundations.

## Claim

The canonical `.ipynb` document model is materialized as typed Rust records with closed vocabularies, a boundary JSON schema, a checked-in packet artifact, worked fixtures, and automated validation so that:

1. `.ipynb` stays canonical.
2. Cell IDs stay durable across reorder, diff, review, and comment anchoring.
3. Attachments preserve unknown namespaces and are not silently externalized.
4. Notebook open/search/review flows remain useful without a selected kernel.

## Evidence

| Evidence kind | Path | State |
|---|---|---|
| Rust implementation | `crates/aureline-notebook/src/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability/` | Landed |
| Schema | `schemas/notebook/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.schema.json` | Landed |
| Checked-in packet | `artifacts/notebook/m5/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.json` | Landed |
| Fixture corpus | `fixtures/notebook/m5/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability/` | Landed |
| Integration tests | `crates/aureline-notebook/tests/canonical_ipynb_document_model.rs` | Landed |
| Docs | `docs/notebook/m5/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.md` | Landed |

## Downgrade rules

- If the checked-in packet JSON is stale, missing, or fails validation, the lane narrows to `Incomplete`.
- If any fixture case fails validation, the lane narrows to `Incomplete`.
- If the schema drifts from the Rust implementation without a version bump, the lane narrows to `Blocked`.
- If no-kernel editability is violated by any consuming surface, the lane narrows to `RollbackMissing`.

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
