# Repair-transaction preview-skeleton beta fixtures

These protected fixtures cover the beta repair-transaction preview-skeleton
contract: each scenario binds a skeleton record (typed blast-radius class,
compensation class, affected object classes, checkpoint disposition, and
cancellable preview disposition) to a comparison record that lets a
reviewer cancel or compare before any apply path runs.

The fixtures are loaded by
[`crates/aureline-support/tests/repair_transaction_preview_beta.rs`](../../../../crates/aureline-support/tests/repair_transaction_preview_beta.rs)
and mirror the boundary schema at
[`/schemas/support/repair_transaction_preview_skeleton.schema.json`](../../../../schemas/support/repair_transaction_preview_skeleton.schema.json).

Scenarios:

- **cache_index_rebuild_single_object** — Disposable derived-cache rebuild.
  `transaction_reversal_class = regenerate`,
  `compensation_class = regenerate_from_authoritative_source`,
  `blast_radius_class = single_object_class`,
  `checkpoint_disposition_class = durable_pre_apply_checkpoint`. The
  comparison surfaces a `compensation_diff` against a no-checkpoint baseline.
- **extension_quarantine_compensating** — Extension quarantine.
  `transaction_reversal_class = compensating`,
  `compensation_class = semantic_inverse_compensation`,
  `blast_radius_class = multi_object_class_same_family`,
  `checkpoint_disposition_class = durable_pre_apply_checkpoint`. The
  comparison surfaces an `affected_object_diff` and `blast_radius_diff`.
- **remote_agent_rollback_manual** — Remote agent rollback.
  `transaction_reversal_class = manual`,
  `compensation_class = manual_followup_required`,
  `blast_radius_class = multi_object_class_cross_family`,
  `checkpoint_disposition_class = durable_pre_apply_checkpoint`. The
  comparison surfaces a `compensation_diff` and `reversal_class_diff`.
- **escalation_only_no_local_blast** — Guided export escalation.
  `transaction_reversal_class = audit_only`,
  `compensation_class = audit_only_no_state_change`,
  `blast_radius_class = no_local_blast_escalation_only`,
  `checkpoint_disposition_class = no_checkpoint_escalation_only`. The
  comparison surfaces a `preserved_state_diff` against an earlier broader
  baseline.
