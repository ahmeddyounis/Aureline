# M5 Responsive-Geometry and Collapse-Priority Registries

- Packet: `m5-responsive-geometry-and-collapse-priority-registries:stable:0001`
- Label: `M5 responsive-geometry and collapse-priority registries with canonical Compact 1024-1279 / Standard 1280-1599 / Expanded 1600+ desktop window-class width bounds, title / rail / sidebar / workspace / inspector / panel / status shell-zone coexistence, declared adaptive-collapse priority order, identity-stable docked / sheet / overlay / temporary-panel transitions, and registry-bound tracing across shell, editor, review, notebook, data, and support surfaces`
- Consumer surfaces: 6
- Window classes: compact_desktop, standard_desktop, expanded_desktop, class_unclassified
- Collapse targets: optional_right_inspector_detail, secondary_bottom_panel_tabs, low_frequency_side_tools, primary_navigation, path_branch_trust_target_identity, editor_workspace, target_unclassified
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves the Compact desktop window class from the shared registry and keeps optional right-inspector detail identity-stable when it moves into a sheet; a private breakpoint and an identity-dropping transition degrade honestly instead of reading as a clean pass
  - Window-class entries: 2 / collapse-step entries: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor resolves the Standard desktop window class and keeps the dominant editor workspace docked while secondary bottom-panel tabs move to overflow; an essential action that would become hover-only degrades honestly
  - Window-class entries: 2 / collapse-step entries: 2
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface resolves the Expanded desktop window class and converts low-frequency side tools to overflow; a compare / editor group that would narrow into an unusable pane and a collapse of the protected editor workspace both degrade honestly
  - Window-class entries: 2 / collapse-step entries: 2
- **data_ui**: `stable`
  - Owner: Data surface owner
  - Scope: The data surface resolves the Compact desktop window class and keeps primary navigation identity-stable when it moves into a temporary panel; a responsive change that would drop recovery-critical state and a collapse that would starve the main workspace both degrade honestly
  - Window-class entries: 2 / collapse-step entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings surface resolves the Standard desktop window class across every shell zone and keeps path / branch / trust / target identity docked and protected; a window class that omits shell zones and a primary navigation hidden behind an overlay-only fallback both degrade honestly
  - Window-class entries: 2 / collapse-step entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved window-class and collapse-step truth, so a private breakpoint or an unstated registry token is visible in evidence rather than hidden behind a screenshot
  - Window-class entries: 2 / collapse-step entries: 2
