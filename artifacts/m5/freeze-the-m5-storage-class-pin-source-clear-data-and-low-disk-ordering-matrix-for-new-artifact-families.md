# Freeze the M5 storage-class, pin-source, clear-data, and low-disk ordering matrix

Evidence record for the per-family storage-governance matrix that covers the
heavy artifacts the M5 depth lanes add.

## What shipped

- A checked-in matrix mapping every M5 heavy artifact family to a frozen
  storage class with an explicit authority posture, default retention, rebuild
  cost, pin sources, allowed clear-data actions, and low-disk eviction step:
  [`/artifacts/storage/m5_artifact_family_storage_matrix.yaml`](../storage/m5_artifact_family_storage_matrix.yaml)
  (12 families).
- The boundary schema:
  [`/schemas/storage/m5_artifact_family_storage_matrix.schema.json`](../../schemas/storage/m5_artifact_family_storage_matrix.schema.json).
- The contract and human-readable table:
  [`/docs/m5/freeze-the-m5-storage-class-pin-source-clear-data-and-low-disk-ordering-matrix-for-new-artifact-families.md`](../../docs/m5/freeze-the-m5-storage-class-pin-source-clear-data-and-low-disk-ordering-matrix-for-new-artifact-families.md)
  and [`/artifacts/storage/m5_artifact_family_storage_matrix.md`](../storage/m5_artifact_family_storage_matrix.md).
- The canonical product object plus its validator and consumer projections:
  `crates/aureline-support/src/m5_storage_governance/`.
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_artifact_family_storage_matrix/support_export.golden.json`](../../fixtures/storage/m5_artifact_family_storage_matrix/support_export.golden.json).

## Families covered

`generated_preview`, `notebook_output`, `docs_pack`, `model_pack`,
`template_pack`, `extension_download`, `prebuild_layer`, `profiler_trace`,
`replay_bundle`, `support_artifact`, `review_incident_evidence`, and
`user_owned_recovery_state`.

## Proof

The matrix is validated against the canonical runtime storage-class contract
at `/artifacts/runtime/storage_classes.yaml`: each family's authority, rebuild
cost, GC policy, pin sources, clear-protection, low-disk step, and
export-before-delete posture must be admissible under the runtime row for its
storage class. Automated proof lives in
`crates/aureline-support/src/m5_storage_governance/tests.rs`:

- complete single-mapping coverage of every family;
- the matrix validates with zero violations against the runtime profiles;
- protected families (evidence and user-owned recovery) never admit a generic
  clear and always require export-before-delete;
- offboarding/reset never silently disposes protected state;
- the low-disk eviction order trims disposable cache first and user-owned
  recovery state last;
- the support export is metadata-safe and family-complete, and matches the
  checked-in golden;
- negative gates reject a protected family mutated to allow a generic clear and
  a row mutated to an inadmissible authority.

## Reuse surfaces

`low_disk_eviction_order()` (low-disk banner), `clear_data_plan_for(family)`
(clear-data review), `offboarding_reset_plan()` (offboarding/reset), and
`support_export(...)` (support/export packets). Part of the canonical M5
evidence train; the row narrows if its artifact, schema, or proof drift.
