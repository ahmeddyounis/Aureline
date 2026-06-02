# Finalized typed repair-transaction preview, checkpoint, rollback, and compensation flows

This document defines the stable M4 contract that promotes the alpha
repair-transaction compiler, beta preview-skeleton evaluator, and recovery-ladder
alpha decision path into a single typed, truthful, narrow, versioned, and
export-safe blocked-user recovery system.

## What this row owns

- The [`FinalizedRepairTransactionFlow`] record that joins one preview, one
checkpoint, one rollback, and one compensation record into a single bounded flow.
- The [`FinalizedRepairPreviewRecord`] that declares preview disposition,
blast-radius class, confirmation requirement, and Project Doctor finding lineage.
- The [`FinalizedRepairCheckpointRecord`] that declares checkpoint class
(durable, ephemeral, none, refused, escalation-only), scoped state classes, and
capture summary.
- The [`FinalizedRepairRollbackRecord`] that declares rollback class
(exact restore, compensating restore, regenerate, manual, none, failed-export),
checkpoint ref consumed, and outcome.
- The [`FinalizedRepairCompensationRecord`] that declares compensation class,
strong-acknowledgement requirement, and follow-up action.
- The [`FinalizedRepairFlowSupportPacket`] that folds validated flows into a
metadata-safe projection for support-export and release-evidence pipelines.
- The [`FinalizedRepairFlowEvaluator`] that validates flow consistency,
checkpoint/rollback alignment, compensation/reversal matching, seeded-scenario
coverage, and recovery-ladder proof bindings.
- The boundary schema at
[`/schemas/support/finalize_typed_repair_transaction_preview_checkpoint_rollback_and.schema.json`](../../schemas/support/finalize_typed_repair_transaction_preview_checkpoint_rollback_and.schema.json).
- The protected fixture corpus at
[`/fixtures/support/m4/finalize_typed_repair_transaction_preview_checkpoint_rollback_and/`](../../fixtures/support/m4/finalize_typed_repair_transaction_preview_checkpoint_rollback_and/).

## Acceptance and how this row meets it

- Every finalized flow preserves `user_authored_files` in
`preserved_state_classes`.
- Every finalized flow cites a `doctor.finding.*` ref as its initiating
finding.
- Checkpoint class and rollback class are consistent: exact rollback requires
a durable or ephemeral checkpoint; refused escalation carries no checkpoint.
- Compensation class matches the previewed reversal class so reviewers see
a truthful cost estimate before apply.
- Preview disposition is one of the closed vocabulary and never silently
mutates to an unauthorized apply.
- No flow declares a destructive reset; any such claim is rejected by the
evaluator.
- All ten seeded support scenarios are covered or explicitly noted as
out-of-scope for the repair class family.
- Recovery-ladder proof bindings cover at least one rung and cite a valid
decision id.
- Exact-build identity is carried by reference so support exports and rollback
manifests quote the same build id.
- The support packet excludes raw private material and ambient authority by
default.

## Failure-drill posture

- Missing `user_authored_files` preservation is rejected with typed
[`FinalizedRepairFlowViolation`] rows.
- Inconsistent checkpoint/rollback pairs are rejected.
- Mismatched compensation and reversal classes are rejected.
- Missing seeded scenarios are rejected.
- Empty recovery-ladder bindings are rejected.
- Destructive-reset assertions are rejected.
- Invalid `doctor.finding.*` refs are rejected.

## First consumers

- The implementation lives in
[`crates/aureline-support/src/finalize_typed_repair_transaction_preview_checkpoint_rollback_and/mod.rs`](../../../crates/aureline-support/src/finalize_typed_repair_transaction_preview_checkpoint_rollback_and/mod.rs).
- The primary evaluator is `FinalizedRepairFlowEvaluator`.

## Related contracts

- [`repair_transaction_contract.md`](../repair_transaction_contract.md) — alpha repair-transaction compiler contract.
- [`repair_transaction_beta.md`](../repair_transaction_beta.md) — beta preview-skeleton evaluator contract.
- [`recovery_ladder_packet.md`](../recovery_ladder_packet.md) — recovery-ladder alpha decision contract.
- [`project_doctor_packet.md`](../project_doctor_packet.md) — Project Doctor finding and scenario matrix.

## Out of scope for this row

- Live checkpoint byte-level capture (owned by the runtime and storage lanes).
- Live apply/rollback execution (owned by the repair execution lane).
- Support bundle upload transport (owned by aureline-support-bundle).
- Crash symbolication (owned by aureline-crash).
