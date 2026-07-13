# M5 Motion / Layer / Iconography Shared Consumers — One Grammar Across Surfaces

**Status:** stable · **Batch:** B137 · **Row:** M05-1153

This lane is the consumer-adoption capstone for the seven B137 visual-interaction families frozen
in the [motion / layer / iconography matrix](./m5_motion_layer_iconography_contract.md) and
implemented by the four registry lanes:

| Family | Domain schema |
| --- | --- |
| `motion_token`, `reduced_motion` | `schemas/design-system/m5-motion-and-reduced-motion.schema.json` |
| `opacity_scrim` | `schemas/design-system/m5-opacity-scrim.schema.json` |
| `layer_order`, `portal_ownership` | `schemas/design-system/m5-layer-order-and-portal.schema.json` |
| `iconography`, `illustration` | `schemas/design-system/m5-iconography-and-illustration.schema.json` |

It proves — by fixtures, not screenshots — that motion, layering, scrim, icon, and illustration
rules are actually **reused** by the shell, editor, help, marketplace / extension, onboarding,
settings, and CLI/export/support surfaces users hit most often, instead of remaining an isolated
design packet.

## What the packet asserts

The packet
(`artifacts/release/m5-motion-layer-iconography-shared-consumers-proof/support_export.json`,
schema `schemas/design-system/m5-motion-layer-iconography-shared-consumers.schema.json`) records one
`consumer_binding` per (interaction object × consumer surface × representation). The three honesty
axes mirror the batch acceptance criteria.

1. **Reuse.** Every one of the seven interaction families is adopted by at least two distinct
   consumers, so a family is proven shared infrastructure rather than a one-surface fork.
2. **One grammar / no drift.** For a given interaction object every consumer surface presents an
   identical `state_facets` block — the same `interaction_role_word` (a frozen
   `motion` / `overlay` / `layer` / `portal` / `icon` / `illustration` / `attention` token), the
   same `family_word`, `token_reference_word`, `state_variant_word`, `surface_context_word`, and
   `accessible_fallback_word`. A surface may narrow *how much* it shows across the desktop, compact,
   remote, and exported representations, but it may never reword the grammar per surface, and a role
   that carries motion, overlay, icon, illustration, or attention meaning may never fall back to
   motion, decoration, or an unlabeled symbol alone.
3. **Map back to one family.** Support and CLI/export bindings point at both the canonical per-domain
   schema and the frozen matrix schema by id, so an exported packet always maps a surface back to one
   shared contract family.

## Hard invariants (guardrail bools, all `false`)

Each binding carries the five B137 track invariants, which validation requires to be false:

- `delays_protected_input_with_motion`
- `lets_scrim_erase_orientation_or_contrast`
- `lets_overlay_bypass_shared_z_order`
- `uses_unlabeled_icon_for_uncommon_or_destructive_action`
- `lets_illustration_impersonate_operational_or_security_truth`

## Narrowing is disclosed, never hidden

A compact, remote, or exported representation carries an explicit `narrow_note` naming the reason,
the preserved grammar, and the next action; remote projections additionally name a `remote_source_note`
and exports a `export_detail_note`. Narrowing is a disclosed change in *depth*, never a change in the
underlying grammar.

## Consumer inventory

| Family | Adopting consumers |
| --- | --- |
| `motion_token` | shell, onboarding, support-export |
| `reduced_motion` | shell, editor |
| `opacity_scrim` | settings, shell (compact), cli-export |
| `layer_order` | shell, marketplace |
| `portal_ownership` | shell, marketplace (remote) |
| `iconography` | editor, help, cli-export |
| `illustration` | onboarding, help, product, support-export |

Any partial or narrowed adoption is explicit in each binding's `representation` and `narrow_note`.

## Regenerating the artifacts

The seed builders in the module are the only mint-from-truth path. Re-emit with:

```text
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_shared_consumers -- support-export
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_shared_consumers -- csv
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_shared_consumers -- report
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_shared_consumers -- fixture-compact-remote-narrowed
cargo run -p aureline-ui --example dump_m5_motion_layer_iconography_shared_consumers -- fixture-exported-redaction-narrowed
```

The checked-in artifacts are byte-locked against these builders by the module's tests.
