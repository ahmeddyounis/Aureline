# Reopen-after-eviction disclosure packet contract

This document freezes the `reopen_after_eviction_packet_record` Aureline emits
when a workspace/profile is reopened after a cleanup path removed or invalidated
storage-class data. The packet exists so low-disk trimming, quota enforcement,
manual cleanup, post-update invalidation, and corrupt-cache repair never degrade
**reopen**, **restore**, **offline continuity**, **exact-build support lookup**,
or **migration recovery** silently.

The contract is normative. Where it disagrees with the PRD, TAD, TDD, UI/UX Spec,
or design-system style guide, those sources win and this contract (plus schema
and fixtures) update in the same change.

## Companion artifacts

- `schemas/storage/reopen_after_eviction_packet.schema.json`
  — boundary schema for the `reopen_after_eviction_packet_record`.
- `fixtures/storage/reopen_after_eviction_cases/`
  — worked YAML cases covering partial eviction + reopen/restore/offline/support/migration consequences.
- `artifacts/storage/eviction_policy.md`
  — published eviction vocabulary summary and disclosure rules.
- `artifacts/storage/eviction_priority_matrix.yaml`
  — machine-readable mapping back to frozen `storage_class_id` and ladder steps.

## Upstream contracts this contract rides on

This contract does not re-mint storage vocabularies. It consumes the frozen sets
by name and by value:

- `docs/runtime/storage_classes_and_gc.md` and `artifacts/runtime/storage_classes.yaml`
  — `storage_class_id`, pin sources, protected-class rules, low-disk ladder.
- `docs/storage/clear_data_and_low_disk_contract.md`
  — clear-data review and low-disk banner disclosure records.
- `docs/storage/pin_retention_cleanup_history_contract.md`
  — cleanup-history attribution records and actor/event vocabularies.
- `docs/support/support_bundle_contract.md`
  — exact-build joins and “retained local vs missing local” honesty rules.
- `docs/migration/migration_restore_and_shortcut_delta_packet.md`
  — migration restore posture and typed recovery gaps.

## 1. Scope

This contract freezes one reopen-after-eviction packet emitted on reopen:

- which cleanup path triggered the eviction/invalidation;
- which storage classes were affected and how (trimmed, invalidated for rebuild, protected/retained);
- which pinned/protected classes were preserved; and
- which cross-surface consequences follow for:
  - workspace reopen;
  - restore/rehydration flows;
  - offline docs access;
  - exact-build support lookup; and
  - migration recovery.

## 2. Out of scope

- Implementing the cache manager, quota accountant, or garbage collector.
- Implementing the UI surfaces that render this packet.
- Platform-specific disk accounting APIs.

## 3. The reopen-after-eviction packet record

### 3.1 Required fields

- `record_kind = reopen_after_eviction_packet_record`.
- `reopen_after_eviction_packet_schema_version = 1`.
- `packet_id` — opaque, stable, safe to log and export.
- `emitted_at` — monotonic timestamp.
- `packet_scope` — typed scope block for the reopened object (workspace/profile/tenant/etc).
- `triggering_cleanup_event_class` — closed vocabulary naming the cleanup path.
- `triggering_*_ref` fields — opaque refs to the originating banner/review/history rows when applicable.
- `class_rows[]` — per-class impact rows (see §4).
- `surface_impacts[]` — typed cross-surface impacts (see §5).
- `redaction_class` and `export_safe` — so support/export tooling can trust the packet.

## 4. Per-class impact rows

Each `class_rows[]` entry freezes:

- the `storage_class_id` (re-export of the runtime vocabulary);
- whether the class was trimmed/invalidated/missing vs preserved;
- which pin sources were preserved for this class (if any);
- the `rebuild_cost_hint`-style summary describing offline/network dependence and user-visible impact; and
- explicit-review linkage when a protected class is removed.

## 5. Cross-surface impacts

Each `surface_impacts[]` entry names:

- the `surface_class` (reopen / restore / offline docs / support lookup / migration recovery);
- a typed `impact_class` (unaffected, degraded, blocked);
- a reviewable sentence describing the concrete consequence; and
- the `driver_storage_class_ids[]` that explain why the impact exists.

## 6. Honesty invariants

These are schema-enforced invariants:

1. **No silent protected deletion.** `user_owned_recovery_state` cannot be marked as removed unless
   `explicit_review_ref` is non-null.
2. **Evidence expiry is explicit.** `evidence_support_cache` can only be marked as removed without an
   explicit review when the removal is classified as the retention-expiry exception.
3. **Driver classes are named.** Every `surface_impacts[]` row names at least one `driver_storage_class_id`.
4. **Scope is explicit.** The packet always carries an explicit scope block; no “global” reopen ambiguity.

