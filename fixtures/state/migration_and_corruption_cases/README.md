# State migration and corruption-routing fixtures

These fixtures are short, reviewable scenarios that anchor the
state-object inventory and corruption-routing matrix frozen in
[`/docs/state/state_object_inventory.md`](../../../docs/state/state_object_inventory.md),
[`/artifacts/state/state_objects.yaml`](../../../artifacts/state/state_objects.yaml),
and
[`/artifacts/state/corruption_routing_matrix.yaml`](../../../artifacts/state/corruption_routing_matrix.yaml),
and validated by
[`/schemas/state/restore_provenance.schema.json`](../../../schemas/state/restore_provenance.schema.json).

Each fixture is one `state_restore_provenance_record` rendered as the
provenance emitted when a detector picked a corruption posture for an
inventory row, or when a destructive migration ran through the
backup-before-migrate rule. The set exists so reviewers can diff a
closed corruption-posture class, authority class, preserved-prior-
artifact rule, and consumer-surface binding without reverse-
engineering it from artifact-specific docs.

## Scope rules

- Fixtures validate against the shared restore-provenance schema; they
  do not encode raw profile bytes, raw cache shards, raw admin
  signatures, or raw support-bundle bodies.
- A new fixture MUST exercise at least one corruption-posture class
  (`block_feature_only`, `rebuild_automatically`, `open_with_warning`,
  `repair_flow`, `backup_rollback`,
  `fail_closed_for_privileged_operations`) or the backup-before-migrate
  rule, and MUST cite the inventory row and posture it illustrates.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- `portable_settings`, `local_context`, `workspace_shared_manifest`,
  and `non_portable_live_authority` are boundaries, not permission to
  flatten unlike things into one blob.

## Index

| Fixture | Posture | Inventory row | Key coverage |
|---|---|---|---|
| [`settings_backup_rollback_digest_mismatch.json`](./settings_backup_rollback_digest_mismatch.json) | `backup_rollback` | `user_global_settings` | integrity-digest mismatch on user-authored durable truth routes to the preserved prior artifact |
| [`workspace_manifest_repair_flow_schema_meaning_changed.json`](./workspace_manifest_repair_flow_schema_meaning_changed.json) | `repair_flow` | `workspace_manifest` | schema-meaning shift on workspace manifest stops at an explicit diff-review rung |
| [`index_cache_rebuild_automatically_digest_mismatch.json`](./index_cache_rebuild_automatically_digest_mismatch.json) | `rebuild_automatically` | `index_cache` | disposable-derived cache rebuilds from authoritative truth without touching user work |
| [`dirty_buffer_journal_open_with_warning_truncated_frame.json`](./dirty_buffer_journal_open_with_warning_truncated_frame.json) | `open_with_warning` | `dirty_buffer_recovery_journal` | replay stops at the last readable frame and opens with a warning; never silently rebuilt |
| [`admin_policy_bundle_fail_closed_signature_verification_failed.json`](./admin_policy_bundle_fail_closed_signature_verification_failed.json) | `fail_closed_for_privileged_operations` | `admin_policy_bundle` | signature-verification failure refuses privileged operations while editing continues under warning |
| [`support_bundle_block_feature_only_io_error.json`](./support_bundle_block_feature_only_io_error.json) | `block_feature_only` | `support_bundles` | storage-IO error on an individual support bundle disables its export action only |
| [`settings_compatible_migration_with_backup_preserved.json`](./settings_compatible_migration_with_backup_preserved.json) | `backup_rollback` (migration path) | `user_global_settings` | destructive migration preserves the pre-translation body via the backup-before-migrate rule |

## Coverage contract

The shared fixture set MUST keep:

- at least one case for each of the six corruption-posture classes
  (`block_feature_only`, `rebuild_automatically`, `open_with_warning`,
  `repair_flow`, `backup_rollback`,
  `fail_closed_for_privileged_operations`);
- at least one case that exercises the backup-before-migrate rule on
  a `migrating_with_equivalence_map` row;
- at least one case that makes the distinction between "blocks
  privileged operations" and "blocks the whole editor" explicit by
  keeping non-privileged editing reachable;
- at least one case for a derived disposable cache that rebuilds
  automatically without preserving a prior artifact.
