# Storage-cleanup drills

QE reviewer doc for the M3 storage-truth and cleanup-safety lane.
Pairs with:

- the protected drill corpus at
  [`fixtures/recovery/m3/storage_and_cleanup_corpus/`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/),
- the storage-class registry at
  [`fixtures/support/m3/storage_cleanup/registry.yaml`](../../../fixtures/support/m3/storage_cleanup/registry.yaml),
- the storage-inspector consumer at
  [`crates/aureline-support/src/storage_inspector/`](../../../crates/aureline-support/src/storage_inspector/mod.rs),
- the boundary schemas at
  [`schemas/support/storage_class.schema.json`](../../../schemas/support/storage_class.schema.json),
  [`schemas/support/clear_data_review.schema.json`](../../../schemas/support/clear_data_review.schema.json),
  and
  [`schemas/support/storage_cleanup_receipt.schema.json`](../../../schemas/support/storage_cleanup_receipt.schema.json),
- the baseline drill report at
  [`artifacts/support/m3/storage_cleanup_drill_report.md`](../../../artifacts/support/m3/storage_cleanup_drill_report.md),
- the storage-class coverage matrix at
  [`artifacts/support/m3/storage_class_matrix.json`](../../../artifacts/support/m3/storage_class_matrix.json),
- the storage-cleanup beta reviewer doc at
  [`docs/support/m3/storage_cleanup_beta.md`](../../../docs/support/m3/storage_cleanup_beta.md).

The drills here keep `clear cache` from becoming a vague destructive
umbrella. Each drill replays the relevant low-disk, quota-pressure,
corruption, pinned-artifact, export-before-delete, or offline /
certified-workspace scenario and verifies that the storage inspector,
the clear-data review sheet, the storage cleanup receipt, the
cleanup-history surface, and the support-export envelope all agree on
what was removed, blocked, or left pinned.

## Drill index

| Drill class | Trigger | Anchor scenario | Expected receipt result |
| ----------- | ------- | --------------- | ----------------------- |
| `low_disk_ordered_eviction` | `low_disk_pressure` | `scenario.low_disk_ordered_eviction.yaml` | `completed` |
| `quota_pressure_managed_explainer` | `quota_pressure` | drill-only | `partial` |
| `prefetch_pause_under_pressure` | `low_disk_pressure` | `scenario.low_disk_ordered_eviction.yaml` | `completed` |
| `pinned_artifact_protection` | `user_requested_cleanup` | `scenario.pinned_artifact_blocks_cleanup.yaml` | `partial` |
| `export_before_delete_durable_class` | `user_requested_cleanup` | `scenario.evidence_export_before_delete.yaml` | `partial` |
| `corruption_targeted_repair_one_index` | `corruption_repair_request` | `scenario.corruption_repair_prebuild_cache.yaml` | `completed` |
| `corruption_targeted_repair_one_docs_pack` | `corruption_repair_request` | drill-only | `completed` |
| `corruption_targeted_repair_one_preview_runtime` | `corruption_repair_request` | drill-only | `completed` |
| `offline_continuity_consequence` | `user_requested_cleanup` | drill-only | `partial` |
| `certified_workspace_evidence_consequence` | `user_requested_cleanup` | `scenario.evidence_export_before_delete.yaml` | `partial` |
| `cleanup_history_receipt_parity` | `low_disk_pressure` | `scenario.low_disk_ordered_eviction.yaml` | `completed` |
| `protected_class_refusal_without_explicit_selection` | `user_requested_cleanup` | refusal-only (no receipt) | `blocked_by_protected_class` |

## `drill:low-disk-ordered-eviction` — Critical low-disk pressure trims disposable and recreatable caches in priority order

- Surface: storage inspector, low-disk banner, cleanup history
- Drill class: `low_disk_ordered_eviction`

### Steps

1. Cross the critical low-disk threshold on a workspace that has hot,
   knowledge, and artifact caches resident.
