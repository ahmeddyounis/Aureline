# M3 storage-cleanup drill report

Baseline projection of the protected storage-cleanup drill corpus at
[`fixtures/recovery/m3/storage_and_cleanup_corpus/`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/).

This artifact pins the failure / recovery, conformance / interop, and
support-export parity drills that the M3 storage-truth lane MUST keep
passing. The drills replay on top of the shared storage-class
registry, the class-selective clear-data review sheets, and the
storage cleanup receipts shipped by the storage inspector
([`crates/aureline-support/src/storage_inspector/`](../../../crates/aureline-support/src/storage_inspector/mod.rs)).
Boundary schemas the drills quote verbatim:

- [`schemas/support/storage_class.schema.json`](../../../schemas/support/storage_class.schema.json)
- [`schemas/support/clear_data_review.schema.json`](../../../schemas/support/clear_data_review.schema.json)
- [`schemas/support/storage_cleanup_receipt.schema.json`](../../../schemas/support/storage_cleanup_receipt.schema.json)

The report is metadata-safe: every row carries closed-vocabulary
tokens from the drill manifest. No raw payload bodies, raw paths,
raw credential bodies, or raw policy bodies appear in this report.

- `record_kind`: `storage_cleanup_drill_report_record`
- `schema_version`: `1`
- `report_id`: `m3.storage_cleanup_drill_report.baseline.v1`
- `corpus_manifest_ref`: [`fixtures/recovery/m3/storage_and_cleanup_corpus/manifest.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/manifest.yaml)
- `matrix_ref`: [`artifacts/support/m3/storage_class_matrix.json`](storage_class_matrix.json)
- `reviewer_doc_ref`: [`docs/qe/m3/storage_cleanup_drills.md`](../../../docs/qe/m3/storage_cleanup_drills.md)
- `storage_cleanup_beta_doc_ref`: [`docs/support/m3/storage_cleanup_beta.md`](../../../docs/support/m3/storage_cleanup_beta.md)
- `raw_private_material_excluded`: `true`
- `ambient_authority_excluded`: `true`

## Required trigger classes

The drill corpus exercises every trigger class declared in the
storage-cleanup review and receipt schemas:

- `user_requested_cleanup`
- `low_disk_pressure`
- `quota_pressure`
- `corruption_repair_request`

## Required drill classes

Twelve closed drill classes are pinned by the corpus manifest:

- `low_disk_ordered_eviction`
- `quota_pressure_managed_explainer`
- `prefetch_pause_under_pressure`
- `pinned_artifact_protection`
- `export_before_delete_durable_class`
- `corruption_targeted_repair_one_index`
- `corruption_targeted_repair_one_docs_pack`
- `corruption_targeted_repair_one_preview_runtime`
- `offline_continuity_consequence`
- `certified_workspace_evidence_consequence`
- `cleanup_history_receipt_parity`
- `protected_class_refusal_without_explicit_selection`

## Per-drill rows

Every row below is one `storage_cleanup_drill_report_row` projected
from the corpus. Reviewer columns name the drill class, the trigger
that fires it, the anchor scenario it replays, the expected receipt
result class, and the closed acceptance assertions the drill enforces.

### `low_disk_ordered_eviction` — Critical low-disk pressure trims disposable and recreatable caches in priority order

- Case file: [`case.low_disk_ordered_eviction.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.low_disk_ordered_eviction.yaml)
- Trigger class: `low_disk_pressure`
- Anchor scenario: [`scenario.low_disk_ordered_eviction.yaml`](../../../fixtures/support/m3/storage_cleanup/scenario.low_disk_ordered_eviction.yaml)
- Expected result class: `completed`
- Eviction order: `interactive_hot_cache` → `knowledge_cache` → `artifact_cache`
- Acceptance assertions: `editing_save_and_local_history_preserved_under_pressure`,
  `prefetch_and_rebuildable_derived_state_shed_in_documented_order`,
  `protected_classes_never_evicted_without_explicit_override`,
  `reopen_inspector_action_present_after_destructive_cleanup`

### `quota_pressure_managed_explainer` — Managed quota cap on the knowledge cache is explained, not silenced

- Case file: [`case.quota_pressure_managed_explainer.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.quota_pressure_managed_explainer.yaml)
- Trigger class: `quota_pressure`
- Anchor scenario: drill-only (no underlying storage-cleanup scenario)
- Expected result class: `partial`
- Quota source class: `managed_policy` (workspace cap on `knowledge_cache`)
- Acceptance assertions: `managed_quota_source_named_in_banner`,
  `prefetch_and_rebuildable_derived_state_shed_in_documented_order`,
  `protected_classes_never_evicted_without_explicit_override`,
  `reopen_inspector_action_present_after_destructive_cleanup`

### `prefetch_pause_under_pressure` — Prefetch and prewarm background work pauses under disk pressure

- Case file: [`case.prefetch_pause_under_pressure.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.prefetch_pause_under_pressure.yaml)
- Trigger class: `low_disk_pressure`
- Anchor scenario: [`scenario.low_disk_ordered_eviction.yaml`](../../../fixtures/support/m3/storage_cleanup/scenario.low_disk_ordered_eviction.yaml)
- Expected result class: `completed`
- Paused work refs: `work.background.knowledge_reindex`, `work.background.artifact_prewarm`
- Acceptance assertions: `editing_save_and_local_history_preserved_under_pressure`,
  `prefetch_and_rebuildable_derived_state_shed_in_documented_order`,
  `protected_classes_never_evicted_without_explicit_override`

