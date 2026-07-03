# M5 Selected-Node Primitive: Canvas Frame, Tree Row, and Inspector Rows

- Packet: `m5-visual-designer-selected-node-primitive:stable:0001`
- Label: `M5 Selected-Node Primitive: Canvas Frame, Tree Row, and Inspector Rows`
- Visual-design surfaces: 6 / 6
- Support states: fully_supported, partially_supported, inspect_only, unsupported_construct, unmapped_node
- Editor kinds: literal_field, token_bound_picker, bound_expression_inspector, inherited_value_trace, mixed_multi_value, unset_placeholder
- Value states: literal, design_token, bound_expression, inherited, mixed, unset

## Visual-design surfaces

- **Desktop Designer**: `visual_surface_mapping`
  - Owner: Visual Designer Platform
  - Scope: Desktop designer canvas, structure tree, and property inspector for a source-bound element
  - Worked selections: 1
    - `selection:desktop:hero-heading:0001` → node `source_element` (HeroHeading) support `fully_supported`, 3 inspector rows
- **Source-First Preview**: `source_first_framework_preview`
  - Owner: Source-First Preview
  - Scope: Source-first preview canvas and inspector for a reviewed component instance
  - Worked selections: 1
    - `selection:preview:card-instance:0001` → node `component_instance` (PricingCard) support `partially_supported`, 2 inspector rows
- **Browser-Runtime Inspector**: `browser_runtime_inspection`
  - Owner: Browser Runtime Inspector
  - Scope: Browser-runtime inspector for a runtime-mirrored node whose value is a bound expression
  - Worked selections: 1
    - `selection:runtime:bound-badge:0001` → node `component_instance` (StatusBadge) support `inspect_only`, 2 inspector rows
- **Framework-Pack Preview**: `visual_edit_transform`
  - Owner: Framework Packs
  - Scope: Framework-pack preview inspector for a hand-authored text leaf
  - Worked selections: 1
    - `selection:framework:cta-label:0001` → node `text_leaf` (CtaLabel) support `fully_supported`, 1 inspector rows
- **Embedded Shell Designer**: `embedded_webview_preview`
  - Owner: Embedded Designer
  - Scope: Embedded shell designer inspector for a protected, read-only source element
  - Worked selections: 1
    - `selection:shell:generated-footer:0001` → node `source_element` (GeneratedFooter) support `inspect_only`, 1 inspector rows
- **Support-Export Replay**: `support_export_projection`
  - Owner: Support Export
  - Scope: Support-export replay of a captured selection for a loop-generated, unmapped node
  - Worked selections: 1
    - `selection:support:loop-item:0001` → node `generated_node` (ListItemGenerated) support `unmapped_node`, 1 inspector rows
