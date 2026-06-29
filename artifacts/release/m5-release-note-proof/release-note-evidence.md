# Aureline M5 release-note evidence

Release `1.8.0` → `1.9.0` — 9 notes (4 action-required, 3 action-recommended), 6 consumers.

## Release notes

| Note | Change class | Readiness | Evidence | Direct link | What's-new | Scope |
|---|---|---|---|---|---|---|
| `breaking_extension_api` | `breaking` | `action_required` | yes | evidence_packet, migration_doc, rollback_control, setting_surface | `active` | extension_packs, schema_contracts |
| `security_dependency_advisory` | `security` | `action_required` | yes | security_advisory, rollback_control | `active` | core_runtime |
| `admin_policy_bundle` | `admin_action_required` | `action_required` | yes | evidence_packet, setting_surface | `active` | configuration |
| `migration_workspace_schema` | `migration_required` | `action_required` | yes | migration_doc, rollback_control, import_surface | `active` | schema_contracts, workspace_state |
| `deprecated_legacy_command` | `deprecated` | `action_recommended` | yes |  | `active` | core_runtime |
| `policy_telemetry_consent` | `policy` | `action_recommended` | yes |  | `active` | configuration |
| `behavioral_default_layout` | `behavioral` | `action_recommended` | yes | evidence_packet, setting_surface | `active` | core_runtime, configuration |
| `compatibility_window_shift` | `compatibility` | `informational` | yes |  | `active` | schema_contracts |
| `docs_only_quickstart` | `docs_only` | `informational` | no |  | `active` | docs_help_content |

## Consumers

- `update_center` → `action_required` (blocked; gap: admin_policy_bundle:action_required, behavioral_default_layout:action_recommended, breaking_extension_api:action_required, deprecated_legacy_command:action_recommended, migration_workspace_schema:action_required, policy_telemetry_consent:action_recommended, security_dependency_advisory:action_required)
- `whats_new_panel` → `action_required` (blocked; gap: admin_policy_bundle:action_required, behavioral_default_layout:action_recommended, breaking_extension_api:action_required, deprecated_legacy_command:action_recommended, migration_workspace_schema:action_required, policy_telemetry_consent:action_recommended, security_dependency_advisory:action_required)
- `help_center` → `action_required` (blocked; gap: admin_policy_bundle:action_required, behavioral_default_layout:action_recommended, breaking_extension_api:action_required, deprecated_legacy_command:action_recommended, migration_workspace_schema:action_required, policy_telemetry_consent:action_recommended, security_dependency_advisory:action_required)
- `docs_help` → `action_required` (blocked; gap: admin_policy_bundle:action_required, behavioral_default_layout:action_recommended, breaking_extension_api:action_required, deprecated_legacy_command:action_recommended, migration_workspace_schema:action_required, policy_telemetry_consent:action_recommended, security_dependency_advisory:action_required)
- `release_center` → `action_required` (blocked; gap: admin_policy_bundle:action_required, behavioral_default_layout:action_recommended, breaking_extension_api:action_required, deprecated_legacy_command:action_recommended, migration_workspace_schema:action_required, policy_telemetry_consent:action_recommended, security_dependency_advisory:action_required)
- `support_export` → `action_required` (blocked; gap: admin_policy_bundle:action_required, behavioral_default_layout:action_recommended, breaking_extension_api:action_required, deprecated_legacy_command:action_recommended, migration_workspace_schema:action_required, policy_telemetry_consent:action_recommended, security_dependency_advisory:action_required)
