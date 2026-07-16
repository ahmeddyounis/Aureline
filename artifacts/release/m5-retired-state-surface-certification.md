# M5 Retired-State Surface Certification

- Packet: `m5-retired-state-surface-certification:stable:0001`
- As of: `2026-07-15T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-retirements/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-retired-state-closure-lane** — profile=live_retired_state_closure_lane claimed=certified_retired_closure certified=certified_retired_closure status=green narrowed_axes=0
- **cert:reviewable-retirement-record-structure** — profile=reviewable_retirement_record_structure claimed=reviewable_retirement_record certified=reviewable_retirement_record status=green narrowed_axes=0
- **cert:disclosed-archive-partial-profile** — profile=disclosed_archive_partial_profile claimed=reviewable_retirement_record certified=archive_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-propagation-profile** — profile=unverified_propagation_profile claimed=reviewable_retirement_record certified=propagation_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-closure-ledger-profile** — profile=unverified_closure_ledger_profile claimed=reviewable_retirement_record certified=closure_ledger_unverified_projection status=yellow narrowed_axes=1
