# Editor viewport cases

These fixtures exercise the editor viewport model and its damage-class
classification. They are intentionally small and deterministic so editor,
renderer, and shell wiring can share one vocabulary for:

- caret-only overlay updates,
- selection overlay updates,
- scroll translate updates,
- IME marked-text overlay updates,
- resize / scale-change invalidation.

The expected ids match the canonical render packets:

- `render_layer.*` (composition layers)
- `render_damage.*` (damage classes)
- `hook id` strings from `aureline-render` (`caret_move`, `scroll_frame`, …)

Some actions are expected to be no-ops (for example scrolling beyond clamp
limits). In those cases the fixture sets `"damage": null` and the harness
asserts that the viewport returns no damage event.
