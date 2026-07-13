# M5 Responsive-Geometry and Collapse-Priority Registries

This document is the human-readable companion to the **responsive-geometry /
collapse-priority implement lane** over the frozen M5 shell-metric / density
matrix. The authoritative gate is the Rust validator in
[`crates/aureline-ui/src/m5_responsive_geometry_and_collapse_priority_registries`](../../crates/aureline-ui/src/m5_responsive_geometry_and_collapse_priority_registries/mod.rs);
this doc explains what the two resolvers produce and how downstream surfaces
consume them.

- Packet id: `m5-responsive-geometry-and-collapse-priority-registries:stable:0001`
- Registries schema: [`schemas/shell/m5-responsive-geometry-and-collapse-priority-registries.schema.json`](../../schemas/shell/m5-responsive-geometry-and-collapse-priority-registries.schema.json)
- Frozen matrix: [`schemas/shell/m5-shell-metric-density-matrix.schema.json`](../../schemas/shell/m5-shell-metric-density-matrix.schema.json)
  and its [contract doc](./m5_shell_metric_density_contract.md)
- Canonical domain schema the resolved entries bind back to:
  [`schemas/shell/m5-density-mode.schema.json`](../../schemas/shell/m5-density-mode.schema.json)
- Canonical proof set:
  [`artifacts/release/m5-responsive-geometry-and-collapse-priority-registries-proof/support_export.json`](../../artifacts/release/m5-responsive-geometry-and-collapse-priority-registries-proof/support_export.json)
  (with `matrix.csv` and `summary.md`)
- Narrowed fixtures: [`fixtures/ui/m5-responsive-geometry-and-collapse-priority-registries/`](../../fixtures/ui/m5-responsive-geometry-and-collapse-priority-registries/)

## Why this exists

The frozen matrix names five shell-geometry families and locks their vocabulary,
but it stops at "the shell has named responsive and collapse roles". This lane
closes the gap to "the shell resolves every window class and collapse step from
one executable registry, with tokenized width bounds and identity-stable
transitions". It implements the `responsive_geometry` and `collapse_priority`
families as resolvers that turn Compact, Standard, and Expanded desktop into
logical-pixel width contracts and turn the declared collapse priority order into
docked / sheet / overlay / temporary-panel transitions that keep the same task
surface, state, history, and keyboard route. The resolvers refuse to read as a
clean pass when a surface invents a private breakpoint, drops recovery-critical
state, makes an essential action hover-only, narrows a compare / editor group
into an unusable pane, collapses a protected target, starves the main workspace,
or hides a primary workflow behind an overlay-only fallback.

## The two resolvers

- `resolve_window_class_entry` resolves a responsive desktop window class. It
  reads as clean only when it names a canonical registry token, a classified
  [`M5WindowClass`], a responsive-geometry role, declares the exact canonical
  width bounds for that class, covers every coexisting
  [`M5ResponsiveShellZone`] (title / context bar, rail, sidebar, main workspace,
  right inspector, bottom panel, status bar), preserves task identity and
  recovery-critical state, and never makes an essential action hover-only or
  narrows a compare / editor group into an unusable pane. Otherwise it degrades
  to one of ten reasons (token unstated, surface-context unresolved, class
  unclassified, drops recovery-critical state, drops task identity, essential
  action becomes hover-only, editor group narrows into an unusable pane, bounds
  outside the canonical class, shell-zone coexistence incomplete, proof stale).
- `resolve_collapse_step_entry` resolves an adaptive-collapse step and its
  identity-stable transition. It reads as clean only when it names a canonical
  registry token, a classified [`M5CollapseTarget`] and
  [`M5IdentityTransitionForm`], declares the canonical collapse-order rank, keeps
  the main workspace dominant, never collapses a protected target, never hides a
  primary workflow behind an overlay-only fallback, and preserves the surface's
  identity, state, history, and keyboard route. Otherwise it degrades (token
  unstated, surface-context unresolved, target unclassified, form unclassified,
  fractures layout with a private width, collapses a protected target, drops
  identity / state / route, starves the main workspace, overlay-only primary
  fallback, collapse order outside the canonical priority, proof stale).

## Canonical window classes

These are the reference tokens the resolvers encode as logical-pixel contracts
before OS scaling. A window class declares exactly its canonical bounds or it
drifts into a private breakpoint that degrades.

| Window class | Lower bound | Upper bound |
| --- | --- | --- |
| Compact desktop | 1024 px | 1279 px |
| Standard desktop | 1280 px | 1599 px |
| Expanded desktop | 1600 px | unbounded |

## Declared collapse priority order

The declared responsive priority order (rank 0 collapses first, rank 3 last):

| Rank | Collapse target | Moves to |
| --- | --- | --- |
| 0 | Optional right-inspector detail | Sheet or inline disclosure |
| 1 | Secondary bottom-panel tabs | Overflow |
| 2 | Low-frequency side tools | Overflow |
| 3 | Primary navigation | Temporary panel (never overlay-only) |

Path / branch / trust / target identity and the dominant editor workspace are
**protected** — they carry no collapse rank and must never collapse. A protected
target that collapses, a collapse that starves the main workspace, or a primary
workflow moved into an overlay-only fallback all degrade honestly.

## Hard invariants

Each registry row carries four boolean invariants that must stay `false`:

1. `responsive_or_collapse_alters_command_focus_or_trust`
2. `extension_sets_private_fracturing_width`
3. `lets_zone_starve_main_workspace_below_minimum`
4. `hides_primary_workflow_behind_overlay_only_fallback`

## Acceptance criteria proven by the resolved examples

The validator proves — from the resolved examples, not from governance bools —
that:

1. **Compact, Standard, and Expanded window classes produce predictable,
   tokenized layout across every surface.** Clean window-class entries cover the
   three canonical classes and the first shell / editor / review / notebook /
   data surfaces, a private-breakpoint example degrades, and no clean entry
   drifts from the canonical bounds.
2. **Docked / sheet / overlay / temporary-panel transitions stay identity-stable,
   no essential action becomes hover-only, and no compare / editor group narrows
   into an unusable pane.** Clean collapse-step entries cover the canonical
   collapse order, an identity-dropping collapse example degrades, a hover-only
   and an unusable-pane window example degrade, and no clean collapse entry drops
   identity / state / route.
3. **Extension surfaces that cannot honor the canonical geometry degrade honestly
   instead of inventing private widths.** A private-breakpoint window example and
   a starves-workspace collapse example both degrade, at least one clean
   main-workspace-dominant collapse step exists, and no clean window entry carries
   a private bound.

## Regenerating the proof set

The seed builders in `seed.rs` are the single producer of the checked-in
artifacts and fixtures. Regenerate them with the headless emitter:

```text
cargo run -p aureline-ui --example dump_m5_responsive_geometry_and_collapse_priority_registries -- support-export
cargo run -p aureline-ui --example dump_m5_responsive_geometry_and_collapse_priority_registries -- csv
cargo run -p aureline-ui --example dump_m5_responsive_geometry_and_collapse_priority_registries -- report
cargo run -p aureline-ui --example dump_m5_responsive_geometry_and_collapse_priority_registries -- fixture-editor-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_responsive_geometry_and_collapse_priority_registries -- fixture-settings-ui-preview-narrowed
cargo run -p aureline-ui --example dump_m5_responsive_geometry_and_collapse_priority_registries -- validate
```
