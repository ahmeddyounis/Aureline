# M5 Visual-Designer Component Consumers

- Packet: `m5-visual-designer-component-consumers:stable:0001`
- As of: `2026-07-03T00:00:00Z`
- Rows: 12 across 4 consumer groups and 7 / 7 frozen families
- Families reused across groups: 5

## Rows

- **consumer:framework-pack:design-canvas** — surface=framework_pack_preview_lane group=framework_pack family=design_canvas authority=full label_parity=preserved handoff=none
- **consumer:framework-pack:breakpoint-preview-row** — surface=framework_pack_preview_lane group=framework_pack family=breakpoint_preview_row authority=compare_only label_parity=disclosed_narrowed handoff=none
- **consumer:framework-pack:structure-tree-row** — surface=framework_pack_preview_lane group=framework_pack family=structure_tree_row authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:preview-runtime:property-inspector-row** — surface=preview_runtime_inspector group=preview_runtime family=property_inspector_row authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:preview-runtime:source-sync-chip** — surface=preview_runtime_inspector group=preview_runtime family=source_sync_chip authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:preview-runtime:breakpoint-preview-row** — surface=preview_runtime_inspector group=preview_runtime family=breakpoint_preview_row authority=compare_only label_parity=disclosed_narrowed handoff=none
- **consumer:browser-runtime:structure-tree-row** — surface=browser_runtime_inspector group=browser_runtime_demo family=structure_tree_row authority=read_only label_parity=disclosed_narrowed handoff=browser_readonly
- **consumer:browser-runtime:round-trip-conflict-banner** — surface=browser_runtime_inspector group=browser_runtime_demo family=round_trip_conflict_banner authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:demo-handoff:unsupported-construct-card** — surface=demo_handoff group=browser_runtime_demo family=unsupported_construct_card authority=export_only label_parity=disclosed_narrowed handoff=handoff_packet
- **consumer:docs-onboarding:design-canvas** — surface=docs_onboarding_walkthrough group=docs_onboarding family=design_canvas authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:docs-onboarding:source-sync-chip** — surface=docs_onboarding_walkthrough group=docs_onboarding family=source_sync_chip authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:help-center:unsupported-construct-card** — surface=help_center group=docs_onboarding family=unsupported_construct_card authority=read_only label_parity=disclosed_narrowed handoff=none
