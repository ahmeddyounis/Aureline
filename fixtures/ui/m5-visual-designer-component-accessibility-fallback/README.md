# M5 Visual-Designer Component Accessibility Fallback Fixtures

Protected fixture corpus for the M5 visual-designer component accessibility
capstone — list / tree / textual fallback parity, keyboard and screen-reader
navigation, and no-drag-only editing rules certified once per frozen component
family (M05-808, batch B94).

- `visual_designer_component_accessibility_fallback_stable.json` — the canonical,
  fully valid accessibility packet. It certifies all seven frozen
  `M5VisualDesignerComponentFamily` families (`design_canvas`,
  `structure_tree_row`, `property_inspector_row`, `source_sync_chip`,
  `breakpoint_preview_row`, `unsupported_construct_card`,
  `round_trip_conflict_banner`) and proves, per family, that no editing path is
  drag-only (AC1: every drag affordance has a command-backed, source-aware
  alternative), that assistive-tech and low-resource modes reach the same
  source-backed truth as the canvas path (AC2: keyboard / screen-reader /
  low-resource reach with a list / tree / textual fallback for canvas-heavy
  families), and that reduced-capability states auto-narrow honestly with a
  precise trigger and preserved context (AC3). The two canvas-heavy families
  (`design_canvas`, `breakpoint_preview_row`) are worked as disclosed-narrowed
  (yellow) rows; the rest are full-parity (green) — five green / two yellow / zero
  red. This is a byte-identical copy of the checked support export at
  `artifacts/release/m5-visual-designer-component-accessibility-fallback-proof/support_export.json`,
  which is the `include_str!` source of truth verified by
  `on_disk_export_matches_builder`.

Every fixture validates against
`schemas/ui/m5-visual-designer-component-accessibility-fallback.schema.json` and
against `ComponentAccessibilityPacket::validate()`. Fixtures carry only typed
class tokens, opaque summary / evidence refs, booleans, and redacted labels —
never raw source bodies, diff hunks, screenshots, runtime payloads, or
credentials.
