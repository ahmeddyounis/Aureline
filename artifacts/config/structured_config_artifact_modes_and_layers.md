# Structured config artifact modes and layers

- Record kind: `structured_config_artifact_modes_and_layers`
- Schema: `schemas/config/structured_config_artifact_modes_and_layers.schema.json`
- Docs: `docs/config/structured_config_artifact_modes_and_layers.md`
- Artifact families: 13
- Environment-bearing families: 8

## Shared vocabulary

### Modes

| Mode | Canonical writable | Description |
|---|---|---|
| `source` | `true` | Canonical authored source object. This is the only mode that may be writable as canonical text. |
| `effective` | `false` | Resolved projection after imports, defaults, policy, secrets, and runtime discovery. It is inspect-only unless promoted back through the source object. |
| `live` | `false` | Observed runtime or mirrored target state. It is never treated as canonical writable text and stays labeled when stale, unresolved, or deferred. |

### Surface bindings

| Surface | Header fields | Modes | Layer fields |
|---|---|---|---|
| `desktop_shell` | `6` | `3` | `7` |
| `cli_inspect` | `6` | `3` | `7` |
| `support_export` | `6` | `3` | `7` |
| `docs_help` | `6` | `3` | `7` |

## Artifact families

| Family | Qualification | Active mode | Layer stack |
|---|---|---|---|
| `request_workspace_environment` | `beta` | `effective` | `true` |
| `database_profile` | `stable` | `live` | `true` |
| `api_profile` | `stable` | `effective` | `true` |
| `notebook_runtime_manifest` | `beta` | `effective` | `true` |
| `preview_runtime_config` | `preview` | `live` | `true` |
| `workflow_bundle_manifest` | `beta` | `source` | `true` |
| `ci_environment_descriptor` | `beta` | `live` | `true` |
| `infra_environment_descriptor` | `beta` | `live` | `true` |
| `managed_policy_overlay` | `stable` | `effective` | `false` |
| `admin_policy_bundle_artifact` | `stable` | `source` | `false` |
| `offline_entitlement_snapshot_artifact` | `stable` | `effective` | `false` |
| `emergency_disable_bundle_artifact` | `stable` | `effective` | `false` |
| `trust_root_signer_update_artifact` | `stable` | `source` | `false` |

## Active mode counts

- `source`: 3
- `effective`: 6
- `live`: 4
