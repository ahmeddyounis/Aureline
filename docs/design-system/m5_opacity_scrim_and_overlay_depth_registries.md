# M5 Opacity / Scrim and Overlay-Depth Registries

This document is the human-readable companion to the **overlay-depth implement lane over the frozen
[M5 motion / layer / iconography matrix][matrix]**. It turns the two interaction families that carry the
*overlay depth* grammar — the **opacity / scrim** primitives (lightweight versus blocking overlays) and the
**layer-order** z-order tiers — into registry resolvers that produce export-safe, honest projections. The
authoritative gate is the Rust validator in
[`crates/aureline-ui/src/m5_opacity_scrim_and_overlay_depth_registries`](../../crates/aureline-ui/src/m5_opacity_scrim_and_overlay_depth_registries/mod.rs);
this doc explains what the registries lock and how the first consumers adopt them.

- Packet id: `m5-opacity-scrim-and-overlay-depth-registries:stable:0001`
- Registries schema:
  [`schemas/design-system/m5-opacity-scrim-and-overlay-depth-registries.schema.json`](../../schemas/design-system/m5-opacity-scrim-and-overlay-depth-registries.schema.json)
- Canonical domain schemas:
  [`schemas/design-system/m5-opacity-scrim.schema.json`](../../schemas/design-system/m5-opacity-scrim.schema.json)
  (scrim family) and
  [`schemas/design-system/m5-layer-order-and-portal.schema.json`](../../schemas/design-system/m5-layer-order-and-portal.schema.json)
  (overlay-depth family)
- Frozen matrix contract: [`m5_motion_layer_iconography_contract.md`](m5_motion_layer_iconography_contract.md)
- Canonical proof set:
  [`artifacts/release/m5-opacity-scrim-and-overlay-depth-registries-proof/support_export.json`](../../artifacts/release/m5-opacity-scrim-and-overlay-depth-registries-proof/support_export.json)
  (with `matrix.csv` and `summary.md`)
- Narrowed fixtures:
  [`fixtures/ui/m5-opacity-scrim-and-overlay-depth-registries/`](../../fixtures/ui/m5-opacity-scrim-and-overlay-depth-registries/)

## Why this exists

The frozen motion / layer / iconography matrix names the seven visual-interaction families and locks their
controlled vocabulary, but it stays a *matrix* — it does not resolve concrete overlay behavior that a
surface can consume. This lane implements the two families that carry the overlay-depth grammar as
registries, so a scrim keeps the workspace orientable and text legible instead of turning it into an
unreadable backdrop, so blocking overlays always offer a dismiss affordance, so every overlay stacks under
one shared z-order model no private overlay can bypass, and so scrims narrow honestly under reduced-motion,
power-saver, and thermal runtime pressure without hiding why behavior changed.

## What the resolvers lock

- **`resolve_scrim_entry`** refuses to read as a clean, orientation-safe scrim entry unless it names a
  canonical token, a classified overlay depth class, an opacity / scrim role, and a contrast treatment,
  covers all three runtime clamps, preserves workspace orientation, preserves text contrast, and traces to a
  canonical token rather than an inlined raw opacity value. Otherwise it degrades — never an
  orientation-erasing, contrast-losing, raw-opacity, or clamp-incomplete pass.
- **`resolve_overlay_depth_entry`** refuses to read as a clean, shared-z-order-safe overlay-depth entry
  unless it names a canonical token, a layer-order role, and a classified overlay depth class, covers the
  reduced-motion / power-saver / thermal clamps, and stacks under the single shared z-order model. A private
  layer that would bypass the shared model degrades honestly.
- **Contrast treatment required.** Every scrim entry names an `M5ScrimContrastTreatment` (dim backdrop with
  readable text, blur with a contrast floor, solid panel behind text, high-contrast border, or a
  screen-reader context announcement). A scrim that would otherwise erase orientation, drop text contrast, or
  carry no contrast cue degrades to `orientation_erased_by_scrim`, `text_contrast_lost`, or
  `contrast_cue_missing`.

## Acceptance criteria proven by resolved examples

1. **The first claimed consumers use one canonical overlay grammar instead of feature-local scrims.** The
   shell, dialog, panel, embedded, and notification surfaces resolve their scrim entries through this lane;
   clean entries cover the overlay / attention semantic families and those five first-consumer surfaces, a
   raw-opacity example degrades, and no clean entry inlines a raw opacity value.
2. **Scrims and overlay-depth classes preserve contrast and orientation instead of turning the workspace
   into an unreadable backdrop.** Every blocking modal / sheet / confirm / wizard / credential depth class is
   covered by a clean scrim entry with full clamp coverage that preserves orientation and text contrast; a
   clamp-incomplete example and an orientation-erased example both degrade.
3. **The first claimed overlays show correct blocking-versus-nonblocking depth truth.** Every blocking depth
   class plus at least one non-blocking class is covered by a clean overlay-depth entry that stacks under the
   shared z-order model; a private-bypass example and a not-stacked example both degrade, and the checked
   support export / narrowed fixtures fail validation on drift.

## How consumers adopt it

Later components, docs / help, exports, and support packets consume this registry (or the canonical
`m5-opacity-scrim.schema.json` and `m5-layer-order-and-portal.schema.json` domain schemas it points at)
instead of re-describing scrim or layering behavior manually. The single mint-from-truth path is the
headless emitter `cargo run -p aureline-ui --example dump_m5_opacity_scrim_and_overlay_depth_registries`; the
checked proof set and fixtures are byte-locked to the seed builder, so an orientation erasure or a private
z-order bypass is caught before release evidence turns green.

[matrix]: ../../crates/aureline-ui/src/m5_motion_layer_iconography_matrix/mod.rs
