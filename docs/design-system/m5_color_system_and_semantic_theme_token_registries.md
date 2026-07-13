# M5 Color-System and Semantic-Theme-Token Registries

This document is the human-readable companion to the **first implement lane over the frozen
[M5 visual-foundation matrix][matrix]**. It turns the two foundation families that carry semantic
*meaning* — the **color system** and the **semantic theme token** — into registry resolvers that
produce export-safe, honest projections. The authoritative gate is the Rust validator in
[`crates/aureline-ui/src/m5_color_system_and_semantic_theme_token_registries`](../../crates/aureline-ui/src/m5_color_system_and_semantic_theme_token_registries/mod.rs);
this doc explains what the registries lock and how the first consumers adopt them.

- Packet id: `m5-color-system-and-semantic-theme-token-registries:stable:0001`
- Registries schema:
  [`schemas/design-system/m5-color-system-and-semantic-theme-token-registries.schema.json`](../../schemas/design-system/m5-color-system-and-semantic-theme-token-registries.schema.json)
- Canonical domain schema:
  [`schemas/design-system/m5-color-system.schema.json`](../../schemas/design-system/m5-color-system.schema.json)
  (both families point at this one target)
- Frozen matrix contract: [`m5_visual_foundations_contract.md`](m5_visual_foundations_contract.md)
- Canonical proof set:
  [`artifacts/release/m5-color-system-and-semantic-theme-token-registries-proof/support_export.json`](../../artifacts/release/m5-color-system-and-semantic-theme-token-registries-proof/support_export.json)
  (with `matrix.csv` and `summary.md`)
- Narrowed fixtures:
  [`fixtures/ui/m5-color-system-and-semantic-theme-token-registries/`](../../fixtures/ui/m5-color-system-and-semantic-theme-token-registries/)

## Why this exists

The frozen visual-foundation matrix names the eight foundation families and locks their controlled
vocabulary, but it stays a *matrix* — it does not resolve concrete color or theme meaning that a surface
can consume. This lane implements the two families that carry semantic meaning as registries, so brand,
interactive, neutral, and the operational status families (success / warning / danger / info / insight
and the trust-sensitive restricted / remote / collaboration / AI / debug states) stop drifting by
surface family and stay stable in dark, light, and high-contrast modes.

## What the resolvers lock

- **`resolve_color_entry`** refuses to read as a clean, distinct color-registry entry unless it names a
  canonical token, a classified operational-state family, a color role, and a non-color cue, covers all
  three theme modes, stays distinguishable in every mode, and traces to a canonical token rather than an
  inlined raw color. Otherwise it degrades — never a color-only, raw-color, or mode-incomplete pass.
- **`resolve_theme_token_entry`** refuses to read as a clean, stable theme-token entry unless it names a
  canonical token and a stable theme-token role, covers the dark / light / high-contrast pair, and keeps
  its role stable across surfaces. A raw hex value inlined on a surface degrades honestly.
- **Non-color cue required.** Every color entry names an `M5NonColorCue` (text label, icon glyph, border
  treatment, shape / pattern, or screen-reader text). Meaning that would otherwise ride on hue alone
  degrades to `meaning_encoded_by_color_alone` or `non_color_cue_missing`.

## Acceptance criteria proven by resolved examples

1. **The first claimed consumers use the canonical color/state families instead of feature-local
   palettes.** The shell, editor, review, notebook, and data surfaces resolve their color entries through
   this lane; clean entries cover the brand / interactive / neutral / status semantic families and those
   five first-consumer surfaces, a raw-color example degrades, and no clean entry inlines a raw color.
2. **Restricted, remote, AI, collaboration, and debug states remain distinguishable in dark, light, and
   high-contrast modes.** Every trust-sensitive state is covered by a clean entry with full mode parity
   and a non-color cue; a mode-parity-incomplete example and a color-only example both degrade.
3. **Raw-color or ambiguous status regressions are detectable by fixtures, linting, or release
   evidence.** A raw-color color example and a raw-hex theme example both degrade, clean entries trace to
   a canonical token, and the checked support export / narrowed fixtures fail validation on drift.

## How consumers adopt it

Later components, docs / help, exports, and support packets consume this registry (or the canonical
`m5-color-system.schema.json` domain schema it points at) instead of re-describing color or theme meaning
manually. The single mint-from-truth path is the headless emitter
`cargo run -p aureline-ui --example dump_m5_color_theme_registries`; the checked proof set and fixtures
are byte-locked to the seed builder, so a raw-color or ambiguous-status regression is caught before
release evidence turns green.

[matrix]: ../../crates/aureline-ui/src/m5_visual_foundation_matrix/mod.rs
