# Fixture: ligatures and stylistic sets

## Scenario

A programmer uses a typeface that exposes programming ligatures
(`=>`, `!=`, `>>=`) and optional stylistic sets. The editor theme
toggles ligatures on and off as a per-workspace preference.

Representative content:

```
fn compose(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |a| g(f(a)) // pipeline: a => f => g
}
```

## Hooks exercised

- `reflow_line_range` — toggling ligatures changes line layout for
  every affected line.
- `frame_submit` — the frame after a theme-level ligature toggle
  repaints only the lines whose shape cache invalidates.

## Stack elements stressed

- Typed feature-flag vocabulary on the shaping run (OpenType `liga`,
  `calt`, stylistic-set `ss01`..`ss20`).
- Shape-cache invalidation keyed on `feature_set` — a ligature toggle
  invalidates shape-cache entries that depend on the toggled
  features, not the whole cache.
- Line-layout cache invalidation keyed on `shaping_features`.

## Expected observable outcomes

- Turning ligatures off invalidates shape-cache entries for runs that
  had a ligature applied and leaves other entries warm.
- Turning stylistic sets on produces distinct shape-cache entries for
  runs that the feature affects; other runs reuse warm entries.
- The accessibility tree preserves the underlying Unicode code points
  regardless of which ligatures or stylistic sets are painted.

## ADR sections motivating this fixture

- Text shaping — feature vocabulary.
- Glyph-cache posture — shape-cache key composition and invalidation.
