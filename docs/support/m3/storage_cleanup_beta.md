# Storage cleanup (beta)

Reviewer doc for the storage inspector, class-selective clear-data review
sheets, low-disk handling, and cleanup history.

This doc names the shared truth model the chrome storage inspector, the
class-selective clear-data review flow, the low-disk banner, the
cleanup-history surface, and support exports all consume. It exists so
`clear cache` can never become a vague destructive umbrella: a user or
admin must always see which classes are selected, which classes stay
protected by default, and what becomes stale or lost before any bytes
are deleted.

## Boundary

- **Schemas**
  - [`schemas/support/storage_class.schema.json`](../../../schemas/support/storage_class.schema.json) —
    one registry entry per storage class.
  - [`schemas/support/clear_data_review.schema.json`](../../../schemas/support/clear_data_review.schema.json) —
    the class-selective review sheet.
  - [`schemas/support/storage_cleanup_receipt.schema.json`](../../../schemas/support/storage_cleanup_receipt.schema.json) —
    the typed receipt minted after cleanup runs.
- **Consumer module**
  - [`crates/aureline-support/src/storage_inspector/`](../../../crates/aureline-support/src/storage_inspector/mod.rs)
- **Protected fixture corpus**
  - [`fixtures/support/m3/storage_cleanup/`](../../../fixtures/support/m3/storage_cleanup/)
- **Integration test**
  - [`crates/aureline-support/tests/storage_cleanup_beta.rs`](../../../crates/aureline-support/tests/storage_cleanup_beta.rs)

## Storage classes

The shared registry covers six classes. Every chrome surface, low-disk
banner, and cleanup receipt names a class by id and reuses the registry's
scope, authority, rebuild-cost, sensitivity, and GC-policy tokens.

| Class | Scope | Authority | Rebuild cost | Sensitivity | GC policy | Protected by default |
| ----- | ----- | --------- | ------------ | ----------- | --------- | -------------------- |
| `interactive_hot_cache` | process-local | runtime disposable | cheap local | non-sensitive | eviction eligible | no |
| `knowledge_cache` | workspace-local | workspace derived | expensive local | metadata only | eviction eligible | no |
| `artifact_cache` | profile-local | provider derived | network required | metadata only | eviction with review | no |
| `prebuild_environment_cache` | machine-local | provider derived | network required | metadata only | eviction with review | no |
| `evidence_support_cache` | profile-local | evidence grade | irrecoverable | evidence grade | never evict silently | **yes** |
| `user_owned_recovery_state` | user-account-local | user-authored recovery | irrecoverable | private user content | never evict silently | **yes** |

`user_owned_recovery_state` covers the user-authored recovery surfaces
this lane must never delete silently:

- local history,
- rollback checkpoints,
- dirty-buffer journals,
- pinned review artifacts,
- offline entitlement bundles,
- last-known-good policy bundles.

`evidence_support_cache` covers the governance evidence the support
pipeline keeps reachable for incidents and repair transactions:

- crash and incident workspace snapshots,
- repair-transaction journals,
- in-flight support-bundle drafts.

Both classes are excluded from ordinary cleanup. Removing them requires
an explicit per-item override row with a written justification and, when
the class is durable, an export-before-delete target.

## Storage inspector

The inspector projects one truth model that the chrome and support
exports share:

- **Total disk use** by workspace/profile scope and by class.
- **Class breakdown** that quotes the registry vocabulary verbatim.
- **Workspace/profile scope** so a single workspace can be cleaned
  without spanning to others.
- **Quota or policy source** when a managed policy caps a class.
- **Largest consumers** named by ref (not raw path).
- **Pin source** for any pinned item that would block cleanup.
- **Stale/corrupt state** flagged by Project Doctor or the corruption
  repair lane.

## Class-selective clear-data review

The review sheet is the only entry point to a cleanup action. It records:

- `actor_lineage_class` and `trigger_class`,
- the user-selected class rows with consequence class and rebuild
  summary,
- the affected workspace scope rows,
- the protected class rows excluded by default,
- pin block rows naming each pin that would block deletion,
- optional `override_protected_class_refs` for the rare cases where a
  durable class must be reclaimed, each with a written justification
  and (for durable classes) an export-before-delete target,
- `consent_state` (`unconfirmed`, `confirmed`, `cancelled`,
  `blocked_by_protected_class`).

A confirmed review must carry a `confirmed_by_ref`. Selecting a
protected class without an override is refused by the harness. Cancelled
reviews never reclaim bytes.

## Low-disk handling

When disk or quota pressure crosses a threshold the inspector opens with
a low-disk banner. The banner reuses the receipt's `low_disk_context`:

- `state_class` — `warning`, `critical`, or `quota_pressure`.
- `ordered_eviction_steps` — strictly increasing by order. Eviction
  always trims disposable classes first (hot, knowledge), then
  recreatable provider-derived caches (artifact, prebuild). Protected
  classes never appear in the eviction list.
- `paused_work_rows` — background work that paused during eviction.
- `reviewer_summary` — explains what was trimmed first and what is now
  stale or needs rebuild.

The banner always offers an **Open inspector** action that routes to the
class breakdown so the user can review what changed.

## Cleanup receipts and history

Every cleanup action mints a `storage_cleanup_receipt_record` paired with
its review sheet:

- `actor_lineage_class`, `executed_at`, `trigger_class`,
- `result_class` — `completed`, `partial`, `blocked_by_pin`,
  `blocked_by_protected_class`, `cancelled`, or `no_op_nothing_to_reclaim`,
- `class_outcomes` — bytes reclaimed and rebuild-state class per class,
- `blocked_pin_rows` — pins that blocked deletion,
- `skipped_protected_class_rows` — protected classes excluded by default,
- optional `low_disk_context` (required when triggered by low-disk
  pressure),
- `reopen_inspector_action_ref` so the cleanup-history surface and
  support exports always carry a reopen route.

Receipts are metadata-only: `raw_content_exported` is `false` and
`redaction_class` is `metadata_safe_default`. The receipt schema refuses
secrets, raw paths, raw policy bodies, and credential bodies.

## Support export

`StorageCleanupCorpus::support_export` folds the registry, every
seeded scenario, and the cleanup receipts into one metadata-safe
envelope (`storage_cleanup_support_export_envelope`). Support packets,
release-evidence projections, and the cleanup-history surface quote
this envelope verbatim instead of inventing divergent shapes.

## Forbidden patterns

The harness refuses any of the following:

- A `clear cache` action that does not name selected classes.
- A review sheet that drops `evidence_support_cache` or
  `user_owned_recovery_state` from `protected_class_rows` without an
  explicit override.
- A receipt that fails to record skipped protected classes after a
  cleanup run.
- A low-disk receipt without a populated `low_disk_context`.
- An override of a durable class (`evidence_support_cache`) without an
  export-before-delete target.
- A cancelled review that reports nonzero `bytes_reclaimed`.

## Adding a new class

Adding a storage class requires:

1. A new entry in `fixtures/support/m3/storage_cleanup/registry.yaml`
   with a unique `low_disk_eviction_priority`.
2. A matching token in `StorageClassId` and the JSON-schema enums.
3. A reviewer-safe summary in this doc.
4. A seeded scenario fixture that exercises the new class in at least
   one trigger context.
