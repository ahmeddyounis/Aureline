# Fixture: colour emoji with ZWJ sequences

## Scenario

An editor line or terminal output contains colour-emoji sequences that
compose into single clusters via zero-width joiners: a family emoji,
a skin-tone modifier, and a flag emoji built from regional indicator
pairs.

Representative content:

```
// Release notes: 👨‍👩‍👧‍👦 team 👋🏽 welcome 🇯🇵
```

## Hooks exercised

- `fallback_glyph_resolution` — colour emoji resolve through the
  `emoji_fallback` group.
- `frame_submit` — a line containing colour-emoji clusters still emits
  one GPU submission per surface per frame (colour fonts do not force
  a second submission).

## Stack elements stressed

- Grapheme segmentation (UAX #29) treating ZWJ sequences as single
  clusters.
- COLR / CPAL and bitmap `sbix` rasterisation paths inside the
  rasteriser.
- Shape cache keyed on the full cluster text hash so a ZWJ sequence
  and its decomposition are distinct entries.

## Expected observable outcomes

- Each ZWJ-composed emoji is a single accessibility-tree text node
  with its Unicode representation preserved.
- `fallback_glyph_resolution.stage` reports `2 (script_aware)` for the
  emoji cluster and does not fall through to the bundled subset on a
  host where the `emoji_fallback` group is populated.
- A skin-tone modifier change produces a distinct shape-cache entry
  for the modified cluster rather than a raster-cache swap alone.

## ADR sections motivating this fixture

- Font discovery and fallback — `emoji_fallback` group and
  colour-font support.
- Glyph-cache posture — shape cache key composition.
