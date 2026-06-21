# `validate_locale_pack`

Contribution-time validator for first-party, community, and extension-owned
locale packs. It checks an on-disk pack against the same stable-id,
compatibility, coverage, and host-stable-label rules every Aureline locale pack
must satisfy, so a contributor catches an incompatible pack or a forbidden label
replacement before it reaches runtime.

It is standard-library only and deterministic: no workspace build, no third-party
dependencies, stable finding order.

## Usage

```sh
# Validate a pack you are authoring.
python3 -m tools.i18n.validate_locale_pack path/to/my-locale-pack

# Validate the terminology governance glossary itself.
python3 -m tools.i18n.validate_locale_pack --check-glossary

# Write a machine-readable report.
python3 -m tools.i18n.validate_locale_pack path/to/pack --report out/report.json
```

Exit code is `0` when there are no errors (warnings are allowed) and `1` on any
error.

Sources of truth (auto-detected from the repo root; override with `--registry` /
`--glossary`):

- stable message-id registry — `fixtures/i18n/message-id-stability/registry.json`
- terminology glossary — `fixtures/i18n/locale-pack-contribution/terminology_glossary.json`

## Finding Codes

Errors block; warnings are disclosed-but-allowed postures.

| Code | Severity | Meaning |
| --- | --- | --- |
| `manifest.missing` / `manifest.invalid_json` | error | The manifest is absent or not valid JSON. |
| `manifest.record_kind` / `manifest.schema_version` | error | The manifest is not the expected record. |
| `manifest.field_missing` | error | A required identity field is missing. |
| `manifest.bad_owner_class` | error | `owner_class` is not a recognized class. |
| `manifest.override_host_stable_labels` | error | `may_override_host_stable_labels` is true. |
| `manifest.owns_policy_recovery_text` | error | A contributed pack claims policy/legal/recovery text. |
| `manifest.extension_namespace_required` | error | An extension/companion pack declared no `owned_namespace_prefix`. |
| `manifest.namespace_collides_host` | error | The owned namespace collides with a reserved host prefix. |
| `manifest.fallback_chain_bad` | error | The fallback chain does not run pack-locale → … → source language. |
| `manifest.fallback_not_disclosed` | error | `discloses_source_language_fallback` is not true. |
| `manifest.compat_range_missing` / `…_malformed` / `…_inverted` | error | The compatibility build range is absent, incomplete, or min > max. |
| `strings.missing_file` / `strings.invalid_json` | error | A declared strings file is absent or not valid JSON. |
| `strings.empty_value` | error | A message maps to an empty string. |
| `strings.id_carries_locale_tag` | error | A message id embeds a locale tag. |
| `strings.unknown_host_id` | error | A host-translating pack uses an id not in the registry. |
| `strings.id_outside_host_namespace` | error | A host-translating pack key is not a host `msg:` id. |
| `strings.contributed_owns_host_id` | error | A contributed pack redefines a host id. |
| `strings.id_outside_owned_namespace` | error | A contributed pack key is outside its owned prefix. |
| `strings.host_stable_namespace_replacement` | error | A key sits under a reserved host namespace. |
| `strings.forbidden_term_replacement` | error | A key replaces a host-stable-locked governed term. |
| `coverage.incomplete_but_claimed_complete` | error | Complete coverage claimed while host ids are untranslated. |
| `coverage.missing_keys` | warning | Owned-surface host ids are untranslated and will fall back. |
| `glossary.missing_file` / `glossary.invalid_json` / `glossary.empty_value` | error | The pack glossary is absent, not valid JSON, or has an empty value. |
| `glossary.unknown_term` | error | A glossary entry names a term not in the governance glossary. |
| `glossary.translates_host_stable_locked` | error | A glossary entry localizes a host-stable-locked term. |

The `glossary.*` codes prefixed with the glossary's own structural checks (run via
`--check-glossary`) validate the governance glossary itself.

## Gate

[`/tools/check_locale_pack_contribution.py`](../../check_locale_pack_contribution.py)
runs this validator over the governance glossary, every shipped template (must
pass), and every rejected fixture (must be rejected with its expected codes).
