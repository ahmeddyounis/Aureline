# M5 Shell Localization Parity

Canonical machine source:

- Localized catalog: [`/fixtures/i18n/shell-command-help/localized-catalog.json`](../../fixtures/i18n/shell-command-help/localized-catalog.json)
- Parity report: [`/fixtures/i18n/shell-command-help/localization-parity.json`](../../fixtures/i18n/shell-command-help/localization-parity.json)
- Right-to-left render sample: [`/fixtures/i18n/shell-command-help/render-ar-SA.json`](../../fixtures/i18n/shell-command-help/render-ar-SA.json)
- Catalog schema: [`/schemas/i18n/m5-localized-catalog.schema.json`](../../schemas/i18n/m5-localized-catalog.schema.json)
- Parity schema: [`/schemas/i18n/m5-localization-parity.schema.json`](../../schemas/i18n/m5-localization-parity.schema.json)
- Runtime contract: `aureline_i18n::M5LocalizedCatalog`, `aureline_i18n::M5LocalizationParityReport`
- Shell projection: `aureline_shell::i18n::localized_surface::LocalizedSurfaceView`

## What This Proves

The [message-id registry](./m5-message-registry-proof.md) binds every translatable
string on the new M5 **shell, command, settings, help, error, and notification**
surfaces to a stable, locale-neutral message id and records *whether* a locale
carries a translation. This packet carries the other half: the **actual
translated display strings** for the claimed localized profiles, bound to those
same ids, plus a render and a parity report that make localization a checked
property rather than a manual pass.

Three claims the spec treats as release-bearing are tested here:

- **Ids and routing survive localization.** `M5LocalizedCatalog::render` joins
  the catalog onto the registry and returns one row per message. Across every
  requested locale the row sequence, the stable message ids, and the stable
  command ids, setting ids, diagnostic ids, telemetry keys, and policy names are
  byte-identical; only the visible prose, effective locale, text direction, and
  source-language fallback flag change. The shell projection keeps the
  command-palette and disabled-state rows discoverable by id, and keyboard-path
  hints stay locale-neutral, so a translated label never moves its shortcut.
- **Truncation and zoom never hide scope or severity.** Severity and surface
  scope live in row metadata, not in the truncatable string. `truncate` shortens
  the visible prose to a grapheme budget while carrying both through untouched,
  so a tighter budget (a zoomed display or a narrow chrome slot) can shorten an
  error label but cannot demote it to a plain string or drop its diagnostic id.
- **Coverage is honest.** Every claimed-locale translation preserves the
  message's placeholder tokens (`{workspace_name}`, `{count}`, `{command}`,
  `{reason}`), and any message a locale does not translate is listed explicitly
  and rendered in the source language rather than hidden.

## Machine-Readable Stability Is Not Localized

Translated bodies live in the catalog (it *is* the first-party locale-pack
content) and in the per-locale render. The metadata-only support-export
projection omits them: `LocalizedSurfaceView` for the support audience keeps
every stable id, length, expansion ratio, and state but drops the localized
label and sets `raw_translated_body_omitted`. Localization therefore introduces
no command aliases, policy semantics, or telemetry forks that exist only in
translated prose; the catalog validator rejects any translation that embeds a
stable identifier verbatim.

## Current Posture

The seeded catalog ships 35 translated strings across three claimed locales
(`es-MX` 15, `ja-JP` 12, `ar-SA` 8) for the 15 registered M5 messages. Parity is
clean for every claimed locale:

| Locale | Direction | Localized / Source-fallback | Max text expansion | Untranslated rows marked |
| --- | --- | --- | --- | --- |
| `es-MX` | left-to-right | 15 / 0 | 207% | 0 |
| `ja-JP` | left-to-right | 12 / 3 | 100% | 3 |
| `ar-SA` | right-to-left | 8 / 7 | 121% | 7 |

For all three, `id_set_matches_source`, `all_stable_refs_preserved`,
`all_placeholders_preserved`, and `severity_preserved_under_truncation` hold, so
`parity_clean` is true. Locales the catalog does not claim (for example `de-DE`)
fall back to the source language through the existing fallback inspector and are
out of scope for this batch rather than silently English.

## Verification

```sh
cargo test -p aureline-i18n --lib localized_catalog --locked
cargo test -p aureline-i18n --test m5_localized_surface_parity --locked
cargo test -p aureline-shell --lib i18n:: --locked
```

Regenerate the canonical fixtures with:

```sh
cargo run -q -p aureline-i18n --example dump_m5_localized_catalog -- catalog > fixtures/i18n/shell-command-help/localized-catalog.json
cargo run -q -p aureline-i18n --example dump_m5_localized_catalog -- parity > fixtures/i18n/shell-command-help/localization-parity.json
cargo run -q -p aureline-i18n --example dump_m5_localized_catalog -- render ar-SA > fixtures/i18n/shell-command-help/render-ar-SA.json
```
