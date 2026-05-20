# Extension manifest editor (beta)

The manifest editor is the native authoring surface for extension/package
manifests inside Aureline. It lets authors open, inspect, validate, and migrate a
manifest without relying on opaque CLI-only error text or undocumented schema
drift. It does **not** generate manifests, write unsupported fields, or suppress
schema errors to feel friendly.

The machine-readable contract is
[`/schemas/extensions/manifest_editor_session.schema.json`](../../../schemas/extensions/manifest_editor_session.schema.json).
The evaluator is `aureline_extensions::manifest_editor`. The shell manifest
editor, CLI / headless inspect, and support exports all read one
`ManifestEditorSession` record instead of re-deriving validation locally.

## What a session carries

A session turns one (possibly incomplete) manifest into:

- **Inline validation with field-level anchors.** Every finding carries a stable
  `check_id`, `suite`, `status`, `severity`, the dotted manifest `field`, and a
  JSON-pointer `anchor` so the editor can highlight the exact field.
- **Blockers vs advisories.** Must-fix publish blockers (`severity = blocker`)
  are separated from recommended UX/performance improvements
  (`suite = editor_advisory`, `severity = warning`/`info`). Authors can always
  tell a release blocker from advice.
- **Permission explanation chips.** Each declared permission resolves to a
  capability class, a privileged/non-privileged flag, the restricted-mode trust
  behavior, target count, purpose/prompt presence, and review posture.
- **Migration / deprecation hints.** Deprecated or replaced manifest fields
  produce actionable replacement guidance — replacement value, removal horizon,
  migration-doc link, and a paste-ready example — instead of a generic
  invalid/unknown error.
- **Version-range targeting.** SDK line/semver, host ABI window, host version
  range, platforms, support class, bridge state, and whether a compatibility
  shim is required.
- **Open-schema / open-docs links** so authors can jump to the schema or the
  migration docs without leaving Aureline.

## Same findings as the validator and conformance kit

The blocker lane is a faithful port of the beta extension validator CLI
([`/tools/extensions/m3/validator_cli/aureline_extension_validator.py`](../../../tools/extensions/m3/validator_cli/aureline_extension_validator.py)).
For any manifest, the editor's `conformance_export` reports the same
`result_class`, `compatibility_badge_class`, `red_flag_classes`, blocker count,
and check ids the CLI and the conformance kit report
([`/schemas/extensions/conformance_kit_report.schema.json`](../../../schemas/extensions/conformance_kit_report.schema.json))
emit. Suites and severities use the shared vocabulary:

| Suite | Meaning |
| --- | --- |
| `manifest_shape` | Identity, schema version, and required-section shape. |
| `compatibility_targets` | SDK line, host ABI window, version range, support/bridge classes. |
| `permission_declarations` | Permission scope, target, purpose, trust-mode, prompt, and review completeness. |
| `lifecycle_metadata` | Activation, degraded path, disable, and rollback metadata. |
| `conformance_fixtures` | Replayable scenario coverage. |
| `editor_advisory` | Editor-only recommended improvements (`advisory.*` ids). |

Editor advisories never collide with validator ids: they live in the
`editor_advisory` suite under the `advisory.*` namespace and never carry blocker
severity, so they cannot turn into hidden publish gates.

## Migration hints come from the lifecycle packet

Migration and deprecation hints are derived from the canonical lifecycle
metadata packet
([`/artifacts/extensions/m3/lifecycle_metadata_packet.json`](../../../artifacts/extensions/m3/lifecycle_metadata_packet.json),
schema [`/schemas/extensions/lifecycle_metadata.schema.json`](../../../schemas/extensions/lifecycle_metadata.schema.json)).
For lifecycle rows whose surface is a `manifest_field:extension_manifest.<path>.<value>`
and whose posture is deprecated or retired, the editor checks whether the
manifest still uses the deprecated value and, if so, emits a hint with the
replacement value, removal target version/date, and migration-guide reference.

For example, a manifest that still sets `lifecycle.state` to `resolved` keeps
passing the `lifecycle_metadata.state_known` blocker (the value is still a known
state) but receives a migration hint to switch to `verified`, anchored at
`/lifecycle/state`, citing
[`/docs/extensions/m3/sdk_versioning_and_deprecation.md`](../../extensions/m3/sdk_versioning_and_deprecation.md).

## Offline and mirror posture

Validation is fully local. The editor never executes extension code and never
performs a network round-trip: `local_validation_only` is always `true` and
`network_round_trip_required` is always `false`. The lifecycle packet is embedded
at build time, so permission reasoning and migration hints stay available under
every `connectivity_class` (`local_offline`, `mirror_reachable`,
`primary_registry_reachable`).

## Publish readiness

| `publish_readiness` | When |
| --- | --- |
| `ready_to_publish` | No blockers and no advisories. |
| `ready_with_advisories` | No blockers, but recommended improvements remain. |
| `blocked_on_must_fix` | One or more must-fix blockers remain. |

Fixtures that exercise each path live in
[`/fixtures/extensions/m3/manifest_editor/`](../../../fixtures/extensions/m3/manifest_editor/)
and are replayed by the crate test suite and the
`dump_manifest_editor_records` example.
