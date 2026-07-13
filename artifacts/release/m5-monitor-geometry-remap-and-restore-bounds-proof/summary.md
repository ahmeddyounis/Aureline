# M5 Monitor-Topology Geometry-Remap and Restore-Bounds Registries

- Packet: `m5-monitor-geometry-remap-and-restore-bounds:stable:0001`
- Label: `M5 monitor-topology geometry-remap and restore-bounds registries with monitor-affinity restore across monitor attach / detach, undock, DPI change, fullscreen, and snapped-layout recovery, visible-bounds clamping with no off-screen or focus-trapped restore, persisted layout intent instead of stale absolute coordinates, recenter / reset affordances under reduced fidelity, and diagnosable geometry-remap provenance across shell, editor, review, notebook, data, and support surfaces`
- Consumer surfaces: 6
- Topology changes: monitor_attach, monitor_detach, undock, dpi_change, fullscreen_transition, snapped_layout_recovery, change_unclassified
- Fidelity outcomes: exact_bounds_restored, proportional_intent_remap, monitor_affinity_fallback, recenter_reset, outcome_unclassified
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves the restorable-window bounds from the shared registry and clamps them into visible bounds after a monitor detach, recording the remap in provenance; a window that would reopen fully off-screen and a remap that would silently drop the workspace both degrade honestly instead of reading as a clean pass
  - Restore-bounds entries: 2 / remap-provenance entries: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor remaps a docked panel through a mixed-DPI change with a proportional-intent restore and a recenter affordance, keeping usable geometry and recording the remap; a restore that would trap focus and a remap that would drop the focus chain both degrade honestly
  - Restore-bounds entries: 2 / remap-provenance entries: 2
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface restores a split layout across a fullscreen transition with exact bounds and records a monitor-affinity fallback in provenance; a restore that would lose usable compare geometry and a remap whose reason is unrecorded both degrade honestly
  - Restore-bounds entries: 2 / remap-provenance entries: 2
- **data_ui**: `stable`
  - Owner: Data surface owner
  - Scope: The data surface restores a dialog after an undock via a monitor-affinity fallback with a recenter affordance and records a recenter-reset in provenance; a restore that would replay stale absolute coordinates and a remap whose provenance omits detail both degrade honestly
  - Restore-bounds entries: 2 / remap-provenance entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings surface restores an approval sheet after a snapped-layout recovery with exact bounds and records the remap; a reduced-fidelity restore that would omit its recenter affordance and a remap whose fidelity outcome is unclassified both degrade honestly
  - Restore-bounds entries: 2 / remap-provenance entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved restore-bounds and remap-provenance truth, so an off-screen restore or an unstated registry token is visible in evidence rather than hidden behind a screenshot
  - Restore-bounds entries: 2 / remap-provenance entries: 2
