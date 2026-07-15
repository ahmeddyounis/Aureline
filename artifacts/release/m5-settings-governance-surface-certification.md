# M5 Settings-Governance Surface Certification

- Packet: `m5-settings-governance-surface-certification:stable:0001`
- As of: `2026-07-15T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-settings-governance-proof/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-trusted-settings-surface** — profile=live_trusted_settings_surface claimed=trusted_settings_surface certified=trusted_settings_surface status=green narrowed_axes=0
- **cert:reviewable-settings-structure** — profile=reviewable_settings_structure claimed=reviewable_settings_surface certified=reviewable_settings_surface status=green narrowed_axes=0
- **cert:disclosed-write-intent-profile** — profile=disclosed_write_intent_profile claimed=reviewable_settings_surface certified=write_intent_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-sync-conflict-profile** — profile=unverified_sync_conflict_profile claimed=reviewable_settings_surface certified=sync_conflict_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-capability-lifecycle-profile** — profile=unverified_capability_lifecycle_profile claimed=reviewable_settings_surface certified=capability_lifecycle_unverified_projection status=yellow narrowed_axes=1
