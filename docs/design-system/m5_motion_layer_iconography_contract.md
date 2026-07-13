# M5 Motion / Layer / Iconography Matrix Contract

This document is the human-readable companion to the frozen **M5 motion-token,
reduced-motion, opacity / scrim, layer-order, portal-ownership, iconography, and
illustration-boundary visual-interaction matrix**. The authoritative gate is the
Rust validator in
[`crates/aureline-ui/src/m5_motion_layer_iconography_matrix`](../../crates/aureline-ui/src/m5_motion_layer_iconography_matrix/mod.rs);
this doc explains what the matrix locks and how downstream surfaces consume it.

- Packet id: `m5-motion-layer-iconography:stable:0001`
- Matrix schema: [`schemas/design-system/m5-motion-layer-iconography-matrix.schema.json`](../../schemas/design-system/m5-motion-layer-iconography-matrix.schema.json)
- Domain schemas:
  [`m5-motion-and-reduced-motion`](../../schemas/design-system/m5-motion-and-reduced-motion.schema.json),
  [`m5-opacity-scrim`](../../schemas/design-system/m5-opacity-scrim.schema.json),
  [`m5-layer-order-and-portal`](../../schemas/design-system/m5-layer-order-and-portal.schema.json),
  [`m5-iconography-and-illustration`](../../schemas/design-system/m5-iconography-and-illustration.schema.json)
- Canonical proof set:
  [`artifacts/release/m5-motion-layer-iconography-proof/support_export.json`](../../artifacts/release/m5-motion-layer-iconography-proof/support_export.json)
  (with `matrix.csv`) and the design report
  [`artifacts/design-system/m5-motion-layer-iconography.md`](../../artifacts/design-system/m5-motion-layer-iconography.md)
- Narrowed fixtures: [`fixtures/ui/m5-motion-layer-iconography/`](../../fixtures/ui/m5-motion-layer-iconography/)

## Why this exists

The current sheet already hardens appearance objects, shell primitives, durable
progress / activity truth, embedded / browser-handoff boundaries, and reusable
component families, but Aureline's motion, layering, scrim, icon, and illustration
rules stayed too implicit. This matrix locks one reviewed baseline so later M5
surface work cannot keep introducing private animation, z-order, or icon semantics.
It does not re-open notification routing, dialog semantics, or browser-boundary
authority — it **binds back** to the already-landed design-system foundations
([`m5-foundations.schema.json`](../../schemas/design-system/m5-foundations.schema.json))
and publication packet
([`m5-foundation-package.schema.json`](../../schemas/design-system/m5-foundation-package.schema.json))
instead of leaving the grammar split across prose and screenshots.

## The one shared vocabulary

Every governed family binds to the single controlled **interaction-role** vocabulary —
`motion`, `overlay`, `layer`, `portal`, `icon`, `illustration`, `attention`. The
meaning-bearing roles (`motion`, `overlay`, `icon`, `illustration`, `attention`) must
always pair their visual cue with a reduced-motion-safe, labeled, or announced
fallback; no feature family invents a parallel word for any of these roles, and no
meaning may be carried by motion, decoration, or an unlabeled symbol alone.

## Governed families and first consumers

The matrix freezes seven interaction families. Each names its canonical domain schema
and its first consumers (the surfaces that must read the matrix rather than
re-describe the meaning):

| Family | Domain schema | First consumers |
| --- | --- | --- |
| `motion_token` | m5-motion-and-reduced-motion | shell, editor, onboarding |
| `reduced_motion` | m5-motion-and-reduced-motion | shell, editor, help, settings |
| `opacity_scrim` | m5-opacity-scrim | shell, editor, onboarding |
| `layer_order` | m5-layer-order-and-portal | shell, editor, marketplace |
| `portal_ownership` | m5-layer-order-and-portal | shell, editor, marketplace |
| `iconography` | m5-iconography-and-illustration | shell, editor, help, marketplace |
| `illustration` | m5-iconography-and-illustration | onboarding, help, marketplace |

Every family also projects to the support export, so release / help / support packets
can point to one canonical proof set for visual-interaction truth.

## Hard invariants

Each row carries five boolean invariants that must stay `false`:

1. `delays_protected_input_with_motion`
2. `scrim_erases_orientation_or_contrast`
3. `overlay_bypasses_shared_z_order`
4. `uses_unlabeled_icon_for_uncommon_or_destructive_action`
5. `lets_illustration_impersonate_operational_or_security_truth`

## Downgrade conditions

A family narrows below its claimed qualification when any downgrade trigger fires —
`motion_delayed_protected_input`, `motion_meaning_lost_under_reduced_motion`,
`scrim_erased_orientation_or_contrast`, `overlay_bypassed_shared_z_order`,
`portal_detached_from_owning_surface`,
`unlabeled_icon_for_uncommon_or_destructive_action`,
`illustration_impersonated_operational_state`, `icon_semantics_ambiguous`,
`layer_tier_unstated`, `semantic_role_unstated`, `token_reference_unstated`, or
`proof_stale`. Stale proof auto-narrows the family (`auto_narrow_on_stale`). No
claimed M5 surface can bypass the shared motion / layer / icon matrix without an
explicit waiver or a narrower lifecycle label — the narrowed fixtures
(`reduced_motion_beta_narrowed`, `illustration_preview_narrowed`) show a family held
at Beta / Preview while every other family stays visible.

## Regenerating the proof set

The seed builders in `seed.rs` are the single producer of the checked-in artifacts and
fixtures. Regenerate them with the headless emitter:

```text
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_matrix -- support-export
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_matrix -- csv
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_matrix -- report
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_matrix -- fixture-reduced-motion-beta-narrowed
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_matrix -- fixture-illustration-preview-narrowed
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_matrix -- validate
```
