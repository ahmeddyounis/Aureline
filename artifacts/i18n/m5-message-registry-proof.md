# M5 Message-ID Registry And Source-Language Fallback Inspector

Canonical machine source:

- Registry packet: [`/fixtures/i18n/message-id-stability/registry.json`](../../fixtures/i18n/message-id-stability/registry.json)
- Prior-release baseline: [`/fixtures/i18n/message-id-stability/baseline-ids.json`](../../fixtures/i18n/message-id-stability/baseline-ids.json)
- Registry schema: [`/schemas/i18n/m5-message-registry.schema.json`](../../schemas/i18n/m5-message-registry.schema.json)
- Baseline schema: [`/schemas/i18n/m5-message-id-baseline.schema.json`](../../schemas/i18n/m5-message-id-baseline.schema.json)
- Runtime contract: `aureline_i18n::M5MessageRegistry`
- Shell inspector: `aureline_shell::i18n::fallback_inspector::LocaleFallbackInspectorView`

## What This Proves

This packet is the one place that binds every translatable string on the new M5
**shell, command, settings, help, error, and notification** surfaces to a
stable, locale-neutral **message id** and a stable **source-language key**. Next
to each id it carries the stable non-prose anchors that business logic routes by
— command ids, setting ids, diagnostic ids, telemetry keys, and policy names —
so command routing, analytics, policy, and machine output keep working when copy
changes or a locale pack is missing.

The registry exists to make two continuity claims testable rather than reviewed
by hand:

- **Across locale changes.** A message id never carries a locale tag, so the id
  set rendered for one locale is identical to the id set rendered for any other.
  `M5MessageRegistry::render` returns the same ids regardless of the requested
  locale; only the effective locale and the per-message source-language fallback
  flag change. `validate()` rejects any id or key that embeds a locale tag.
- **Across release builds.** `M5MessageRegistry::continuity_against` diffs the
  current registry against a frozen baseline snapshot from a prior build. Ids may
  be **added**, but a **removed** id or a **drifted** source-language key fails
  the continuity check.

## Localized Prose Cannot Break Contracts

Every entry asserts `machine_identifier_fields_locale_neutral` and
`routed_by_localized_prose == false`, and must declare at least one stable
non-prose anchor. The registry therefore lets localized prose change freely
while command, policy, analytics, and machine-output contracts stay pinned to
ids. Translated body text never ships in this packet — only source-language
template summaries used as translation seeds.

## Fallback Is Inspectable, Not Hidden

`aureline_shell::i18n::fallback_inspector` projects the registry into one
`LocaleFallbackInspectorView` shared by the user-facing Settings / Help/About
surfaces and the metadata-only support export. For any requested locale it
reports the requested locale, the effective locale, the requested → base →
source fallback chain, the fallback origin (requested-locale authoritative,
base-language fill, or source-language only), and the **missing-key count** both
overall and per surface. Both audiences quote the same numbers, so users and
support exports see the identical fallback truth; no fallback state lives only in
debug logs.

## Current Posture

The seeded registry holds 15 messages across all six surfaces (shell chrome 3,
command palette 3, settings 3, help 2, error 2, notification 2). 13 ids are
preserved from the prior-release baseline and 2 are newly introduced; continuity
diffs to 0 removed and 0 key-drift, so the release-build continuity gate is
stable. Five requested locales are profiled: `en-US` (source, authoritative) and
`es-MX` (fully localized) report zero missing keys; `ja-JP` and `ar-SA` are
partial with disclosed per-surface gaps; `de-DE` is source-language only after a
locale-pack signature failure, with every key falling back.

## Verification

```sh
cargo test -p aureline-i18n --test message_id_stability --locked
cargo test -p aureline-shell --lib i18n:: --locked
```

Regenerate the canonical fixtures with:

```sh
cargo run -q -p aureline-i18n --example dump_m5_message_registry -- registry > fixtures/i18n/message-id-stability/registry.json
cargo run -q -p aureline-i18n --example dump_m5_message_registry -- baseline > fixtures/i18n/message-id-stability/baseline-ids.json
```
