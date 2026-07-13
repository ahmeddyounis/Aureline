# M5 Motion-Layer-Iconography Surface Certification

- Packet: `m5-motion-layer-iconography-surface-certification:stable:0001`
- As of: `2026-07-13T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-motion-layer-iconography-proof/support_export.json`
- Profiles: 7 / 7 certified (2 green, 5 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 5
- Report clean: true

## Profiles

- **cert:live-trusted-interaction-surface** — profile=live_trusted_interaction_surface claimed=trusted_interaction_surface certified=trusted_interaction_surface status=green narrowed_axes=0
- **cert:reviewable-layer-structure** — profile=reviewable_layer_structure claimed=reviewable_interaction_surface certified=reviewable_interaction_surface status=green narrowed_axes=0
- **cert:stale-motion-timing-surface** — profile=stale_motion_timing_surface claimed=reviewable_interaction_surface certified=motion_timing_unverified_projection status=yellow narrowed_axes=1
- **cert:unconfirmed-reduced-motion-surface** — profile=unconfirmed_reduced_motion_surface claimed=reviewable_interaction_surface certified=reduced_motion_clamp_unverified_projection status=yellow narrowed_axes=1
- **cert:orientation-erasing-scrim-surface** — profile=orientation_erasing_scrim_surface claimed=reviewable_interaction_surface certified=scrim_orientation_unverified_projection status=yellow narrowed_axes=1
- **cert:detached-portal-surface** — profile=detached_portal_surface claimed=reviewable_interaction_surface certified=portal_ownership_unverified_projection status=yellow narrowed_axes=1
- **cert:impersonating-illustration-surface** — profile=impersonating_illustration_surface claimed=reviewable_interaction_surface certified=illustration_boundary_disclosed_projection status=yellow narrowed_axes=1
