# Storage-cleanup drill corpus

Protected drill corpus for the M3 storage-truth and cleanup-safety
lane. Each fixture in this directory is one
`storage_cleanup_drill_case_record` that pins:

- one `drill_class` from the closed list declared in
  [`manifest.yaml`](manifest.yaml),
- one `trigger_class` from the closed enum shared with
  [`schemas/support/clear_data_review.schema.json`](../../../../schemas/support/clear_data_review.schema.json)
  and
  [`schemas/support/storage_cleanup_receipt.schema.json`](../../../../schemas/support/storage_cleanup_receipt.schema.json),
- one anchor storage-cleanup scenario fixture under
  [`fixtures/support/m3/storage_cleanup/`](../../../../fixtures/support/m3/storage_cleanup/)
  (or `none` when the drill is a refusal case that must never produce
  a receipt at all),
- a closed `expected_result_class` drawn from the receipt enum,
- at least one acceptance assertion from the closed list in
  [`manifest.yaml`](manifest.yaml).

The drill corpus exists so future changes to the cleanup, low-disk
banner, or quota-pressure paths cannot silently regress into
destructive `clear cache` umbrella behavior. The boundary schemas the
inspector consumes stay frozen; this corpus pins the failure /
recovery drills the lane MUST keep replaying.

## Drill matrix

| Drill class | Trigger | Anchor scenario | Expected result |
| ----------- | ------- | --------------- | --------------- |
| `low_disk_ordered_eviction` | `low_disk_pressure` | `scenario.low_disk_ordered_eviction.yaml` | `completed` |
| `quota_pressure_managed_explainer` | `quota_pressure` | n/a (drill record only) | `partial` |
| `prefetch_pause_under_pressure` | `low_disk_pressure` | `scenario.low_disk_ordered_eviction.yaml` | `completed` |
| `pinned_artifact_protection` | `user_requested_cleanup` | `scenario.pinned_artifact_blocks_cleanup.yaml` | `partial` |
| `export_before_delete_durable_class` | `user_requested_cleanup` | `scenario.evidence_export_before_delete.yaml` | `partial` |
| `corruption_targeted_repair_one_index` | `corruption_repair_request` | `scenario.corruption_repair_prebuild_cache.yaml` | `completed` |
| `corruption_targeted_repair_one_docs_pack` | `corruption_repair_request` | n/a (drill record only) | `completed` |
| `corruption_targeted_repair_one_preview_runtime` | `corruption_repair_request` | n/a (drill record only) | `completed` |
| `offline_continuity_consequence` | `user_requested_cleanup` | n/a (drill record only) | `partial` |
| `certified_workspace_evidence_consequence` | `user_requested_cleanup` | `scenario.evidence_export_before_delete.yaml` | `partial` |
| `cleanup_history_receipt_parity` | `low_disk_pressure` | `scenario.low_disk_ordered_eviction.yaml` | `completed` |
| `protected_class_refusal_without_explicit_selection` | `user_requested_cleanup` | n/a (refusal case) | `blocked_by_protected_class` |

## Invariants every drill enforces

- `evidence_support_cache` and `user_owned_recovery_state` appear in
  `protected_class_rows` and `skipped_protected_class_rows` unless
  they are listed in `override_protected_class_refs` with a written
  justification.
- Editing buffers, save targets, dirty-buffer journals, rollback
  checkpoints, and local history are never deleted by an ordinary
  cleanup or by low-disk eviction.
- Disposable and recreatable derived classes are evicted in the
  documented priority order (hot → knowledge → artifact → prebuild)
  before any review-required class is touched.
- Every destructive drill produces a `reopen_inspector_action_ref` so
  support exports and the cleanup-history surface can route back to
  the inspector.
- `raw_content_exported` is `false` and `redaction_class` is
  `metadata_safe_default` on every emitted review or receipt; no raw
  payload bodies, raw paths, or raw credential or policy bodies are
  permitted in any drill case file.

## Adding a new drill case

1. Add a new `drill_class` token to
   `required_drill_classes` in [`manifest.yaml`](manifest.yaml).
2. Add a corresponding `case.<drill_class>.yaml` file in this
   directory, pinning a `trigger_class`, an anchor scenario (or
   `none`), an `expected_result_class`, and at least one acceptance
   assertion from the closed list.
3. Update [`docs/qe/m3/storage_cleanup_drills.md`](../../../../docs/qe/m3/storage_cleanup_drills.md)
   and [`artifacts/support/m3/storage_cleanup_drill_report.md`](../../../../artifacts/support/m3/storage_cleanup_drill_report.md)
   so reviewers see the new drill described identically across the
   corpus, the reviewer doc, and the baseline report.
4. Extend [`artifacts/support/m3/storage_class_matrix.json`](../../../../artifacts/support/m3/storage_class_matrix.json)
   so the coverage row for the new drill class is checked in
   alongside the case file.
