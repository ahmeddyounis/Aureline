# M5 Visual-Designer Component Matrix

- Packet: `m5-visual-designer-component-matrix:stable:0001`
- Label: `M5 Visual-Designer Component Matrix`
- Components: 10 across 7 / 7 families (6 degraded)

## Components

- **component:design-canvas:0001** (design_canvas) — Visual design canvas bound to the canonical source revision
  - Source-bound editable canvas whose state is derivative of source and whose selection stays synchronized with the tree and source
  - family=design_canvas surface=visual_surface_mapping sync=in_sync_from_source round_trip=exact_source_round_trip export_safe=true assistive=true
- **component:structure-tree-row:0001** (structure_tree_row) — Structure tree row for a hand-authored source element
  - A source-element tree row that maps to an exact span and keeps selection synchronized with the canvas and source
  - family=structure_tree_row surface=visual_surface_mapping sync=in_sync_from_source round_trip=exact_source_round_trip export_safe=true assistive=true
- **component:structure-tree-row:0002** (structure_tree_row) — Structure tree row for a loop-generated node with no source span
  - An unmapped node discloses it has no source span and stays inspect-only rather than fake a mapping
  - family=structure_tree_row surface=visual_surface_mapping sync=unidentified_source_sync round_trip=inspect_only_no_write export_safe=true assistive=true
  - Degraded: trigger=unmapped_source — This node has no resolvable source span; it is inspect-only and never claims a mapping
- **component:property-inspector-row:0001** (property_inspector_row) — Property inspector row for a design-token color value
  - A design-token edit names its shared token-definition write scope, previews the real multi-file source diff, and requires review
  - family=property_inspector_row surface=visual_edit_transform sync=in_sync_from_source round_trip=approximate_source_round_trip export_safe=true assistive=true
- **component:property-inspector-row:0002** (property_inspector_row) — Property inspector row for a runtime-bound style value
  - A runtime-bound value is inspect-only; the inspector shows no diff and never widens the write scope silently
  - family=property_inspector_row surface=visual_edit_transform sync=in_sync_from_source round_trip=source_only_fallback export_safe=true assistive=true
  - Degraded: trigger=unsupported_construct — This value is bound to a runtime expression; the inspector stays inspect-only rather than widen the write scope
- **component:source-sync-chip:0001** (source_sync_chip) — Source-sync chip on a preview that drifted from source
  - A drifted source-sync chip discloses the drift and offers a rebuild-from-source recovery route
  - family=source_sync_chip surface=source_first_framework_preview sync=drifted_from_source round_trip=inspect_only_no_write export_safe=true assistive=true
  - Degraded: trigger=drifted_from_source — This preview drifted from the canonical source; rebuild from source before relying on it
- **component:breakpoint-preview-row:0001** (breakpoint_preview_row) — Breakpoint preview row for a mobile viewport over a live runtime
  - A mobile-viewport preview keeps its runtime origin, live data posture, and exact mapping quality visible
  - family=breakpoint_preview_row surface=device_or_simulator_preview sync=in_sync_from_source round_trip=approximate_source_round_trip export_safe=true assistive=true
- **component:breakpoint-preview-row:0002** (breakpoint_preview_row) — Breakpoint preview row for a simulator showing mock data
  - A simulator preview names its mock data posture and unmapped mapping quality rather than blur the runtime truth
  - family=breakpoint_preview_row surface=device_or_simulator_preview sync=runtime_only_no_source round_trip=inspect_only_no_write export_safe=true assistive=true
  - Degraded: trigger=unmapped_source — This simulator preview shows mock data with no source mapping; it stays inspect-only and never claims live source fidelity
- **component:unsupported-construct-card:0001** (unsupported_construct_card) — Unsupported-construct card for a dynamically bound attribute
  - An unsupported-construct card degrades a dynamically bound attribute to a code-first suggestion and preserves the selection context
  - family=unsupported_construct_card surface=visual_edit_transform sync=in_sync_from_source round_trip=source_only_fallback export_safe=true assistive=true
  - Degraded: trigger=unsupported_construct — This construct is dynamically bound and cannot round-trip; the surface degrades to a code-first suggestion with the selection preserved
- **component:round-trip-conflict-banner:0001** (round_trip_conflict_banner) — Round-trip conflict banner after source changed under an edit
  - A round-trip conflict banner names the source-changed-under-edit conflict and offers a reload-and-reapply route instead of a silent writeback
  - family=round_trip_conflict_banner surface=visual_edit_transform sync=pending_rebuild round_trip=exact_source_round_trip export_safe=true assistive=true
  - Degraded: trigger=round_trip_conflict_open — The canonical source changed under this edit; reload the source and re-apply rather than write back over the change
