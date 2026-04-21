# Text-stack prototype

This prototype validates the text-stack contract frozen in
[`docs/adr/0002-renderer-text-stack-and-shaping-fallback.md`](../../docs/adr/0002-renderer-text-stack-and-shaping-fallback.md)
early enough that later shell, renderer, benchmark, IME, and
accessibility work can instrument against concrete hook names and a
named fallback chain rather than against a moving target.

It is a **prototype**, not a production text pipeline. The goal is
shape correctness at the contract level (fallback stages, cache
invalidation, hook firing), not rendering fidelity.

## Where the code lives

| Piece | Path |
|---|---|
| Prototype shaper, fallback chain, cache layers, text layer | [`crates/aureline-text/src/prototype.rs`](../../crates/aureline-text/src/prototype.rs) |
| Bench harness (corpus → structural metrics) | [`crates/aureline-bench/src/text_stack.rs`](../../crates/aureline-bench/src/text_stack.rs) |
| Bench binary | [`crates/aureline-bench/src/bin/bench_text_stack.rs`](../../crates/aureline-bench/src/bin/bench_text_stack.rs) |
| Smoke corpus | [`fixtures/text/shaping_smoke_cases.txt`](../../fixtures/text/shaping_smoke_cases.txt) |
| Committed metrics seed | [`artifacts/bench/text_stack_metrics_seed.json`](../../artifacts/bench/text_stack_metrics_seed.json) |
| Shell-spike integration smoke | [`crates/aureline-shell-spike/src/text_layer.rs`](../../crates/aureline-shell-spike/src/text_layer.rs) |
| Bench wrapper | [`tools/bench_text_stack.sh`](../../tools/bench_text_stack.sh) |

## What the prototype models

- **Grapheme segmentation.** A simplified rule set: combining marks,
  variation selectors, and emoji modifiers attach to the previous
  base; ZWJ (U+200D) additionally joins adjacent emoji; two adjacent
  regional-indicator codepoints pair into a flag cluster. Enough to
  keep the smoke corpus honest — *not* a UAX #29 conformant
  segmenter.
- **Script detection.** Coarse buckets (Latin, Han, Kana, Hangul,
  Arabic, Hebrew, Emoji, Unknown) chosen from the cluster's first
  non-mark codepoint via Unicode block ranges.
- **Fallback chain** (ADR §Font discovery and fallback):
  1. `explicit_family` — caller-declared family (`editor_default`
     covers Latin).
  2. `script_preference_group` — per-script font (Han/Kana →
     `han_fallback`, Hangul → `hangul_fallback`, Arabic/Hebrew →
     `arabic_fallback`, Emoji → `emoji_fallback`). **Stage 2 or
     later fires `fallback_glyph_resolution`.**
  3. `system_ui` — OS system-UI family.
  4. `bundled_subset` — bundled Noto-class signed subset (terminal
     stage on a supported host).
  5. `missing` — `.notdef`, MUST be zero.
- **Shape cache** keyed on
  `(cluster_text, font_handle, feature_set, direction, script)`.
- **Raster cache** keyed on
  `(glyph_id, font_handle, px_size, subpixel_variant, scale_bucket)`.
  Scale changes drop the raster cache but leave the shape cache
  warm, matching ADR §Invalidation model.
- **Metrics.** Structural counts only: shape-cache hits / misses,
  raster-cache hits / misses, cluster counts, missing-glyph counts,
  per-stage fallback histogram. No wall-clock times live in the
  committed seed so it can be diffed across hosts.

## How to run

From the repo root:

```
./tools/bench_text_stack.sh
```

Defaults:

- corpus: `fixtures/text/shaping_smoke_cases.txt`
- iterations: `2` (so the second iteration exercises both caches)
- emit: `artifacts/bench/text_stack_metrics_seed.json`

Flags:

- `--release` — build/run the release profile.
- `--iterations N` — override the iteration count.
- `--corpus PATH` — point at an alternative TSV corpus.
- `--emit PATH` — write the metrics JSON to a different file (pass
  `/dev/stdout` to print).

The Rust harness has its own tests (`cargo test -p aureline-bench`);
one of them (`committed_seed_matches_harness_output`) asserts
byte-equality with the committed seed, so any change to the
corpus, the prototype, or the harness requires regenerating the
seed in the same change.

## Shell-spike integration

The shell-spike binary gains a `--text-stack-smoke` mode (see
`crates/aureline-shell-spike/src/bin/shell_spike.rs`) that runs the
corpus through the prototype text layer and prints the per-case
metrics. It reuses the same corpus file and the same prototype
API, so there is one place to change a case or rename a fallback
stage. The spike's **fixture scene is unchanged** — the frozen
`artifacts/render/spike_trace_samples/` and `spike_capabilities.json`
outputs continue to be byte-stable.

## Known holes — carried forward, not hidden in comments

These are recorded here rather than left implicit in source comments.
Every item below is a visible carry-forward task; none is a silent
capability of the prototype.

1. **No real font files.** `FontHandle` values are synthetic; no
   TTF/OTF is loaded, no CoreText/DirectWrite/fontconfig is queried.
   Production shaper and fontdb-class discovery replace this
   wholesale.
