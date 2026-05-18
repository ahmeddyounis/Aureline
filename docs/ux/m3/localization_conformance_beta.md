# Localization Conformance Beta

This contract binds the beta dense-surface internationalization corpus to the
runtime fixture, review export, and release evidence packets. It extends the
locale-pack beta contract with proof for pseudolocalization, RTL, bidi, IME
composition, CJK/font fallback, text expansion, locale fallback, translated
surface parity, and localized date/number formatting.

## Sources

- `docs/i18n/locale_input_readiness.md`
- `artifacts/i18n/test_mode_matrix.yaml`
- `docs/i18n/locale_surface_matrix.md`
- `docs/ux/localization_and_locale_pack_contract.md`
- `docs/accessibility/locale_fallback_and_copy_representation_contract.md`
- `fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/manifest.json`

## Corpus Scope

The corpus is claim-bearing for the beta surfaces that must stay usable under
real translated and international-input stress:

| Surface | Required proof |
|---|---|
| Editor | IME preedit/commit survives focus changes, completion preview, snippet traversal, bidi inspection, and CJK fallback. |
| Command palette | RTL chrome mirrors, command IDs and keybindings stay literal, IME query text survives result re-ranking and command preview. |
| Settings | Locale fallback is disclosed, stale or missing packs do not block core use, setting/schema IDs remain stable. |
| Trees | Directional disclosure chrome mirrors while paths and command IDs remain unmirrored and copy-safe. |
| Tables | Pseudoloc expansion, CJK/full-width glyphs, counts, blocked labels, and localized numbers stay legible. |
| Logs | Mixed-direction hostnames, flags, timestamps, JSON keys, and fallback headings keep raw/escaped copy truth. |
| Terminal | IME input, transcript copy, target headers, and mixed-direction command lines remain exact and labelled. |
| Review panes | Bidi/suspicious text, policy copy, citations, command IDs, and source-language exports remain governed. |
| Guided tours | Localized steps keep command tips, progress, source-language routes, and IME exercise fields intact. |
| Docs/help | Citations, docs anchors, command IDs, glossary terms, and source-language toggles survive RTL and fallback. |

## Release Gate

The gate is automated by the `aureline-i18n` fixture replay test:

```sh
cargo test -p aureline-i18n --test pseudoloc_rtl_ime_corpus --locked
```

The same command is recorded in the corpus lane bindings for nightly,
release-candidate, and localization/input-path pull-request runs. Claimed beta
localized surfaces are blocked if the replay detects missing surface coverage,
missing stress-mode coverage, silent IME commit/cancel risk, literal token
mirroring, missing source-language hatches, stale-pack blocking, or translated
stable-ID drift.

## Exported Evidence

- Machine corpus: `fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/manifest.json`
- Review export: `fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/review_export.json`
- Reviewer packet: `artifacts/ux/m3/localized_surface_review_packet.md`
- RTL and text-expansion audit: `artifacts/ux/m3/rtl_and_text_expansion_audit.md`

## Waiver Rule

The current corpus has no active bounded waivers. Any future waiver must name
the affected case, bounded failure classes, expiry, user-visible fallback or
claim narrowing, and the release packet carrying it. General accessibility
waivers cannot mask locale, RTL, bidi, IME, or text-expansion failures.
