# Aureline M5 service-health communication

4 tiers (0 downgraded, 0 no-live-data), 3 admin notes, 6 consumers — data state `live_verified`; local editing safe.

## Service boundaries

| Boundary | Health | Data | Local-safe | Optional |
|---|---|---|---|---|
| `local_machine` | `operational` | `live_verified` | yes | no |
| `remote_target` | `operational` | `live_verified` | yes | no |
| `enterprise_control_plane` | `operational` | `live_verified` | yes | no |
| `vendor_hosted_service` | `operational` | `live_verified` | yes | yes |

## Admin notes

| Note | Affected tier | Data | Acknowledged | Effective from |
|---|---|---|---|---|
| `channel_change` | `vendor_hosted_service` | `live_verified` | yes | 2026-06-01 |
| `mirror_change` | `vendor_hosted_service` | `live_verified` | yes | 2026-06-01 |
| `deployment_change` | `enterprise_control_plane` | `live_verified` | yes | 2026-06-01 |

## Consumers

- `service_health_panel` → `live_trusted` (governed)
- `help_about` → `live_trusted` (governed)
- `docs_help` → `live_trusted` (governed)
- `support_export` → `live_trusted` (governed)
- `admin_console` → `live_trusted` (governed)
- `release_center` → `live_trusted` (governed)
