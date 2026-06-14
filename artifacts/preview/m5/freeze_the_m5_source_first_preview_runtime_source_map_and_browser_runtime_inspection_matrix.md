# M5 Source-First Preview / Browser-Runtime Inspection Matrix

- Packet: `m5-preview-inspection-matrix:stable:0001`
- Label: `M5 Source-First Preview / Browser-Runtime Inspection Matrix`
- Rows: 8 (8 claimed, 1 narrowed)
- Surfaces: 8 / 8
- Evidence freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Rows

- **preview-row:source-first-framework:0001** (source_first_framework_preview): claim `stable` -> effective `stable`
  - Source-first framework preview rendered from the canonical source by the design renderer
  - session=`source_bound_live_preview` source_sync=`in_sync_from_source` target=`design_renderer_target` mapping=`exact`
  - attach_depth=`not_applicable_non_browser` round_trip=`exact_source_round_trip` write_capable=true
- **preview-row:visual-surface-mapping:0001** (visual_surface_mapping): claim `beta` -> effective `beta`
  - Component/DOM/widget mapping inspector over a source-bound design render
  - session=`source_bound_live_preview` source_sync=`in_sync_from_source` target=`design_renderer_target` mapping=`heuristic`
  - attach_depth=`not_applicable_non_browser` round_trip=`approximate_source_round_trip` write_capable=true
- **preview-row:browser-runtime-inspection:0001** (browser_runtime_inspection): claim `beta` -> effective `beta`
  - Browser-runtime DOM/CSS/network/storage inspection over an attached local browser tab
  - session=`runtime_backed_inspection` source_sync=`runtime_only_no_source` target=`browser_tab_target` mapping=`partial`
  - attach_depth=`dom_styles_network_storage` round_trip=`inspect_only_no_write` write_capable=false
- **preview-row:device-or-simulator:0001** (device_or_simulator_preview): claim `beta` -> effective `beta`
  - Simulator preview tethered over a workspace-bound transport with an exact source map
  - session=`device_tethered_session` source_sync=`in_sync_from_source` target=`simulator_target` mapping=`exact`
  - attach_depth=`not_applicable_non_browser` round_trip=`exact_source_round_trip` write_capable=true
- **preview-row:full-stack-loop:0001** (full_stack_preview_loop): claim `beta` -> effective `beta`
  - Full-stack preview loop where edits fall back to editing the source directly
  - session=`source_bound_live_preview` source_sync=`pending_rebuild` target=`browser_tab_target` mapping=`stale`
  - attach_depth=`dom_styles_network` round_trip=`source_only_fallback` write_capable=false
- **preview-row:embedded-webview:0001** (embedded_webview_preview): claim `preview` -> effective `preview`
  - Embedded webview preview hosted in the shell with inspect-only DOM/CSS depth
  - session=`embedded_renderer_session` source_sync=`in_sync_from_source` target=`embedded_webview_target` mapping=`heuristic`
  - attach_depth=`dom_and_styles` round_trip=`inspect_only_no_write` write_capable=false
- **preview-row:visual-edit-transform:0001** (visual_edit_transform): claim `beta` -> effective `beta`
  - Visual-edit transform that previews the real source diff before committing an exact round-trip
  - session=`source_bound_live_preview` source_sync=`in_sync_from_source` target=`design_renderer_target` mapping=`exact`
  - attach_depth=`not_applicable_non_browser` round_trip=`exact_source_round_trip` write_capable=true
- **preview-row:support-export:0001** (support_export_projection): claim `beta` -> effective `held`
  - Support/export projection of a preview row whose mapping-quality class is not yet identified
  - session=`snapshot_projection` source_sync=`drifted_from_source` target=`viewport_preset_only` mapping=`unidentified`
  - attach_depth=`not_applicable_non_browser` round_trip=`no_round_trip` write_capable=false
  - Degraded: Mapping-quality class not yet identified for this projected row; held below preview until a source map is published
