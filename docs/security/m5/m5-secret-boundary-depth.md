# M5 Secret Boundary Depth

This document is the contract companion to `artifacts/security/m5/m5-secret-boundary-depth.json`.

- Packet id: `m5-secret-boundary-depth:2026-06-12`
- Matrix id: `m5.secret_boundary.depth.v1`
- Schema ref: `schemas/security/m5-secret-boundary-depth.schema.json`
- Shared contract ref: `security:m5_secret_boundary_depth:v1`

## Canonical Evidence Index

The packet carries a canonical secret-boundary evidence index in
`qualification_rows`.

- Every `surface_row` MUST project one qualification row per deployment
  profile.
- `qualification_row_id` is the stable row/profile key every consumer reuses.
- `claimed_label` records the maximum claim the lane wants to make.
- `displayed_label` records the narrowed claim consumers may actually show.
- `qualification_packet`, when present, names the checked packet or checked
  support-export artifact that proves the claim.
- `evidence_index_ref` and `proof_index_ref` are the canonical refs reused by
  docs, Help/About, admin docs, diagnostics, support export, and
  release/public-truth.

Qualification labels:

- `qualified_current`
- `limited_local_continuity`
- `support_review_only`

Proof freshness:

- `current`
- `stale`
- `missing`

Qualification narrow reasons:

- `not_narrowed`
- `profile_local_continuity_only`
- `proof_packet_missing`
- `proof_packet_stale`
- `handle_classes_unproven`
- `vault_picker_unproven`
- `delegated_identity_unproven`
- `projection_pause_unproven`
- `trust_store_drift_unproven`
- `rotation_revoke_lineage_unproven`
- `export_redaction_unproven`
- `repair_flow_unproven`

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

Consumer identity classes:

- `local_workflow`
- `remote_helper`
- `registry_client`
- `database_connector`
- `preview_publisher`
- `cluster_connector`
- `companion_handoff`
- `service_issued_delegate`

Projection controls:

- `pause_forwarding`
- `stop_using_secret`
- `drop_delegated_identity`

Deployment profiles:

- `local_desktop`
- `ssh_or_container`
- `managed_workspace`
- `mirror_offline`

Projection parity classes:

- `local_handle`
- `forwarded_local_credential`
- `remote_vault_fetch`
- `session_only_secret`
- `delegated_identity`
- `missing`

Credential health and degraded states:

- `healthy`
- `expiring_soon`
- `expired`
- `revoked`
- `unavailable`
- `policy_blocked`
- `forwarding_paused`
- `remote_vault_unavailable`
- `missing`
- `not_configured`

Repairable change classes:

- `ca_untrusted`
- `bundle_stale`
- `pin_mismatch`
- `rotation_required`
- `credential_revoked`
- `ssh_host_key_unknown`
- `ssh_host_key_mismatch`
- `client_certificate_required`
- `client_certificate_expired`
- `browser_handoff_return_lost`
- `device_code_renewal_required`

Last-known-good classes:

- `os_trust_store_descriptor`
- `org_ca_bundle_epoch`
- `pinned_control_plane_root`
- `ssh_host_proof`
- `client_certificate_binding`
- `device_code_session`
- `browser_handoff_session`
- `remote_vault_lineage`
- `delegated_scope_binding`

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

- Every surface must preserve the same `local_desktop`, `ssh_or_container`, `managed_workspace`, and `mirror_offline` parity rows instead of replacing degraded states with generic connector failures.
- Every consumer surface MUST reuse the packet's `qualification_rows` and
  `evidence_index_ref` rather than minting a local claim table.
- A consumer MAY display only `displayed_label`; it MUST NOT widen a row back
  to `qualified_current` when proof freshness or required coverage is missing.
- `limited_local_continuity` is reserved for parity rows that preserve local
  metadata continuity without a live credential claim.
- `support_review_only` is reserved for rows that remain visible for review,
  diagnostics, or support while current proof or required handle/vault/delegate
  coverage is incomplete.
- The packet MUST expose first-class `missing`, `expired`, `revoked`, `policy_blocked`, `forwarding_paused`, and `remote_vault_unavailable` states with bounded next actions.
- Every surface row MUST carry at least one typed repairable state naming the exact affected target, the last-known-good class, the minimally destructive next action, the Project Doctor finding code, and the support-bundle lineage ref.
- Consumer-identity receipts and projection-mode audit rows MUST preserve actor class, consumer identity, issuer label, target boundary, projection mode, and result without carrying raw secret values or raw handle ids.
- Remote, managed, registry, preview, connector, and companion lanes MUST expose bounded `pause_forwarding`, `stop_using_secret`, or `drop_delegated_identity` controls when the authority can outlive the original prompt.
- Docs/help, Help/About, and admin docs must project the same `matrix_id`,
  `qualification_row_id`, row ids, and shared vocabulary rather than restating
  a generic connected state.
- Diagnostics must name the row id, acting identity, trust dependency, repair owner, and typed repairable state before surfacing the downstream error.
- Support export must preserve row ids, qualification rows, consumer
  projections, consumer identities, projection modes, projection controls,
  repairable states, workflow-history lineage, durable-activity lineage,
  Project Doctor finding codes, and support-bundle lineage while excluding raw
  secret values and raw handle ids.
- Workflow history and durable activity must project the same export-safe event ids, impacted workflows, and next safe repair or rebind action for rotation, revoke, rebind, and policy-denied projection outcomes.
- Release/public-truth surfaces may publish only checked row ids, qualification
  labels, narrow reasons, and summary vocabulary; they may not widen a row with
  custom prose.

## Artifact Export Rules

The packet now carries one export rule per artifact family so export, import,
restore, replay, rerun, and offboarding surfaces do not invent credential copy
behavior locally.

Covered families:

- `profiles`
- `workflow_bundles`
- `portable_state_packages`
- `recipes`
- `support_bundles`
- `ai_evidence_packets`
- `incident_exports`
- `offboarding_exports`

Each rule MUST preserve:

- credential aliases
- handle classes
- source labels
- consumer identity or acting-identity class

Each rule MUST omit by default:

- `raw_tokens`
- `private_keys`
- `refresh_tokens`
- `ambient_delegated_credentials`
- `raw_handle_ids`

Each rule MUST expose:

- an omission marker label
- an export-safety banner
- a typed rebind action label
- a typed rebind failure label explaining why raw material was not reused

Import/replay rules:

- profile, workflow-bundle, portable-state, recipe, incident, and offboarding imports MUST stop at a typed rebind step rather than claiming the original secret crossed the boundary.
- support-bundle and AI-evidence replay MUST preserve omission markers and require a fresh reviewed bind before any credentialed action resumes.
