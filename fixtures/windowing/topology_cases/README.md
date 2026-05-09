# Windowing topology fixtures (low-level geometry)

These JSON fixtures are consumed by `crates/aureline-shell/src/windowing/display_safety.rs`
unit tests. They intentionally keep the scope narrow: they model display rectangles and a
single window rectangle in **physical pixels** so the recenter/clamp math stays deterministic.

They are not a replacement for the schema-backed platform fixtures under
`fixtures/platform/window_display_cases/`, which exercise the full
[`docs/ux/window_display_contract.md`](../../../docs/ux/window_display_contract.md) vocabulary.

## Shape

- `displays[]`
  - `x`, `y`, `width`, `height`: physical pixel bounds for each display.
  - `scale_factor` (optional): included for readability when authoring mixed-DPI cases.
  - `primary` (optional): included for readability; the unit tests choose the display that
    contains `(0, 0)` as the best-effort primary when validating anchors.
- `window`
  - `x`, `y`, `width`, `height`: physical pixel bounds of the window.
- `expected`
  - `offscreen`: whether the window is fully outside every display.
  - `anchor_within_safe_bounds`: whether the computed safe anchor point lands within a safe
    display rectangle.
