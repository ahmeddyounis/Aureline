# Dense M5 Surface Pseudoloc / RTL / IME / CJK Qualification

Canonical machine source:

- Packet: [`/fixtures/i18n/pseudoloc-rtl-ime-cjk/qualification.json`](../../../fixtures/i18n/pseudoloc-rtl-ime-cjk/qualification.json)
- Review export: [`/fixtures/i18n/pseudoloc-rtl-ime-cjk/review_export.json`](../../../fixtures/i18n/pseudoloc-rtl-ime-cjk/review_export.json)
- Narrowing scenarios: [`/fixtures/i18n/pseudoloc-rtl-ime-cjk/narrowing_cases.json`](../../../fixtures/i18n/pseudoloc-rtl-ime-cjk/narrowing_cases.json)
- Summary projection: [`./qualification_summary.json`](./qualification_summary.json)
- Runtime contract: `aureline_i18n::M5DenseSurfaceI18nQualification`
- Companion doc: [`/docs/i18n/m5-dense-surface-i18n-lab.md`](../../../docs/i18n/m5-dense-surface-i18n-lab.md)

## What This Proves

This packet gives the claimed localized M5 profiles dense-surface localization
proof rather than simple-form-only coverage. It exercises seven localization
harnesses — pseudolocalization, text-expansion, RTL/bidi, font-fallback, IME
composition, CJK, and localized date/number formatting — across ten dense M5
surfaces: editor-adjacent panes, the command palette, settings, terminal/help,
notebooks, data grids, pipeline/log views, docs/help panes, guided tours, and
support/report surfaces. Each harness runs against every claimed locale, so the
proof is per-surface, per-harness, and per-locale.

## Claims Cannot Outrun Dense-Surface Proof

Each claimed locale gets a qualification gate derived from its harness results,
not asserted by hand. A failed IME, RTL/bidi, font-fallback, or
localized-format result **blocks** the profile (claim narrows to
source-language fallback and promotion is held); a surface that falls back to the
source language for that locale **narrows** the profile without blocking core
use. A profile cannot stay green while any required harness is failing, which is
the invariant that holds promotion. The `narrowing_cases.json` scenarios replay
this: a notebook IME loss, a pipeline/log mirrored command id, a data-grid CJK
glyph drop, and a data-grid decimal-separator drift each block the claimed row,
while a support/report source-language fallback narrows it.

## Reusable, Not One-Off

The review export and per-profile gate are release-consumable. The
`consumption_bindings` rows bind the qualification to release promotion
(`aureline-release`), support export (`aureline-support`), and diagnostics
(`aureline-shell`), naming the exact packet fields each reads. QA, shiproom, and
support ingest the same evidence instead of re-running manual review sessions.

## Current Posture

The seeded qualification reports 10 inventoried surfaces, 10 harness cases, and
192 harness results across the 3 claimed locales (es-MX, ja-JP, ar-SA). All 192
results pass, all 7 harnesses and all 10 surface families are covered, and all 3
profiles hold a green claim with no active waivers. Promotion is green.

## Verification

```sh
cargo test -p aureline-i18n --test m5_dense_surface_i18n_lab --locked
cargo run -q -p aureline-i18n --bin aureline_i18n_locale_pack_beta -- m5-dense-lab-validate
```

Regenerate the canonical packet and projections with:

```sh
cargo run -q -p aureline-i18n --bin aureline_i18n_locale_pack_beta -- m5-dense-lab > fixtures/i18n/pseudoloc-rtl-ime-cjk/qualification.json
cargo run -q -p aureline-i18n --bin aureline_i18n_locale_pack_beta -- m5-dense-lab-review > fixtures/i18n/pseudoloc-rtl-ime-cjk/review_export.json
cargo run -q -p aureline-i18n --bin aureline_i18n_locale_pack_beta -- m5-dense-lab-narrowing > fixtures/i18n/pseudoloc-rtl-ime-cjk/narrowing_cases.json
cargo run -q -p aureline-i18n --example dump_m5_dense_surface_lab -- summary > artifacts/i18n/m5-pseudoloc-rtl-ime-report/qualification_summary.json
```
