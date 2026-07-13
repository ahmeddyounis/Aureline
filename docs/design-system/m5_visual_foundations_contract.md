# M5 Visual-Foundation Matrix Contract

This document is the human-readable companion to the frozen **M5 color-system,
semantic-theme-token, syntax / diff / chart-token, typography, and
spacing / sizing / radii / elevation visual-foundation matrix**. The authoritative
gate is the Rust validator in
[`crates/aureline-ui/src/m5_visual_foundation_matrix`](../../crates/aureline-ui/src/m5_visual_foundation_matrix/mod.rs);
this doc explains what the matrix locks and how downstream surfaces consume it.

- Packet id: `m5-visual-foundations:stable:0001`
- Matrix schema: [`schemas/design-system/m5-visual-foundation-matrix.schema.json`](../../schemas/design-system/m5-visual-foundation-matrix.schema.json)
- Domain schemas:
  [`m5-color-system`](../../schemas/design-system/m5-color-system.schema.json),
  [`m5-syntax-diff-chart-tokens`](../../schemas/design-system/m5-syntax-diff-chart-tokens.schema.json),
  [`m5-typography-and-geometry`](../../schemas/design-system/m5-typography-and-geometry.schema.json)
- Canonical proof set:
  [`artifacts/release/m5-visual-foundations-proof/support_export.json`](../../artifacts/release/m5-visual-foundations-proof/support_export.json)
  (with `matrix.csv`) and the design report
  [`artifacts/design-system/m5-visual-foundations.md`](../../artifacts/design-system/m5-visual-foundations.md)
- Narrowed fixtures: [`fixtures/ui/m5-visual-foundations/`](../../fixtures/ui/m5-visual-foundations/)

## Why this exists

The current sheet already hardens appearance-session objects, token overlays,
design-system publication, component-state taxonomy, shell primitives, and reusable
control / feedback families, but the concrete visual foundation stayed too implicit.
This matrix locks one reviewed baseline so later M5 surface work cannot keep inventing
local token meaning, typography scales, or geometry rules. It does not re-open theme
package / import / appearance-session object design — it **binds back** to the
already-landed design-system foundations
([`m5-foundations.schema.json`](../../schemas/design-system/m5-foundations.schema.json))
and publication packet
([`m5-foundation-package.schema.json`](../../schemas/design-system/m5-foundation-package.schema.json))
instead of leaving the baseline split across prose and screenshots.

## The one shared vocabulary

Every governed family binds to the single controlled **semantic-role** vocabulary —
`brand`, `interactive`, `neutral`, `status`, `syntax`, `diff`, `chart`. Status, syntax,
diff, and chart roles must always pair color with a non-color cue; no feature family
invents a parallel word for any of these roles, and none may be conveyed by hue alone.

## Governed families and first consumers

The matrix freezes eight foundation families. Each names its canonical domain schema and
its first consumers (the surfaces that must read the matrix rather than re-describe the
meaning):

| Family | Domain schema | First consumers |
| --- | --- | --- |
| `color_system` | m5-color-system | shell, editor, review, data |
| `semantic_theme_token` | m5-color-system | shell, editor, review, settings |
| `syntax_token` | m5-syntax-diff-chart-tokens | editor, review, docs |
| `diff_token` | m5-syntax-diff-chart-tokens | review, editor, data |
| `chart_token` | m5-syntax-diff-chart-tokens | data, review, docs |
| `typography` | m5-typography-and-geometry | shell, editor, docs, data |
| `spacing_sizing_radii_elevation` | m5-typography-and-geometry | shell, editor, review, data |
| `hit_target` | m5-typography-and-geometry | shell, editor, review, settings |

Every family also projects to the support export, so release / help / support packets can
point to one canonical proof set for visual-foundation truth.

## Hard invariants

Each row carries five boolean invariants that must stay `false`:

1. `collapses_status_or_trust_into_color_only`
2. `lets_syntax_or_diff_palette_collide_with_diagnostics`
3. `shrinks_hit_target_below_supported_minimum`
4. `lets_chart_meaning_depend_on_color_alone`
5. `forks_local_spacing_or_elevation_from_shared_geometry`

## Downgrade conditions

A family narrows below its claimed qualification when any downgrade trigger fires —
`status_or_trust_collapsed_to_color_only`,
`syntax_or_diff_palette_collided_with_diagnostics`,
`hit_target_shrunk_below_minimum`, `chart_meaning_depended_on_color_alone`,
`local_geometry_forked_from_foundation`, `typography_scale_drifted`,
`font_stack_unstable`, `theme_pair_incomplete`, `tabular_numerals_missing`,
`semantic_role_unstated`, `token_reference_unstated`, or `proof_stale`. Stale proof
auto-narrows the family (`auto_narrow_on_stale`). No claimed M5 surface can bypass the
shared visual-foundation matrix without an explicit waiver or a narrower lifecycle label
— the narrowed fixtures (`typography_beta_narrowed`, `chart_token_preview_narrowed`) show
a family held at Beta / Preview while every other family stays visible.

## Regenerating the proof set

The seed builders in `seed.rs` are the single producer of the checked-in artifacts and
fixtures. Regenerate them with the headless emitter:

```text
cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- support-export
cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- csv
cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- report
cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- fixture-typography-beta-narrowed
cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- fixture-chart-token-preview-narrowed
cargo run -p aureline-ui --example dump_m5_visual_foundation_matrix -- validate
```
