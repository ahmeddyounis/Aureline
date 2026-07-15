# Shared Settings-Governance Consumers: One Registry Across Surfaces

- Packet: `m5-settings-governance-shared-consumers:stable:0001`
- Surface: `M5 settings-governance shared consumers (one registry across surfaces)`
- Consumer bindings: 15 (6 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer bindings

- **Resolve setting (effective value from the winning scope, shadow chain inspectable)** [`sgsc-resolve-setting-resolver`]: family `resolve_setting` on `settings_resolver`, representation `desktop_full`, role `effective_resolution`
- **Resolve setting (effective value from the winning scope, shadow chain inspectable)** [`sgsc-resolve-setting-shell`]: family `resolve_setting` on `shell_ui`, representation `desktop_full`, role `effective_resolution`
- **Resolve setting (effective value from the winning scope, shadow chain inspectable)** [`sgsc-resolve-setting-cli`]: family `resolve_setting` on `cli_export`, representation `exported_redacted`, role `effective_resolution`
- **Write setting (write intent lands in the chosen scope with preview / checkpoint / rollback evidence)** [`sgsc-write-setting-policy`]: family `write_setting` on `policy_service`, representation `desktop_full`, role `write_intent`
- **Write setting (write intent lands in the chosen scope with preview / checkpoint / rollback evidence)** [`sgsc-write-setting-shell`]: family `write_setting` on `shell_ui`, representation `desktop_full`, role `write_intent`
- **Write setting (write intent lands in the chosen scope with preview / checkpoint / rollback evidence)** [`sgsc-write-setting-support`]: family `write_setting` on `support_export`, representation `exported_redacted`, role `write_intent`
- **Sync scope (conflict packet surfaced field-by-field, local authoritative state preserved during an outage)** [`sgsc-sync-scope-sync`]: family `sync_scope` on `sync_service`, representation `desktop_full`, role `sync_conflict`
- **Sync scope (conflict packet surfaced field-by-field, local authoritative state preserved during an outage)** [`sgsc-sync-scope-diagnostics`]: family `sync_scope` on `diagnostics`, representation `desktop_full`, role `sync_conflict`
- **Sync scope (conflict packet surfaced field-by-field, local authoritative state preserved during an outage)** [`sgsc-sync-scope-docs`]: family `sync_scope` on `docs_help`, representation `remote_projected`, role `sync_conflict`
- **Migrate schema (schema-migration record preserves setting-ID continuity with a compare surface)** [`sgsc-migrate-schema-capability`]: family `migrate_schema` on `capability_service`, representation `remote_projected`, role `schema_migration`
- **Migrate schema (schema-migration record preserves setting-ID continuity with a compare surface)** [`sgsc-migrate-schema-diagnostics`]: family `migrate_schema` on `diagnostics`, representation `desktop_full`, role `schema_migration`
- **Migrate schema (schema-migration record preserves setting-ID continuity with a compare surface)** [`sgsc-migrate-schema-sync`]: family `migrate_schema` on `sync_service`, representation `desktop_full`, role `schema_migration`
- **Rollout capability (lifecycle dependency published, kill-switch / policy-disable cause preserved)** [`sgsc-rollout-capability-docs`]: family `rollout_capability` on `docs_help`, representation `desktop_full`, role `capability_lifecycle`
- **Rollout capability (lifecycle dependency published, kill-switch / policy-disable cause preserved)** [`sgsc-rollout-capability-capability`]: family `rollout_capability` on `capability_service`, representation `compact_narrowed`, role `capability_lifecycle`
- **Rollout capability (lifecycle dependency published, kill-switch / policy-disable cause preserved)** [`sgsc-rollout-capability-support`]: family `rollout_capability` on `support_export`, representation `exported_redacted`, role `capability_lifecycle`
