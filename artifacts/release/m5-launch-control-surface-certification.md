# M5 Launch-Control Surface Certification

- Packet: `m5-launch-control-surface-certification:stable:0001`
- As of: `2026-07-15T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-orr-rehearsal-proof/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-certified-widening-lane** — profile=live_certified_widening_lane claimed=certified_widening_lane certified=certified_widening_lane status=green narrowed_axes=0
- **cert:reviewable-launch-control-structure** — profile=reviewable_launch_control_structure claimed=reviewable_launch_control_surface certified=reviewable_launch_control_surface status=green narrowed_axes=0
- **cert:disclosed-freeze-exception-profile** — profile=disclosed_freeze_exception_profile claimed=reviewable_launch_control_surface certified=freeze_exception_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-rehearsal-currency-profile** — profile=unverified_rehearsal_currency_profile claimed=reviewable_launch_control_surface certified=rehearsal_currency_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-regression-asset-profile** — profile=unverified_regression_asset_profile claimed=reviewable_launch_control_surface certified=go_no_go_evidence_unverified_projection status=yellow narrowed_axes=1
