# M5 Change-Intent Surface Certification

- Packet: `m5-change-intent-surface-certification:stable:0001`
- As of: `2026-07-16T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-change-intent-proof/support_export.json`
- Profiles: 8 / 8 certified (2 green, 6 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 6
- Report clean: true

## Profiles

- **cert:certified-change-intent-lane** — profile=certified_change_intent_lane claimed=certified_change_intent_truth certified=certified_change_intent_truth status=green narrowed_axes=0
- **cert:reviewable-change-intent-record-structure** — profile=reviewable_change_intent_record_structure claimed=reviewable_change_intent_record certified=reviewable_change_intent_record status=green narrowed_axes=0
- **cert:local-only-or-reconcile-commit-state-profile** — profile=local_only_or_reconcile_commit_state_profile claimed=reviewable_change_intent_record certified=commit_state_unverified_projection status=yellow narrowed_axes=1
- **cert:undisclosed-start-work-side-effect-profile** — profile=undisclosed_start_work_side_effect_profile claimed=reviewable_change_intent_record certified=side_effect_disclosure_unverified_projection status=yellow narrowed_axes=1
- **cert:flattened-linked-relation-source-profile** — profile=flattened_linked_relation_source_profile claimed=reviewable_change_intent_record certified=linked_relation_source_unverified_projection status=yellow narrowed_axes=1
- **cert:blocked-handoff-publishability-profile** — profile=blocked_handoff_publishability_profile claimed=reviewable_change_intent_record certified=handoff_publishability_unverified_projection status=yellow narrowed_axes=1
- **cert:local-only-resolution-authority-profile** — profile=local_only_resolution_authority_profile claimed=reviewable_change_intent_record certified=resolution_authority_unverified_projection status=yellow narrowed_axes=1
- **cert:unresolved-blocker-continuity-profile** — profile=unresolved_blocker_continuity_profile claimed=reviewable_change_intent_record certified=blocker_continuity_unverified_projection status=yellow narrowed_axes=1
