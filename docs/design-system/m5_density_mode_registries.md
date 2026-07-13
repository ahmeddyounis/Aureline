# M5 Density-Mode Registries

This document is the human-readable companion to the **density-mode implement lane**
over the frozen M5 shell-metric / density matrix. The authoritative gate is the Rust
validator in
[`crates/aureline-ui/src/m5_density_mode_registries`](../../crates/aureline-ui/src/m5_density_mode_registries/mod.rs);
this doc explains what the two resolvers produce and how downstream surfaces
consume them.

- Packet id: `m5-density-mode-registries:stable:0001`
- Registries schema: [`schemas/shell/m5-density-mode-registries.schema.json`](../../schemas/shell/m5-density-mode-registries.schema.json)
- Frozen matrix: [`schemas/shell/m5-shell-metric-density-matrix.schema.json`](../../schemas/shell/m5-shell-metric-density-matrix.schema.json)
  and its [contract doc](./m5_shell_metric_density_contract.md)
- Canonical domain schema the resolved entries bind back to:
  [`schemas/shell/m5-density-mode.schema.json`](../../schemas/shell/m5-density-mode.schema.json)
- Canonical proof set:
  [`artifacts/release/m5-density-mode-registries-proof/support_export.json`](../../artifacts/release/m5-density-mode-registries-proof/support_export.json)
  (with `matrix.csv` and `summary.md`)
- Narrowed fixtures: [`fixtures/ui/m5-density-mode-registries/`](../../fixtures/ui/m5-density-mode-registries/)

## Why this exists

The frozen matrix names five shell-geometry families and locks their vocabulary,
but it stops at "the shell has named density-mode roles". This lane closes the gap
to "the shell resolves every density mode from one executable registry, with a
tokenized presentation scale, and persists the choice at profile scope". It
implements the `density_mode` family as resolvers that turn Compact, Standard, and
Comfortable into logical-pixel token contracts and refuse to read as a clean pass
when a surface invents a private scale, shrinks a hit target below its supported
minimum, rearranges information architecture, or silently switches density.

## The two resolvers

- `resolve_density_scale_entry` resolves a density mode's tokenized presentation
  scale. It reads as clean only when it names a canonical registry token, a
  classified [`M5DensityMode`], a density-mode role, declares the exact canonical
  row / control / spacing / padding / gutter tokens for that mode, covers every
  [`M5DensitySurfaceElement`] (lists, trees, tables, tabs, panels, editors,
  inspectors), keeps hit targets at or above their supported minimum, changes
  presentation only, and preserves command meaning, focus order, and trust
  visibility. Otherwise it degrades to one of nine reasons (token unstated,
  surface-context unresolved, mode unclassified, changes information architecture,
  changes command / focus / trust, hit target below minimum, scale outside the
  canonical tokens, surface-element coverage incomplete, proof stale).
- `resolve_density_persistence_entry` resolves a density preference's persistence.
  It reads as clean only when it names a canonical registry token, keeps a
  classified [`M5DensityPersistenceScope`], explains any local override (a
  presentation or accessibility viewer), and never switches silently because a
  provider, theme, or workflow changed. Otherwise it degrades (token unstated,
  surface-context unresolved, persistence scope unclassified, silent density switch,
  unexplained local override, proof stale).

## Canonical density scale

These are the reference tokens the resolvers encode as logical-pixel contracts
before OS scaling. A density mode declares exactly its canonical tokens or it drifts
into a private scale that degrades. Density affects only these presentation
dimensions — never command semantics, focus order, shell zoning, state vocabulary,
or hit-target minimums.

| Density mode | Row height | Control height | Tab / chip spacing | Panel padding | Gutter spacing |
| --- | --- | --- | --- | --- | --- |
| Compact | 24 px | 28 px | 4 px | 8 px | 8 px |
| Standard | 28 px | 32 px | 6 px | 12 px | 12 px |
| Comfortable | 32 px | 36 px | 8 px | 16 px | 16 px |

The compact control height (28 px) is the supported hit-target minimum; a control
height below it shrinks the hit target below its minimum and degrades. Density
persists at profile scope by default; a local override (a presentation or
accessibility viewer) must be explicitly explained, and density is never switched
silently by a provider, theme, or workflow change.

## Hard invariants

Each registry row carries four boolean invariants that must stay `false`:

1. `density_change_alters_information_architecture`
2. `density_change_alters_command_focus_or_trust`
3. `shrinks_hit_target_below_supported_minimum`
4. `silently_switches_density_outside_profile_scope`

## Acceptance criteria proven by the resolved examples

The validator proves — from the resolved examples, not from governance bools —
that:

1. **Compact, Standard, and Comfortable modes produce predictable, tokenized changes
   across lists, trees, tables, tabs, panels, editors, and inspectors.** Clean
   density-scale entries cover the three canonical modes and the first shell / editor
   / review / notebook / data surfaces, a private-scale example degrades, and no clean
   entry drifts from the canonical scale.
2. **At 400% zoom or equivalent assistive use, density preferences remain operable
   without shrinking hit targets below supported minimums.** Clean density-scale
   entries meet the hit-target minimum across the canonical modes, a below-minimum
   example degrades, and no clean entry shrinks below its minimum.
3. **Extension surfaces that cannot honor canonical density tokens degrade honestly
   instead of inventing private scales.** A private-scale example and a silent-switch
   persistence example both degrade, at least one clean profile-scoped persistence
   entry exists, and no clean entry carries a private scale.

## Regenerating the proof set

The seed builders in `seed.rs` are the single producer of the checked-in artifacts
and fixtures. Regenerate them with the headless emitter:

```text
cargo run -p aureline-ui --example dump_m5_density_mode_registries -- support-export
cargo run -p aureline-ui --example dump_m5_density_mode_registries -- csv
cargo run -p aureline-ui --example dump_m5_density_mode_registries -- report
cargo run -p aureline-ui --example dump_m5_density_mode_registries -- fixture-editor-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_density_mode_registries -- fixture-settings-ui-preview-narrowed
cargo run -p aureline-ui --example dump_m5_density_mode_registries -- validate
```
