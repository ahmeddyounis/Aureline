# M5 Stable-Line Surface Certification

- Packet: `m5-stable-line-surface-certification:stable:0001`
- As of: `2026-07-15T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-stable-line-correction-reports/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-certified-widening-lane** — profile=live_supported_line_operating_lane claimed=certified_operating_line certified=certified_operating_line status=green narrowed_axes=0
- **cert:reviewable-stable-line-structure** — profile=reviewable_stable_line_structure claimed=reviewable_stable_line_surface certified=reviewable_stable_line_surface status=green narrowed_axes=0
- **cert:disclosed-correction-packet-profile** — profile=disclosed_correction_ownership_profile claimed=reviewable_stable_line_surface certified=correction_ownership_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-refresh-currency-profile** — profile=unverified_bundle_currentness_profile claimed=reviewable_stable_line_surface certified=bundle_currentness_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-backport-decision-profile** — profile=unverified_lts_readiness_profile claimed=reviewable_stable_line_surface certified=lts_readiness_unverified_projection status=yellow narrowed_axes=1
