# Storage eviction policy, priority order, and reopen disclosure

This document publishes the shared eviction vocabulary Aureline uses for
storage-class garbage collection, low-disk / quota trimming, and disclosure
when reopening after cleanup. It exists so reclamation work never silently
breaks **reopen**, **restore**, **offline continuity**, or **evidence**
guarantees.

This document is additive. It MUST NOT mint a parallel vocabulary for storage
classes, pins, or eviction steps; it maps common “what data is this?” questions
to the already-frozen `storage_class_id` set.

## Source of truth

- `docs/runtime/storage_classes_and_gc.md` and `artifacts/runtime/storage_classes.yaml`
  — canonical `storage_class_id`, `pin_source_class`, `clear_cache_protection_class`,
  `gc_policy_class`, `low_disk_ladder_step`, and protected-class invariants.
- `docs/storage/clear_data_and_low_disk_contract.md`
  — clear-data review and low-disk / quota banner disclosure rules.
- `docs/storage/pin_retention_cleanup_history_contract.md`
  — cleanup-history attribution for low-disk trimming, manual cleanup, post-update
  invalidation, and corrupt-cache repair.
- `docs/storage/reopen_after_eviction_contract.md`
  — reopen-after-eviction packet contract that freezes cross-surface consequences.

## Eviction classes (storage_class_id)

The six storage classes are the only shared eviction vocabulary. Every cache
store, inspector surface, support export, and reopen/restore disclosure MUST
project back to these ids:

- `interactive_hot_cache` — disposable derived state (trim first).
- `knowledge_cache` — rebuildable derived state (correctness-relevant).
- `artifact_cache` — imported durable artifacts (docs packs, extensions, symbol bundles).
- `prebuild_environment_cache` — prebuild layers, toolchain packs, environment capsules.
- `evidence_support_cache` — evidence/support artifacts (protected; class-specific review).
- `user_owned_recovery_state` — local history, checkpoints, session restore metadata
  (authoritative; explicit review only).

## Priority order under pressure

Low-disk and quota pressure follow the ordered ladder frozen in
`artifacts/runtime/storage_classes.yaml`:

1. `stop_speculative_fetch_and_prefetch`
2. `pause_managed_replication_and_pack_refresh`
3. `trim_interactive_hot_cache`
4. `trim_knowledge_cache_rebuildable`
5. `trim_artifact_cache_unpinned`
6. `trim_prebuild_environment_unpinned`
7. `expire_unpinned_evidence_past_retention`
8. `user_owned_recovery_state_only_under_explicit_review`

Protected-continuity rules:

- `evidence_support_cache` is never eligible for a generic clear-cache path and is
  only eligible for expiry when it is both **unpinned** and **past retention**.
- `user_owned_recovery_state` is never eligible for generic clear-cache and is never
  eligible for low-disk trimming without an explicit, reviewed exception path.

## Data-family mapping (what lives where)

This table maps common data families onto the shared `storage_class_id` set.
The rows are not new ids; they are a stable explanation layer for humans.

| Data family | storage_class_id | Default behavior | Reopen consequence (summary) |
|---|---|---|---|
| Indexes and graph shards | `knowledge_cache` | rebuildable; may be trimmed when unpinned | search/graph rebuild; first query slower |
| Language snapshots / embeddings | `knowledge_cache` | rebuildable; schema drift invalidates | rebuild pending until rehydrated |
| Cached docs (docs packs) | `artifact_cache` | preserved when pinned by offline/mirror/release refs | offline docs remain available when pinned; otherwise may require re-download |
| Exact-build support artifacts (symbol bundles) | `artifact_cache` | preserved when pinned by release/case refs | exact-build lookup stays supportable when pinned; otherwise requires refetch |
| Prebuild layers / toolchain packs | `prebuild_environment_cache` | rebuildable; may be trimmed when unpinned | first build slower; may require network/mirror |
| Temp outputs and previews | `interactive_hot_cache` | disposable; trimmed first | no correctness loss; short warmup cost |
| Logs and traces (evidence-grade) | `evidence_support_cache` | protected; retention + export-before-delete | never silently removed; deletion requires class-specific review or retention expiry |
| Local history and checkpoints | `user_owned_recovery_state` | protected; explicit review only | never silently removed; restore/reopen must remain honest about availability |

## Cleanup paths and what must be disclosed

Every cleanup path MUST disclose:

- which storage classes were affected;
- what can be rebuilt vs what is authoritative;
- which pins/protected classes were preserved; and
- which reopen/restore/offline/support/migration consequences follow.

The disclosure packet families are:

- **Low disk / quota pressure** → `low_disk_banner_record` (pressure posture + next ladder steps).
- **Manual cleanup (user/admin)** → `clear_data_review_record` (loss/rebuild/pin/protected preview).
- **Post-action attribution** → `cleanup_history_event_record` (what happened, why, what was blocked).
- **Reopen after cleanup** → `reopen_after_eviction_packet_record` (cross-surface consequences).

## Machine-readable matrix

`artifacts/storage/eviction_priority_matrix.yaml` is the machine-readable mapping
layer that ties the above data families back to the storage-class vocabulary and
pressure ladder without minting new runtime ids.

