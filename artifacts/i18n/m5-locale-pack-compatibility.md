# M5 Locale-Pack Compatibility, Skew, And Degraded-Localization Report

Canonical machine source:

- Compatibility report: [`/fixtures/i18n/pack-skew-and-signature/compatibility_report.json`](../../fixtures/i18n/pack-skew-and-signature/compatibility_report.json)
- Checked-in core pack artifacts: [`/locale-packs/core/`](../../locale-packs/core/)
- Pack artifact schema: [`/schemas/i18n/locale-pack.schema.json`](../../schemas/i18n/locale-pack.schema.json)
- Report schema: [`/schemas/i18n/locale-pack-compatibility-report.schema.json`](../../schemas/i18n/locale-pack-compatibility-report.schema.json)
- Runtime contract: `aureline_i18n::locale_pack_delivery`
- Shell inspector: `aureline_shell::i18n::pack_compatibility::LocalePackCompatibilityView`

## What This Proves

Each locale pack ships as a **versioned, signed, mirrorable artifact**
(`LocalePackArtifact`) that carries a compatibility build range, signer
identity, mirrorability metadata, and a content integrity digest. The
artifacts under `/locale-packs/core/` are the checked-in first-party packs;
the report is what diagnostics, support export, and release tooling read to
learn, for every evaluated pack, the exact **pack version**, the
**compatibility** and **signature** state, the active **fallback**, the
**missing-key count** (overall and per surface), and — for degraded packs —
the **degraded-localization reason**.

Whether a pack *renders* is never a property of the artifact on disk. It is
decided at evaluation time by `LocalePackArtifact::evaluate` against the
observed environment (signature verification, version match, integrity digest,
compatibility range, presence, and policy). The report records the observed
inputs next to the decision, and `validate()` re-runs the evaluation from those
inputs, so a hand-edited row cannot misstate skew handling.

## Skew Degrades Fully, Never Partially

The central invariant is that an unsupported, unsigned, tampered, or
version-skewed pack does **not** partially apply stale translations. It
degrades **fully** to source-language behavior, with a recorded reason, so a
half-localized shell or help surface can never sit in an undefined state. A
pack that *is* renderable but only partially translated still applies, with its
per-surface missing-key count disclosed.

The seeded report makes the distinction concrete:

- **`ja-JP`** ships a *complete* translation on disk, but its signature fails to
  verify. The decision is `degrade_to_source_language_only`
  (`signature_failed`): all 36 keys fall back to source language. None of the
  on-disk translations are applied.
- **`de-DE`** is signed and verifies, but the installed revision targets a build
  window the active build is outside of. The decision is
  `degrade_to_source_language_only` (`build_outside_compatibility_range`); again
  all keys fall back rather than applying a stale-window translation.
- **`fr-FR`** is signed, verifies, and is compatible, but its docs surface is
  only partially translated. It **applies**, disclosing 3 missing keys that fall
  back to source per key.
- **`pt-BR`** is an unsigned community pack admitted only through an explicit
  acceptance decision. It applies with disclosed missing keys but is **not**
  promoted to a claimed localized profile — an unsigned pack never masquerades
  as supported localized.

## Install, Upgrade, Mirror, And Downgrade

The report's `operations` capture the governed install, update, mirror-import,
rollback, and offline-import actions that produced each row. Every operation
requires signature verification and a compatibility check before applying;
mirror and offline imports must preserve provenance metadata, and rollbacks
must cite a rollback target. Each operation references the compatibility row it
resolved to, so support can report exactly what was applied and why.

## Guardrail

`validate()` enforces that no degraded pack backs a claimed localized profile
and that every claimed localized profile renders from a signed, compatible,
present pack. The summary's `guardrail_clean` flag stays true only while that
holds, so an unsigned or incompatible pack cannot masquerade as a supported
localized stable row.

## Current Posture

Six packs are evaluated against the active build: `en-US` (source) and `es-MX`
are fully localized claimed profiles; `fr-FR` is a claimed profile with 3
disclosed missing keys; `ja-JP` and `de-DE` are degraded to source language
(signature and version skew) and auto-narrowed out of any localized claim;
`pt-BR` applies as an accepted unsigned community pack but is not claimed.
`guardrail_clean` is true.

## Verification

```sh
cargo test -p aureline-i18n --test locale_pack_compatibility --locked
cargo test -p aureline-shell --lib i18n:: --locked
```

Regenerate the canonical report and core artifacts with:

```sh
cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- report \
  > fixtures/i18n/pack-skew-and-signature/compatibility_report.json
cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- artifact locale-pack:core:source:en-us \
  > locale-packs/core/en-US/pack.json
cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- artifact locale-pack:core:es-mx \
  > locale-packs/core/es-MX/pack.json
cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- artifact locale-pack:core:fr-fr \
  > locale-packs/core/fr-FR/pack.json
cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- artifact locale-pack:core:ja-jp \
  > locale-packs/core/ja-JP/pack.json
cargo run -q -p aureline-i18n --example dump_locale_pack_compatibility -- artifact locale-pack:core:de-de \
  > locale-packs/core/de-DE/pack.json
```
