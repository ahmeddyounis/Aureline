# M5 Translated Help And Onboarding

Canonical machine source:

- Translated-help pack: [`/fixtures/i18n/docs-tour-auth-recovery/translated-help-packs.json`](../../fixtures/i18n/docs-tour-auth-recovery/translated-help-packs.json)
- Parity report: [`/fixtures/i18n/docs-tour-auth-recovery/translated-help-parity.json`](../../fixtures/i18n/docs-tour-auth-recovery/translated-help-parity.json)
- Right-to-left render sample: [`/fixtures/i18n/docs-tour-auth-recovery/render-ar-SA.json`](../../fixtures/i18n/docs-tour-auth-recovery/render-ar-SA.json)
- Pack schema: [`/schemas/help/translated-doc-pack.schema.json`](../../schemas/help/translated-doc-pack.schema.json)
- Parity schema: [`/schemas/help/translated-doc-pack-parity.schema.json`](../../schemas/help/translated-doc-pack-parity.schema.json)
- Translated bodies: [`/docs/help/locales/`](../../docs/help/locales/)
- Runtime contract: `aureline_i18n::M5TranslatedHelpPack`, `aureline_i18n::M5TranslatedHelpParityReport`

## What This Proves

The shell-localization lane covers the *short* M5 surfaces — chrome, command,
settings, help, error, and notification strings. This packet carries the
*long-form* half: the actual translated **docs/help pages, guided tours,
glossary cards, auth copy, recovery copy, and onboarding** assets for the
claimed localized profiles, each bound to a stable, locale-neutral asset id, with
a render and a parity report that make the spec's guardrails checked properties
rather than a manual pass.

Three claims the spec treats as release-bearing are tested here:

- **Translations stay citation-faithful and command-faithful.** Every translated
  asset preserves the source asset's citation anchors, command ids, keyboard
  paths, and scope labels byte-for-byte; only the title and prose change. The
  pack validator rejects a translation whose preserved refs drift from its source
  asset, and `build_translated_help_parity_report` proves `citation_faithful`,
  `command_faithful`, and `all_refs_preserved` per claimed locale. The
  integration test goes one level deeper: it opens every referenced body under
  `docs/help/locales/` and asserts each command id and citation anchor is present
  in the translated file, so faithfulness is proven against the real prose, not
  just the metadata.
- **Source-language truth stays reachable.** Every asset — translated or fallen
  back — exposes an `Open in source language` escape hatch that is keyboard
  reachable, carries a stable `cmd:` route, and points at the canonical source
  body. Support, troubleshooting, and learning can always reach exact wording.
- **Imported or stale translated help never masquerades as live truth.** Each
  translation discloses its freshness, the source revision it was translated
  from, and its mirror/offline posture. When freshness diverges from the live
  source the rendered row is marked `distinct_from_live_source`; missing
  translations fall back to the source language with a `not_installed` posture
  instead of silently rendering English as if current; and escalation-critical
  auth/recovery copy keeps its escalation command routes even when stale.

## Stable Refs And Escalation Routes Are Not Localized

Citation anchors, command ids, keyboard paths, and scope labels are the
machine-routable truth; the translated prose merely describes them. They never
localize. Auth and recovery copy additionally keep their escalation routes
(`cmd:auth.contact_support`, `cmd:support.open_recovery_runbook`) under
translation, partial coverage, stale freshness, and source-language fallback —
the validator and the parity proof both reject a row that drops them.

## Current Posture

The seeded pack ships 6 source assets (one per family) and 12 translated assets
across three claimed locales (`es-MX` 6, `ja-JP` 3, `ar-SA` 3). Parity is clean
for every claimed locale:

| Locale | Direction | Translated / Source-fallback | Distinct-from-live coverage | Untranslated assets marked |
| --- | --- | --- | --- | --- |
| `es-MX` | left-to-right | 6 / 0 | all current with live source | 0 |
| `ja-JP` | left-to-right | 3 / 3 | warm-cached glossary card flagged distinct | 3 |
| `ar-SA` | right-to-left | 3 / 3 | stale recovery card flagged distinct | 3 |

For all three, `asset_id_set_matches_source`, `citation_faithful`,
`command_faithful`, `all_refs_preserved`, `all_escape_hatches_present`,
`stale_or_offline_distinct_from_live`, and `escalation_routes_preserved` hold, so
`parity_clean` is true. The `ar-SA` recovery card is intentionally stale (its
basis is `help-source-rev:restore-checkpoint:2026.05.10-01`, behind the live
`2026.05.18-01`): it renders with a `stale_translation` badge, is marked distinct
from live source, and still carries both recovery escalation routes. Assets a
locale does not claim fall back to the source language and are listed explicitly
rather than silently English.

## Verification

```sh
cargo test -p aureline-i18n --lib translated_help_packs --locked
cargo test -p aureline-i18n --test m5_translated_help_pack_parity --locked
```

Regenerate the canonical fixtures with:

```sh
cargo run -q -p aureline-i18n --example dump_m5_translated_help_packs -- pack > fixtures/i18n/docs-tour-auth-recovery/translated-help-packs.json
cargo run -q -p aureline-i18n --example dump_m5_translated_help_packs -- parity > fixtures/i18n/docs-tour-auth-recovery/translated-help-parity.json
cargo run -q -p aureline-i18n --example dump_m5_translated_help_packs -- render ar-SA > fixtures/i18n/docs-tour-auth-recovery/render-ar-SA.json
```
