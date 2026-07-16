# M5 Constrained-Object Surface Certification

- Packet: `m5-constrained-object-surface-certification:stable:0001`
- As of: `2026-07-16T00:00:00Z`
- Canonical bundle: `artifacts/support/m5-constrained-object-state/support_export.json`
- Profiles: 6 / 6 certified (2 green, 4 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 4
- Report clean: true

## Profiles

- **cert:certified-constrained-object-lane** — profile=certified_constrained_object_lane claimed=certified_constrained_object_truth certified=certified_constrained_object_truth status=green narrowed_axes=0
- **cert:reviewable-constrained-record-structure** — profile=reviewable_constrained_record_structure claimed=reviewable_constrained_state_record certified=reviewable_constrained_state_record status=green narrowed_axes=0
- **cert:disclosed-generated-divergence-partial-profile** — profile=disclosed_generated_divergence_partial_profile claimed=reviewable_constrained_state_record certified=generated_divergence_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-canonical-source-profile** — profile=unverified_canonical_source_profile claimed=reviewable_constrained_state_record certified=canonical_source_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-write-target-review-profile** — profile=unverified_write_target_review_profile claimed=reviewable_constrained_state_record certified=write_target_review_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-actor-parity-profile** — profile=unverified_actor_parity_profile claimed=reviewable_constrained_state_record certified=actor_parity_unverified_projection status=yellow narrowed_axes=1