1. Open the storage inspector via the banner's `Open inspector`
   action.
1. Confirm the banner names the ordered eviction
   (`interactive_hot_cache` → `knowledge_cache` → `artifact_cache`)
   and the protected classes pinned under pressure.
1. Verify the receipt's `low_disk_context.ordered_eviction_steps` is
   strictly increasing by `order` and that every step's `class_id`
   appears in the eviction priority order.

### Expected assertions

- `editing_save_and_local_history_preserved_under_pressure`
- `prefetch_and_rebuildable_derived_state_shed_in_documented_order`
- `protected_classes_never_evicted_without_explicit_override`
- `reopen_inspector_action_present_after_destructive_cleanup`

## `drill:quota-pressure-managed-explainer` — Managed quota cap on the knowledge cache is explained, not silenced

- Surface: storage inspector, low-disk banner with quota source
- Drill class: `quota_pressure_managed_explainer`

### Steps

1. A workspace-managed quota policy caps `knowledge_cache`.
1. Cross the quota threshold for that workspace.
1. Confirm the banner names the `managed_policy` quota source, the
   capped class, and the consequence of trimming to the cap.
1. Verify the receipt is `partial` (the trim respected the cap) and
   that protected classes appear in `skipped_protected_class_rows`.

### Expected assertions

- `managed_quota_source_named_in_banner`
- `prefetch_and_rebuildable_derived_state_shed_in_documented_order`
- `protected_classes_never_evicted_without_explicit_override`
- `reopen_inspector_action_present_after_destructive_cleanup`

## `drill:prefetch-pause-under-pressure` — Prefetch and prewarm background work pauses under disk pressure

- Surface: storage inspector, low-disk banner, paused-work rows
- Drill class: `prefetch_pause_under_pressure`

### Steps

1. Start `work.background.knowledge_reindex` and
   `work.background.artifact_prewarm`.
1. Cross the critical low-disk threshold.
1. Confirm both background jobs appear in the receipt's
   `low_disk_context.paused_work_rows`.
1. Verify no editing buffer, save target, dirty-buffer journal,
   rollback checkpoint, or local-history entry was deleted.

### Expected assertions

- `editing_save_and_local_history_preserved_under_pressure`
- `prefetch_and_rebuildable_derived_state_shed_in_documented_order`
- `protected_classes_never_evicted_without_explicit_override`

## `drill:pinned-artifact-protection` — Pinned review artifact and rollback checkpoint block partial cleanup

- Surface: storage inspector, class-selective review sheet, receipt
- Drill class: `pinned_artifact_protection`

### Steps

1. Pin a review artifact (`pin.review_artifact.workspace_release_candidate`)
   and a rollback checkpoint
   (`pin.rollback_checkpoint.workspace_release_candidate`).
1. Open the review sheet for the `artifact_cache` class.
1. Confirm both pins appear in `pin_block_rows` with the right
   `pin_source_class` (`user_pin`, `rollback_checkpoint_pin`).
1. Run cleanup and verify the receipt records both pins under
   `blocked_pin_rows`, `result_class` is `partial`, and a
   `reopen_inspector_action_ref` is present.

### Expected assertions

- `protected_classes_never_evicted_without_explicit_override`
- `support_export_matches_cleanup_history_receipt`
- `reopen_inspector_action_present_after_destructive_cleanup`

## `drill:export-before-delete-durable-class` — Evidence cache override requires export-before-delete

- Surface: storage inspector, class-selective review sheet,
  export-before-delete chooser, receipt
- Drill class: `export_before_delete_durable_class`

### Steps

1. Request an override of `evidence_support_cache` as an admin.
1. Confirm the review sheet refuses to proceed without a named
   `export_target_ref` (here `target.support_export.local_bundle`).
1. Complete the export-before-delete and run cleanup.
1. Verify the active incident pin
   (`pin.evidence.active_incident_workspace`) blocks one row, that
   `user_owned_recovery_state` stays in
   `skipped_protected_class_rows`, and that the receipt names the
   `reopen_inspector_action_ref`.

