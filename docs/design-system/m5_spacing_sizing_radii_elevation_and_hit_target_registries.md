# M5 spacing / sizing / radii / border / elevation geometry and hit-target registries

This lane is the closing geometry implement lane over the frozen
[M5 visual-foundation matrix](./m5_visual_foundations_contract.md). It turns the **spacing / sizing /
radii / border / elevation** geometry family and the **minimum hit-target** family — the last two of the
eight visual-foundation families — into registry resolvers that produce export-safe, honest projections,
so controls and panes share one canonical geometry instead of acquiring private layout rules, so compact /
standard / comfortable density changes presentation without shrinking hit targets below the supported
minima, so overlays and dialogs keep their elevation hierarchy, and so a local geometry fork is visible in
fixtures and proof packets before stable promotion.

It is the geometry analog of the
[color-system / semantic-theme-token registries lane](./m5_color_system_and_semantic_theme_token_registries.md),
the [syntax / diff / chart registries lane](./m5_syntax_diff_and_chart_token_registries.md), and the
[typography registries lane](./m5_typography_scale_font_stack_and_overflow_registries.md): those lanes
resolve palette, code / data, and type meaning; this one resolves spacing, sizing, radii, borders,
elevation, and minimum hit-target meaning.

## What the lane owns

- **Rust module** `crates/aureline-ui/src/m5_spacing_sizing_radii_elevation_and_hit_target_registries` —
  two resolvers (`resolve_geometry_entry`, `resolve_hit_target_entry`), the registry-row and packet types,
  and the authoritative `validate()` gate.
- **Schema**
  `schemas/design-system/m5-spacing-sizing-radii-elevation-and-hit-target-registries.schema.json` — the
  packet shape. Every registry row also names the canonical domain schema
  `schemas/design-system/m5-typography-and-geometry.schema.json`, which is the single-entry contract a
  downstream surface consumes instead of restating geometry or hit-target meaning by hand.
- **Release proof**
  `artifacts/release/m5-spacing-sizing-radii-elevation-and-hit-target-registries-proof/` — the
  `support_export.json` support export, the `matrix.csv` machine-readable matrix, and the `summary.md`
  Markdown report, all minted from truth by the headless emitter.
- **Fixtures** `fixtures/ui/m5-spacing-sizing-radii-elevation-and-hit-target-registries/` — two narrowed
  variants (`shell_ui_beta_narrowed.json`, `data_ui_preview_narrowed.json`) that prove honest
  auto-narrowing keeps every row visible and every example honest.

## Implementation requirements

1. **Shared geometry primitives and minimum hit-target rules.** Geometry entries name a canonical token, a
   primitive kind (`spacing`, `sizing`, `radius`, `border`, `elevation`), and a geometry role that matches
   the kind (never the disallowed local fork). Hit-target entries name a canonical token, a control kind
   (`button`, `icon_button`, `resize_handle`, `toggle`, `menu_item`), and a hit-target rule.
2. **Density-aware application.** A geometry entry that applies one geometry regardless of density mode
   degrades to `not_density_aware`, and a hit-target entry that shrinks below the supported minimum for its
   density degrades to `shrinks_below_minimum` — so compact / standard / comfortable modes change
   presentation without violating accessibility or command semantics.
3. **First consumers and drift checks.** The shell, list / table, editor, dialog, and review surfaces are
   wired as first consumers, and a forked, elevation-broken, or raw-value entry can never read as a clean
   pass.

## Acceptance criteria (proven by resolved examples, not asserted)

- **Canonical geometry primitives.** Clean geometry entries cover the `spacing` / `sizing` / `radius` /
  `elevation` primitive kinds across the first shell / list-table / editor / dialog / review surfaces; a
  forked regression degrades and no clean entry inlines a raw value.
- **Compact minima and elevation hierarchy.** A clean compact-density hit-target entry meets the supported
  minimum, a sub-minimum regression degrades, a clean elevation entry preserves the overlay / dialog
  hierarchy, an elevation-broken regression degrades, and no clean hit-target entry shrinks below minimum.
- **Geometry drift caught.** A clean geometry entry is density-aware, a not-density-aware regression
  degrades, a raw-geometry regression degrades, and no clean geometry entry forks the foundation.

## Regenerating the proof and fixtures

```text
cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- support-export
cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- csv
cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- report
cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- fixture-shell-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- fixture-data-ui-preview-narrowed
```

The inline tests re-read the checked-in artifact and both fixtures and assert byte-for-byte equality with
the seed builders, so a drift between the code and the checked-in evidence fails the build.
