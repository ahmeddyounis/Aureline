# Targeted cache / index repair, stale-label propagation, no-reset-everything fallback

Evidence record for the cache-repair plan that turns a corrupt or stale cache /
index into a targeted, disclosed repair — with stale-label propagation and a
no-reset-everything fallback — across the M5 heavy-artifact storage classes
(search / graph caches, docs / model packs, preview runtimes, profiler / replay
traces, prebuild layers, and user-owned recovery state).

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  metadata-safe support-export projection:
  `crates/aureline-support/src/m5_cache_repair/`.
- The boundary schema:
  [`/schemas/storage/m5_cache_repair.schema.json`](../../schemas/storage/m5_cache_repair.schema.json).
- The contract and human-readable summary:
  [`/docs/storage/m5_cache_repair_contract.md`](../../docs/storage/m5_cache_repair_contract.md)
  and [`/artifacts/storage/m5_cache_repair.md`](../storage/m5_cache_repair.md).
- A scenario corpus across a corrupt search / graph index, a docs / model pack
  checksum mismatch, a torn generated preview, a quarantined evidence trace, a
  repaired-in-place recovery journal, and a failed prebuild repair with a targeted
  fallback:
  [`/fixtures/storage/m5_cache_repair_cases/`](../../fixtures/storage/m5_cache_repair_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_cache_repair/support_export.golden.json`](../../fixtures/storage/m5_cache_repair/support_export.golden.json).

## Scenarios covered

`knowledge_cache_corrupt_index_reindex`,
`artifact_pack_checksum_mismatch_refetch`, `generated_preview_torn_rederive`,
`evidence_trace_corrupt_quarantined_for_review`,
`recovery_state_torn_repair_in_place`, and
`prebuild_missing_backing_repair_failed_fallback`.

## Acceptance

- Corrupt / stale storage classes produce targeted repair guidance — rebuild one
  index, refetch one pack by digest, re-derive one cache, or repair one
  workspace's recovery state — and preserve quarantined copies when they still
  hold user-owned data or forensic value.
- Affected surfaces show stale / rebuild-needed / corrupt state until the repair
  actually completes; the labels clear only on `repair_complete_healthy`.
- No plan offers a factory reset or a reset-everything fallback; a failed repair
  falls back to a narrower-or-equal action.

## Proof

The composer `compose_plan` folds the canonical runtime storage-class profiles at
`/artifacts/runtime/storage_classes.yaml` (loaded through the storage-governance
lane), so the plan's protection posture and export-before-delete requirement come
from the same contract the storage-governance matrix validates rather than a
local synonym set. The repair action, quarantine disposition, propagated label,
and fallback are pure functions of the storage class, the detected fault, and the
repair state. The validator, the schema, and the fixture corpus all enforce the
invariants in `crates/aureline-support/src/m5_cache_repair/tests.rs`, including
negative tests that reject an offered factory reset, a stale label cleared before
the repair completes, and a protected class cleared without a quarantine copy.
The metadata-safe support export carries the same truth for Help / About,
service-health, and support-bundle surfaces, and matches its checked-in golden.
