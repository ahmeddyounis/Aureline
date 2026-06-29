# Aureline M5 change-impact cards

Update `1.8.0` → `1.9.0` on channel `stable` — 10 cards (0 review, 0 hold), 6 consumers.

## Change-impact cards

| Dimension | Risk | Confidence | Readiness | Follow-up | Rollback / pin | Scope |
|---|---|---|---|---|---|---|
| `workspace_migration` | `no_impact` | `confirmed` | `clear_to_apply` | `no_task_required` | `not_applicable` | workspace_state |
| `profile_migration` | `no_impact` | `confirmed` | `clear_to_apply` | `no_task_required` | `not_applicable` | configuration |
| `schema_migration` | `no_impact` | `confirmed` | `clear_to_apply` | `no_task_required` | `not_applicable` | schema_contracts |
| `cache_migration` | `low_risk_cache_churn` | `confirmed` | `clear_to_apply` | `cache_rebuild` | `not_applicable` | core_runtime |
| `extension_compatibility` | `no_impact` | `confirmed` | `clear_to_apply` | `no_task_required` | `not_applicable` | extension_packs |
| `remote_helper_skew` | `no_impact` | `not_applicable` | `clear_to_apply` | `no_task_required` | `not_applicable` | core_runtime |
| `toolchain_floor` | `no_impact` | `confirmed` | `clear_to_apply` | `no_task_required` | `not_applicable` | language_runtimes |
| `toolchain_ceiling` | `no_impact` | `confirmed` | `clear_to_apply` | `no_task_required` | `not_applicable` | language_runtimes |
| `certified_archetype` | `no_impact` | `confirmed` | `clear_to_apply` | `no_task_required` | `not_applicable` | workspace_state |
| `behavior_change` | `no_impact` | `confirmed` | `clear_to_apply` | `no_task_required` | `not_applicable` | core_runtime |

## Consumers

- `update_center` → `clear_to_apply` (governed)
- `migration_assistant` → `clear_to_apply` (governed)
- `release_center` → `clear_to_apply` (governed)
- `team_lead_review` → `clear_to_apply` (governed)
- `admin_console` → `clear_to_apply` (governed)
- `support_export` → `clear_to_apply` (governed)
