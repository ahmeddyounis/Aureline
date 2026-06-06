# Stable Locale-Pack Lifecycle And Translated-Surface Parity

Canonical machine source:

- Packet: [`/fixtures/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity/manifest.json`](../../../fixtures/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity/manifest.json)
- Schema: [`/schemas/i18n/locale-pack-lifecycle.schema.json`](../../../schemas/i18n/locale-pack-lifecycle.schema.json)
- Runtime contract: `aureline_i18n::StableLocaleLifecycleParityPacket`
- Companion doc: [`/docs/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity.md`](../../../docs/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity.md)

## What This Proves

This packet makes localized rows claimable only when the active locale pack is signed, compatible, mirrorable where required, and backed by release-gated proof. Claimed localized rows in the seeded packet are green; missing, unverified, or signature-failed rows downgrade to source language with visible state.

The packet also binds translatable messages to stable non-prose identifiers: command ids, semantic action ids, diagnostic ids, schema refs, docs-pack keys, policy names, and telemetry keys. Translated prose is never the route key.

Fallback truth is inspectable as requested locale to language base to source language, and each fallback row is visible in Settings, diagnostics, and support export. Source-language routes remain present on translated docs, tours, auth/recovery, help/glossary, and CLI/help rows.

## Release Gates

The release gate rows require green proof for:

- locale-pack signing and compatibility
- fallback-chain truth
- stable message ids
- translated-surface parity
- pseudolocalization and text expansion
- RTL, bidi, IME, and font fallback

The exact verification command is:

```sh
cargo test -p aureline-i18n --test stable_locale_lifecycle_translated_surface_parity --locked
```

## Current Posture

The seeded summary reports 5 claimed localized rows, all green, with 4 degraded source-language rows and 0 blocked rows. Degraded rows are intentional examples for community partial coverage, missing packs, and signature failure; they preserve local product use and expose the source-language route.
