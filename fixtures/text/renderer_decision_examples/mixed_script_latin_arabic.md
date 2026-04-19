# Fixture: mixed-script Latin + Arabic line

## Scenario

A single editor line contains a run of Latin code, a run of Arabic
prose, and a return to Latin inside the same paragraph. The surrounding
lines are monolingual Latin.

Representative content:

```
let greeting = "Hello, مرحبا, world"; // greeting also covers ٱلسَّلَامُ
```

## Hooks exercised

- `reflow_line_range` — a local edit inside the Latin prefix must not
  re-shape or re-reflow the Latin suffix run.
- `fallback_glyph_resolution` — the Arabic run resolves through the
  `arabic_fallback` script-aware preference group.

## Stack elements stressed

- Bidi segmentation in `crates/aureline-text` (UAX #9).
- Script-aware fallback group registration in the font-discovery
  abstraction.
- Shape-cache keying on `(font_handle, feature_set, direction, script,
  cluster_text_hash)` — a bidi direction change across the line must
  produce distinct shape-cache entries rather than one entry that
  bleeds across directions.

## Expected observable outcomes

- The accessibility tree publishes the Arabic run with `direction = rtl`
  and the Latin prefix / suffix with `direction = ltr`.
- The renderer spike reports the Arabic run's primary font via
  `fallback_glyph_resolution.stage = 2 (script_aware)` and the Latin
  runs via `fallback_glyph_resolution.stage = 1 (explicit_family)`.
- A single-character edit inside the Latin prefix emits
  `reflow_line_range` covering only that line; neighbouring lines do
  not reflow.

## ADR sections motivating this fixture

- Text shaping — `Shaper` trait and segmentation ownership.
- Font discovery and fallback — `arabic_fallback` script group.
- Invalidation model — localised reflow without cross-line cascade.
