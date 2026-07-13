# M5 Shell-Metric and Minimum-Size Registries

- Packet: `m5-shell-metric-and-minimum-size-registries:stable:0001`
- Label: `M5 shell-metric and minimum-size registries with canonical logical-pixel envelopes for the title / context bar, activity rail, sidebar, dominant main editor group, right inspector, bottom panel, and status bar, tab / resize-handle / icon-only control hit-target minima, comfortable / standard / compact density coverage, and registry-bound tracing across shell, editor, review, notebook, data, and support surfaces`
- Consumer surfaces: 6
- Shell zones: title_context_bar, activity_rail, sidebar, main_editor_group, right_inspector, bottom_panel, status_bar, zone_unclassified
- Density modes: comfortable, standard, compact
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves the title / context bar and activity-rail geometry from the shared metric registry and keeps every tab above its minimum width; a hand-copied sidebar constant and a below-minimum tab degrade honestly instead of reading as a clean pass
  - Shell-metric entries: 3 / minimum-size entries: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor resolves the sidebar and keeps the main editor group dominant above its 420 px minimum while binding resize-handle hit areas to the registry; a sidebar minimum below the canonical envelope degrades honestly
  - Shell-metric entries: 3 / minimum-size entries: 1
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface resolves the right-inspector geometry and keeps icon-only control hit targets above 28 px; a metric that would starve the main workspace and an unclassified control both degrade honestly
  - Shell-metric entries: 2 / minimum-size entries: 2
- **data_ui**: `stable`
  - Owner: Data surface owner
  - Scope: The data surface resolves the bottom-panel geometry across every density mode and keeps tab minimum widths above their floor; a density-incomplete metric and a density-incomplete hit target degrade honestly
  - Shell-metric entries: 2 / minimum-size entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings surface resolves the status-bar geometry and keeps icon-only control hit targets above their minimum; a metric that fails under a snapped window width degrades honestly instead of fracturing the layout
  - Shell-metric entries: 2 / minimum-size entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved shell-metric and minimum-size truth, so a hand-copied constant or an unstated registry token is visible in evidence rather than hidden behind a screenshot
  - Shell-metric entries: 2 / minimum-size entries: 1
