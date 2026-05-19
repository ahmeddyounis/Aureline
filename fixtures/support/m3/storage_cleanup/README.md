# M3 storage cleanup corpus

This corpus seeds the storage-class registry, class-selective
clear-data review sheets, and storage cleanup receipts consumed by the
storage inspector at
[`crates/aureline-support/src/storage_inspector/`](../../../../crates/aureline-support/src/storage_inspector/mod.rs)
and the reviewer doc at
[`docs/support/m3/storage_cleanup_beta.md`](../../../../docs/support/m3/storage_cleanup_beta.md).

Every scenario pairs a [`clear_data_review_record`](../../../../schemas/support/clear_data_review.schema.json)
with a matching [`storage_cleanup_receipt_record`](../../../../schemas/support/storage_cleanup_receipt.schema.json)
so support exports, low-disk banners, and cleanup history never relabel
ordinary cleanup as a destructive umbrella. The registry at
[`registry.yaml`](registry.yaml) mirrors
[`storage_class.schema.json`](../../../../schemas/support/storage_class.schema.json)
and covers all six required classes:

| Class | Authority | GC policy | Protected by default |
| ----- | --------- | --------- | -------------------- |
| `interactive_hot_cache` | `runtime_disposable` | `eviction_eligible` | no |
| `knowledge_cache` | `workspace_derived` | `eviction_eligible` | no |
| `artifact_cache` | `provider_derived` | `eviction_with_review` | no |
| `prebuild_environment_cache` | `provider_derived` | `eviction_with_review` | no |
| `evidence_support_cache` | `evidence_grade` | `never_evict_silently` | yes |
| `user_owned_recovery_state` | `user_authored_recovery` | `never_evict_silently` | yes |

The seeded scenarios exercise the trigger classes the inspector must
keep separate and explainable:

| Scenario fixture | Trigger | Expected result |
| ---------------- | ------- | --------------- |
| `scenario.user_requested_hot_and_knowledge_cleanup.yaml` | `user_requested_cleanup` | `completed` |
| `scenario.low_disk_ordered_eviction.yaml` | `low_disk_pressure` | `completed` |
| `scenario.evidence_export_before_delete.yaml` | `user_requested_cleanup` (admin override) | `partial` |
| `scenario.pinned_artifact_blocks_cleanup.yaml` | `user_requested_cleanup` (pin blocked) | `partial` |
| `scenario.corruption_repair_prebuild_cache.yaml` | `corruption_repair_request` | `completed` |
| `scenario.cancelled_recovery_state_override.yaml` | `user_requested_cleanup` (cancelled) | `cancelled` |

Every scenario preserves the same baseline:

- `evidence_support_cache` and `user_owned_recovery_state` are listed
  in `protected_class_rows` and `skipped_protected_class_rows` unless
  an explicit per-item override appears in
  `override_protected_class_refs`;
- a `low_disk_pressure` trigger always carries a populated
  `low_disk_context` with at least one ordered eviction step,
  any paused work, and a reviewer summary;
- the receipt carries a `reopen_inspector_action_ref` so support
  exports can quote the same reopen route;
- `raw_content_exported` is `false` and `redaction_class` is
  `metadata_safe_default` so the projection stays metadata-safe.

Adding a new trigger or storage class requires both a new fixture
here and a matching enum extension in the consumer module — the
harness refuses a corpus that is missing a required trigger or that
folds protected classes into the disposable set.
