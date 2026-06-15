# Implement low-disk or quota banners with visible eviction ordering

Evidence record for the low-disk and managed-quota pressure banner that
discloses how the heavy artifacts the M5 depth lanes add are paused, trimmed,
expired, and protected under storage pressure.

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  metadata-safe support-export projection:
  `crates/aureline-support/src/m5_storage_pressure/`.
- The boundary schema:
  [`/schemas/storage/m5_storage_pressure.schema.json`](../../schemas/storage/m5_storage_pressure.schema.json).
- The contract and human-readable summary:
  [`/docs/storage/m5_storage_pressure_contract.md`](../../docs/storage/m5_storage_pressure_contract.md)
  and [`/artifacts/storage/m5_storage_pressure.md`](../storage/m5_storage_pressure.md).
- A scenario corpus across constrained / degraded / protect-core low-disk
  pressure, a managed-quota ceiling, and a quota refusal:
  [`/fixtures/storage/m5_storage_pressure_cases/`](../../fixtures/storage/m5_storage_pressure_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_storage_pressure/support_export.golden.json`](../../fixtures/storage/m5_storage_pressure/support_export.golden.json).

## Scenarios covered

`low_disk_constrained_pauses_then_trims_disposable`,
`low_disk_degraded_trims_rebuildable_unpinned`,
`low_disk_protect_core_expires_unpinned_evidence_only`,
`managed_quota_ceiling_narrows_surface`, and
`quota_pressure_refuses_user_owned_state`.

## Acceptance

- Banners state the pressure class, pressure source, paused work, next eviction
  order, protected classes, and open-inspector action using stable vocabulary.
- The eviction order follows the frozen low-disk sequence and never deletes
  authoritative recovery or evidence state without reviewed escalation.

## Proof

The banner anchors its eviction order to the frozen low-disk ladder shared by
the runtime storage-class contract at
`/artifacts/runtime/storage_classes.yaml` and drilled in
`/artifacts/runtime/low_disk_drills.yaml`; the matrix-backed composer folds the
frozen artifact-family matrix at
`/artifacts/storage/m5_artifact_family_storage_matrix.yaml`, so the banner and
the storage-governance matrix share one ladder. Per-class no-authoritative-state-loss
guards prove pressure freed only disposable, rebuildable, or
unpinned-past-retention bytes; the user-owned recovery guard always reclaims
zero bytes, and a pending escalation never reclaims protected bytes. The
metadata-safe support export carries the same truth for Help/About,
service-health, and support-bundle surfaces, and matches its checked-in golden.
The validator, the schema, and the fixture corpus all enforce the invariants in
`crates/aureline-support/src/m5_storage_pressure/tests.rs`.
