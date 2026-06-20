# Materialized-view-class policy — evidence report

The canonical materialized-view-class persistence, export, and delete
policy is implemented in
[`crates/aureline-reactive-state/src/materialized_view_policy/mod.rs`](../../crates/aureline-reactive-state/src/materialized_view_policy/mod.rs)
and serialized to
[`artifacts/state/materialized_view_policy.json`](./materialized_view_policy.json).

The reviewer contract lives at
[`docs/state/materialized_view_policy.md`](../../docs/state/materialized_view_policy.md);
the boundary schema at
[`schemas/state/materialized_view_policy.schema.json`](../../schemas/state/materialized_view_policy.schema.json).

It is the checked-in truth source for:

- the per-surface materialized-view declarations in
  [`crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs`](../../crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs)
- the metadata-safe support-export consumer in
  [`crates/aureline-support/src/materialized_view_policy/mod.rs`](../../crates/aureline-support/src/materialized_view_policy/mod.rs)
- fixture replay in
  [`crates/aureline-reactive-state/tests/materialized_view_policy.rs`](../../crates/aureline-reactive-state/tests/materialized_view_policy.rs)

## Frozen evidence

The packet proves:

- every materialized view declares one of four classes whose
  persistence, retention, export, delete, hold/offboarding, and
  support-bundle behavior follow from the class — not from a storage
  location alone;
- no class presents exact current truth on read — every read is a
  derived projection of an authority;
- one full per-class disposition matrix for the clear-data, export,
  support-bundle, offboarding, and restore operations, so each flow
  ingests the same table instead of reimplementing class behavior;
- the guardrail that managed-replicated and exportable-snapshot classes
  carry distinct retention, delete, hold, export, and support-bundle
  semantics from the ephemeral class — neither can silently inherit
  ephemeral eviction or retention;
- that exportable snapshots are user-authored artifacts: a clear-data
  sweep preserves them and they are restored from the saved copy, never
  rebuilt from authority.

## Class policy

| Class | Persistence | Retention | Export | Clear-data | Hold / offboarding | Support bundle |
| --- | --- | --- | --- | --- | --- | --- |
| ephemeral_projection | memory_only | until_scope_change | not_exportable | evict_on_scope_change | no_persisted_state_to_hold | excluded_from_bundle |
| durable_local_materialization | local_cache_or_db | until_cache_eviction_or_clear | metadata_only_export | clear_or_rebuild | cleared_on_offboarding | metadata_safe_in_bundle |
| exportable_snapshot | saved_artifact | until_artifact_deleted | exportable_snapshot_artifact | preserve_saved_artifact | retainable_under_hold | snapshot_eligible_with_consent |
| managed_replicated_view | service_or_local_mirror | until_replication_lease_ends | replica_metadata_export | revoke_replica_reconcile_on_reconnect | governed_by_managed_retention | replica_metadata_in_bundle |

## Disposition matrix

| Class | clear_data | export | support_bundle | offboarding | restore |
| --- | --- | --- | --- | --- | --- |
| ephemeral_projection | evicted_from_memory | excluded_no_persisted_state | excluded_from_bundle | nothing_to_hold | rebuilt_from_authority |
| durable_local_materialization | cleared_rebuildable_from_authority | metadata_only_exported | metadata_safe_in_bundle | local_cache_cleared_on_offboarding | rebuilt_from_authority |
| exportable_snapshot | saved_artifact_preserved | snapshot_artifact_exported | snapshot_eligible_with_consent | retained_under_hold | restored_from_saved_artifact |
| managed_replicated_view | local_replica_revoked_reconcile_later | replica_metadata_exported | replica_metadata_in_bundle | governed_by_managed_retention | reconciled_from_managed_source |

## Replay

The fixtures under
[`fixtures/state/materialized_view_policy/`](../../fixtures/state/materialized_view_policy/)
each bind one class and one lifecycle operation to the expected
disposition. The integration test replays the on-disk packet and fixture
corpus against the seeded projection, so any drift between the policy,
the artifact, the schema, and the fixtures fails CI.