2. **No real shaping.** Each cluster resolves to exactly one
   synthetic glyph id derived from its UTF-8 bytes plus the font
   handle. Real ligatures, kerning, cursive shaping, contextual
   alternates, variable-font axes, and OT feature lookups are out of
   scope. The `FeatureSet` struct declares the vocabulary (ligatures,
   stylistic set) without applying it.
3. **Simplified grapheme segmentation.** The rule set is *not* UAX #29
   conformant: prepend+extend sequences (Devanagari, Indic
   conjuncts), split-base Hangul syllables in Jamo form, and
   Extended_Pictographic_Extend boundaries are not handled; only
   basic combining, ZWJ, variation selectors, emoji modifiers, and
   regional-indicator pairs are modelled.
4. **Coarse script detection.** Unicode blocks only. Runs with mixed
   scripts inside one syllable (e.g. Indic vowel carriers) will
   resolve to the first base's script and stop; no
   script-itemisation boundary is drawn mid-run.
5. **No bidi algorithm.** RTL is a per-cluster bit driven by script
   bucket; UBA (UAX #9) level resolution, explicit embedding
   overrides, and paragraph direction are out of scope. Mixed-bidi
   strings render with the visual order of the input codepoints,
   not the resolved visual order.
6. **No caret or selection model.** The prototype emits
   `ShapedCluster` records but does not expose caret hit-testing,
   grapheme-safe selection endpoints, cluster-to-logical mapping
   beyond the byte offset, or IME-safe caret-inside-composition
   positions. The shell spike still owns caret/selection via
   [`crates/aureline-shell-spike/src/input_path.rs`](../../crates/aureline-shell-spike/src/input_path.rs);
   wiring the two together is a follow-up.
7. **No colour-font or layered emoji raster.** Emoji clusters resolve
   to `emoji_fallback` but the raster cache only stores a single
   `(glyph_id, font)` pair; COLR/CPAL layers, sbix bitmap selection,
   and emoji variation-selector fallback (monochrome vs colour) are
   deferred to the production rasteriser.
8. **No platform-native shaper seam.** `ShaperPolicy` declares
   `RustNative` vs `PlatformNative`, and the stub always behaves as
   `RustNative`; no CoreText / DirectWrite / Pango adapter is wired.
   The seam exists for the production engine to back.
9. **No GPU atlas.** The raster cache is a `HashMap<RasterKey, u32>`
   in CPU memory. Atlas sharding per scale bucket, LRU eviction
   reason codes (`lru`, `atlas_full`, `font_unloaded`,
   `scale_bucket_retired`), and the `atlas_shard_rebind` /
   `atlas_eviction` hooks are placeholder-only.
10. **No accessibility tree publication.** Shaped runs carry enough
    information to populate an AT tree, but the prototype does not
    emit one. The `accessibility_tree_update` hook is not fired by
    the prototype; the accessibility bridge work lands separately.
11. **No performance budget enforcement.** The harness captures
    counts, not wall-clock latency; the benchmark lab layers
    timing on top of these counts against the ADR's protected
    budgets. Nothing in this prototype claims to meet any latency
    ceiling.
12. **Platform font discrepancies are unmodelled.** The system-UI
    stage resolves by script classification only. Real platforms
    disagree (system-UI on macOS differs from Windows' Segoe UI
    family differs from Linux fontconfig defaults); the prototype
    records `system_ui` as a single bucket and leaves per-platform
    divergence to the production fontdb seam.
13. **Locale and language are ignored.** Shaping is purely
    script-driven; CJK Han unification (`ja` vs `zh-Hans` vs
    `zh-Hant` vs `ko`) and Arabic locale variants (`ar-EG` vs
    `ar-MA`) resolve to the same font bucket. The production
    pipeline carries locale through the shaping run.
14. **Corpus coverage is representative, not exhaustive.** The
    smoke corpus covers file-name, code-comment, and UI-label
    idioms plus the checked-in scripts; languages like Devanagari,
    Thai, Burmese, Khmer, Tibetan, and ethiopic are intentionally
    out of scope for this prototype and open later corpus rows.

## Carry-forward items (what the next wave of work picks up)

- Replace `StubShaper` with a HarfBuzz-class Rust-native engine
  and, behind the same trait, a platform-native adapter selected
  by the shaping-run policy.
- Replace `FontHandle` with an opaque handle issued by a
  `fontdb`-class discovery layer that honours CoreText / DirectWrite
  / fontconfig and the bundled Noto subset.
- Add a bidi pass (UAX #9) between segmentation and shaping.
- Publish the accessibility tree from shaped runs through the
  `accesskit`-class bridge.
- Back the raster cache with a real atlas-per-scale-bucket GPU
  resource plus the LRU eviction reason codes named in the ADR.
- Extend the benchmark-lab reproducibility pack to capture the
  hardware, driver, and fallback-chain composition alongside the
  structural counts the prototype emits today.
- Grow the smoke corpus to cover Indic, SEA, and Ethiopic scripts
  and add a parallel "code-files" corpus sampled from common
  language identifiers and comment styles.
- Wire the text layer into the shell-spike fixture scene (today
  only the smoke mode invokes it) so fixture-scene trace samples
  include `fallback_glyph_resolution` marks.
