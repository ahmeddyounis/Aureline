# M5 Window-Restore Surface Certification

- Packet: `m5-window-restore-surface-certification:stable:0001`
- As of: `2026-07-14T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-window-restore-proof/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-trusted-restore-surface** — profile=live_trusted_restore_surface claimed=trusted_restore_surface certified=trusted_restore_surface status=green narrowed_axes=0
- **cert:reviewable-restore-structure** — profile=reviewable_restore_structure claimed=reviewable_restore_surface certified=reviewable_restore_surface status=green narrowed_axes=0
- **cert:disclosed-layout-skeleton-profile** — profile=disclosed_layout_skeleton_profile claimed=reviewable_restore_surface certified=layout_skeleton_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-session-replay-profile** — profile=unverified_session_replay_profile claimed=reviewable_restore_surface certified=session_replay_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-display-recovery-profile** — profile=unverified_display_recovery_profile claimed=reviewable_restore_surface certified=display_recovery_unverified_projection status=yellow narrowed_axes=1
