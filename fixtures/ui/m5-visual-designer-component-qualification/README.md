# M5 Visual-Designer Component Qualification Fixtures

Protected fixture corpus for the M5 visual-designer component qualification capstone
— one referenceable certification bundle proving that every claimed visual-design,
preview, framework-pack, docs / demo, and handoff consumer of the shared
visual-designer components either passes the shared component parity check on every
dimension or narrows automatically (M05-811, batch B94).

- `visual_designer_component_qualification_stable.json` — the canonical, fully valid
  qualification packet. It qualifies all eight claimed
  `M5QualifiedComponentConsumer` consumers (`visual_design_surface`,
  `preview_runtime`, `framework_pack_preview`, `docs_demo_embeds`,
  `handoff_consumer`, `support_packet`, `help_center`, `release_evidence`) and
  proves, per consumer, that the shared components hold on all five
  `M5ComponentQualificationDimension` parity dimensions — source ownership, mapping
  quality, round-trip state, token / binding provenance, and accessibility / export
  behavior (AC1: every claim-bearing consumer uses the same shared components and
  either passes or narrows). It consolidates the whole B94 lane by listing every
  canonical component packet in `certified_component_packets` — the frozen matrix,
  the selected-node / source-round-trip / breakpoint-preview primitives, the
  accessibility fallback, the consumer adoption, and the surface certification — and
  every row cites the single certification bundle and draws only from that set (AC2),
  while every row's export preserves the mandatory per-dimension parity fields as
  text / JSON / Markdown and every narrowed consumer additionally exports its
  narrowed reason (AC3). Three consumers are worked as disclosed-narrowed (yellow)
  rows — `preview_runtime` (an approximate source mapping narrows the mapping-quality
  dimension), `framework_pack_preview` (an open round-trip conflict narrows the
  round-trip dimension to inspect-only), and `handoff_consumer` (a bound expression
  drifted from its source binding narrows the token / binding provenance dimension) —
  the other five are fully qualified (green): five green / three yellow / zero red.
  This is a byte-identical copy of the checked support export at
  `artifacts/release/m5-visual-designer-component-qualification-proof/support_export.json`,
  which is the `include_str!` source of truth verified by
  `on_disk_export_matches_builder`.

Every fixture validates against
`schemas/ui/m5-visual-designer-component-qualification.schema.json` and against
`VisualDesignerQualificationPacket::validate()`. Fixtures carry only typed class
tokens, opaque summary / evidence refs, booleans, and redacted labels — never raw
source bodies, diff hunks, screenshots, runtime payloads, or credentials.
