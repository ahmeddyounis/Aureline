# M5 Setting-Definition, Write-Intent, Sync-Conflict, and Capability-Lifecycle Matrix

- Packet: `m5-settings-governance:stable:0001`
- Label: `M5 setting-definition, write-intent, sync-conflict, and capability-lifecycle matrix`
- Settings-governance families: 5 (5 stable)
- Settings-governance roles: setting_definition, effective_resolution, write_intent, policy_constraint, sync_conflict, schema_migration, capability_lifecycle
- Resolve-setting roles: effective_value_resolved_from_winning_scope, shadowed_values_and_scope_chain_inspectable, restart_posture_and_lock_source_disclosed, stable_setting_id_preserved_never_recycled, bound_to_settings_governance_registry, recycled_retired_setting_id_disallowed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Settings-governance families

- **resolve_setting**: `stable`
  - Owner: Settings-resolver owner
  - Canonical schema: `schemas/config/m5-setting-definition.schema.json`
  - Scope: One resolve-setting profile naming the effective value resolved from the winning scope, the shadowed values and scope chain kept inspectable, the restart posture and lock source disclosed, and the stable setting ID preserved so resolving a setting stays inspectable and never recycles a retired setting ID
  - Required labels: identity, semantic_role, registry_reference, winning_scope
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **write_setting**: `stable`
  - Owner: Settings-write owner
  - Canonical schema: `schemas/config/m5-setting-write-intent.schema.json`
  - Scope: One write-setting profile naming the write intent targeting the chosen artifact and scope, the preview / checkpoint / rollback evidence created, the material behavior change disclosed before apply, and the chosen scope preserved so a write lands only in the chosen artifact and scope and never widens a scoped write into a broader scope because it is easier downstream
  - Required labels: identity, semantic_role, registry_reference, write_intent
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **sync_scope**: `stable`
  - Owner: Sync-service owner
  - Canonical schema: `schemas/config/m5-sync-conflict-packet.schema.json`
  - Scope: One sync-scope profile naming the sync scope bundle and session resolved, the conflict packet surfaced rather than auto-overwritten, the local authoritative state preserved during an outage, and machine-only state never marked portable so syncing a scope bundle never silently overwrites local authoritative state and never lets machine-only state masquerade as portable
  - Required labels: identity, semantic_role, registry_reference, winning_scope
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **migrate_schema**: `stable`
  - Owner: Migration-service owner
  - Canonical schema: `schemas/config/m5-setting-definition.schema.json`
  - Scope: One migrate-schema profile naming the schema-migration record resolved, the setting-ID continuity preserved across versions, the migration preview shown before rewrite, and the reversible migration checkpoint recorded so migrating a settings schema preserves setting-ID continuity and never silently rewrites a schema without a checkpoint
  - Required labels: identity, semantic_role, registry_reference, winning_scope
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **rollout_capability**: `stable`
  - Owner: Capability-lifecycle owner
  - Canonical schema: `schemas/config/m5-capability-lifecycle.schema.json`
  - Scope: One rollout-capability profile naming the capability lifecycle state resolved, the Labs and rollout dependency markers published, the kill-switch and policy-disable cause explained, and the disabled state preserving user data so a capability rollout keeps lifecycle and experiment dependencies visible and never hides a kill-switch or policy-disable cause behind generic unavailable copy
  - Required labels: identity, semantic_role, registry_reference, lifecycle_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
