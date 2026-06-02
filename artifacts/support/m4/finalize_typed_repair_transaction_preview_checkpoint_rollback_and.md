# Finalized Typed Repair-Transaction Preview, Checkpoint, Rollback, and Compensation Artifact

## Summary

This artifact documents the finalized typed repair-transaction preview,
checkpoint, rollback, and compensation flow delivered under the M04 stable lane.

## Schema

- [`schemas/support/finalize_typed_repair_transaction_preview_checkpoint_rollback_and.schema.json`](../../schemas/support/finalize_typed_repair_transaction_preview_checkpoint_rollback_and.schema.json)

## Crate consumer

- [`crates/aureline-support/src/finalize_typed_repair_transaction_preview_checkpoint_rollback_and/mod.rs`](../../crates/aureline-support/src/finalize_typed_repair_transaction_preview_checkpoint_rollback_and/mod.rs)

## Fixture corpus

- [`fixtures/support/m4/finalize_typed_repair_transaction_preview_checkpoint_rollback_and/`](../../fixtures/support/m4/finalize_typed_repair_transaction_preview_checkpoint_rollback_and/)

## Key finalized contracts

| Contract | Record kind | Purpose |
|---|---|---|
| Finalized repair flow | `finalized_repair_transaction_preview_checkpoint_rollback_and_flow_record` | Joins preview, checkpoint, rollback, and compensation into one bounded flow |
| Finalized repair preview | `finalized_repair_preview_record` | Preview disposition, blast radius, and finding lineage |
| Finalized repair checkpoint | `finalized_repair_checkpoint_record` | Checkpoint class, scoped state, and capture summary |
| Finalized repair rollback | `finalized_repair_rollback_record` | Rollback class, checkpoint consumption, and outcome |
| Finalized repair compensation | `finalized_repair_compensation_record` | Compensation class, acknowledgement, and follow-up |
| Finalized support packet | `finalized_repair_transaction_preview_checkpoint_rollback_and_support_packet_record` | Metadata-safe export projection |

## Vocabulary additions

- `FinalizedCheckpointClass`: `durable_pre_apply`, `ephemeral_pre_apply`, `no_checkpoint_needed`, `checkpoint_refused`, `no_checkpoint_escalation_only`
- `FinalizedRollbackClass`: `exact_restore_from_checkpoint`, `compensating_restore`, `regenerate_from_authoritative_source`, `manual_recovery_path`, `no_rollback_not_applicable`, `rollback_failed_export_only`
- `FinalizedCompensationClass`: `no_compensation_needed`, `regenerate_from_authoritative_source`, `semantic_inverse_compensation`, `manual_followup_required`, `audit_only_no_state_change`, `compensation_failed_escalation`
- `FinalizedPreviewDispositionClass`: `pending_review`, `compared_with_baseline`, `authorized_for_apply`, `cancelled_before_apply`, `blocked_pending_evidence`, `refused_escalation_only`
- `FinalizedRepairFlowClass`: `preview_checkpoint_apply_with_rollback`, `preview_checkpoint_apply_with_compensation`, `preview_observe_only_audit`, `preview_refused_escalation`, `comparison_then_authorize`
- `FinalizedSeededScenarioClass`: `cache_index_repair`, `extension_quarantine_bisect`, `toolchain_reresolve`, `remote_agent_rollback`, `policy_entitlement_refresh`, `trust_reacquire`, `watcher_restart_reseed`, `docs_mirror_refresh`, `escalation_only_no_local_repair`, `observe_only_no_repair`
