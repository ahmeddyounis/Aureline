# Write-Review Sheets: Reviewed Fallback Transitions Across Flows

- Packet: `m5-write-review-sheet-fallback-paths:stable:0001`
- Surface: `M5 write-review sheets (reviewed fallback transitions across flows)`
- Review bindings: 15 (9 narrowed, 6 multi-state)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Review bindings

- **Read-only vendored source (duplicate into an editable copy)** [`wrs-readonly-directsave`]: object `read_only`, fallback `duplicate_to_editable_copy` via `direct_save`, posture `full_review_sheet`, recovery `new_copy_leaves_original_intact`
- **Read-only vendored source (duplicate into an editable copy)** [`wrs-readonly-codeaction`]: object `read_only`, fallback `duplicate_to_editable_copy` via `code_action`, posture `precondition_notice_compact`, recovery `new_copy_leaves_original_intact`
- **Read-only vendored source (duplicate into an editable copy)** [`wrs-readonly-export`]: object `read_only`, fallback `duplicate_to_editable_copy` via `importer`, posture `export_redacted`, recovery `new_copy_leaves_original_intact`
- **Generated artifact that is also policy locked (regenerate with preview)** [`wrs-generated-aiapply`]: object `generated` (+ policy_locked), fallback `regenerate_with_preview` via `ai_apply`, posture `full_review_sheet`, recovery `regenerate_preview_discardable`
- **Generated artifact that is also policy locked (regenerate with preview)** [`wrs-generated-batch`]: object `generated` (+ policy_locked), fallback `regenerate_with_preview` via `batch_edit`, posture `precondition_notice_compact`, recovery `regenerate_preview_discardable`
- **Generated artifact that is also policy locked (regenerate with preview)** [`wrs-generated-export`]: object `generated` (+ policy_locked), fallback `regenerate_with_preview` via `repair`, posture `export_redacted`, recovery `regenerate_preview_discardable`
- **Policy-locked protected config (request approval)** [`wrs-policy-directsave`]: object `policy_locked`, fallback `request_approval` via `direct_save`, posture `full_review_sheet`, recovery `approval_request_withdrawable`
- **Policy-locked protected config (request approval)** [`wrs-policy-repair`]: object `policy_locked`, fallback `request_approval` via `repair`, posture `precondition_notice_compact`, recovery `approval_request_withdrawable`
- **Managed mirror that is also a captured snapshot (detach from managed source)** [`wrs-managed-aiapply`]: object `managed` (+ captured_snapshot), fallback `detach_from_managed_source` via `ai_apply`, posture `full_review_sheet`, recovery `detach_checkpoint_restorable`
- **Managed mirror that is also a captured snapshot (detach from managed source)** [`wrs-managed-importer`]: object `managed` (+ captured_snapshot), fallback `detach_from_managed_source` via `importer`, posture `precondition_notice_compact`, recovery `detach_checkpoint_restorable`
- **Managed mirror that is also a captured snapshot (detach from managed source)** [`wrs-managed-export`]: object `managed` (+ captured_snapshot), fallback `detach_from_managed_source` via `batch_edit`, posture `export_redacted`, recovery `detach_checkpoint_restorable`
- **Projection / virtual view (create overlay patch)** [`wrs-projection-codeaction`]: object `projection`, fallback `create_overlay_patch` via `code_action`, posture `full_review_sheet`, recovery `overlay_patch_revertible`
- **Projection / virtual view (create overlay patch)** [`wrs-projection-batch`]: object `projection`, fallback `create_overlay_patch` via `batch_edit`, posture `precondition_notice_compact`, recovery `overlay_patch_revertible`
- **Captured snapshot of a preserved past state (duplicate into an editable copy)** [`wrs-snapshot-directsave`]: object `captured_snapshot`, fallback `duplicate_to_editable_copy` via `direct_save`, posture `full_review_sheet`, recovery `new_copy_leaves_original_intact`
- **Captured snapshot of a preserved past state (duplicate into an editable copy)** [`wrs-snapshot-aiapply`]: object `captured_snapshot`, fallback `duplicate_to_editable_copy` via `ai_apply`, posture `precondition_notice_compact`, recovery `new_copy_leaves_original_intact`
