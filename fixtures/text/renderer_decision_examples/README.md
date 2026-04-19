# Renderer decision-example fixtures

These fixtures are short, reviewable scenarios that anchor the
protected-hot-path hook names defined in
[ADR 0002](../../../docs/adr/0002-renderer-text-stack-and-shaping-fallback.md)
to concrete inputs and observable outcomes. They are not a test suite;
they are the vocabulary the renderer spike and the benchmark lab use
when they instrument a hook.

**Scope rules**

- Every fixture names the hooks it exercises, the stack element it
  stresses, and the observable outcome the renderer spike or
  benchmark lab should capture.
- Fixtures never assert latency numbers; the benchmark lab owns
  budgets. Fixtures only describe *what* to measure, not *how fast*.
- Do not add third-party font files to this tree. If a fixture
  requires a specific font, name the face and the fallback stage it
  must hit; the benchmark lab resolves the actual file through the
  discovery abstraction.
- A new fixture MUST hit at least one protected-hot-path hook and
  MUST cite the ADR section that motivates it.

**Index**

| Fixture                                   | Primary hooks                                     | Stack element stressed                                              |
|-------------------------------------------|---------------------------------------------------|---------------------------------------------------------------------|
| [`mixed_script_latin_arabic.md`](./mixed_script_latin_arabic.md)         | `reflow_line_range`, `fallback_glyph_resolution`  | Bidi, script-aware fallback group                                   |
| [`cjk_latin_interleaved.md`](./cjk_latin_interleaved.md)                 | `reflow_line_range`, `fallback_glyph_resolution`  | Han fallback group, line layout                                     |
| [`color_emoji_zwj_sequences.md`](./color_emoji_zwj_sequences.md)         | `fallback_glyph_resolution`, `frame_submit`       | Emoji fallback group, colour-font rasterisation                     |
| [`bidi_ime_composition.md`](./bidi_ime_composition.md)                   | `ime_composition_update`, `caret_move`            | Overlay layer, platform-input adapter                               |
| [`ligatures_and_stylistic_sets.md`](./ligatures_and_stylistic_sets.md)   | `reflow_line_range`, `frame_submit`               | Shape cache, feature-flag routing                                   |
| [`missing_glyph_fallback_chain.md`](./missing_glyph_fallback_chain.md)   | `fallback_glyph_resolution`                       | Full fallback chain through to bundled subset                       |
| [`multi_monitor_scale_change.md`](./multi_monitor_scale_change.md)       | `multi_monitor_scale_change`, `atlas_shard_rebind`| Atlas-per-scale bucket, invalidation model                          |
| [`warm_start_first_paint.md`](./warm_start_first_paint.md)               | `warm_start_to_first_paint`, `first_paint`        | GPU / windowing boot path, shape-cache warmth                       |
