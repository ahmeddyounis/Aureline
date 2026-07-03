# M5 Selected-Node Primitive Fixtures

Protected fixture corpus for the M5 selected-node primitive — the design-canvas
frame, structure-tree row, and property-inspector rows resolved once per selected
node (M05-805, batch B94).

- `selected_node_primitive_stable.json` — the canonical, fully valid primitive
  packet. It binds all six claimed visual-design surface families
  (`desktop_designer`, `source_first_preview`, `browser_runtime_inspector`,
  `framework_pack_preview`, `embedded_shell_designer`, `support_export_replay`)
  and carries worked selection cases that exercise selection-identity
  preservation across canvas / tree / inspector (AC1), distinct property-editor
  kinds per value state (AC2), and mutation gated by disclosed source ownership
  and support state (AC3), including inspect-only, protected-blocked, and unmapped
  degraded cases. This is a byte-identical copy of the checked support export at
  `artifacts/release/m5-visual-designer-selected-node-proof/support_export.json`,
  which is the `include_str!` source of truth verified by
  `checked_support_export_matches_builder`.

Every fixture validates against
`schemas/ui/m5-visual-designer-selected-node-primitive.schema.json` and against
`M5SelectedNodePrimitivePacket::validate()`. Each worked selection case is
self-consistent: its stored resolution equals a fresh
`resolve_visual_selection(&input)`. Fixtures carry only typed class tokens,
opaque selection / span refs, booleans, and redacted labels — never raw source
bodies, diff hunks, file contents, or credentials.
