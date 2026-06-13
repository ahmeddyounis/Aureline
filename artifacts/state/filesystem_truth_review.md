# Filesystem truth review report

The checked-in packet seeds four review lanes so M5 consumers can reuse one
filesystem-safety vocabulary instead of inventing separate banners or sheets.

## Seeded lanes

| Scenario | Root class | Surface class | Watch mode | Ignore class | Compare outcome | Boundary crossing |
|---|---|---|---|---|---|---|
| `scenario.local.notebook` | `local_filesystem` | `notebook_document` | `reduced_fidelity_watch` | `ignore_hidden` | `external_change_detected` | `container_mount_crossing` |
| `scenario.remote.request` | `remote_agent` | `request_workspace_document` | `polling_fallback` | `scope_limited_results` | `save_conflict` | `remote_authority_change` |
| `scenario.container.preview` | `container_mount` | `preview_output_artifact` | `reduced_fidelity_watch` | `generated_overlay_hidden` | `external_change_detected` | `container_mount_crossing` |
| `scenario.generated.draft` | `generated_managed` | `provider_local_draft` | `provider_refresh_only` | `policy_hidden` | `external_change_detected` | `generated_lineage_detach` |

## Review expectations

- Watch strips always explain what may lag or weaken under the current mode.
- Ignore drawers preserve the difference between hidden, generated, policy,
  and scope-limited absence.
- External-change reviews stay compare-first and keep silent overwrite
  forbidden.
- Cross-root reviews surface metadata and authority consequences before any
  proceed path.
