# M5 Repair-Transaction-Preview-Card and Rollback-Class-Strip Controls

- Packet: `m5-repair-transaction-preview-card-rollback-class-strip-controls:stable:0001`
- Label: `M5 repair-transaction-preview-card and rollback-class-strip controls with repair ids, linked findings, prerequisites, checkpoint state, reversal class, and local/remote/managed impact truth`
- Consumer surfaces: 5
- Reversal classes: exact_reversal, compensating_reversal, regenerate_reversal, manual_follow_up, audit_only, reversal_unknown
- Target classes: local_workspace, remote_host, managed_workspace, mixed_target, external_target, target_unknown
- Checkpoint states: checkpoint_available, checkpoint_partial, checkpoint_missing, checkpoint_expired, checkpoint_external, checkpoint_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **doctor_ui**: `stable`
  - Owner: Project Doctor owner
  - Scope: Project Doctor renders one repair-transaction preview card naming repair id, linked findings, prerequisites, checkpoint state, impact scope, and local target class, and one rollback-class strip naming the controlled reversal class before anything is applied
  - Card examples: 4 / strip examples: 4
- **remote_ui**: `stable`
  - Owner: Remote repair owner
  - Scope: The remote / workspace UI reuses the same transaction-preview grammar for a remote-host target, degrading honestly when the target class is unresolved or collapsed into a generic target
  - Card examples: 3 / strip examples: 2
- **safe_mode_ui**: `stable`
  - Owner: Safe mode owner
  - Scope: Safe mode previews a managed-workspace repair with its checkpoint state and a manual-follow-up rollback class, degrading honestly when a checkpoint or review path cannot be resolved
  - Card examples: 3 / strip examples: 3
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved card and strip truth, so a repair with unstated ids, an unresolved reversal class, or a hidden checkpoint absence is visible in evidence rather than hidden
  - Card examples: 3 / strip examples: 3
- **product_ui**: `stable`
  - Owner: In-product repair owner
  - Scope: In-product surfaces reuse the same transaction-preview grammar and reversal vocabulary a user sees in Project Doctor, keeping an audit-only change honest about its reversal limits and disclosing a missing checkpoint before apply
  - Card examples: 2 / strip examples: 2
