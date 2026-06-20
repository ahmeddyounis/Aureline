# Materialized-view-class persistence, export, and delete policy

The canonical materialized-view-class policy is implemented in
[`crates/aureline-reactive-state/src/materialized_view_policy/mod.rs`](../../crates/aureline-reactive-state/src/materialized_view_policy/mod.rs)
and serialized to
[`artifacts/state/materialized_view_policy.json`](../../artifacts/state/materialized_view_policy.json).

It is the single checked-in truth source for how each materialized-view
class persists, retains, exports, deletes, holds, and contributes to a
support bundle. The per-surface reactive-governance matrix in
[`crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs`](../../crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs)
tags each M5 surface with a view *class*; this policy is where that class
is bound to concrete lifecycle behavior so clear-data, support,
offboarding, restore, and export flows ingest one table instead of
inferring behavior from a storage location.

The four classes mirror Appendix DB.3 and the subscription-envelope
vocabulary frozen in
[`crates/aureline-reactive-state/src/envelope.rs`](../../crates/aureline-reactive-state/src/envelope.rs)
token for token.

## The four classes

- **`ephemeral_projection`** — a memory-only derived projection, rebuilt
  on demand and evicted when its scope changes. Never exported, never in
  a support bundle, nothing to hold.
- **`durable_local_materialization`** — a local cache/db materialization.
  Clear-data drops the cache and the view rebuilds from authority;
  exports metadata only; cleared on offboarding.
- **`exportable_snapshot`** — a saved, user-authored snapshot artifact.
  It **survives a clear-data sweep**, can be **retained under a hold**
  during offboarding, is itself the export, and is **restored from the
  saved copy**, never rebuilt from authority.
- **`managed_replicated_view`** — a service-backed or locally mirrored
  replica. Clear-data revokes the local replica and it reconciles from
  the managed source; hold and offboarding follow the managed retention
  policy.

## What the policy binds per class

Each class row declares:

- **`authority_on_read`** — always `derived_projection`; a read of a
  materialized view never presents the owning authority's exact current
  truth.
- **`persistence`** — `memory_only`, `local_cache_or_db`,
  `saved_artifact`, or `service_or_local_mirror`.
- **`retention`** — `until_scope_change`, `until_cache_eviction_or_clear`,
  `until_artifact_deleted`, or `until_replication_lease_ends`.
- **`export`** — `not_exportable`, `metadata_only_export`,
  `exportable_snapshot_artifact`, or `replica_metadata_export`.
- **`delete_semantics`** — what a clear-data sweep does:
  `evict_on_scope_change`, `clear_or_rebuild`, `preserve_saved_artifact`,
  or `revoke_replica_reconcile_on_reconnect`.
- **`hold_offboarding`** — `no_persisted_state_to_hold`,
  `cleared_on_offboarding`, `retainable_under_hold`, or
  `governed_by_managed_retention`.
- **`support_bundle`** — `excluded_from_bundle`, `metadata_safe_in_bundle`,
  `snapshot_eligible_with_consent`, or `replica_metadata_in_bundle`.
- **`rebuildable_from_authority`** and **`survives_clear_data`** booleans.

| Class | Persistence | Retention | Export | Clear-data | Hold / offboarding | Support bundle |
| --- | --- | --- | --- | --- | --- | --- |
| ephemeral_projection | memory_only | until_scope_change | not_exportable | evict_on_scope_change | no_persisted_state_to_hold | excluded_from_bundle |
| durable_local_materialization | local_cache_or_db | until_cache_eviction_or_clear | metadata_only_export | clear_or_rebuild | cleared_on_offboarding | metadata_safe_in_bundle |
| exportable_snapshot | saved_artifact | until_artifact_deleted | exportable_snapshot_artifact | preserve_saved_artifact | retainable_under_hold | snapshot_eligible_with_consent |
| managed_replicated_view | service_or_local_mirror | until_replication_lease_ends | replica_metadata_export | revoke_replica_reconcile_on_reconnect | governed_by_managed_retention | replica_metadata_in_bundle |

## Disposition matrix

The packet also carries the full per-class disposition for the five
lifecycle operations. Flows quote this matrix rather than re-deriving
behavior from a storage location.

| Class | clear_data | export | support_bundle | offboarding | restore |
| --- | --- | --- | --- | --- | --- |
| ephemeral_projection | evicted_from_memory | excluded_no_persisted_state | excluded_from_bundle | nothing_to_hold | rebuilt_from_authority |
| durable_local_materialization | cleared_rebuildable_from_authority | metadata_only_exported | metadata_safe_in_bundle | local_cache_cleared_on_offboarding | rebuilt_from_authority |
| exportable_snapshot | saved_artifact_preserved | snapshot_artifact_exported | snapshot_eligible_with_consent | retained_under_hold | restored_from_saved_artifact |
| managed_replicated_view | local_replica_revoked_reconcile_later | replica_metadata_exported | replica_metadata_in_bundle | governed_by_managed_retention | reconciled_from_managed_source |

## Guardrails

- **No ephemeral inheritance.** The exportable-snapshot and
  managed-replicated classes deliberately use distinct retention,
  delete, hold, export, and support-bundle tokens from the ephemeral
  class. Validation rejects any class row that inherits ephemeral
  eviction or retention, so a durable or managed view can never be
  silently swept like a cache.
- **Distinct clear-data per class.** Every class has a distinct
  clear-data disposition, so clear-data tooling cannot infer behavior
  from a storage location alone.
- **User-authored artifacts are preserved.** An exportable snapshot is a
  user-authored file; a clear-data sweep preserves it and it is restored
  from the saved copy.

## Consumers

Later clear-data, support, offboarding, restore, and export surfaces
ingest this policy instead of inventing local lifecycle wording:

- [`crates/aureline-support/src/materialized_view_policy/mod.rs`](../../crates/aureline-support/src/materialized_view_policy/mod.rs)
  — the metadata-safe support-export consumer.
- [`crates/aureline-support/src/m5_clear_data_review/mod.rs`](../../crates/aureline-support/src/m5_clear_data_review/mod.rs)
- [`crates/aureline-support/src/m5_offboarding_continuity/mod.rs`](../../crates/aureline-support/src/m5_offboarding_continuity/mod.rs)
- [`crates/aureline-support/src/records_export_delete_governance/mod.rs`](../../crates/aureline-support/src/records_export_delete_governance/mod.rs)
