# M5 Shell-Metric-Density Surface Certification

- Packet: `m5-shell-metric-density-surface-certification:stable:0001`
- As of: `2026-07-13T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-shell-metric-density-proof/support_export.json`
- Profiles: 5 / 5 certified (2 green, 3 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 3
- Report clean: true

## Profiles

- **cert:live-trusted-geometry-surface** — profile=live_trusted_geometry_surface claimed=trusted_geometry_surface certified=trusted_geometry_surface status=green narrowed_axes=0
- **cert:reviewable-geometry-structure** — profile=reviewable_geometry_structure claimed=reviewable_geometry_surface certified=reviewable_geometry_surface status=green narrowed_axes=0
- **cert:unverified-density-mode-surface** — profile=unverified_density_mode_surface claimed=reviewable_geometry_surface certified=density_mode_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-responsive-geometry-surface** — profile=unverified_responsive_geometry_surface claimed=reviewable_geometry_surface certified=responsive_geometry_unverified_projection status=yellow narrowed_axes=1
- **cert:disclosed-collapse-priority-surface** — profile=disclosed_collapse_priority_surface claimed=reviewable_geometry_surface certified=collapse_priority_disclosed_projection status=yellow narrowed_axes=1
