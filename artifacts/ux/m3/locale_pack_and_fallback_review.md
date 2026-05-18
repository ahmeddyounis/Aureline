# Locale-Pack And Fallback Review

Review date: 2026-05-18

## Evidence

- Contract fixture: `fixtures/i18n/m3/locale_fallback/manifest.json`
- Settings projection: `fixtures/i18n/m3/locale_fallback/settings_projection.json`
- Help/About projection: `fixtures/i18n/m3/locale_fallback/help_about_projection.json`
- Support projection: `fixtures/i18n/m3/locale_fallback/support_projection.json`
- Support export: `fixtures/i18n/m3/locale_fallback/support_export.json`
- Schema: `schemas/i18n/locale_pack_manifest.schema.json`

## Review Result

The beta locale-pack contract exposes the active locale, requested-to-source
fallback chain, active pack versions, pack signature states, compatibility
results, and bounded waiver state through the same typed record used by
Settings, Help/About, and support export.

The current compatibility packet contains:

- `locale-pack:core:es-mx:beta` compatible with a verified signature;
- `locale-pack:community:pt-br:beta` compatible with a bounded glossary
  fallback waiver expiring on 2026-05-25;
- `locale-pack:extension:docs-helper:de-de:blocked` blocked by signature
  failure and rendered through source-language fallback only.

## Guardrails Checked

- Machine-facing identifiers remain locale-neutral.
- Command labels preserve canonical command ids and semantic action ids.
- Doctor finding codes and CLI JSON keys remain stable.
- Locale packs carry version, signature, compatibility, mirrorability,
  source-class, offline-import, and rollback metadata.
- Extension locale declarations cannot override host-owned stable ids.
- Support exports omit raw translated bodies and retain only metadata refs.

## Follow-Up Watch Items

- Replace the bounded community-pack waiver with full glossary coverage before
  making broader locale claims.
- Add schema-level JSON validation to CI once the existing schema validation
  harness includes `schemas/i18n/`.
