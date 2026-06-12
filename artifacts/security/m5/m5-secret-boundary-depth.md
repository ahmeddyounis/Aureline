# M5 Secret Boundary Depth Matrix

- Packet id: `m5-secret-boundary-depth:2026-06-12`
- Matrix id: `m5.secret_boundary.depth.v1`
- Contract ref: `security:m5_secret_boundary_depth:v1`
- Schema: `schemas/security/m5-secret-boundary-depth.schema.json`
- Shared vocabulary: `docs/security/m5/m5-secret-boundary-depth.md#shared-vocabulary`

## Coverage

The canonical matrix covers 12 credential-bearing surfaces across:

- request workspaces
- database tooling
- provider/model lanes
- registries
- preview routes
- infrastructure connectors
- companion handoff
- managed surfaces

Every row carries:

- explicit `matrix_row_id`
- allowed credential modes
- projection modes
- storage classes
- acting identity classes
- trust-store dependencies
- export posture
- repair owner
- local-safe continuity note

## Row Summary

| Row id | Domain | Default mode | Repair owner | Export posture |
| --- | --- | --- | --- | --- |
| `m5.secret.request_workspace.send_http` | `request_workspaces` | `handle_only` | `user` | `redacted_support_export` |
| `m5.secret.request_workspace.history_replay` | `request_workspaces` | `handle_only` | `user` | `metadata_only` |
| `m5.secret.database.connection_picker` | `database_tooling` | `handle_only` | `data_operator` | `alias_only` |
| `m5.secret.database.query_history_portability` | `database_tooling` | `handle_only` | `data_operator` | `metadata_only` |
| `m5.secret.provider_model.route_resolution` | `provider_model_lanes` | `delegated` | `provider_operator` | `redacted_support_export` |
| `m5.secret.provider_model.scope_registry` | `provider_model_lanes` | `delegated` | `provider_operator` | `redacted_support_export` |
| `m5.secret.registry.package_auth` | `registries` | `handle_only` | `user` | `alias_only` |
| `m5.secret.preview_route.remote_preview` | `preview_routes` | `delegated` | `remote_operator` | `release_summary_only` |
| `m5.secret.infra_connector.target_context` | `infra_connectors` | `handle_only` | `remote_operator` | `redacted_support_export` |
| `m5.secret.companion.session_handoff` | `companion_handoff` | `browser_handoff` | `user` | `metadata_only` |
| `m5.secret.managed.workspace_runtime` | `managed_surfaces` | `remote_vault_fetch` | `remote_operator` | `redacted_support_export` |
| `m5.secret.managed.sync_plane` | `managed_surfaces` | `delegated` | `service_operator` | `release_summary_only` |

## Consumer Projections

- `docs_help` must quote the checked `matrix_id`, row ids, and shared vocabulary.
- `diagnostics` must show row ids, acting identity, and repair owner before downstream failure details.
- `support_export` must preserve row ids, default modes, export posture, and repair owner while excluding raw secret values and raw handle ids.
- `release_public_truth` must publish only checked matrix ids, row ids, and summary vocabulary.
