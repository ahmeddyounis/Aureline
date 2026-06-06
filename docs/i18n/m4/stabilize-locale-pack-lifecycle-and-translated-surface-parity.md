# Stable Locale-Pack Lifecycle And Translated-Surface Parity

This page is the human-readable companion for the stable localization lifecycle packet implemented by `crates/aureline-i18n`.

Canonical machine source:

- Fixture: [`/fixtures/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity/manifest.json`](../../../fixtures/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity/manifest.json)
- Schema: [`/schemas/i18n/locale-pack-lifecycle.schema.json`](../../../schemas/i18n/locale-pack-lifecycle.schema.json)
- Artifact packet: [`/artifacts/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity.md`](../../../artifacts/i18n/m4/stabilize-locale-pack-lifecycle-and-translated-surface-parity.md)

## Contract

Localized rows are claimable only when:

- locale packs are signed or built in, inside their compatibility window, rollbackable, and mirrorable where required;
- stable message ids remain anchored to command ids, semantic action ids, diagnostic/schema ids, docs-pack keys, policy names, or telemetry keys;
- fallback state is visible as requested locale to base locale to source language in Settings, diagnostics, and support export;
- docs, tours, auth/recovery, help/glossary, and CLI/help rows preserve citations, command refs, keyboard paths, screen-reader labels, recovery routes, and a source-language route;
- machine-readable flags, JSON keys, command ids, telemetry keys, policy names, and finding codes remain locale-neutral.

## Downgrade Behavior

Missing packs, signature failures, and partial community coverage do not block local product use. They render source-language strings, disclose degraded localization state, keep source-language access available, and keep support/export metadata free of raw translated bodies.

## Verification

Run:

```sh
cargo test -p aureline-i18n --test stable_locale_lifecycle_translated_surface_parity --locked
```

Regenerate the canonical packet with:

```sh
cargo run -q -p aureline-i18n --bin aureline_i18n_locale_pack_beta -- stable-lifecycle
```
