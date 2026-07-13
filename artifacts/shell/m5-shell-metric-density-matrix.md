# M5 Shell-Metric, Minimum-Size, Density-Mode, Responsive-Geometry, and Collapse-Priority Shell-Geometry Matrix

- Packet: `m5-shell-metric-density:stable:0001`
- Label: `M5 shell-metric, minimum-size, density-mode, responsive-geometry, and collapse-priority shell-geometry matrix`
- Geometry families: 5 (5 stable)
- Shell-geometry roles: zone, metric, hit_target, density, responsive, collapse, workspace_dominance
- Shell-metric roles: default_size, minimum_size, recommended_size, maximum_size, bound_to_registry, hand_copied_constant_disallowed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Geometry families

- **shell_metric**: `stable`
  - Owner: Shell layout owner
  - Canonical schema: `schemas/shell/m5-shell-metrics.schema.json`
  - Scope: One shell-metric table naming default, minimum, recommended, and maximum sizes for the title / context bar, rail, sidebar, main editor group, right inspector, bottom panel, and status bar so every zone honors one registry-bound size rather than a scattered local constant
  - Required labels: identity, semantic_role, registry_reference, size_metric
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, snapped_width_safe, cli_exportable, support_packet_present
- **minimum_size**: `stable`
  - Owner: Shell layout owner
  - Canonical schema: `schemas/shell/m5-shell-metrics.schema.json`
  - Scope: One minimum-size contract naming the tab minimum width, resize-handle hit area, and icon-only hit targets so every control stays reachable by pointer and keyboard and never shrinks below the supported minimum under zoom or snapped widths
  - Required labels: identity, semantic_role, registry_reference, size_metric
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, snapped_width_safe, cli_exportable, support_packet_present
- **density_mode**: `stable`
  - Owner: Design-token fidelity owner
  - Canonical schema: `schemas/shell/m5-density-mode.schema.json`
  - Scope: One density-mode contract naming the comfortable, standard, and compact modes as presentation-only changes that preserve the information architecture so command meaning, focus order, and trust visibility never move when density changes
  - Required labels: identity, semantic_role, registry_reference, density_mode
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, snapped_width_safe, cli_exportable, support_packet_present
- **responsive_geometry**: `stable`
  - Owner: Adaptive-layout owner
  - Canonical schema: `schemas/shell/m5-density-mode.schema.json`
  - Scope: One responsive-geometry contract naming the compact, standard, and expanded window classes so snapped or narrow widths preserve task identity and recovery-critical state rather than dropping in-progress work
  - Required labels: identity, semantic_role, registry_reference, responsive_class
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, snapped_width_safe, cli_exportable, support_packet_present
- **collapse_priority**: `stable`
  - Owner: Adaptive-layout owner
  - Canonical schema: `schemas/shell/m5-density-mode.schema.json`
  - Scope: One collapse-priority contract naming the declared collapse order and no-fracture geometry so the main workspace stays dominant, collapsed zones restore on re-expand, and extension or embedded surfaces never set a private width that fractures the shell
  - Required labels: identity, semantic_role, registry_reference, responsive_class
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, snapped_width_safe, cli_exportable, support_packet_present
