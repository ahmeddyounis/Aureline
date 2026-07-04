# M5 Visual-Designer Component Consumer Fixtures

Protected fixture corpus for the M5 visual-designer component first-consumer
adoption lane — proof that the seven frozen `M5VisualDesignerComponentFamily`
primitives are reusable across the claimed M5 handoff surfaces rather than a
single designer-page implementation (M05-809, batch B94).

- `visual_designer_component_consumers_stable.json` — the canonical, fully valid
  consumer packet. Twelve adoption rows span all four claimed consumer classes
  (`framework_pack`, `preview_runtime`, `browser_runtime_demo`,
  `docs_onboarding`) and adopt all seven frozen families. Each row points back to
  exactly one canonical primitive family (the M05-805 selected-node, M05-806
  source round-trip honesty, or M05-807 breakpoint / device-preview schema and
  release-proof packet) instead of cloning surface-local prose, and proves:
  - **AC1** — multiple M5 surfaces point back to one canonical family
    (`design_canvas`, `structure_tree_row`, `source_sync_chip`,
    `breakpoint_preview_row`, and `unsupported_construct_card` are each adopted by
    two or more consumer groups).
  - **AC2** — degraded / inspect-only / compare-only / read-only / export-only
    consumers stay label- and state-parity with the primary designer surface:
    they preserve the identical controlled label families (`support_class`,
    `runtime_origin`, `unsupported_construct`, `round_trip_conflict`,
    `open_source_fallback`), keep the same token / density / motion behavior from
    the design-system contract, and disclose every reduction with a
    reduced-capability banner (and a handoff note when they punt to another
    surface).
  - **AC3** — the docs / help / onboarding consumers reference the canonical
    component families rather than cloning local visual-designer semantics.

  This is a byte-identical copy of the checked support export at
  `artifacts/release/m5-visual-designer-component-consumer-proof/support_export.json`,
  which is the `include_str!` source of truth verified by
  `on_disk_export_matches_builder`.

Every fixture validates against
`schemas/ui/m5-visual-designer-component-consumer.schema.json` and against
`VisualDesignerConsumerPacket::validate()`. Fixtures carry only typed class
tokens, opaque summary / evidence refs, booleans, and redacted labels — never raw
source bodies, diff hunks, screenshots, runtime payloads, or credentials.