### `pinned_artifact_protection` — Pinned review artifact and rollback checkpoint block partial cleanup

- Case file: [`case.pinned_artifact_protection.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.pinned_artifact_protection.yaml)
- Trigger class: `user_requested_cleanup`
- Anchor scenario: [`scenario.pinned_artifact_blocks_cleanup.yaml`](../../../fixtures/support/m3/storage_cleanup/scenario.pinned_artifact_blocks_cleanup.yaml)
- Expected result class: `partial`
- Blocked pin sources: `user_pin`, `rollback_checkpoint_pin`
- Acceptance assertions: `protected_classes_never_evicted_without_explicit_override`,
  `support_export_matches_cleanup_history_receipt`,
  `reopen_inspector_action_present_after_destructive_cleanup`

### `export_before_delete_durable_class` — Evidence cache override requires export-before-delete to a named local target

- Case file: [`case.export_before_delete_durable_class.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.export_before_delete_durable_class.yaml)
- Trigger class: `user_requested_cleanup`
- Anchor scenario: [`scenario.evidence_export_before_delete.yaml`](../../../fixtures/support/m3/storage_cleanup/scenario.evidence_export_before_delete.yaml)
- Expected result class: `partial`
- Override class: `evidence_support_cache` (export target `target.support_export.local_bundle`)
- Acceptance assertions: `protected_classes_never_evicted_without_explicit_override`,
  `offline_or_certified_evidence_consequence_named_before_delete`,
  `reopen_inspector_action_present_after_destructive_cleanup`,
  `support_export_matches_cleanup_history_receipt`

### `corruption_targeted_repair_one_index` — Rebuild one corrupted knowledge index instead of clearing every cache

- Case file: [`case.corruption_targeted_repair_one_index.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.corruption_targeted_repair_one_index.yaml)
- Trigger class: `corruption_repair_request`
- Anchor scenario: [`scenario.corruption_repair_prebuild_cache.yaml`](../../../fixtures/support/m3/storage_cleanup/scenario.corruption_repair_prebuild_cache.yaml)
- Expected result class: `completed`
- Targeted action class: `rebuild_one_knowledge_index` (preferred over `clear_all_caches`)
- Acceptance assertions: `targeted_repair_preferred_over_delete_all`,
  `prefetch_and_rebuildable_derived_state_shed_in_documented_order`,
  `protected_classes_never_evicted_without_explicit_override`,
  `reopen_inspector_action_present_after_destructive_cleanup`

### `corruption_targeted_repair_one_docs_pack` — Refresh one docs pack instead of dropping the whole artifact cache

- Case file: [`case.corruption_targeted_repair_one_docs_pack.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.corruption_targeted_repair_one_docs_pack.yaml)
- Trigger class: `corruption_repair_request`
- Anchor scenario: drill-only
- Expected result class: `completed`
- Targeted action class: `refresh_one_docs_pack` (preferred over `clear_artifact_cache_for_all_workspaces`)
- Acceptance assertions: `targeted_repair_preferred_over_delete_all`,
  `prefetch_and_rebuildable_derived_state_shed_in_documented_order`,
  `protected_classes_never_evicted_without_explicit_override`,
  `reopen_inspector_action_present_after_destructive_cleanup`

### `corruption_targeted_repair_one_preview_runtime` — Drop one corrupted preview runtime instead of clearing the prebuild cache

- Case file: [`case.corruption_targeted_repair_one_preview_runtime.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.corruption_targeted_repair_one_preview_runtime.yaml)
- Trigger class: `corruption_repair_request`
- Anchor scenario: drill-only
- Expected result class: `completed`
- Targeted action class: `drop_one_preview_runtime` (preferred over `clear_prebuild_environment_cache`)
- Acceptance assertions: `targeted_repair_preferred_over_delete_all`,
  `prefetch_and_rebuildable_derived_state_shed_in_documented_order`,
  `protected_classes_never_evicted_without_explicit_override`,
  `reopen_inspector_action_present_after_destructive_cleanup`

### `offline_continuity_consequence` — Artifact cleanup names the offline-readiness consequence before deleting

- Case file: [`case.offline_continuity_consequence.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.offline_continuity_consequence.yaml)
- Trigger class: `user_requested_cleanup`
- Anchor scenario: drill-only
- Expected result class: `partial`
- Affected continuity class: `offline_workspace_readiness` (workspace `workspace.local.aureline_offline_mirror`)
- Acceptance assertions: `offline_or_certified_evidence_consequence_named_before_delete`,
  `protected_classes_never_evicted_without_explicit_override`,
  `support_export_matches_cleanup_history_receipt`,
  `reopen_inspector_action_present_after_destructive_cleanup`

