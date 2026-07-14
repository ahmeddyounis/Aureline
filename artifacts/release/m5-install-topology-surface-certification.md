# M5 Install-Topology Surface Certification

- Packet: `m5-install-topology-surface-certification:stable:0001`
- As of: `2026-07-14T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-install-topology-proof/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-trusted-delivery-surface** — profile=live_trusted_delivery_surface claimed=trusted_delivery_surface certified=trusted_delivery_surface status=green narrowed_axes=0
- **cert:reviewable-delivery-structure** — profile=reviewable_delivery_structure claimed=reviewable_delivery_surface certified=reviewable_delivery_surface status=green narrowed_axes=0
- **cert:disclosed-state-boundary-profile** — profile=disclosed_state_boundary_profile claimed=reviewable_delivery_surface certified=state_boundary_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-repair-verify-profile** — profile=unverified_repair_verify_profile claimed=reviewable_delivery_surface certified=repair_verify_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-rollout-evidence-profile** — profile=unverified_rollout_evidence_profile claimed=reviewable_delivery_surface certified=rollout_evidence_unverified_projection status=yellow narrowed_axes=1
