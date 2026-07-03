# M5 Visual-Designer Component Fixtures

Protected fixture corpus for the frozen M5 visual-designer component matrix
(M05-804, batch B94).

- `visual_designer_component_matrix_stable.json` — the canonical, fully valid
  matrix packet. It defines all seven reusable component families
  (`design_canvas`, `structure_tree_row`, `property_inspector_row`,
  `source_sync_chip`, `breakpoint_preview_row`, `unsupported_construct_card`,
  `round_trip_conflict_banner`) and exercises the degraded / narrowed states
  (unmapped source, drift, inspect-only bound expression, unsupported construct,
  and an open round-trip conflict). This is a byte-identical copy of the checked
  support export at
  `artifacts/preview/m5/m5_visual_designer_component_matrix/support_export.json`,
  which is the `include_str!` source of truth verified by
  `checked_support_export_matches_builder`.

Every fixture validates against
`schemas/ui/m5-visual-designer-component-matrix.schema.json` and against
`VisualDesignerComponentMatrix::validate()`. Fixtures carry only typed class
tokens, opaque refs, booleans, and redacted labels — never raw source, diff
hunks, or credentials.
