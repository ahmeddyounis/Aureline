# M5 Visual-Designer Surface Certification Fixtures

Protected fixture corpus for the M5 visual-designer surface-certification capstone
— per claimed visual-design surface, certifies that a visual-design claim
self-narrows when its mapping quality, round-trip support, or preview-runtime
freshness weakens, and that the export / support packet preserves the same
mapping / runtime truth visible in-product (M05-810, batch B94).

- `visual_designer_surface_certification_stable.json` — the canonical, fully valid
  certification packet. It certifies all ten claimed
  `M5VisualDesignClaimedSurface` surfaces (`design_canvas_workspace`,
  `structure_tree_panel`, `property_inspector_panel`, `source_round_trip_rail`,
  `breakpoint_device_preview_deck`, `framework_pack_preview`,
  `browser_runtime_inspection`, `docs_help_embeds`, `support_export`,
  `release_proof`) and proves, per surface, that the effective claim never exceeds
  what the mapping / round-trip / runtime truth supports (AC1: a stale or partial
  lane can no longer present as fully writable or fully mapped), that the export
  preserves every mandatory truth field — selection identity, mapping quality,
  round-trip state, runtime origin, preview freshness, effective claim, and
  narrowed reason — as text / JSON / Markdown (AC2), and that every narrowing is
  disclosed with an honest `ClaimAutoNarrow` matching the binding dimension while
  the docs / help, support, and release evidence surfaces are all certified (AC3).
  Three surfaces are worked as disclosed-narrowed (yellow) rows —
  `source_round_trip_rail` (approximate mapping narrows to inspect-only),
  `breakpoint_device_preview_deck` (a stale runtime narrows to read-only), and
  `framework_pack_preview` (an aging runtime narrows to inspect-only) — the other
  seven are certified (green): seven green / three yellow / zero red. This is a
  byte-identical copy of the checked support export at
  `artifacts/release/m5-visual-designer-surface-certification-proof/support_export.json`,
  which is the `include_str!` source of truth verified by
  `on_disk_export_matches_builder`.

Every fixture validates against
`schemas/ui/m5-visual-designer-surface-certification.schema.json` and against
`VisualDesignSurfaceCertPacket::validate()`. Fixtures carry only typed class
tokens, opaque summary / evidence refs, booleans, and redacted labels — never raw
source bodies, diff hunks, screenshots, runtime payloads, or credentials.
