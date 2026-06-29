# Aureline M5 update-center summary

Channel `stable` — 6 families, 10 delta rows, 3 consumers.

## Update summary entries

| Family | Current | Target | Posture | Verification | Restart | Rollback | Data | Gate |
|---|---|---|---|---|---|---|---|---|
| `desktop_app` | 1.8.0 | 1.8.0 | `applied` | `verified` | `restart_app` | `rollback_supported` | `live` | `governed` |
| `extension` | 3.2.1 | 3.4.0 | `staged` | `verified` | `reload_window` | `side_by_side_fallback` | `mirrored` | `governed` |
| `docs_pack` | 2025.6 | 2025.7 | `downloaded` | `verified` | `none` | `rollback_supported` | `offline` | `governed` |
| `policy_bundle` | 2.0.0 | 2.1.0 | `staged` | `verified` | `reload_window` | `rollback_supported` | `live` | `governed` |
| `framework_pack` | 0.9.0 | 0.10.0 | `downloaded` | `verified` | `restart_app` | `reinstall_only` | `live` | `governed` |
| `runtime_toolchain` | 1.84.0 | 1.85.0 | `downloaded` | `verified` | `restart_app` | `reinstall_only` | `live` | `governed` |

## Consumers

- `release_center` → `stable` (governed)
- `update_center` → `stable` (governed)
- `help_about` → `stable` (governed)
