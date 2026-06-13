# Structured config, policy bundle, and offline entitlement matrix

- Record kind: `config_structured_config_policy_bundle_and_entitlement_matrix`
- Schema: `schemas/config/structured_config_policy_bundle_and_entitlement_matrix.schema.json`
- Docs: `docs/config/structured_config_policy_bundle_and_entitlement_matrix.md`
- Artifact families: 13
- Bundle classes: 4
- Profiles: 5

## Artifact families

| Family | Qualification | Authored | Effective | Live state |
|---|---|---|---|---|
| `request_workspace_environment` | `beta` | `true` | `true` | `deferred_by_runtime` |
| `database_profile` | `stable` | `true` | `true` | `inspect_only_observed` |
| `api_profile` | `stable` | `true` | `true` | `inspect_only_observed` |
| `notebook_runtime_manifest` | `beta` | `true` | `true` | `inspect_only_observed` |
| `preview_runtime_config` | `preview` | `true` | `true` | `inspect_only_observed` |
| `workflow_bundle_manifest` | `beta` | `true` | `true` | `deferred_by_runtime` |
| `ci_environment_descriptor` | `beta` | `true` | `true` | `inspect_only_mirrored` |
| `infra_environment_descriptor` | `beta` | `true` | `true` | `inspect_only_mirrored` |
| `managed_policy_overlay` | `stable` | `true` | `true` | `not_supported` |
| `admin_policy_bundle_artifact` | `stable` | `true` | `true` | `not_supported` |
| `offline_entitlement_snapshot_artifact` | `stable` | `true` | `true` | `not_supported` |
| `emergency_disable_bundle_artifact` | `stable` | `true` | `true` | `not_supported` |
| `trust_root_signer_update_artifact` | `stable` | `true` | `true` | `not_supported` |

## Bundle taxonomy

| Bundle class | Supersedes | Revokes | Distribution paths |
|---|---|---|---|
| `admin_policy_bundle` | `true` | `false` | `5` |
| `offline_entitlement_snapshot` | `true` | `true` | `5` |
| `emergency_disable_bundle` | `true` | `true` | `5` |
| `trust_root_signer_update` | `true` | `true` | `5` |

## Deployment profiles

| Profile | Qualification | Local-safe posture | Authoritative live observation |
|---|---|---|---|
| `local_only` | `stable` | `continue_local_only` | `false` |
| `managed` | `stable` | `continue_with_last_known_good` | `true` |
| `self_hosted` | `stable` | `continue_with_last_known_good` | `true` |
| `mirrored` | `stable` | `continue_with_mirror_snapshot` | `false` |
| `fully_air_gapped` | `stable` | `continue_with_offline_snapshot` | `false` |
