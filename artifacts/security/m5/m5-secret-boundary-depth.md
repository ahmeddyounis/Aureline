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
- four deployment-profile parity rows
- allowed credential modes
- projection modes
- consumer identities
- storage classes
- acting identity classes
- bounded projection controls
- per-profile bounded next actions
- trust-store dependencies
- typed repairable trust/certificate/SSH/browser-renewal states
- export posture
- repair owner
- local-safe continuity note

The packet also carries a canonical evidence index:

- `qualification_rows` contains one row/profile qualification per deployment
  profile, for 48 total qualifications across the 12 surface rows.
- `claimed_label` captures the maximum claim a lane wants to make.
- `displayed_label` captures the narrowed claim every consumer must actually
  show.
- `qualification_packet` and `proof_index_ref` name the checked proof packet
  behind each current claim.
- `consumer_projections` carry the same `evidence_index_ref`,
  `qualification_row_refs`, and qualification counts for every downstream
  surface.

The packet-wide summary now proves coverage for:

- `local_desktop`, `ssh_or_container`, `managed_workspace`, and `mirror_offline`
- `local_handle`, `forwarded_local_credential`, `remote_vault_fetch`, `session_only_secret`, `delegated_identity`, and `missing`
- `local_workflow`, `remote_helper`, `registry_client`, `database_connector`, `preview_publisher`, `cluster_connector`, `companion_handoff`, `service_issued_delegate`
- `pause_forwarding`, `stop_using_secret`, and `drop_delegated_identity`
- `missing`, `expired`, `revoked`, `policy_blocked`, `forwarding_paused`, and `remote_vault_unavailable` as first-class states
- `ca_untrusted`, `bundle_stale`, `pin_mismatch`, `rotation_required`, `credential_revoked`, `ssh_host_key_unknown`, `ssh_host_key_mismatch`, `client_certificate_required`, `client_certificate_expired`, `browser_handoff_return_lost`, and `device_code_renewal_required` as typed repairable changes
- `profiles`, `workflow_bundles`, `portable_state_packages`, `recipes`, `support_bundles`, `ai_evidence_packets`, `incident_exports`, and `offboarding_exports` as governed artifact export families
- `raw_tokens`, `private_keys`, `refresh_tokens`, `ambient_delegated_credentials`, and `raw_handle_ids` as omission classes that every governed export family proves explicitly
- export-safe workflow-history rows and durable-activity rows derived from the same credential-lineage event ids used by support export
- `qualified_current`, `limited_local_continuity`, and `support_review_only`
  as the only consumer-visible qualification labels
- `current` and `missing` as currently proven proof-freshness classes in the
  checked packet
- `not_narrowed`, `profile_local_continuity_only`, and `proof_packet_missing`
  as the currently exercised narrow reasons in the checked packet

## Qualification Summary

Current checked counts:

- 48 total qualification rows
- 30 `qualified_current`
- 8 `limited_local_continuity`
- 10 `support_review_only`

Auto-narrowing in the checked packet is intentional:

- `mirror_offline` parity rows narrow to `limited_local_continuity` when the
  profile preserves only local-safe continuity.
- Rows without a current checked M5 proof packet narrow to
  `support_review_only` with `proof_packet_missing`.
- Consumers must render `displayed_label`, not `claimed_label`.

## Artifact-Family Export Rules

Every governed artifact family now carries one packet-backed export rule proving:

- credential aliases, handle classes, source labels, and consumer identity survive export;
- raw tokens, private keys, refresh tokens, ambient delegated credentials, and raw handle ids do not;
- omission markers and export-safety banners are explicit rather than implied; and
- import, restore, replay, rerun, and offboarding flows stop at a typed rebind step instead of pretending the original secret crossed the boundary.

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
- `help_about` must reuse the same `evidence_index_ref`, qualification rows,
  and narrowing results surfaced in docs/help.
- `admin_docs` must reuse the same `evidence_index_ref`, qualification rows,
  and narrowing results surfaced in docs/help.
- `diagnostics` must show row ids, acting identity, repair owner, and the bound repairable state before downstream failure details.
- `support_export` must preserve row ids, qualification rows, consumer
  projections, default modes, projection modes, consumer identities,
  projection controls, repairable states, per-profile parity rows, export
  posture, repair owner, Project Doctor finding codes, workflow-history
  lineage, durable-activity lineage, and support-bundle lineage while excluding
  raw secret values and raw handle ids.
- `release_public_truth` must publish only checked matrix ids, row ids,
  qualification vocabulary, and summary vocabulary.
