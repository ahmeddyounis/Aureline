# Fixture: CJK + Latin interleaved

## Scenario

An editor line mixes Han ideographs, Hiragana, and Latin identifiers,
as might appear in a comment in a Japanese-codebase Rust file.

Representative content:

```
// 変数 `counter` は、ユーザーがクリックするたびに 1 ずつ増加する
let counter: u32 = 0;
```

## Hooks exercised

- `reflow_line_range` — wrap behaviour changes with line-break rules
  (UAX #14) for CJK-adjacent content.
- `fallback_glyph_resolution` — Han ideographs resolve through the
  `han_fallback` script-aware preference group; Hiragana through
  `kana_fallback`.

## Stack elements stressed

- Line-break segmentation (UAX #14) in `crates/aureline-text`.
- Distinct fallback groups for Han and Kana; a single "CJK" group is
  explicitly not how the fallback chain is registered.
- Line-layout cache keyed on `(buffer_line_id, content_hash,
  shaping_features, direction, scale_bucket)` — Japanese IME
  substitution during editing must invalidate only the affected line.

## Expected observable outcomes

- `fallback_glyph_resolution` reports one entry per fallback stage the
  run actually uses; a fixture that mixes Han + Hiragana + Latin
  produces three stage records for the line, not one.
- The accessibility tree preserves run boundaries as separate nodes so
  screen readers can announce script transitions.
- A buffer edit that replaces a kana cluster with a Han cluster
  invalidates the line-layout cache entry for that line and no
  neighbouring lines.

## ADR sections motivating this fixture

- Text shaping — shaper consumes already-segmented runs.
- Font discovery and fallback — script-aware preference groups.
- Glyph-cache posture — line-layout cache invalidation per line.
