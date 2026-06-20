# CLI, Project Doctor, And Support Report Localization Posture

Canonical machine source:

- CLI/help posture packet: [`/fixtures/i18n/cli-doctor-support/cli-help-localization.json`](../../fixtures/i18n/cli-doctor-support/cli-help-localization.json)
- Doctor report posture packet: [`/fixtures/i18n/cli-doctor-support/doctor-report-localization.json`](../../fixtures/i18n/cli-doctor-support/doctor-report-localization.json)
- Doctor locale support export: [`/fixtures/i18n/cli-doctor-support/doctor-report-support-export.json`](../../fixtures/i18n/cli-doctor-support/doctor-report-support-export.json)
- CLI schema: [`/schemas/i18n/cli-help-locale.schema.json`](../../schemas/i18n/cli-help-locale.schema.json)
- Doctor schema: [`/schemas/i18n/doctor-report-locale.schema.json`](../../schemas/i18n/doctor-report-locale.schema.json)
- Runtime contracts: `aureline_cli::CliLocalizationPacket`, `aureline_doctor::DoctorReportLocalizationPacket`
- Shared vocabulary: `aureline_i18n` (locale, fallback origin, degraded state, escape hatch, machine-output class)

## What This Proves

These two packets are the one place that lets the human-facing prose of the CLI,
`--help`, Project Doctor, and the support reports those surfaces export localize
**without breaking the contracts automation and support escalation depend on**.

Every translatable string binds to a stable, locale-neutral **message id**, a
stable **source-language key**, and the locale-neutral anchors a consumer routes
or parses by:

- **CLI/help** — subcommand paths, flag tokens (`--format`, `--locale-neutral`),
  `--format json` output keys, canonical exit classes, command ids, and
  telemetry keys.
- **Project Doctor** — finding codes, probe ids, canonical exit classes,
  evidence-ref kinds, scope labels, recovery command ids, and policy names.

## Localized Prose Cannot Break Automation

`CliLocalizationPacket::render` and `DoctorReportLocalizationPacket::render`
return the same message ids and the **byte-identical** locale-neutral anchors for
every requested locale; only the effective locale and the per-message
source-language fallback flag change. The CLI packet additionally carries a
`CliMachineOutputContract` that pins three invariants — JSON keys, flag tokens,
and subcommand names are never localized — while allowing exactly one optional
human field (`message`) to carry translated prose beside the locale-neutral keys.

`parity_report()` turns this into a release-gated proof: for every claimed locale
it asserts the rendered id set matches the source render and that flags, JSON
keys, exit classes, finding codes, evidence refs, and scope labels all survive
the render unchanged. Every entry also asserts
`machine_identifier_fields_locale_neutral` and `routed_by_localized_prose ==
false`, so behavior can never route by localized prose. Translated body text
never ships in these packets — only source-language template summaries used as
translation seeds.

## Translation-Safe Copy And Export

Both packets project a metadata-only `support_export`. Each export preserves the
exact stable anchors and source-language keys an escalation needs — finding
codes, exit classes, flag tokens, command ids — while every row sets
`raw_translated_body_omitted = true` and the export sets
`raw_translated_bodies_exported = false`. The `omitted_material_classes` list
names what is deliberately dropped (raw translated bodies, raw evidence payloads,
locale-pack signing keys, provider payloads), so a copied report or a bundled
support packet keeps the IDs and commands a support engineer pastes back without
leaking translated content.

## Locale And Fallback Are Inspectable

Each packet declares per-locale `locale_profiles` exposing the requested → base →
source `fallback_chain`, the `fallback_origin`, the `degraded_state`, the
`source_language_route_active` flag, and the `missing_key_count`. The seeded
`support_export` captures a claimed localized profile under partial fallback
(`ja-JP`), so the active locale, the effective locale, the fallback chain, and
the disclosed degraded state remain inspectable on the exported Doctor and
support artifacts — not hidden in debug logs.

## Current Posture

- **CLI/help** holds 11 messages across all seven surfaces (usage, subcommand
  summary, flag description, argument description, error prose, hint prose, and
  the optional JSON human field), preserving 18 distinct locale-neutral anchors.
- **Project Doctor** holds 7 messages across all six surfaces (finding title,
  explanation, recommended action, unsupported-state note, report heading, and
  support-export heading) over 2 finding families, preserving 17 distinct
  anchors.
- Five requested locales are profiled for each packet: `en-US` (source,
  authoritative) and `es-MX` (fully localized) report zero missing keys; `ja-JP`
  and `ar-SA` are partial with disclosed gaps; `de-DE` falls back to source
  language only (a failed/missing locale pack), with every key disclosed as
  source-language.

## Verification

```sh
cargo test -p aureline-cli --locked
cargo test -p aureline-doctor --locked
```

Regenerate the canonical fixtures with:

```sh
cargo run -q -p aureline-cli --example dump_cli_localization -- packet \
  > fixtures/i18n/cli-doctor-support/cli-help-localization.json
cargo run -q -p aureline-doctor --example dump_doctor_report_localization -- packet \
  > fixtures/i18n/cli-doctor-support/doctor-report-localization.json
cargo run -q -p aureline-doctor --example dump_doctor_report_localization -- support-export \
  > fixtures/i18n/cli-doctor-support/doctor-report-support-export.json
```
