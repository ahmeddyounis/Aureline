# Locale Diagnostics, Help/About, And Support-Export Posture

Canonical machine source:

- Diagnostics packet: [`/fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-packet.json`](../../fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-packet.json)
- Support export: [`/fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-support-export.json`](../../fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-support-export.json)
- Schema: [`/schemas/i18n/locale-diagnostics.schema.json`](../../schemas/i18n/locale-diagnostics.schema.json)
- Runtime contract: `aureline_shell::i18n::LocaleDiagnosticsPacket`
- Composed from: `aureline_i18n::seeded_locale_pack_compatibility_report` (locale, pack version, signature, compatibility, fallback, and missing-key truth)

## What This Proves

This is the one packet that lets Help/About, the diagnostics surface, support
exports, and release/shiproom tooling all explain localization state — active
locale, installed pack versions, compatibility, fallback chain, missing-key
counts, and the degraded-localization reason — **without raw log spelunking and
without cloning status text per surface**. It owns no localization truth: every
number is projected from the canonical locale-pack compatibility report, so the
four surfaces always quote the same figures.

## One Problem-Origin Vocabulary

Each installed pack and each requested-locale profile carries a
`problem_origin`, the bucket a support engineer assigns a localization issue to:

- `requested_locale` — the requested locale was authoritative.
- `base_fallback` — a base-language fill served some keys.
- `source_language_fallback` — the surface fell back to the source language
  (for example, a signature failure or policy-disabled locale).
- `pack_skew` — an incompatible pack version or build range forced a degrade.
- `missing_translations` — the pack applied but some keys are untranslated.

The seeded posture spans the spread deliberately: `en-US` is the authoritative
source, `es-MX` is fully localized, `fr-FR` and `pt-BR` apply with disclosed
gaps, `ja-JP` ships a **signature-failed** pack that degrades to source language,
and `de-DE` ships a pack that is **incompatible** with the active build. The
active session is rendering `de-DE`, so the headline state demonstrates pack
skew and a disclosed source-language fallback.

## Help/About Can Explain Localization State

`LocaleDiagnosticsPacket::help_about_card` answers "what language am I in, what
packs are installed, and is anything degraded?" with the active locale, the
fallback chain, the installed and incompatible pack counts, the active
missing-key count, and a single `honesty_marker_present` flag. The marker lights
whenever the active locale is degraded or any installed pack is incompatible, so
the chrome can never render a clean "fully localized" label over a skewed pack.

## Exports Are Metadata-Only And Origin-Bearing

The support export preserves the exact stable anchors an escalation pastes back —
pack ids, locale tags, and the same-surface source-language route refs — while
every row sets `raw_translated_body_omitted = true` and the export sets
`raw_translated_bodies_exported = false`. The `omitted_material_classes` list
names what is deliberately dropped (raw translated bodies, locale-pack signing
keys, provider payloads, and raw diagnostic logs), so a copied or bundled report
keeps the ids and the per-locale `problem_origin` a support engineer needs
without leaking translated content or private material.

## Release And Shiproom Narrow Locale-Bearing Claims

`LocaleDiagnosticsReleaseGate` evaluates every non-source locale against its
diagnosed state and publishes a gate decision no input can exceed:

- `claim_holds_fully_localized` — the localized claim holds.
- `claim_narrowed_partial` — narrowed to a disclosed-partial localization; still
  publishable with disclosure.
- `claim_narrowed_source_language` — narrowed to source-language fallback; not
  publishable as localized.
- `claim_blocked_incompatible_pack` — blocked by an incompatible pack; not
  publishable.

In the seeded posture `de-DE` is blocked and `ja-JP` is narrowed to source
language, so both `any_claim_narrowed` and `any_claim_blocked` are true and a
release or shiproom packet can narrow the locale-bearing claim automatically
instead of inheriting trust from an adjacent locale.

## Verification

```sh
cargo test -p aureline-shell --locked locale_diagnostics
```

Regenerate the canonical fixtures with:

```sh
cargo run -q -p aureline-shell --example dump_locale_diagnostics -- packet \
  > fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-packet.json
cargo run -q -p aureline-shell --example dump_locale_diagnostics -- support-export \
  > fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-support-export.json
```
