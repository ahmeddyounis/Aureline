# M5 Update / Support-Lifecycle Governance Matrix

- Packet: `m5-update-lifecycle:stable:0001`
- Label: `M5 update / support-lifecycle governance matrix`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Facets: 8 (8 current, 0 stale, 0 expired, 0 missing)
- Consumers: 8 (8 certified, 0 narrowed, 0 blocked)
- Release gate: pass
- Consumed by: release center, update center, Help/About, docs/help, diagnostics, support, shiproom, companion

## Canonical lifecycle state families

| Family | State | Gate posture | Effective floor |
|--------|-------|--------------|-----------------|
| `update` | `up_to_date` | `governed` | `stable` |
| `update` | `update_offered` | `governed` | `stable` |
| `update` | `update_recommended` | `narrowed` | `beta` |
| `update` | `update_required` | `narrowed` | `beta` |
| `update` | `update_blocked` | `blocked` | `unavailable` |
| `readiness` | `ready_no_restart` | `governed` | `stable` |
| `readiness` | `restart_required` | `governed` | `stable` |
| `readiness` | `rollback_available` | `governed` | `stable` |
| `readiness` | `action_required` | `narrowed` | `beta` |
| `readiness` | `not_ready` | `blocked` | `unavailable` |
| `migration` | `no_migration` | `governed` | `stable` |
| `migration` | `automatic_migration` | `governed` | `stable` |
| `migration` | `assisted_migration` | `narrowed` | `beta` |
| `migration` | `manual_migration` | `narrowed` | `beta` |
| `migration` | `blocking_migration` | `blocked` | `unavailable` |
| `support_window` | `full_support` | `governed` | `stable` |
| `support_window` | `maintenance_support` | `narrowed` | `beta` |
| `support_window` | `security_support` | `narrowed` | `beta` |
| `support_window` | `grace_window` | `narrowed` | `beta` |
| `support_window` | `out_of_support` | `blocked` | `unavailable` |
| `end_of_support` | `supported` | `governed` | `stable` |
| `end_of_support` | `sunset_announced` | `narrowed` | `beta` |
| `end_of_support` | `deprecated` | `narrowed` | `beta` |
| `end_of_support` | `retired` | `blocked` | `unavailable` |
| `end_of_support` | `removed` | `blocked` | `unavailable` |

## Governed facets

| Facet | Dimension | State family | Current state | Channels | Profiles | Stale-data | Owner | Proof | Freshness | Status |
|-------|-----------|--------------|---------------|----------|----------|-----------|-------|-------|-----------|--------|
| `update_availability` | `change_disclosure` | `update` | `up_to_date` | stable beta preview nightly lts | managed self_hosted | `mirrored_labelled` | `release_update_center_owner` | `artifacts/release-proof/m5-update-lifecycle/update-availability.json` | `current` | `mapped` |
| `change_impact` | `change_disclosure` | `readiness` | `restart_required` | stable beta preview nightly | managed self_hosted | `stale_banner_shown` | `release_update_center_owner` | `artifacts/release-proof/m5-update-lifecycle/change-impact.json` | `current` | `mapped` |
| `release_note_evidence` | `change_disclosure` | `readiness` | `ready_no_restart` | stable beta preview | managed self_hosted | `offline_cached` | `release_notes_owner` | `artifacts/release-proof/m5-update-lifecycle/release-note-evidence.json` | `current` | `mapped` |
| `migration_assistant` | `migration_continuity` | `migration` | `automatic_migration` | stable beta lts | managed self_hosted | `local_only_no_live_data` | `migration_continuity_owner` | `artifacts/release-proof/m5-update-lifecycle/migration-assistant.json` | `current` | `mapped` |
| `service_health` | `migration_continuity` | `readiness` | `ready_no_restart` | stable beta preview nightly lts | managed self_hosted | `local_only_no_live_data` | `migration_continuity_owner` | `artifacts/release-proof/m5-update-lifecycle/service-health.json` | `current` | `mapped` |
| `support_window` | `support_lifecycle` | `support_window` | `full_support` | stable lts | managed self_hosted | `mirrored_labelled` | `support_lifecycle_owner` | `artifacts/release-proof/m5-update-lifecycle/support-window.json` | `current` | `mapped` |
| `compatibility_window` | `support_lifecycle` | `support_window` | `full_support` | stable beta lts | managed self_hosted | `offline_cached` | `support_lifecycle_owner` | `artifacts/release-proof/m5-update-lifecycle/compatibility-window.json` | `current` | `mapped` |
| `end_of_support` | `support_lifecycle` | `end_of_support` | `supported` | stable lts | managed self_hosted | `mirrored_labelled` | `support_lifecycle_owner` | `artifacts/release-proof/m5-update-lifecycle/end-of-support.json` | `current` | `mapped` |

## Claimed consumers

| Consumer | Owner | Status | Claim → effective | Gate | Reads | Artifact classes |
|----------|-------|--------|-------------------|------|-------|------------------|
| `release_center` | `release_center_owner` | `mapped` | `stable` → `stable` | `governed` | update_availability change_impact release_note_evidence migration_assistant service_health support_window compatibility_window end_of_support | core_runtime extension_packs schema_contracts workspace_state configuration language_runtimes docs_help_content |
| `update_center` | `update_center_owner` | `mapped` | `stable` → `stable` | `governed` | update_availability change_impact release_note_evidence migration_assistant service_health | core_runtime extension_packs schema_contracts workspace_state configuration language_runtimes docs_help_content |
| `help_about` | `help_about_owner` | `mapped` | `stable` → `stable` | `governed` | update_availability release_note_evidence service_health support_window end_of_support | core_runtime extension_packs language_runtimes docs_help_content |
| `docs_help` | `docs_help_owner` | `mapped` | `stable` → `stable` | `governed` | release_note_evidence migration_assistant support_window compatibility_window end_of_support | core_runtime extension_packs schema_contracts workspace_state configuration docs_help_content |
| `diagnostics` | `diagnostics_owner` | `mapped` | `stable` → `stable` | `governed` | update_availability change_impact service_health compatibility_window | core_runtime extension_packs schema_contracts workspace_state configuration language_runtimes |
| `support_export` | `support_export_owner` | `mapped` | `stable` → `stable` | `governed` | change_impact migration_assistant service_health support_window compatibility_window end_of_support | core_runtime extension_packs schema_contracts workspace_state configuration language_runtimes docs_help_content |
| `shiproom` | `shiproom_owner` | `mapped` | `stable` → `stable` | `governed` | update_availability change_impact release_note_evidence support_window compatibility_window end_of_support | core_runtime extension_packs schema_contracts workspace_state configuration language_runtimes docs_help_content |
| `companion_handoff` | `companion_owner` | `mapped` | `stable` → `stable` | `governed` | update_availability service_health end_of_support | core_runtime extension_packs language_runtimes docs_help_content |
