# Certify storage-class truth, low-disk behavior, clear-data previews, and pin/retention integrity on M5 heavy-artifact profiles

Evidence record for the M5 storage certification index that binds storage-class
truth, class-selective clear-data previews, low-disk / managed-quota pressure
behavior, pin/retention integrity, corruption-repair, and export-before-delete
proof for every claimed M5 heavy-artifact family on every claimed M5 profile.

## What shipped

- The canonical product object plus its validator, matrix cross-check, and
  shared-surface bindings:
  `crates/aureline-support/src/m5_storage_certification/`.
- The boundary schema:
  [`/schemas/storage/m5_storage_certification.schema.json`](../../schemas/storage/m5_storage_certification.schema.json).
- The contract and human-readable review:
  [`/docs/storage/m5_storage_certification_contract.md`](../../docs/storage/m5_storage_certification_contract.md)
  and
  [`/artifacts/storage/m5_storage_certification.md`](../storage/m5_storage_certification.md).
- A canonical fixture plus two degraded fixtures:
  [`/fixtures/storage/m5_storage_certification/`](../../fixtures/storage/m5_storage_certification/).

## Fixtures covered

`packet.json` (canonical), `stale_pin_retention_gated.json`, and
`blurred_cache_authority_blocked.json`.

## Acceptance

- The certification packet exercises disk-pressure / managed-quota behavior,
  class-selective clear-data review, pinned-evidence retention, corruption-repair
  drills, and export-before-delete validation across every heavy-artifact family
  introduced by M5 — not one artifact class alone.
- Rows that blur cache versus authoritative state or hide pressure behavior
  downgrade automatically; stale pin/retention evidence can no longer keep a row
  green.
- A class-selective clear-data and offboarding posture is required for protected
  classes; managed quota and storage pressure never silently delete local
  user-owned state.
- The metadata-safe certification index is exposed to Help/About, service health,
  support export, and release manifest by reference.

## Proof

The seeded packet folds the frozen artifact-family storage matrix at
`/artifacts/storage/m5_artifact_family_storage_matrix.yaml` (which itself
projects the canonical runtime storage-class contract), so each row's storage
class, authority, and protection posture come from the same contract the
storage-governance matrix, the clear-data review, the low-disk pressure banners,
the pin/retention managers, the cache-repair plans, and the offboarding
continuity plans validate — never a local synonym set. The validator, the
schema, and the fixture corpus all enforce the invariants in
`crates/aureline-support/tests/m5_storage_certification.rs`, including the
managed-quota protected-exclusion guard, the matrix consistency cross-check, the
shared-surface binding parity, and the two degraded fixtures that prove stale
pin/retention gates protected families and stale/blurred storage-class truth
blocks authoritative families and narrows disposable ones. The certification
carries the same metadata-safe storage-class and proof refs for Help/About,
service health, support, and release surfaces, and each fixture replays exactly
against its seeded packet.
