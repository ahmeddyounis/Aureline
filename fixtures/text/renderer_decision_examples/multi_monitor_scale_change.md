# Fixture: multi-monitor scale change

## Scenario

A window containing a populated editor viewport is dragged from a
`scale = 2.0` monitor to a `scale = 1.25` monitor, then back. During
each move the OS emits a scale-change event mid-drag; fractional
scaling is on.

## Hooks exercised

- `multi_monitor_scale_change` — fires on every OS scale-change event.
- `atlas_shard_rebind` — the active raster-cache shard rebinds to the
  target scale bucket.
- `first_paint` — the first frame on the new monitor is a fresh
  `first_paint` for that surface.

## Stack elements stressed

- winit-class window event loop delivering scale-change deltas.
- Atlas-per-scale-bucket posture: moving across monitors rebinds a
  pre-existing shard rather than rebuilding every glyph.
- Scale invalidation rule: active raster-cache shard invalidates;
  shape cache does not.

## Expected observable outcomes

- `atlas_shard_rebind` fires once per monitor move with
  `reason = multi_monitor_scale_change`.
- `first_paint` on the new monitor reuses shape-cache entries;
  `fallback_glyph_resolution` does not fire except for glyphs whose
  raster variants are still cold on the target bucket.
- Returning to the original monitor reuses its previously-built shard
  without producing a flood of `atlas_eviction` events.
- The accessibility tree is continuous across the move; no tree
  rebuild is required by scale change alone.

## ADR sections motivating this fixture

- GPU / windowing assumptions — fractional and per-monitor DPI
  scaling, monitor hot-plug.
- Glyph-cache posture — atlas sharding per scale bucket.
- Invalidation model — scale invalidation.
