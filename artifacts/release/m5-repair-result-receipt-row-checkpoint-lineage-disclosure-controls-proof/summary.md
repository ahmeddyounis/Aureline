# M5 Repair-Result-Receipt-Row and Checkpoint-Lineage-Disclosure Controls

- Packet: `m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls:stable:0001`
- Label: `M5 repair-result receipt rows and checkpoint-lineage disclosures with applied-versus-skipped scope, partial-success markers, compensation and manual follow-up state, and support-export linkage`
- Consumer surfaces: 5
- Repair outcomes: repair_applied_exact, repair_compensated, repair_regenerated, repair_partial_success, repair_manual_required, repair_failed
- Checkpoint states: checkpoint_available, checkpoint_partial, checkpoint_missing, checkpoint_expired, checkpoint_external, checkpoint_unknown
- Reversal classes: exact_reversal, compensating_reversal, regenerate_reversal, manual_follow_up, audit_only, reversal_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **doctor_ui**: `stable`
  - Owner: Project Doctor owner
  - Scope: Project Doctor renders repair-result receipt rows for an exact success and a first-class partial success naming applied-versus-skipped scope, plus checkpoint-lineage disclosures tracing finding to preview to apply to result, and degrades honestly when an outcome collapses into a generic success, a partial success reads as complete, the lineage stages collapse into one status, or the canonical result ref is unstated
  - Receipt examples: 4 / lineage examples: 4
- **remote_ui**: `stable`
  - Owner: Remote workspace owner
  - Scope: The remote workspace UI carries the compensated receipt with its stated compensation follow-up and a lineage traced finding to result, degrading honestly when the checkpoint ref is unstated or the lineage skips the preview stage
  - Receipt examples: 2 / lineage examples: 2
- **safe_mode_ui**: `stable`
  - Owner: Safe mode owner
  - Scope: Safe mode shows the manual-required receipt leaving a stated follow-up and a lineage traced finding to result, degrading honestly when the manual follow-up state is unstated or the lineage skips the apply stage
  - Receipt examples: 2 / lineage examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved receipt and lineage truth, so a receipt with no command-backed support-export path, an unresolved reversal class, a missing lineage path, or an unlinked finding is visible in evidence rather than hidden behind feature-local translation
  - Receipt examples: 2 / lineage examples: 2
- **product_ui**: `stable`
  - Owner: In-product repair owner
  - Scope: In-product surfaces reuse the same receipt and lineage grammar the Doctor UI shows for a regenerated and a failed outcome, keeping failure honest with skipped scope, and degrading honestly when the repair id or the applied scope of a non-failure outcome is unstated
  - Receipt examples: 4 / lineage examples: 2
