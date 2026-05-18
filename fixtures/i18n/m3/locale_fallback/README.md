# Locale-Pack Fallback Fixtures

These fixtures are generated from `aureline_i18n_locale_pack_beta` and replayed
by `crates/aureline-i18n/tests/locale_pack_beta.rs`.

- `manifest.json` is the governed beta locale-pack contract.
- `settings_projection.json` is the Settings locale inspector projection.
- `help_about_projection.json` is the Help/About locale provenance projection.
- `support_projection.json` is the support-surface row projection.
- `support_export.json` is the metadata-only support export.

The records intentionally omit raw translated bodies and preserve only stable
message ids, pack refs, fallback states, command ids, diagnostic ids, schema
ids, and docs-pack keys.
