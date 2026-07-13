# M5 Motion-Token and Reduced-Motion Registries

This document is the human-readable companion to the **first implement lane over the frozen
[M5 motion / layer / iconography matrix][matrix]**. It turns the two interaction families that carry the
*motion* grammar — the **motion token** (duration / easing families) and the **reduced motion** clamp —
into registry resolvers that produce export-safe, honest projections. The authoritative gate is the Rust
validator in
[`crates/aureline-ui/src/m5_motion_token_and_reduced_motion_registries`](../../crates/aureline-ui/src/m5_motion_token_and_reduced_motion_registries/mod.rs);
this doc explains what the registries lock and how the first consumers adopt them.

- Packet id: `m5-motion-token-and-reduced-motion-registries:stable:0001`
- Registries schema:
  [`schemas/design-system/m5-motion-token-and-reduced-motion-registries.schema.json`](../../schemas/design-system/m5-motion-token-and-reduced-motion-registries.schema.json)
- Canonical domain schema:
  [`schemas/design-system/m5-motion-and-reduced-motion.schema.json`](../../schemas/design-system/m5-motion-and-reduced-motion.schema.json)
  (both families point at this one target)
- Frozen matrix contract: [`m5_motion_layer_iconography_contract.md`](m5_motion_layer_iconography_contract.md)
- Canonical proof set:
  [`artifacts/release/m5-motion-token-and-reduced-motion-registries-proof/support_export.json`](../../artifacts/release/m5-motion-token-and-reduced-motion-registries-proof/support_export.json)
  (with `matrix.csv` and `summary.md`)
- Narrowed fixtures:
  [`fixtures/ui/m5-motion-token-and-reduced-motion-registries/`](../../fixtures/ui/m5-motion-token-and-reduced-motion-registries/)

## Why this exists

The frozen motion / layer / iconography matrix names the seven visual-interaction families and locks their
controlled vocabulary, but it stays a *matrix* — it does not resolve concrete motion behavior that a
surface can consume. This lane implements the two families that carry the motion grammar as registries, so
transitions clarify origin and completion without ever competing with typing, palette input, or
decision-making, and so reduced-motion / power-saver / thermal clamps are respected with a static fallback
that preserves meaning.

## What the resolvers lock

- **`resolve_motion_entry`** refuses to read as a clean, protected-path-safe motion entry unless it names a
  canonical token, a classified motion surface class, a motion role, and a reduced-motion fallback, covers
  all three clamps, respects input priority, introduces no layout shift, and traces to a canonical token
  rather than an inlined raw duration. Otherwise it degrades — never a protected-path-delaying,
  layout-shifting, raw-duration, or clamp-incomplete pass.
- **`resolve_reduced_motion_entry`** refuses to read as a clean, clamp-safe reduced-motion entry unless it
  names a canonical token and a reduced-motion role, covers the reduced-motion / power-saver / thermal
  clamps, and keeps a static fallback that preserves meaning. Meaning that would otherwise ride on motion
  alone degrades honestly.
- **Reduced-motion fallback required.** Every motion entry names an `M5ReducedMotionFallback` (instant
  state change, opacity crossfade, static indicator, textual status, or screen-reader announcement).
  Motion that would otherwise delay input, shift layout, or carry the only cue degrades to
  `protected_path_delayed_by_motion`, `layout_shift_introduced`, or `reduced_motion_fallback_missing`.

## Acceptance criteria proven by resolved examples

1. **The first claimed consumers use one canonical motion grammar instead of feature-local transitions.**
   The shell, dialog, panel, embedded, and notification surfaces resolve their motion entries through this
   lane; clean entries cover the motion / attention semantic families and those five first-consumer
   surfaces, a raw-duration example degrades, and no clean entry inlines a raw duration.
2. **Protected input paths are not delayed by decorative motion, and reduced-motion behavior is explicit
   and testable.** Every protected command-palette / menu / typing / inline-editor / diagnostic surface
   class is covered by a clean entry with full clamp coverage that respects input priority and preserves no
   layout shift; a clamp-incomplete example and a protected-path-delay example both degrade.
3. **Animation regressions are detectable by fixtures or release evidence before promotion.** A
   raw-duration motion example and a motion-only reduced-motion example both degrade, clean entries trace
   to a canonical token, and the checked support export / narrowed fixtures fail validation on drift.

## How consumers adopt it

Later components, docs / help, exports, and support packets consume this registry (or the canonical
`m5-motion-and-reduced-motion.schema.json` domain schema it points at) instead of re-describing motion or
reduced-motion behavior manually. The single mint-from-truth path is the headless emitter
`cargo run -p aureline-ui --example dump_m5_motion_token_and_reduced_motion_registries`; the checked proof
set and fixtures are byte-locked to the seed builder, so a protected-path delay or raw-duration regression
is caught before release evidence turns green.

[matrix]: ../../crates/aureline-ui/src/m5_motion_layer_iconography_matrix/mod.rs
