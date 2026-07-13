# M5 typography-scale, font-stack, and text-overflow registries

This lane is the typography implement lane over the frozen
[M5 visual-foundation matrix](./m5_visual_foundations_contract.md). It turns the **typography** family —
the type scale, the code / UI font stacks, the line-height guards, the tabular-numeral rule, and the
sentence-case / default-text rule — plus the text-layout guard that governs **overflow, truncation, and
wrap behavior** into registry resolvers that produce export-safe, honest projections, so titles, body,
labels, and code read as one hierarchy across the shell, editor, review, docs, and dense data consumers,
so counts / timings / diagnostics use tabular numerals, and so overflow never silently destroys meaning
under zoom or density changes.

It is the text-system analog of the
[color-system / semantic-theme-token registries lane](./m5_color_system_and_semantic_theme_token_registries.md)
and the [syntax / diff / chart registries lane](./m5_syntax_diff_and_chart_token_registries.md): those
lanes resolve palette, theme, and code / data meaning; this one resolves the type scale, font policy, and
overflow behavior.

## What the lane owns

- **Rust module** `crates/aureline-ui/src/m5_typography_scale_font_stack_and_overflow_registries` — two
  resolvers (`resolve_type_scale_entry`, `resolve_overflow_entry`), the registry-row and packet types, and
  the authoritative `validate()` gate.
- **Schema**
  `schemas/design-system/m5-typography-scale-font-stack-and-overflow-registries.schema.json` — the packet
  shape. Every registry row also names the canonical domain schema
  `schemas/design-system/m5-typography-and-geometry.schema.json`, which is the single-entry contract a
  downstream surface consumes instead of restating type-scale or overflow meaning by hand.
- **Release proof**
  `artifacts/release/m5-typography-scale-font-stack-and-overflow-registries-proof/` — the
  `support_export.json` support export, the `matrix.csv` machine-readable matrix, and the `summary.md`
  Markdown report, all minted from truth by the headless emitter.
- **Fixtures** `fixtures/ui/m5-typography-scale-font-stack-and-overflow-registries/` — two narrowed
  variants (`editor_ui_beta_narrowed.json`, `data_ui_preview_narrowed.json`) that prove honest
  auto-narrowing keeps every row visible and every example honest.

## Implementation requirements

1. **Canonical type scale with stable stacks and line-height guards.** Type-scale entries name a canonical
   token, a type-hierarchy role (`title`, `body`, `label`, `code`, `numeric_data`), a stable font stack
   that matches the role (code uses the monospace stack, UI text uses the sans stack), and a guarded
   line-height that never drifts.
2. **Tabular numerals and case rules.** A type-scale entry whose role is numeric data (counts / timings /
   diagnostics) degrades to `tabular_numerals_missing` unless tabular numerals are enabled, and every
   entry states a sentence-case / default-text rule or degrades to `case_rule_unstated`.
3. **Overflow, truncation, and wrap behavior.** Overflow entries govern tabs, rows, inspectors, banners,
   and code-adjacent metadata; each declares a treatment that preserves meaning (`truncate_with_tooltip`,
   `wrap_to_next_line`, `ellipsis_with_reveal`, `horizontal_scroll` — never a silent clip), keeps the full
   meaning reachable off the truncation, and survives both a zoom change and a density change.

## Acceptance criteria (proven by resolved examples, not asserted)

- **One readable type hierarchy.** Clean type-scale entries cover the `title` / `body` / `label` / `code`
  roles and both the UI-sans and code-mono font stacks across the first shell / editor / review / docs /
  data surfaces; a raw-type regression degrades and no clean entry inlines a raw value.
- **Tabular numerals and overflow safety.** A clean numeric entry enables tabular numerals, a
  tabular-missing regression degrades, a clean overflow entry preserves meaning, a meaning-destroyed
  regression degrades, and no clean overflow entry silently destroys meaning.
- **Zoom / density regressions caught.** A clean overflow entry survives zoom and density, a zoom
  regression degrades, a density regression degrades, a clean type-scale entry guards line-height while a
  line-height-drift regression degrades, and no clean overflow entry fails zoom / density.

## Regenerating the proof and fixtures

```text
cargo run -p aureline-ui --example dump_m5_typography_overflow_registries -- support-export
cargo run -p aureline-ui --example dump_m5_typography_overflow_registries -- csv
cargo run -p aureline-ui --example dump_m5_typography_overflow_registries -- report
cargo run -p aureline-ui --example dump_m5_typography_overflow_registries -- fixture-editor-ui-beta-narrowed
cargo run -p aureline-ui --example dump_m5_typography_overflow_registries -- fixture-data-ui-preview-narrowed
```

The inline tests re-read the checked-in artifact and both fixtures and assert byte-for-byte equality with
the seed builders, so a drift between the code and the checked-in evidence fails the build.