### `certified_workspace_evidence_consequence` — Certified workspace evidence override states irrecoverable consequence

- Case file: [`case.certified_workspace_evidence_consequence.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.certified_workspace_evidence_consequence.yaml)
- Trigger class: `user_requested_cleanup`
- Anchor scenario: [`scenario.evidence_export_before_delete.yaml`](../../../fixtures/support/m3/storage_cleanup/scenario.evidence_export_before_delete.yaml)
- Expected result class: `partial`
- Affected continuity class: `certified_workspace_evidence` (workspace `workspace.profile.support_profile`)
- Acceptance assertions: `offline_or_certified_evidence_consequence_named_before_delete`,
  `protected_classes_never_evicted_without_explicit_override`,
  `support_export_matches_cleanup_history_receipt`,
  `reopen_inspector_action_present_after_destructive_cleanup`

### `cleanup_history_receipt_parity` — Cleanup-history surface, support export, and receipt agree row-for-row

- Case file: [`case.cleanup_history_receipt_parity.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.cleanup_history_receipt_parity.yaml)
- Trigger class: `low_disk_pressure`
- Anchor scenario: [`scenario.low_disk_ordered_eviction.yaml`](../../../fixtures/support/m3/storage_cleanup/scenario.low_disk_ordered_eviction.yaml)
- Expected result class: `completed`
- Parity targets: `cleanup_history_row`, `support_export_envelope_row`, `storage_cleanup_receipt_record`
- Parity fields: `receipt_id`, `review_ref`, `executed_at`, `actor_lineage_class`,
  `trigger_class`, `result_class`, `class_outcomes`, `blocked_pin_rows`,
  `skipped_protected_class_rows`, `reopen_inspector_action_ref`
- Acceptance assertions: `support_export_matches_cleanup_history_receipt`,
  `reopen_inspector_action_present_after_destructive_cleanup`,
  `protected_classes_never_evicted_without_explicit_override`

### `protected_class_refusal_without_explicit_selection` — Vague clear-cache request that would touch protected classes is refused

- Case file: [`case.protected_class_refusal_without_explicit_selection.yaml`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/case.protected_class_refusal_without_explicit_selection.yaml)
- Trigger class: `user_requested_cleanup`
- Anchor scenario: refusal-only (no anchor scenario, no receipt emitted)
- Expected result class: `blocked_by_protected_class`
- Refused class ids: `evidence_support_cache`, `user_owned_recovery_state`
- Acceptance assertions: `protected_classes_never_evicted_without_explicit_override`,
  `clear_all_without_class_selection_is_refused`

## How acceptance assertions become claim downgrades

The corpus enforces three exit-gate guarantees the M3 storage lane
relies on. A drill regression flips the matching closed downgrade
trigger from green to red, which scorecards downstream of this report
quote verbatim:

- `protected_classes_never_evicted_without_explicit_override` —
  regression flags `red_blocks_beta_claim` on the storage-cleanup
  lane. Any drill that deletes `evidence_support_cache` or
  `user_owned_recovery_state` without an `override_protected_class_refs`
  row is refused.
- `editing_save_and_local_history_preserved_under_pressure` —
  regression flags `red_blocks_beta_claim`. Any pressure drill that
  trims editing buffers, save targets, dirty-buffer journals,
  rollback checkpoints, or local history fails the gate.
- `support_export_matches_cleanup_history_receipt` — regression
  ages the lane to `yellow_aging_drill_evidence`. The receipt,
  cleanup-history row, and support-export envelope row MUST agree on
  what was removed, blocked, or left pinned. Divergence is treated
  as stale evidence until the three projections re-converge.

The remaining assertions
(`prefetch_and_rebuildable_derived_state_shed_in_documented_order`,
`reopen_inspector_action_present_after_destructive_cleanup`,
`targeted_repair_preferred_over_delete_all`,
`offline_or_certified_evidence_consequence_named_before_delete`,
`managed_quota_source_named_in_banner`,
`clear_all_without_class_selection_is_refused`) propagate as drill
findings on the affected case; any regression on more than one is
treated as a `red_blocks_beta_claim` signal until the underlying
fixture or implementation change has been reviewed.

## How to refresh

1. Update the relevant case file in
   [`fixtures/recovery/m3/storage_and_cleanup_corpus/`](../../../fixtures/recovery/m3/storage_and_cleanup_corpus/)
   and the manifest if a drill or acceptance class is added.
2. Update the matching reviewer row in
   [`docs/qe/m3/storage_cleanup_drills.md`](../../../docs/qe/m3/storage_cleanup_drills.md)
   so the QE drill doc, this report, and the corpus stay aligned.
3. Update [`artifacts/support/m3/storage_class_matrix.json`](storage_class_matrix.json)
   if the coverage between drills and storage classes changes.
4. Land all four files in the same change so a reviewer can confirm
   the corpus, the matrix, the report, and the QE doc agree row for
   row.
