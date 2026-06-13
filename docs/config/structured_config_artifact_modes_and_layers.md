# Structured config artifact modes and layers

This document defines the shared contract for config-bearing artifact headers,
source/effective/live mode switches, and environment-layer stacks.

The canonical packet lives in:

- [`crates/aureline-config/src/structured_config_artifact_modes_and_layers/`](../../crates/aureline-config/src/structured_config_artifact_modes_and_layers/)
- [`artifacts/config/structured_config_artifact_modes_and_layers.json`](../../artifacts/config/structured_config_artifact_modes_and_layers.json)
- [`schemas/config/structured_config_artifact_modes_and_layers.schema.json`](../../schemas/config/structured_config_artifact_modes_and_layers.schema.json)
- [`fixtures/config/structured_config_artifact_modes_and_layers/canonical.json`](../../fixtures/config/structured_config_artifact_modes_and_layers/canonical.json)

Related contracts:

- [`docs/config/m4/structured-config-manifest-environment-editor-qualification.md`](./m4/structured-config-manifest-environment-editor-qualification.md)
  freezes the broader source/effective/live and round-trip-risk vocabulary.
- [`docs/config/structured_config_parameter_source_and_round_trip_review.md`](./structured_config_parameter_source_and_round_trip_review.md)
  freezes the per-parameter rows, value chips, compare-before-save sheets, and
  export/support disclosure that build on these headers and mode switches.
- [`docs/config/structured_config_policy_bundle_and_entitlement_matrix.md`](./structured_config_policy_bundle_and_entitlement_matrix.md)
  freezes the family, signed-bundle, and deployment-profile matrix this packet
  deepens.
- [`docs/settings/effective_settings_contract.md`](../settings/effective_settings_contract.md)
  remains the settings-specific effective-value contract; this packet applies to
  non-settings artifacts.

## What this packet adds

The higher-level family matrix says which config-bearing families exist and
whether they have authored, effective, or live truth. This packet adds the
shared presentation contract those families must reuse:

1. **Artifact headers**
   Every row carries:

   - file or artifact identity,
   - artifact class,
   - canonical-source note,
   - target context,
   - schema or validator state, and
   - active mode.

2. **Mode switches**
   Every row carries all three labels:

   - `source`
   - `effective`
   - `live`

   Each mode row must disclose:

   - resolution time or freshness label,
   - target boundary,
   - write eligibility,
   - current/stale/unresolved/deferred/unsupported state, and
   - a short summary.

   Only `source` may be canonical writable text. `effective` and `live` stay
   inspect-only, deferred, unresolved, or signed-bundle review-only.

3. **Environment-layer stacks**
   Environment-bearing families keep their layer stacks visible without leaving
   the IDE. Every layer row carries:

   - explicit order,
   - stable source class,
   - tracked/ignored/derived-read-only state,
   - policy-lock posture,
   - secret-bearing note, and
   - reset/open-source actions.

## Covered families

The packet covers the current config-bearing families:

- request-workspace environments
- database profiles
- API profiles
- notebook runtime manifests
- preview runtime config
- workflow bundle manifests
- CI environment descriptors
- infrastructure environment descriptors
- managed policy overlays
- admin policy bundles
- offline entitlement snapshots
- emergency-disable bundles
- trust-root or signer updates

The eight environment-bearing families carry layer stacks. Signed bundle and
policy review objects reuse the same header and mode vocabulary but do not
pretend to be ordinary environment stacks.

## Surface parity

The packet requires the same vocabulary on these consumers:

- desktop shell
- CLI inspect
- support export
- docs/help

Each surface binding must expose the full shared header fields, all three mode
labels, and the complete layer vocabulary.

## Validation rules

The packet is non-conforming when:

- a family row is missing or duplicated;
- a row omits any required header disclosure;
- the header active mode does not match the active switch row;
- a row omits `source`, `effective`, or `live`;
- any non-source mode is marked as canonical writable source;
- an environment-bearing family omits its layer stack;
- a layer stack hides the winning layer or omits reset/open-source metadata; or
- a consumer surface fails to reuse the full shared vocabulary.

## Regeneration

Regenerate the canonical packet and review markdown with:

```sh
cargo run -q -p aureline-config --bin aureline_config_structured_artifact_modes_and_layers -- json
cargo run -q -p aureline-config --bin aureline_config_structured_artifact_modes_and_layers -- markdown
```
