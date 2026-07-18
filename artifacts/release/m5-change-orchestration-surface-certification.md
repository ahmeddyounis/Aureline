# M5 Change-Orchestration Surface Certification

- Packet: `m5-change-orchestration-surface-certification:stable:0001`
- As of: `2026-07-16T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-change-orchestration-proof/support_export.json`
- Profiles: 8 / 8 certified (2 green, 6 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 6
- Report clean: true

## Profiles

- **cert:certified-change-orchestration-lane** — profile=certified_change_orchestration_lane claimed=certified_change_orchestration_truth certified=certified_change_orchestration_truth status=green narrowed_axes=0
- **cert:reviewable-change-orchestration-record-structure** — profile=reviewable_change_orchestration_record_structure claimed=reviewable_change_orchestration_record certified=reviewable_change_orchestration_record status=green narrowed_axes=0
- **cert:unbound-worktree-binding-profile** — profile=unbound_worktree_binding_profile claimed=reviewable_change_orchestration_record certified=worktree_binding_unverified_projection status=yellow narrowed_axes=1
- **cert:inferred-stack-membership-profile** — profile=inferred_stack_membership_profile claimed=reviewable_change_orchestration_record certified=stack_membership_unverified_projection status=yellow narrowed_axes=1
- **cert:silently-reordered-stack-profile** — profile=silently_reordered_stack_profile claimed=reviewable_change_orchestration_record certified=stack_order_unverified_projection status=yellow narrowed_axes=1
- **cert:ambient-branch-landing-profile** — profile=ambient_branch_landing_profile claimed=reviewable_change_orchestration_record certified=landing_authority_unverified_projection status=yellow narrowed_axes=1
- **cert:stale-validation-shelf-profile** — profile=stale_validation_shelf_profile claimed=reviewable_change_orchestration_record certified=validation_freshness_unverified_projection status=yellow narrowed_axes=1
- **cert:partial-cleanup-evidence-profile** — profile=partial_cleanup_evidence_profile claimed=reviewable_change_orchestration_record certified=cleanup_evidence_unverified_projection status=yellow narrowed_axes=1