### Expected assertions

- `protected_classes_never_evicted_without_explicit_override`
- `offline_or_certified_evidence_consequence_named_before_delete`
- `reopen_inspector_action_present_after_destructive_cleanup`
- `support_export_matches_cleanup_history_receipt`

## `drill:corruption-targeted-repair-one-index` — Rebuild one corrupted knowledge index instead of clearing every cache

- Surface: storage inspector, repair preview, review sheet
- Drill class: `corruption_targeted_repair_one_index`

### Steps

1. Project Doctor flags a single corrupted knowledge index for
   `workspace.local.aureline_core`.
1. The review sheet offers `rebuild_one_knowledge_index` and
   explicitly downranks `clear_all_caches`.
1. Run the targeted repair.
1. Verify the receipt is `completed`, no other workspace's knowledge
   cache was touched, and `evidence_support_cache` /
   `user_owned_recovery_state` stay in `skipped_protected_class_rows`.

### Expected assertions

- `targeted_repair_preferred_over_delete_all`
- `prefetch_and_rebuildable_derived_state_shed_in_documented_order`
- `protected_classes_never_evicted_without_explicit_override`
- `reopen_inspector_action_present_after_destructive_cleanup`

## `drill:corruption-targeted-repair-one-docs-pack` — Refresh one docs pack instead of dropping the whole artifact cache

- Surface: storage inspector, repair preview, review sheet
- Drill class: `corruption_targeted_repair_one_docs_pack`

### Steps

1. Project Doctor flags a single docs pack in
   `workspace.profile.docs_packs` as integrity-failing.
1. The review sheet offers `refresh_one_docs_pack` and downranks
   `clear_artifact_cache_for_all_workspaces`.
1. Run the targeted refetch.
1. Verify offline readiness of unrelated docs packs is preserved and
   protected classes stay pinned.

### Expected assertions

- `targeted_repair_preferred_over_delete_all`
- `prefetch_and_rebuildable_derived_state_shed_in_documented_order`
- `protected_classes_never_evicted_without_explicit_override`
- `reopen_inspector_action_present_after_destructive_cleanup`

## `drill:corruption-targeted-repair-one-preview-runtime` — Drop one corrupted preview runtime instead of clearing the prebuild cache

- Surface: storage inspector, repair preview, review sheet
- Drill class: `corruption_targeted_repair_one_preview_runtime`

### Steps

1. Project Doctor flags one corrupt preview runtime in
   `workspace.machine.toolchains`.
1. The review sheet offers `drop_one_preview_runtime` and downranks
   `clear_prebuild_environment_cache`.
1. Run the targeted removal.
1. Verify the other toolchains in the prebuild cache stay warm and
   protected classes remain in `skipped_protected_class_rows`.

### Expected assertions

- `targeted_repair_preferred_over_delete_all`
- `prefetch_and_rebuildable_derived_state_shed_in_documented_order`
- `protected_classes_never_evicted_without_explicit_override`
- `reopen_inspector_action_present_after_destructive_cleanup`

## `drill:offline-continuity-consequence` — Artifact cleanup names the offline-readiness consequence before deleting

- Surface: storage inspector, class-selective review sheet,
  offline-continuity consequence banner
- Drill class: `offline_continuity_consequence`

### Steps

1. Select `artifact_cache` cleanup on a workspace with a mirrored
   offline workspace (`workspace.local.aureline_offline_mirror`).
1. Confirm the review sheet states the offline-readiness consequence
   (`offline_workspace_readiness`) and offers an export-before-delete
   of the offline entitlement bundle index.
1. Run cleanup and verify the receipt is `partial` (the mirror is
   the bytes-in-scope) and the user-owned offline entitlement bundle
   stays in `skipped_protected_class_rows`.

### Expected assertions

