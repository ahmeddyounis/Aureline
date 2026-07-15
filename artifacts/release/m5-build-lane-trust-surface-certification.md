# M5 Build-Lane-Trust Surface Certification

- Packet: `m5-build-lane-trust-surface-certification:stable:0001`
- As of: `2026-07-15T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-exact-build-supportability-proof/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-exact-build-supportable-lane** — profile=live_exact_build_supportable_lane claimed=trusted_exact_build_supportable_lane certified=trusted_exact_build_supportable_lane status=green narrowed_axes=0
- **cert:reviewable-reproducibility-structure** — profile=reviewable_reproducibility_structure claimed=reviewable_reproducibility_surface certified=reviewable_reproducibility_surface status=green narrowed_axes=0
- **cert:disclosed-cache-discipline-profile** — profile=disclosed_cache_discipline_profile claimed=reviewable_reproducibility_surface certified=cache_discipline_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-clean-room-parity-profile** — profile=unverified_clean_room_parity_profile claimed=reviewable_reproducibility_surface certified=clean_room_parity_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-exact-build-supportability-profile** — profile=unverified_exact_build_supportability_profile claimed=reviewable_reproducibility_surface certified=exact_build_supportability_unverified_projection status=yellow narrowed_axes=1
