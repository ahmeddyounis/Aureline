# M5 Supported-Line Surface Certification

- Packet: `m5-supported-line-surface-certification:stable:0001`
- As of: `2026-07-15T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-supported-line-transparency/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-supported-line-operating-lane** — profile=live_supported_line_operating_lane claimed=certified_operating_line certified=certified_operating_line status=green narrowed_axes=0
- **cert:reviewable-transparency-structure** — profile=reviewable_transparency_structure claimed=reviewable_transparency_surface certified=reviewable_transparency_surface status=green narrowed_axes=0
- **cert:disclosed-correction-archive-profile** — profile=disclosed_correction_archive_profile claimed=reviewable_transparency_surface certified=correction_archive_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-migration-scoreboard-profile** — profile=unverified_migration_scoreboard_profile claimed=reviewable_transparency_surface certified=migration_scoreboard_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-orr-history-profile** — profile=unverified_orr_history_profile claimed=reviewable_transparency_surface certified=orr_history_unverified_projection status=yellow narrowed_axes=1
