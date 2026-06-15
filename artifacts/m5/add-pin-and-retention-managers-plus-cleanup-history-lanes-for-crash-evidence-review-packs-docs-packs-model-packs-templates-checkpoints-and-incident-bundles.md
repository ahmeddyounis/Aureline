# Add pin / retention managers plus cleanup-history lanes

Evidence record for the pin / retention manager and cleanup-history lane that
make on-disk retention visible and keep eviction attributable for the heavy
artifacts the M5 depth lanes add — crash evidence, review packs, docs / model /
template packs, certified templates, checkpoints, and incident bundles.

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  metadata-safe support-export projection:
  `crates/aureline-support/src/m5_pin_retention/`.
- The boundary schema:
  [`/schemas/storage/m5_pin_retention.schema.json`](../../schemas/storage/m5_pin_retention.schema.json).
- The contract and human-readable summary:
  [`/docs/storage/m5_pin_retention_contract.md`](../../docs/storage/m5_pin_retention_contract.md)
  and [`/artifacts/storage/m5_pin_retention.md`](../storage/m5_pin_retention.md).
- A scenario corpus across evidence / checkpoint pins, offline packs and
  certified templates, pin-blocked cleanup history, and managed-quota refusal:
  [`/fixtures/storage/m5_pin_retention_cases/`](../../fixtures/storage/m5_pin_retention_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_pin_retention/support_export.golden.json`](../../fixtures/storage/m5_pin_retention/support_export.golden.json).

## Scenarios covered

`evidence_and_checkpoint_pins`, `offline_packs_and_certified_templates`,
`cleanup_history_blocked_by_pins`, and
`managed_quota_preserves_user_owned_state`.

## Acceptance

- Pin / retention managers show the pin source, who pinned it, the expiry /
  policy window, the referenced object, the unpin path, and the export path
  across evidence, offline packs, certified templates, and checkpoints.
- Cleanup history preserves actor, class, family, reclaimed bytes, blocked pins,
  and resulting stale / reindex-needed state without unsafe payload capture.

## Proof

The manager anchors every pin to the same frozen storage classes and pin sources
the storage-governance matrix at
`/artifacts/storage/m5_artifact_family_storage_matrix.yaml` validates; the
matrix-backed composer `compose_manager` folds that matrix so the manager and
the matrix can never disagree. The pin actor, unpin path, and export path are
pure functions of the pin source and the matrix row. Cleanup history keeps every
eviction attributable and proves storage pressure never reclaims user-owned
recovery bytes — the only recovery-state cleanup that may reclaim bytes is an
explicit, exported-then-deleted user action — while blocked pins are always
recorded, never hidden. The metadata-safe support export carries the same truth
for Help / About, service-health, and support-bundle surfaces, and matches its
checked-in golden. The validator, the schema, and the fixture corpus all enforce
the invariants in `crates/aureline-support/src/m5_pin_retention/tests.rs`,
including negative tests that reject a silent recovery delete under pressure and
a derived-field mismatch.
