# M5 Historical-Evidence Surface Certification

- Packet: `m5-historical-evidence-surface-certification:stable:0001`
- As of: `2026-07-16T00:00:00Z`
- Canonical bundle: `artifacts/support/m5-historical-evidence/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:current-non-live-evidence-lane** — profile=current_non_live_evidence_lane claimed=certified_non_live_evidence certified=certified_non_live_evidence status=green narrowed_axes=0
- **cert:reviewable-snapshot-record-structure** — profile=reviewable_snapshot_record_structure claimed=reviewable_snapshot_record certified=reviewable_snapshot_record status=green narrowed_axes=0
- **cert:disclosed-imported-offline-partial-profile** — profile=disclosed_imported_offline_partial_profile claimed=reviewable_snapshot_record certified=imported_offline_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-live-target-profile** — profile=unverified_live_target_profile claimed=reviewable_snapshot_record certified=live_target_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-expiry-removal-ledger-profile** — profile=unverified_expiry_removal_ledger_profile claimed=reviewable_snapshot_record certified=expiry_removal_unverified_projection status=yellow narrowed_axes=1
