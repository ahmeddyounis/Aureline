# Localized Surface Review Packet

Review date: 2026-05-18

## Evidence

- Corpus manifest: `fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/manifest.json`
- Review export: `fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/review_export.json`
- Contract: `docs/ux/m3/localization_conformance_beta.md`
- Locale-pack runtime contract: `docs/ux/m3/locale_pack_beta_contract.md`

## Result

Status: passed seed corpus, no active bounded waivers.

The corpus covers editor, command palette, settings, trees, tables, logs,
terminal, review panes, guided tours, and docs/help. It exercises
pseudolocalization, RTL chrome, mixed-direction technical text, IME composition,
CJK/font fallback, text expansion, locale fallback, translated-surface parity,
and localized date/number formatting.

## Assertions Checked

- Stable command IDs, keyboard paths, citation anchors, schema IDs, policy IDs,
  docs anchors, JSON keys, and scope labels remain locale-neutral and literal.
- Translated and fallback docs/help/tour surfaces keep source-language access
  available in the same workflow.
- IME composition is not silently committed, cancelled, or retargeted by focus
  changes, completion previews, snippet traversal, command previews, filtering,
  or overlay transitions.
- Missing, stale, partial, incompatible, or signature-failed locale packs fall
  back with disclosure instead of blocking core product use.
- Policy, trust, recovery, and source-language fallback terms stay governed and
  do not drift under translation.

## Lane Binding

Run:

```sh
cargo test -p aureline-i18n --test pseudoloc_rtl_ime_corpus --locked
```

The corpus manifest records this command for nightly, release-candidate, and
pull-request runs touching localization or input-path surfaces.

## Follow-Up Watch Items

- Replace seed-only fixture references with rendered screenshots and platform
  IME captures as the custom shell surfaces become executable.
- Keep any future waiver bounded to a case, expiry, failure class, and visible
  fallback; do not fold localization regressions into generic accessibility
  waivers.
