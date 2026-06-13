# Structured config parameter-source and round-trip review

- Record kind: `structured_config_parameter_source_and_round_trip_review`
- Schema: `schemas/config/structured_config_parameter_source_and_round_trip_review.schema.json`
- Docs: `docs/config/structured_config_parameter_source_and_round_trip_review.md`
- Artifact families: 9
- Compare-before-save families: 5
- Raw secret export blocked everywhere: `true`

## Value chip classes

| Class | Default secret export blocked |
|---|---|
| `literal_value` | `false` |
| `env_reference` | `true` |
| `secret_handle` | `true` |
| `policy_injected` | `true` |
| `runtime_discovered` | `true` |

## Artifact families

| Family | Qualification | Compare-before-save | Export disclosures |
|---|---|---|---|
| `request_workspace_environment` | `stable` | `false` | `literal_value, reference_handle, key_path_metadata_only` |
| `database_profile` | `stable` | `false` | `literal_value, reference_handle, redacted_placeholder` |
| `api_profile` | `stable` | `false` | `literal_value, reference_handle, redacted_placeholder, key_path_metadata_only` |
| `notebook_runtime_manifest` | `beta` | `true` | `literal_value, reference_handle, redacted_placeholder` |
| `preview_runtime_config` | `preview` | `true` | `literal_value, redacted_placeholder, key_path_metadata_only` |
| `workflow_bundle_manifest` | `beta` | `true` | `literal_value, reference_handle, redacted_placeholder` |
| `ci_environment_descriptor` | `stable` | `true` | `literal_value, reference_handle, redacted_placeholder` |
| `infra_environment_descriptor` | `beta` | `true` | `reference_handle, key_path_metadata_only` |
| `managed_policy_overlay` | `beta` | `false` | `redacted_placeholder, key_path_metadata_only` |

## Coverage

- Value chip classes: `literal_value`, `env_reference`, `secret_handle`, `policy_injected`, `runtime_discovered`
- Output disclosure classes: `literal_value`, `reference_handle`, `redacted_placeholder`, `key_path_metadata_only`
