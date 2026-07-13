# M5 Shell-Metric / Density Matrix Contract

This document is the human-readable companion to the frozen **M5 shell-metric,
minimum-size, density-mode, responsive-geometry, and collapse-priority
shell-geometry matrix**. The authoritative gate is the Rust validator in
[`crates/aureline-ui/src/m5_shell_metric_density_matrix`](../../crates/aureline-ui/src/m5_shell_metric_density_matrix/mod.rs);
this doc explains what the matrix locks and how downstream surfaces consume it.

- Packet id: `m5-shell-metric-density:stable:0001`
- Matrix schema: [`schemas/shell/m5-shell-metric-density-matrix.schema.json`](../../schemas/shell/m5-shell-metric-density-matrix.schema.json)
- Domain schemas:
  [`m5-shell-metrics`](../../schemas/shell/m5-shell-metrics.schema.json),
  [`m5-density-mode`](../../schemas/shell/m5-density-mode.schema.json)
- Canonical proof set:
  [`artifacts/release/m5-shell-metric-density-proof/support_export.json`](../../artifacts/release/m5-shell-metric-density-proof/support_export.json)
  (with `matrix.csv`) and the design report
  [`artifacts/shell/m5-shell-metric-density-matrix.md`](../../artifacts/shell/m5-shell-metric-density-matrix.md)
- Narrowed fixtures: [`fixtures/ui/m5-shell-metric-density/`](../../fixtures/ui/m5-shell-metric-density/)

## Why this exists

The current sheet already hardens shell zones and adaptive-layout ownership,
native desktop continuity, appearance objects and theme packages, reusable shell
primitives, and the base visual foundation, but Aureline's concrete shell
geometry and density behavior stayed too implicit. This matrix locks one reviewed
baseline so later M5 surface work cannot keep introducing private shell-zone
widths, hit-target rules, or density behavior. It does not redesign navigation
content, start-center flows, or native protocol-handler ownership — it **binds
back** to the already-landed shell-zone contract
([`m5-shell-zone.schema.json`](../../schemas/shell/m5-shell-zone.schema.json))
and reusable-shell-primitive contract
([`m5-shell-primitives.schema.json`](../../schemas/shell/m5-shell-primitives.schema.json))
instead of leaving the geometry split across scattered local constants and
screenshots.

## The one shared vocabulary

Every governed family binds to the single controlled **shell-geometry-role**
vocabulary — `zone`, `metric`, `hit_target`, `density`, `responsive`, `collapse`,
`workspace_dominance`. The collapse-sensitive roles (`density`, `responsive`,
`collapse`, `workspace_dominance`) must preserve task identity, focus order, trust
visibility, and recovery-critical state whenever density changes or the layout
collapses; no surface invents a parallel word for any of these roles.

## Canonical shell metric table

These are the frozen default / minimum / recommended shell-zone metrics every
claimed M5 desktop surface inherits. They are bound to the machine-readable
shell-metric registry rather than copied by hand across packages.

| Zone / control | Minimum | Recommended default | Maximum |
| --- | --- | --- | --- |
| Title / context bar (height) | 36 px | 40 px | 48 px |
| Activity rail (width) | 48 px | 48 px | 56 px |
| Sidebar (width) | 220 px | 288 px | 480 px |
| Main editor group (width) | 480 px | — (dominant, fills remainder) | — |
| Right inspector (width) | 240 px | 320 px | 520 px |
| Bottom panel (height) | 120 px | 240 px | 60% of window |
| Status bar (height) | 22 px | 24 px | 28 px |
| Tab (minimum width) | 96 px | 160 px | 240 px |
| Resize-handle hit area | 6 px (visual) / 12 px (hit) | — | — |
| Icon-only control hit target | 32 × 32 px | 40 × 40 px | — |

The main editor group is the dominant zone: chrome zones honor their declared
minimums first, and the main workspace fills the remainder. No zone may starve
the main workspace below its minimum, and extension or embedded surfaces may not
invent private widths that fracture this layout.

## Governed families and first consumers

The matrix freezes five shell-geometry families. Each names its canonical domain
schema and its first consumers (the surfaces that must read the matrix rather than
re-describe the meaning):

| Family | Domain schema | First consumers |
| --- | --- | --- |
| `shell_metric` | m5-shell-metrics | shell, editor, review |
| `minimum_size` | m5-shell-metrics | shell, editor, data, settings |
| `density_mode` | m5-density-mode | shell, editor, data, settings |
| `responsive_geometry` | m5-density-mode | shell, editor, review, notebook |
| `collapse_priority` | m5-density-mode | shell, editor, notebook, data |

Every family also projects to the support export, so release / help / support
packets can point to one canonical proof set for shell-geometry truth.

## Hard invariants

Each row carries five boolean invariants that must stay `false`:

1. `density_or_collapse_changes_command_focus_or_trust`
2. `extension_or_embedded_sets_private_fracturing_width`
3. `shrinks_hit_target_below_supported_minimum`
4. `hides_primary_workflow_behind_overlay_only_fallback`
5. `lets_zone_starve_main_workspace_below_minimum`

## Downgrade conditions

A family narrows below its claimed qualification when any downgrade trigger fires —
`density_changed_command_or_focus_or_trust`,
`responsive_collapse_dropped_recovery_state`, `zone_starved_main_workspace`,
`extension_set_private_fracturing_width`, `hit_target_shrank_below_minimum`,
`primary_workflow_hidden_behind_overlay_only_fallback`,
`metric_copied_by_hand_across_packages`, `size_metric_unstated`,
`density_mode_unstated`, `responsive_class_unstated`,
`registry_reference_unstated`, or `proof_stale`. Stale proof auto-narrows the
family (`auto_narrow_on_stale`). No claimed M5 surface can bypass the shared
shell-geometry matrix without an explicit waiver or a narrower lifecycle label —
the narrowed fixtures (`responsive_geometry_beta_narrowed`,
`collapse_priority_preview_narrowed`) show a family held at Beta / Preview while
every other family stays visible.

## Regenerating the proof set

The seed builders in `seed.rs` are the single producer of the checked-in artifacts
and fixtures. Regenerate them with the headless emitter:

```text
cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- support-export
cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- csv
cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- report
cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- fixture-responsive-geometry-beta-narrowed
cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- fixture-collapse-priority-preview-narrowed
cargo run -p aureline-ui --example dump_m5_shell_metric_density_matrix -- validate
```
