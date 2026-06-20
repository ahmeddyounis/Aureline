# Dense M5 Surface I18n Lab

This is the human-readable companion to the dense-surface localization
qualification that gates the claimed localized M5 profiles. It turns
pseudolocalization, text-expansion, RTL/bidi, font-fallback, IME composition,
CJK, and localized date/number correctness from one-off screenshot review into
release-gated, machine-readable proof.

Canonical machine source:

- Packet: [`/fixtures/i18n/pseudoloc-rtl-ime-cjk/qualification.json`](../../fixtures/i18n/pseudoloc-rtl-ime-cjk/qualification.json)
- Review export: [`/fixtures/i18n/pseudoloc-rtl-ime-cjk/review_export.json`](../../fixtures/i18n/pseudoloc-rtl-ime-cjk/review_export.json)
- Narrowing scenarios: [`/fixtures/i18n/pseudoloc-rtl-ime-cjk/narrowing_cases.json`](../../fixtures/i18n/pseudoloc-rtl-ime-cjk/narrowing_cases.json)
- Report: [`/artifacts/i18n/m5-pseudoloc-rtl-ime-report/report.md`](../../artifacts/i18n/m5-pseudoloc-rtl-ime-report/report.md)
- Runtime contract: `aureline_i18n::M5DenseSurfaceI18nQualification`

Companion contracts:

- [`/docs/i18n/m5-localization-scope.md`](./m5-localization-scope.md) — the
  localized-profile matrix and surface inventory.
- [`/docs/i18n/locale_surface_matrix.md`](./locale_surface_matrix.md) — what may
  localize versus what must stay machine-stable per surface.
- The dense beta corpus at
  [`/fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/`](../../fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/),
  which this lab extends to the dense M5 surfaces and binds to profile claims.

## The seven harnesses

Each claimed locale is exercised by seven localization harnesses:

- **Pseudolocalization** — accent wrapping and clip detection.
- **Text expansion** — long translated strings against an explicit expansion
  budget; overflow is forbidden and a same-flow full-text route is required.
- **RTL/bidi** — directional chrome mirrors while literal technical tokens
  (command ids, paths, hostnames, flags, citations) stay unmirrored and
  copy-safe.
- **Font fallback** — CJK, full-width, and emoji glyphs render through an
  accepted font-fallback chain.
- **IME composition** — preedit, candidate, and commit survive dense churn
  (focus change, completion preview, snippet traversal, command preview, filter
  re-rank, overlay transition) without silent commit or cancel.
- **CJK** — full-width layout, counting, and wrapping behavior.
- **Localized date/number** — locale-sensitive dates, numbers, durations, and
  counts keep their meaning and stable semantics.

## The ten dense surfaces

The lab covers dense product workflows, not simple forms: editor-adjacent panes,
the command palette, settings, terminal/help, **notebooks**, **data grids**,
**pipeline/log views**, docs/help panes, guided tours, and **support/report**
surfaces. Surfaces that accept free text additionally run the IME harness, and
surfaces that render dates, numbers, or counts additionally run the localized
date/number harness.

## How a claim narrows or blocks

Every claimed locale gets a qualification gate derived from its harness results:

- **Green** — every required harness passes; the profile holds its localized
  claim.
- **Narrowed** — a surface falls back to source language for that locale; the
  claim narrows to source-language fallback but core use is not blocked.
- **Blocked** — an IME, RTL/bidi, font-fallback, pseudoloc, text-expansion, CJK,
  or localized-format harness fails; the claim narrows to source-language
  fallback and promotion is held.

The gate is derived, never hand-asserted — a stored packet whose gates disagree
with its results fails validation. The narrowing scenarios exercise this end to
end so the gate logic is itself release-gated proof.

## Downstream consumption

The qualification is release-consumable rather than re-keyed into a checklist.
Release promotion reads the gate state and promotion block, support export reuses
the narrowing reasons and affected surfaces, and diagnostics surface the
effective claim class. QA and shiproom replay the same fixtures instead of
running fresh manual review sessions.

## Verification

```sh
cargo test -p aureline-i18n --test m5_dense_surface_i18n_lab --locked
```