- `offline_or_certified_evidence_consequence_named_before_delete`
- `protected_classes_never_evicted_without_explicit_override`
- `support_export_matches_cleanup_history_receipt`
- `reopen_inspector_action_present_after_destructive_cleanup`

## `drill:certified-workspace-evidence-consequence` — Certified workspace evidence override states irrecoverable consequence

- Surface: storage inspector, class-selective review sheet,
  override-protected-class row, receipt
- Drill class: `certified_workspace_evidence_consequence`

### Steps

1. An admin overrides `evidence_support_cache` for a certified
   workspace (`workspace.profile.support_profile`).
1. Confirm the review sheet states the irrecoverable consequence
   (`certified_workspace_evidence`) and refuses to proceed without
   the named export-before-delete target.
1. Run cleanup and verify the active incident pin blocks one row,
   `user_owned_recovery_state` remains protected, and the receipt
   names the reopen-inspector route.

### Expected assertions

- `offline_or_certified_evidence_consequence_named_before_delete`
- `protected_classes_never_evicted_without_explicit_override`
- `support_export_matches_cleanup_history_receipt`
- `reopen_inspector_action_present_after_destructive_cleanup`

## `drill:cleanup-history-receipt-parity` — Cleanup-history surface, support export, and receipt agree row-for-row

- Surface: cleanup-history surface, support-export envelope, receipt
- Drill class: `cleanup_history_receipt_parity`

### Steps

1. Run the low-disk ordered-eviction scenario to mint a receipt.
1. Read the receipt back through the support-export envelope
   (`storage_cleanup_support_export_envelope`).
1. Read the receipt back through the cleanup-history surface.
1. Diff `receipt_id`, `review_ref`, `executed_at`,
   `actor_lineage_class`, `trigger_class`, `result_class`,
   `class_outcomes`, `blocked_pin_rows`,
   `skipped_protected_class_rows`, and `reopen_inspector_action_ref`
   across all three projections.

### Expected assertions

- `support_export_matches_cleanup_history_receipt`
- `reopen_inspector_action_present_after_destructive_cleanup`
- `protected_classes_never_evicted_without_explicit_override`

## `drill:protected-class-refusal-without-explicit-selection` — Vague clear-cache request that would touch protected classes is refused

- Surface: storage inspector, class-selective review sheet, refusal
  routing
- Drill class: `protected_class_refusal_without_explicit_selection`

### Steps

1. Request a cleanup action that does not name selected classes (a
   legacy `clear cache` umbrella).
1. Confirm the inspector refuses the action with consent state
   `blocked_by_protected_class` and lists `evidence_support_cache`
   and `user_owned_recovery_state` in the refused class ids.
1. Verify no receipt is emitted, no bytes are deleted, and the user
   is routed back to a class-selective review sheet.

### Expected assertions

- `protected_classes_never_evicted_without_explicit_override`
- `clear_all_without_class_selection_is_refused`

## Reviewer checklist

A drill replay PASSES when, for every drill above:

1. The case file in
   [`fixtures/recovery/m3/storage_and_cleanup_corpus/`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/)
   matches the report row in
   [`artifacts/support/m3/storage_cleanup_drill_report.md`](../../../artifacts/support/m3/storage_cleanup_drill_report.md).
2. The closed acceptance assertions in the case file all hold for
   the replayed receipt, review sheet, and support-export envelope.
3. `evidence_support_cache` and `user_owned_recovery_state` never
   appear outside `skipped_protected_class_rows` (or inside
   `override_protected_class_refs` with a written justification).
4. The cleanup-history row, the support-export envelope row, and the
   receipt agree row for row on what was removed, blocked, or left
   pinned.
5. No raw payload bodies, raw paths, raw credential bodies, or raw
   policy bodies appear in any drill projection.

A regression on (1), (3), or (4) downgrades the lane to
`red_blocks_beta_claim`. A drift between report and corpus that does
not match a fixture change is treated as `stale_corpus_blocks_release_candidate`.
