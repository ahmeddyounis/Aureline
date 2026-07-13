# M5 syntax-, diff-, and chart-token registries

This lane is the syntax / diff / chart implement lane over the frozen
[M5 visual-foundation matrix](./m5_visual_foundations_contract.md). It turns the three visual-foundation
families that carry **code and data** meaning — the **syntax token**, the **diff token**, and the
**chart token** — into registry resolvers that produce export-safe, honest projections, so source-code
highlighting, diff regions, and chart series mean the same thing across the editor, review, notebook,
data, and docs consumers, never collide with the diagnostics palette, never depend on hue alone, and
never lose their meaning under high contrast or export.

It is the code-and-data analog of the
[color-system / semantic-theme-token registries lane](./m5_color_system_and_semantic_theme_token_registries.md):
the two color families resolve palette and theme meaning; these three resolve syntax scopes, diff regions,
and chart series.

## What the lane owns

- **Rust module** `crates/aureline-ui/src/m5_syntax_diff_and_chart_token_registries` — three resolvers
  (`resolve_syntax_entry`, `resolve_diff_entry`, `resolve_chart_entry`), the registry-row and packet
  types, and the authoritative `validate()` gate.
- **Schema** `schemas/design-system/m5-syntax-diff-and-chart-token-registries.schema.json` — the packet
  shape. Every registry row also names the canonical domain schema
  `schemas/design-system/m5-syntax-diff-chart-tokens.schema.json`, which is the single-entry contract a
  downstream surface consumes instead of restating syntax / diff / chart meaning by hand.
- **Release proof** `artifacts/release/m5-syntax-diff-and-chart-token-registries-proof/` — the
  `support_export.json` support export, the `matrix.csv` machine-readable matrix, and the `summary.md`
  Markdown report, all minted from truth by the headless emitter.
- **Fixtures** `fixtures/ui/m5-syntax-diff-and-chart-token-registries/` — two narrowed variants
  (`editor_ui_beta_narrowed.json`, `data_ui_preview_narrowed.json`) that prove honest auto-narrowing keeps
  every row visible and every example honest.

## Implementation requirements

1. **Canonical syntax / diff / chart role tokens with explicit notes.** Syntax entries name a canonical
   token, keep their scope distinct from diagnostics, and declare a diagnostics-precedence posture so
   diagnostics visually outrank syntax color where they overlap. Diff entries additionally state a
   **moved-block confidence** and a **historical-vs-current emphasis**. Chart entries pair a
   legend / pattern / marker cue at accessible contrast.
2. **Non-color-only chart / diff cues.** Every diff and chart entry names a non-color cue
   (`text_label`, `fill_pattern`, `series_marker`, `legend`, or `screen_reader_text`) and degrades when
   meaning would otherwise ride on hue alone.
3. **Export / render survival.** Every entry carries the export channels it survives (`screenshot`,
   `pdf`, `support_packet`, `high_contrast`, `monochrome_print`); an entry that cannot survive the
   required screenshot / PDF / support-packet / high-contrast channels degrades to `export_meaning_lost`.

## Acceptance criteria (proven by resolved examples, not asserted)

- **Shared semantic mapping.** Clean syntax, diff, and chart entries name the canonical `syntax` / `diff`
  / `chart` semantic roles and cover the first editor / review / notebook / data / docs surfaces; a
  raw-color regression degrades and no clean entry inlines a raw color.
- **Diagnostics precedence and export survival.** A clean syntax entry honors diagnostics precedence, a
  syntax-diagnostics collision degrades, clean diff and chart entries survive export, and an export-loss
  regression degrades.
- **Legend / pattern parity.** Clean chart and diff entries pair a non-color cue, a color-alone chart
  regression degrades, a cue-missing diff regression degrades, and no clean chart / diff entry lacks the
  cue.

## Regenerating the proof and fixtures

```text
cargo run -p aureline-ui --example dump_m5_syntax_diff_chart_registries -- support-export
cargo run -p aureline-ui --example dump_m5_syntax_diff_chart_registries -- csv
cargo run -p aureline-ui --example dump_m5_syntax_diff_chart_registries -- report
cargo run -p aureline-ui --example dump_m5_syntax_diff_chart_registries -- fixture-editor-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_syntax_diff_chart_registries -- fixture-data-ui-preview-narrowed
```

The inline tests re-read the checked-in artifact and both fixtures and assert byte-for-byte equality with
the seed builders, so a drift between the code and the checked-in evidence fails the build.
