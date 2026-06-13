# Structured config, policy, and entitlement certification

- Record kind: `structured_config_policy_entitlement_certification`
- Schema: `schemas/config/structured_config_policy_entitlement_certification.schema.json`
- Config doc: `docs/config/structured_config_policy_entitlement_certification.md`
- Help doc: `docs/help/structured_config_policy_entitlement_certification.md`
- As of: `2026-06-12T18:00:00Z`

## Artifact families

| Family | Ceiling | Published state | Evidence age (days) |
|---|---|---|---|
| `request_workspace_environment` | `beta` | `limited` | `2` |
| `database_profile` | `stable` | `certified` | `0` |
| `api_profile` | `stable` | `certified` | `0` |
| `notebook_runtime_manifest` | `beta` | `limited` | `2` |
| `preview_runtime_config` | `preview` | `retest_pending` | `3` |
| `workflow_bundle_manifest` | `beta` | `limited` | `2` |
| `ci_environment_descriptor` | `beta` | `limited` | `2` |
| `infra_environment_descriptor` | `beta` | `limited` | `2` |
| `managed_policy_overlay` | `stable` | `certified` | `0` |
| `admin_policy_bundle_artifact` | `stable` | `certified` | `0` |
| `offline_entitlement_snapshot_artifact` | `stable` | `certified` | `0` |
| `emergency_disable_bundle_artifact` | `stable` | `certified` | `0` |
| `trust_root_signer_update_artifact` | `stable` | `certified` | `0` |

## Profiles

| Profile | Published state | Local-safe floor | Evidence age (days) |
|---|---|---|---|
| `local_only` | `certified` | `continue_local_only` | `0` |
| `managed` | `certified` | `continue_with_last_known_good` | `0` |
| `self_hosted` | `certified` | `continue_with_last_known_good` | `0` |
| `mirrored` | `certified` | `continue_with_mirror_snapshot` | `0` |
| `fully_air_gapped` | `certified` | `continue_with_offline_snapshot` | `1` |

## Publication surfaces

- `release_center` ingests `artifacts/config/structured_config_policy_entitlement_certification.json`
- `help_about` ingests `artifacts/config/structured_config_policy_entitlement_certification.json`
- `support_export` ingests `artifacts/config/structured_config_policy_entitlement_certification.json`
- `docs_help` ingests `artifacts/config/structured_config_policy_entitlement_certification.json`
- `shiproom` ingests `artifacts/config/structured_config_policy_entitlement_certification.json`

## Summary

- Certified artifact families: 7
- Narrowed artifact families: 6
- Certified profiles: 5
