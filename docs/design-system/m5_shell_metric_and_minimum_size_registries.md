# M5 Shell-Metric and Minimum-Size Registries

This document is the human-readable companion to the **first implement lane** over
the frozen M5 shell-metric / density matrix. The authoritative gate is the Rust
validator in
[`crates/aureline-ui/src/m5_shell_metric_and_minimum_size_registries`](../../crates/aureline-ui/src/m5_shell_metric_and_minimum_size_registries/mod.rs);
this doc explains what the two resolvers produce and how downstream surfaces
consume them.

- Packet id: `m5-shell-metric-and-minimum-size-registries:stable:0001`
- Registries schema: [`schemas/shell/m5-shell-metric-and-minimum-size-registries.schema.json`](../../schemas/shell/m5-shell-metric-and-minimum-size-registries.schema.json)
- Frozen matrix: [`schemas/shell/m5-shell-metric-density-matrix.schema.json`](../../schemas/shell/m5-shell-metric-density-matrix.schema.json)
  and its [contract doc](./m5_shell_metric_density_contract.md)
- Canonical domain schema the resolved entries bind back to:
  [`schemas/shell/m5-shell-metrics.schema.json`](../../schemas/shell/m5-shell-metrics.schema.json)
- Canonical proof set:
  [`artifacts/release/m5-shell-metric-and-minimum-size-registries-proof/support_export.json`](../../artifacts/release/m5-shell-metric-and-minimum-size-registries-proof/support_export.json)
  (with `matrix.csv` and `summary.md`)
- Narrowed fixtures: [`fixtures/ui/m5-shell-metric-and-minimum-size-registries/`](../../fixtures/ui/m5-shell-metric-and-minimum-size-registries/)

## Why this exists

The frozen matrix names five shell-geometry families and locks their vocabulary,
but it stops at "the shell has named zones and metric roles". This lane closes the
gap to "the shell resolves every zone size and hit target from one executable
registry". It implements the two families that carry the concrete **size** grammar
— `shell_metric` and `minimum_size` — as resolvers that turn the reference metrics
into logical-pixel contracts and refuse to read as a clean pass when a surface
hand-copies a constant, drifts outside the canonical envelope, starves the main
workspace, or shrinks a hit target below its supported minimum.

## The two resolvers

- `resolve_shell_metric_entry` resolves a shell-zone metric. It reads as clean only
  when it names a canonical registry token, a classified [`M5ShellZone`], a
  shell-metric role, covers comfortable / standard / compact density, declares
  logical-pixel bounds inside the zone's canonical envelope, never starves the main
  workspace, and preserves task identity under snapped widths. Otherwise it degrades
  to one of nine reasons (token unstated, surface-context unresolved, zone
  unclassified, not bound to the registry, outside the canonical envelope, starves
  the main workspace, density coverage incomplete, snapped-width unsafe, proof
  stale).
- `resolve_minimum_size_entry` resolves a minimum hit target. It reads as clean only
  when it names a canonical registry token, a classified [`M5ShellControlClass`], a
  minimum-size role, covers every density mode, declares a minimum hit dimension at
  or above the control's canonical minimum, and stays reachable by pointer and
  keyboard. Otherwise it degrades (token unstated, surface-context unresolved,
  control unclassified, hit target below minimum, density coverage incomplete, proof
  stale).

## Canonical logical-pixel envelope

These are the reference metrics the resolvers encode as logical-pixel contracts
before OS scaling. A metric may declare exactly the canonical values or a tighter
window, but never a minimum below the canonical floor.

| Zone / control | Minimum | Default | Recommended | Maximum |
| --- | --- | --- | --- | --- |
| Title / context bar (height) | 32 px | 36 px | 40 px | 40 px |
| Activity rail (width) | 44 px | 48 px | 48 px | 56 px |
| Sidebar (width) | 220 px | 260 px | 320 px | 420 px |
| Main editor group (width) | 420 px | 720 px | 720 px | — (dominant, fills remainder) |
| Right inspector (width) | 280 px | 320 px | 360 px | 420 px |
| Bottom panel (height) | 180 px | 240 px | 320 px | 45% of window height |
| Status bar (height) | 24 px | 24 px | 26 px | 28 px |
| Tab (minimum width) | 96 px | — | 160 px | — |
| Resize-handle hit area | 4 px | — | 8 px | — |
| Icon-only control hit target | 28 px | 32 px | 36 px | — |

The main editor group is the dominant zone: chrome zones honor their declared
minimums first, and the main workspace fills the remainder. No zone may starve the
main workspace below its minimum, and extension or embedded surfaces may not invent
private widths that fracture this layout.

## Hard invariants

Each registry row carries four boolean invariants that must stay `false`:

1. `lets_zone_starve_main_workspace_below_minimum`
2. `shrinks_hit_target_below_supported_minimum`
3. `extension_or_embedded_sets_private_fracturing_width`
4. `metric_hand_copied_instead_of_registry`

## Acceptance criteria proven by the resolved examples

The validator proves — from the resolved examples, not from governance bools —
that:

1. **All claimed M5 shell surfaces resolve their geometry from the shared metric
   registry.** Clean shell-metric entries cover the `zone` and `metric` semantic-role
   families and the first shell / editor / review / notebook / data surfaces, a
   hand-copied example degrades, and no clean entry is unbound.
2. **Minimum editor and control hit-target guarantees hold under supported density
   modes and snapped-window widths.** Clean minimum-size entries cover the tab /
   resize-handle / icon-only control classes with full density coverage while meeting
   the supported minimum, a below-minimum example degrades, a density-incomplete
   example degrades, and no clean entry shrinks below its minimum.
3. **Regression suites fail when a surface drifts outside the canonical B138 metric
   envelope.** A metric-outside-envelope example and a below-minimum example both
   degrade, at least one clean shell-metric and one clean minimum-size entry trace to
   the registry, and no clean entry drifts.

## Regenerating the proof set

The seed builders in `seed.rs` are the single producer of the checked-in artifacts
and fixtures. Regenerate them with the headless emitter:

```text
cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- support-export
cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- csv
cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- report
cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- fixture-editor-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- fixture-data-ui-preview-narrowed
cargo run -p aureline-ui --example dump_m5_shell_metric_and_minimum_size_registries -- validate
```
