# M5 Visual-Designer Component Accessibility Fallback

- Packet: `m5-visual-designer-component-accessibility-fallback:stable:0001`
- As of: `2026-07-03T00:00:00Z`
- Families: 7 certified across 7 / 7 frozen families
- Status: 5 green / 2 yellow / 0 red

## Rows

- **a11y:design-canvas** (design_canvas) — family=design_canvas keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable low_resource=reachable_and_labeled drag=gated_disclosed export=reconstructable_without_screenshot status=narrowed_disclosed
  - Auto-narrow: trigger=unidentified_posture — The visual canvas exposes a keyboard-reachable structure tree and textual outline; complex spatial relationships are summarized for screen readers rather than dropped
- **a11y:structure-tree-row** (structure_tree_row) — family=structure_tree_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled low_resource=reachable_and_labeled drag=command_backed_parity export=reconstructable_without_screenshot status=parity
- **a11y:property-inspector-row** (property_inspector_row) — family=property_inspector_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled low_resource=reachable_and_labeled drag=command_backed_parity export=reconstructable_without_screenshot status=parity
- **a11y:source-sync-chip** (source_sync_chip) — family=source_sync_chip keyboard=reachable_and_labeled screen_reader=reachable_and_labeled low_resource=reachable_and_labeled drag=command_backed_parity export=reconstructable_without_screenshot status=parity
- **a11y:breakpoint-preview-row** (breakpoint_preview_row) — family=breakpoint_preview_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled low_resource=disclosed_reduced_but_reachable drag=gated_disclosed export=reconstructable_without_screenshot status=narrowed_disclosed
  - Auto-narrow: trigger=runtime_unavailable — In low-resource mode the device preview renders a textual viewport/data-posture summary instead of the live runtime pixels while keeping the runtime origin and mapping quality visible
- **a11y:unsupported-construct-card** (unsupported_construct_card) — family=unsupported_construct_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled low_resource=reachable_and_labeled drag=command_backed_parity export=reconstructable_without_screenshot status=parity
- **a11y:round-trip-conflict-banner** (round_trip_conflict_banner) — family=round_trip_conflict_banner keyboard=reachable_and_labeled screen_reader=reachable_and_labeled low_resource=reachable_and_labeled drag=command_backed_parity export=reconstructable_without_screenshot status=parity
