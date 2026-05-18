# RTL And Text Expansion Audit

Review date: 2026-05-18

## Evidence

- Corpus manifest: `fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/manifest.json`
- Review export: `fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/review_export.json`
- Readiness matrix: `artifacts/i18n/test_mode_matrix.yaml`
- Locale/input baseline: `docs/i18n/locale_input_readiness.md`

## Audit Result

Status: passed seed corpus, no active bounded waivers.

The manifest enforces these layout and directionality rules across dense beta
surfaces:

- Single-line chrome, state labels, tabs, and command rows tolerate at least
  1.35x source-language width before truncation.
- Multi-line banners, review summaries, permission sheets, guided steps, and
  help surfaces tolerate at least 1.60x source-language length before
  truncation.
- Overflow is forbidden on claim-bearing rows; a same-flow full-text route is
  required before truncating important copy.
- Directional chrome mirrors in RTL, but code, paths, hostnames, flags,
  command IDs, JSON keys, citations, and keyboard paths remain unmirrored.
- Focus order must track the mirrored visual order.
- Raw copy preserves authored logical order, and escaped copy remains available
  for mixed-direction or invisible-control review.

## Dense Surfaces Audited

| Surface | RTL / expansion posture |
|---|---|
| Editor | Bidi inspector and literal paths remain unmirrored; IME and completion/snippet churn are covered. |
| Command palette | RTL chrome and result ordering are covered; command IDs and keybindings stay literal. |
| Settings | Text expansion and locale fallback rows preserve setting/schema IDs and full-text routes. |
| Trees | Disclosure chrome mirrors; path labels and command IDs stay copy-safe. |
| Tables | CJK/full-width glyphs, selected counts, blocked counts, and number formatting stay legible. |
| Logs | Hostnames, flags, timestamps, and JSON keys remain raw/escaped-copy safe. |
| Terminal | Mixed-direction command lines and transcript copy retain raw versus escaped labels. |
| Review panes | Bidi/suspicious text and governed policy copy stay inspectable. |
| Guided tours | Expanded steps keep source-language and IME exercise routes reachable. |
| Docs/help | Citations, docs anchors, command examples, and source-language toggles survive RTL and fallback. |

## Verification

```sh
cargo test -p aureline-i18n --test pseudoloc_rtl_ime_corpus --locked
```

The test fails if any corpus case drops the expansion budget, RTL expectation,
literal-token copy requirement, or source-language/fallback assertion.
