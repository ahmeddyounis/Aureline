# M5 Visual-Designer Surface Certification

- Packet: `m5-visual-designer-surface-certification:stable:0001`
- As of: `2026-07-04T00:00:00Z`
- Bundle: `artifacts/release/m5-visual-designer-surface-certification-proof/support_export.json`
- Surfaces: 10 certified across 10 / 10 claimed surfaces
- Status: 7 green / 3 yellow / 0 red

## Rows

- **cert:design-canvas-workspace** (design_canvas_workspace) — surface=design_canvas_workspace declared=fully_interactive_writable effective=fully_interactive_writable mapping=certified round_trip=certified freshness=certified status=certified
- **cert:structure-tree-panel** (structure_tree_panel) — surface=structure_tree_panel declared=inspect_only effective=inspect_only mapping=certified round_trip=certified freshness=certified status=certified
- **cert:property-inspector-panel** (property_inspector_panel) — surface=property_inspector_panel declared=fully_interactive_writable effective=fully_interactive_writable mapping=certified round_trip=certified freshness=certified status=certified
- **cert:source-round-trip-rail** (source_round_trip_rail) — surface=source_round_trip_rail declared=fully_interactive_writable effective=inspect_only mapping=disclosed_narrowed round_trip=certified freshness=certified status=narrowed_disclosed
  - Auto-narrow: dimension=mapping_quality trigger=unmapped_source — Source mapping resolved only approximately; the rail narrows to inspect-only and keeps the source-first diff before any write-back
- **cert:breakpoint-device-preview-deck** (breakpoint_device_preview_deck) — surface=breakpoint_device_preview_deck declared=fully_interactive_writable effective=read_only mapping=certified round_trip=disclosed_narrowed freshness=disclosed_narrowed status=narrowed_disclosed
  - Auto-narrow: dimension=preview_runtime_freshness trigger=runtime_unavailable — Preview runtime is past its freshness SLO; the deck narrows to a read-only captured view and keeps the runtime origin and mapping quality visible
- **cert:framework-pack-preview** (framework_pack_preview) — surface=framework_pack_preview declared=fully_interactive_writable effective=inspect_only mapping=certified round_trip=certified freshness=disclosed_narrowed status=narrowed_disclosed
  - Auto-narrow: dimension=preview_runtime_freshness trigger=runtime_unavailable — Framework-pack runtime is aging toward its freshness SLO; the preview narrows to inspect-only until the runtime refreshes
- **cert:browser-runtime-inspection** (browser_runtime_inspection) — surface=browser_runtime_inspection declared=inspect_only effective=inspect_only mapping=certified round_trip=certified freshness=certified status=certified
- **cert:docs-help-embeds** (docs_help_embeds) — surface=docs_help_embeds declared=read_only effective=read_only mapping=certified round_trip=certified freshness=certified status=certified
- **cert:support-export** (support_export) — surface=support_export declared=source_only effective=source_only mapping=certified round_trip=certified freshness=certified status=certified
- **cert:release-proof** (release_proof) — surface=release_proof declared=read_only effective=read_only mapping=certified round_trip=certified freshness=certified status=certified
