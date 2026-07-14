# M5 Repository-Bootstrap Surface Certification

- Packet: `m5-repository-bootstrap-surface-certification:stable:0001`
- As of: `2026-07-14T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-repository-bootstrap-proof/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-trusted-acquisition-surface** — profile=live_trusted_acquisition_surface claimed=trusted_acquisition_surface certified=trusted_acquisition_surface status=green narrowed_axes=0
- **cert:reviewable-acquisition-structure** — profile=reviewable_acquisition_structure claimed=reviewable_acquisition_surface certified=reviewable_acquisition_surface status=green narrowed_axes=0
- **cert:disclosed-checkout-plan-profile** — profile=disclosed_checkout_plan_profile claimed=reviewable_acquisition_surface certified=checkout_plan_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-trust-stage-profile** — profile=unverified_trust_stage_profile claimed=reviewable_acquisition_surface certified=trust_stage_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-bootstrap-evidence-profile** — profile=unverified_bootstrap_evidence_profile claimed=reviewable_acquisition_surface certified=bootstrap_evidence_unverified_projection status=yellow narrowed_axes=1
