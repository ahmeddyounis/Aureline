# M5 Secret Boundary Depth

This document is the contract companion to `artifacts/security/m5/m5-secret-boundary-depth.json`.

- Packet id: `m5-secret-boundary-depth:2026-06-12`
- Matrix id: `m5.secret_boundary.depth.v1`
- Schema ref: `schemas/security/m5-secret-boundary-depth.schema.json`
- Shared contract ref: `security:m5_secret_boundary_depth:v1`

## Shared Vocabulary

Credential modes:

- `os_store`
- `enterprise_vault`
- `session_only`
- `handle_only`
- `delegated`
- `device_code`
- `browser_handoff`
- `remote_vault_fetch`
- `not_configured`

Acting identity classes:

- `human_account`
- `installation_app_grant`
- `delegated_credential`
- `forwarded_local_credential`
- `local_only_handle`
- `service_issued_authority`

Trust-store dependencies:

- `os_store`
- `org_ca_bundle`
- `pinned_control_plane`
- `known_hosts`
- `vault_ref`

Export postures:

- `metadata_only`
- `alias_only`
- `redacted_support_export`
- `release_summary_only`

Repair owners:

- `user`
- `admin`
- `provider_operator`
- `remote_operator`
- `data_operator`
- `service_operator`

## Consumer Rules

- Docs/help must project the same `matrix_id` and row ids rather than restating a generic connected state.
- Diagnostics must name the row id, acting identity, trust dependency, and repair owner before surfacing the downstream error.
- Support export must preserve row ids and shared vocabulary while excluding raw secret values and raw handle ids.
- Release/public-truth surfaces may publish only checked row ids and summary vocabulary; they may not widen a row with custom prose.
