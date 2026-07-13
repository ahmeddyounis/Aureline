# M5 Layer-Order and Portal Registries

This document is the human-readable companion to the **layer / portal implement lane over the frozen
[M5 motion / layer / iconography matrix][matrix]**. It turns the two interaction families that carry the
*overlay stack* grammar — the **layer-order** z-tier registry (one canonical base / sticky / floating / menu /
dialog / toast / critical ordering) and the **portal-ownership** registry (portals that attach to their owning
surface and restore safely) — into registry resolvers that produce export-safe, honest projections. The
authoritative gate is the Rust validator in
[`crates/aureline-ui/src/m5_layer_order_and_portal_registries`](../../crates/aureline-ui/src/m5_layer_order_and_portal_registries/mod.rs);
this doc explains what the registries lock and how the first consumers adopt them.

- Packet id: `m5-layer-order-and-portal-registries:stable:0001`
- Registries schema:
  [`schemas/design-system/m5-layer-order-and-portal-registries.schema.json`](../../schemas/design-system/m5-layer-order-and-portal-registries.schema.json)
- Canonical domain schema:
  [`schemas/design-system/m5-layer-order-and-portal.schema.json`](../../schemas/design-system/m5-layer-order-and-portal.schema.json)
  (both the layer-order and portal-ownership families map to this single domain schema)
- Frozen matrix contract: [`m5_motion_layer_iconography_contract.md`](m5_motion_layer_iconography_contract.md)
- Canonical proof set:
  [`artifacts/release/m5-layer-order-and-portal-registries-proof/support_export.json`](../../artifacts/release/m5-layer-order-and-portal-registries-proof/support_export.json)
  (with `matrix.csv` and `summary.md`)
- Narrowed fixtures:
  [`fixtures/ui/m5-layer-order-and-portal-registries/`](../../fixtures/ui/m5-layer-order-and-portal-registries/)

## Why this exists

The frozen motion / layer / iconography matrix names the seven visual-interaction families and locks their
controlled vocabulary, but it stays a *matrix* — it does not resolve the concrete overlay stack a surface can
consume. This lane implements the two families that carry the overlay-stack grammar as registries, so every
floating surface, menu, dialog, toast, and critical prompt keeps one canonical z-tier ordering, so no
first-party or extension surface hard-codes always-on-top behavior to bypass the shared model, and so a portal
stays attached to its owning window and tears down or restores with its owner rather than stranding an
orphaned overlay.

## What the resolvers lock

- **`resolve_layer_tier_entry`** refuses to read as a clean, shared-z-order-safe layer-tier entry unless it
  names a canonical token, a classified z-tier (base / sticky / floating / menu / dialog / toast / critical), a
  layer-order role, and a surface context, stacks under the single shared z-order model, never hard-codes
  always-on-top behavior, and traces to a canonical token rather than an inlined raw z-index. A private layer
  or a hard-coded always-on-top overlay degrades to `always_on_top_bypasses_shared_model`.
- **`resolve_portal_entry`** refuses to read as a clean, owning-surface-attached portal entry unless it names
  a canonical token, a portal-ownership role, a classified z-tier, and an attachment mode, attaches to its
  owning surface, and restores safely when its owning surface changes. A detached, orphaned, or restore-unsafe
  portal degrades honestly to `portal_detached_from_owning_surface` or `restore_unsafe_on_owner_change`.
- **Attachment mode required.** Every portal entry names an `M5PortalAttachmentMode` (anchored to the owning
  window, tracked to an anchor element, contained within a focus scope, torn down with its owner, or
  re-parented restore-safe). A portal that carries no attachment mode degrades to `attachment_mode_missing`.

## Acceptance criteria proven by resolved examples

1. **The first claimed consumers obey one canonical layer-order model with correct attachment and restore
   behavior.** The shell, dialog, panel, embedded, and notification surfaces resolve their layer-tier and
   portal entries through this lane; clean entries cover the layer / portal semantic families and those five
   first-consumer surfaces, a raw-z-index example degrades, and no clean entry inlines a raw z-index value.
2. **Menus, toasts, dialogs, and critical prompts no longer compete through ad hoc z-order rules.** Every
   competing tier (menu, dialog, toast, critical) is covered by a clean layer-tier entry that stacks under the
   single shared z-order model; a hard-coded always-on-top example and a not-stacked example both degrade, and
   no clean entry hard-codes always-on-top.
3. **Layer-order drift is visible to fixtures, diagnostics, or release proof before stable promotion.** Clean
   portal entries cover the first surfaces with owning-surface attachment and restore-safety; a detached
   example, a restore-unsafe example, and an unclassified-tier drift example all degrade, and the checked
   support export / narrowed fixtures fail validation on drift.

## How consumers adopt it

Later components, docs / help, exports, and support packets consume this registry (or the canonical
`m5-layer-order-and-portal.schema.json` domain schema it points at) instead of re-describing z-order or portal
behavior manually. The single mint-from-truth path is the headless emitter
`cargo run -p aureline-ui --example dump_m5_layer_order_and_portal_registries`; the checked proof set and
fixtures are byte-locked to the seed builder, so a hard-coded always-on-top bypass or a detached portal is
caught before release evidence turns green.

[matrix]: ../../crates/aureline-ui/src/m5_motion_layer_iconography_matrix/mod.rs
