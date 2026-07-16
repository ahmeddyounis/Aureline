# Cross-Actor Constrained-Write Enforcement: One Gate Across Actors

- Packet: `m5-cross-actor-constrained-write-enforcement:stable:0001`
- Surface: `M5 cross-actor constrained-write enforcement (one gate across actors)`
- Gate bindings: 16 (8 narrowed, 7 multi-state)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Gate bindings

- **Read-only vendored source (duplicate into an editable copy)** [`caw-readonly-directsave`]: object `read_only`, actor `direct_edit_save`, reason `read_only_path_not_directly_writable`, safe next step `duplicate_to_editable_copy`, posture `enforced_gate`
- **Read-only vendored source (duplicate into an editable copy)** [`caw-readonly-codeaction`]: object `read_only`, actor `code_action`, reason `read_only_path_not_directly_writable`, safe next step `duplicate_to_editable_copy`, posture `fail_closed_on_actor_drift`
- **Read-only vendored source (duplicate into an editable copy)** [`caw-readonly-importer`]: object `read_only`, actor `importer`, reason `read_only_path_not_directly_writable`, safe next step `duplicate_to_editable_copy`, posture `export_redacted`
- **Generated artifact that is also policy locked (regenerate with preview)** [`caw-generated-aiapply`]: object `generated` (+ policy_locked), actor `ai_apply`, reason `generated_artifact_regenerate_only`, safe next step `regenerate_with_preview`, posture `enforced_gate`
- **Generated artifact that is also policy locked (regenerate with preview)** [`caw-generated-automation`]: object `generated` (+ policy_locked), actor `automation_recipe`, reason `generated_artifact_regenerate_only`, safe next step `regenerate_with_preview`, posture `fail_closed_on_actor_drift`
- **Generated artifact that is also policy locked (regenerate with preview)** [`caw-generated-repair`]: object `generated` (+ policy_locked), actor `repair`, reason `generated_artifact_regenerate_only`, safe next step `regenerate_with_preview`, posture `export_redacted`
- **Policy-locked protected config (request approval)** [`caw-policy-directsave`]: object `policy_locked`, actor `direct_edit_save`, reason `policy_lock_requires_approval`, safe next step `request_approval`, posture `enforced_gate`
- **Policy-locked protected config (request approval)** [`caw-policy-repair`]: object `policy_locked`, actor `repair`, reason `policy_lock_requires_approval`, safe next step `request_approval`, posture `fail_closed_on_actor_drift`
- **Managed mirror that is also a captured snapshot (detach from managed source)** [`caw-managed-aiapply`]: object `managed` (+ captured_snapshot), actor `ai_apply`, reason `managed_source_requires_detach`, safe next step `detach_from_managed_source`, posture `enforced_gate`
- **Managed mirror that is also a captured snapshot (detach from managed source)** [`caw-managed-importer`]: object `managed` (+ captured_snapshot), actor `importer`, reason `managed_source_requires_detach`, safe next step `detach_from_managed_source`, posture `enforced_gate`
- **Managed mirror that is also a captured snapshot (detach from managed source)** [`caw-managed-repair`]: object `managed` (+ captured_snapshot), actor `repair`, reason `managed_source_requires_detach`, safe next step `detach_from_managed_source`, posture `enforced_gate`
- **Managed mirror that is also a captured snapshot (detach from managed source)** [`caw-managed-directsave`]: object `managed` (+ captured_snapshot), actor `direct_edit_save`, reason `managed_source_requires_detach`, safe next step `detach_from_managed_source`, posture `export_redacted`
- **Projection / virtual view (create overlay patch)** [`caw-projection-codeaction`]: object `projection`, actor `code_action`, reason `projection_requires_overlay_or_detach`, safe next step `create_overlay_patch`, posture `enforced_gate`
- **Projection / virtual view (create overlay patch)** [`caw-projection-automation`]: object `projection`, actor `automation_recipe`, reason `projection_requires_overlay_or_detach`, safe next step `create_overlay_patch`, posture `export_redacted`
- **Captured snapshot of a preserved past state (duplicate into an editable copy)** [`caw-snapshot-directsave`]: object `captured_snapshot`, actor `direct_edit_save`, reason `captured_snapshot_restore_only`, safe next step `duplicate_to_editable_copy`, posture `enforced_gate`
- **Captured snapshot of a preserved past state (duplicate into an editable copy)** [`caw-snapshot-aiapply`]: object `captured_snapshot`, actor `ai_apply`, reason `captured_snapshot_restore_only`, safe next step `duplicate_to_editable_copy`, posture `fail_closed_on_actor_drift`
