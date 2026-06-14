# M5 heavy-artifact-family storage-class, pin-source, clear-data, and low-disk matrix

This document is the **per-family storage-governance contract** for the heavy
artifacts the M5 depth lanes add. It freezes one
`m5_artifact_family_storage_matrix` record so that every heavy artifact family
— notebook outputs, profiler traces, replay bundles, docs/model/template
packs, generated previews, extension downloads, prebuild layers, support
artifacts, and review/incident evidence — plus the user-owned recovery state
those lanes touch is mapped, **exactly once**, to a frozen storage class with
an explicit authority posture, default retention, rebuild cost, pin sources,
allowed clear-data actions, and low-disk eviction position.

The matrix exists so a storage inspector, a low-disk banner, a clear-data
review sheet, the cleanup-history lane, an offboarding/reset flow, and a
support-bundle storage section all explain the **same** thing about a family —
what is disposable, what is rebuildable, what is durable evidence, what is
user-owned recovery state, which pins protect it, and what a clear-data action
may do to it — without inventing a surface-local cleanup vocabulary.

The contract is normative. Where it disagrees with the PRD, TAD, TDD, UI/UX
Spec, or design-system style guide, those sources win and this document plus
its schema and artifact update in the same change. Where any M5 artifact lane
mints private cleanup semantics outside the shared storage-class registry,
this contract wins and the lane is non-conforming.

## Companion artifacts

- [`/artifacts/storage/m5_artifact_family_storage_matrix.yaml`](../../artifacts/storage/m5_artifact_family_storage_matrix.yaml)
  — the checked-in matrix, one row per family.
- [`/schemas/storage/m5_artifact_family_storage_matrix.schema.json`](../../schemas/storage/m5_artifact_family_storage_matrix.schema.json)
  — boundary schema for the `m5_artifact_family_storage_matrix` record.
- [`/artifacts/storage/m5_artifact_family_storage_matrix.md`](../../artifacts/storage/m5_artifact_family_storage_matrix.md)
  — the human-readable matrix table and rationale.
- [`/fixtures/storage/m5_artifact_family_storage_matrix/`](../../fixtures/storage/m5_artifact_family_storage_matrix/)
  — the golden support-export projection replay case.
- `crates/aureline-support/src/m5_storage_governance/` — the typed record,
  the runtime-profile validator, and the inspector / low-disk / clear-data /
  offboarding / support-export projections every consumer reuses.

## Upstream contracts this contract rides on

This contract does **not** re-mint storage vocabulary that is already frozen
upstream; it consumes the frozen sets by name and by value:

- [`/artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml)
  — the six `storage_class_id` values, the four `authority_class` values, the
  four `rebuild_cost_class` values, the five `gc_policy_class` values, the ten
  `pin_source_class` values, the four `clear_cache_protection_class` values,
  and the eight `low_disk_ladder_step` values. Every matrix row's declared
  posture **must** be admissible under the canonical runtime row for its
  storage class; the validator rejects a family that strays.
- [`/artifacts/storage/eviction_priority_matrix.yaml`](../../artifacts/storage/eviction_priority_matrix.yaml)
  and [`/docs/storage/storage_inspector_contract.md`](../storage/storage_inspector_contract.md)
  — the cross-surface storage-inspector posture and the generic eviction
  ordering this per-family matrix extends with the specific M5 families.
- The M3 storage-cleanup registry in
  `crates/aureline-support/src/storage_inspector/` — the `StorageClassId`
  vocabulary this matrix reuses for its class column.

## What the matrix adds

The only vocabularies introduced here are bounded explanatory labels, not
storage primitives:

- **`default_retention_class`** — `evict_on_session_end`,
  `evict_under_pressure_if_unpinned`, `retain_until_version_replace`,
  `retain_for_policy_window`, `retain_until_explicit_user_reset`.
- **`clear_data_action_class`** — `generic_clear_in_bulk`,
  `generic_clear_excluding_pins`, `class_selective_clear`,
  `class_specific_review_required`, `explicit_per_item_review_required`,
  `export_before_delete_offered`.

## Invariants

The validator (`M5ArtifactFamilyStorageMatrix::validate`) enforces:

1. **Complete, single-mapping coverage.** Every M5 heavy artifact family
   appears exactly once.
2. **No private cleanup semantics.** Each row's `authority_class`,
   `rebuild_cost_class`, `gc_policy_class`, and every `pin_source_class` is
   admissible under the runtime row for its `storage_class_id`, and its
   `clear_cache_protection_class`, `low_disk_ladder_step`, and
   `export_before_delete_required` equal the runtime row exactly.
3. **Protected continuity is honest.** `protected_continuity` is true exactly
   for the `evidence_support_cache` and `user_owned_recovery_state` classes,
   and those families require export-before-delete.
4. **Class-selective, previewable clear-data only.** A protected family never
   admits a generic bulk clear; it requires a class-specific or per-item
   review and offers export-before-delete first. An always-clearable family
   admits no pin sources (no entry survives a generic clear).
5. **Ordered, visible low-disk eviction.** The low-disk eviction order
   projects each family by its `low_disk_ladder_step` position (early → late),
   trimming disposable hot cache first and touching user-owned recovery state
   only last, under explicit review.

## Consumption

The shared `m5_storage_governance` module projects the matrix into the views
the inspectable surfaces reuse:

- `low_disk_eviction_order()` — the low-disk banner's ordered eviction list.
- `clear_data_plan_for(family)` — the clear-data review sheet's protection
  posture, allowed actions, preserved pins, and export requirement per family.
- `offboarding_reset_plan()` — the offboarding/reset split between families
  disposed without review and families gated by export-before-delete.
- `support_export(...)` — a metadata-safe support/export envelope, with no raw
  payloads, that the support-bundle pipeline quotes.

This matrix is part of the canonical M5 evidence train described in
[`certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`](./certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md);
if its artifact, schema, or proof drift, the affected storage-governance row
narrows until the proof is refreshed.
