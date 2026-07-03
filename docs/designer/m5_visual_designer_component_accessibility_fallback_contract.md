# M5 Visual-Designer Component Accessibility Fallback Contract (M05-808)

This contract hardens the frozen M5 visual-designer component matrix
(`schemas/ui/m5-visual-designer-component-matrix.schema.json`, M05-804) so the
visual designer stays **source-first and accessible rather than
gesture-dependent**. Where M05-805..807 resolve each primitive's per-target
truth, this capstone certifies — per frozen `M5VisualDesignerComponentFamily` —
that every canvas / tree / inspector / chip / preview-row primitive carries a
keyboard-complete, screen-reader-reachable, list / tree / textual equivalent, and
that no editing path depends on a pointer-drag gesture alone.

- **Module.**
  `crates/aureline-preview/src/add_visual_designer_fallback_parity_keyboard_and_screen_reader_navigation_and_no_drag_only_editing_rules/`
- **Boundary schema.**
  `schemas/ui/m5-visual-designer-component-accessibility-fallback.schema.json`
- **Support export (`include_str!` canonical).**
  `artifacts/release/m5-visual-designer-component-accessibility-fallback-proof/support_export.json`
- **CSV matrix.**
  `artifacts/release/m5-visual-designer-component-accessibility-fallback-proof/matrix.csv`
- **Fixtures.** `fixtures/ui/m5-visual-designer-component-accessibility-fallback/`

## Certified families

Each of the seven frozen families is certified at least once. Two families render
a visual surface and are **canvas-heavy** — the design canvas and the
breakpoint / device-preview row — and MUST bind a canvas modality to at least one
non-visual (list / tree / textual) modality. Four families expose a **drag
affordance** (design canvas move / resize, structure-tree reorder, inspector
sliders, viewport resize) and MUST expose a command-backed, source-aware
alternative.

## Acceptance criteria

Every `ComponentAccessibilityRow` is scored on three acceptance criteria plus
export and narrowing parity, and derives a green / yellow / red status.

- **AC1 — No drag-only editing (`no_drag_only_editing`).** `drag_editing` is
  never `drag_only_trap`, and any family with a drag affordance names at least one
  `command_backed_action`. Claimed visual-design workflows remain usable without
  pointer-drag interactions.
- **AC2 — Assistive-tech / low-resource reach the same source-backed truth
  (`reaches_source_backed_truth_via_at`).** `keyboard_reach`,
  `screen_reader_reach`, and `low_resource_reach` never resolve to
  `view_only_trap`; the non-visual path reaches the same source span / selection /
  state; and every canvas-heavy family offers a non-visual fallback modality.
- **AC3 — Honest auto-narrowing (`narrowing_disclosed`).** Every narrowed
  rendering surface carries a disclosure that never `silently_dropped` state and
  preserves its labels; any reduced-capability row auto-narrows with a precise,
  non-generic trigger label (reusing `M5VisualDesignerDowngradeTrigger`) and
  `preserves_source_backed_context`, rather than dropping key context.

Export parity requires an `export_summary` that is never `absent_needs_screenshot`
plus text / JSON / Markdown copy parity with a named export field and screenshots
prohibited as the sole export.

### Status derivation

- **`parity` (green).** All ACs and export parity hold with no disclosed
  reductions.
- **`narrowed_disclosed` (yellow).** All ACs and export parity hold, but at least
  one axis (screen-reader, low-resource, drag, export, or a rendering surface) is
  in a disclosed-reduction state backed by an honest auto-narrow block.
- **`stranded` (red).** Any AC or export-parity check fails. Red rows may not
  ship — `validate()` rejects them.

## Boundary posture

The packet is metadata-only. It carries typed class tokens, opaque summary /
evidence refs, booleans, and redacted labels. Raw source bodies, diff hunks,
screenshots, runtime payloads, and credentials never cross this boundary;
`validate()` rejects any export containing obvious secret material.

## Verification

```
cargo test -p aureline-preview --lib add_visual_designer_fallback
cargo run -p aureline-preview --example dump_m5_visual_designer_a11y_fallback support
```

The seeded builder, the example dump, and the on-disk support export stay
byte-aligned; `on_disk_export_matches_builder` fails if they drift.
