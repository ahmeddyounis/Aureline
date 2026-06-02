# Hardened Identity and Admin Support-Export Parity — Proof Packet

- Packet: `policy:harden-identity-admin:seeded:0001`
- Schema version: `1`
- Contract ref: `policy:harden_identity_admin_support_export_parity:v1`
- Qualification: `stable` (derived, not asserted)
- Identity-admin defects: 0
- Withdrawn rows: 0
- Stable rows: all 5

## Lane coverage

| Row | Row class | Provisioning class | Sync freshness | Local vs tenant scope | Fallback path | Failure kind | Action lineage |
|---|---|---|---|---|---|---|---|
| `harden-identity-admin:directory_provider_card` | `directory_provider_card` | `oidc` | `live` | `hybrid_local_tenant` | Manual local account; SCIM fallback via signed file import | — | yes |
| `harden-identity-admin:user_seat_lifecycle` | `user_seat_lifecycle` | `scim` | `cached` | `hybrid_local_tenant` | Manual user creation; seat transfer via admin-signed request file | — | yes |
| `harden-identity-admin:provisioning_failure_log` | `provisioning_failure_log` | `scim` | `stale` | `tenant_scoped` | Fallback to last-known-good signed snapshot; manual admin review | `provider_outage` | yes |
| `harden-identity-admin:policy_target_dry_run` | `policy_target_dry_run` | `signed_file_bundle` | `live` | `hybrid_local_tenant` | Manual policy bundle import via admin console | — | yes |
| `harden-identity-admin:local_governance_path` | `local_governance_path` | `manual` | `live` | `local_state_only` | No external dependency; local policy engine enforces governance | — | yes |

## Key invariants verified

1. All five required row classes (`directory_provider_card`, `user_seat_lifecycle`, `provisioning_failure_log`, `policy_target_dry_run`, `local_governance_path`) have rows.
2. Every row carries `raw_secret_or_private_material_excluded: true`; no credential bodies, tenant raw data, user emails, or private keys cross this boundary.
3. Every row requiring a provisioning class carries a non-empty token from the closed vocabulary (`oidc`, `scim`, `signed_file_bundle`, `manual`).
4. Every row carries a non-empty `sync_freshness_token` from the closed vocabulary (`live`, `cached`, `stale`, `expired`, `missing`).
5. Every row carries a non-empty `local_tenant_scope_token` explaining what remains local versus tenant-scoped.
6. Provisioning failure and dry-run rows that carry a failure kind use a specific [`ProvisioningFailureKind`] token (`provider_outage`, `auth_drift`, `scope_mismatch`, `seat_loss`, `deprovisioning_impact`) rather than generic error copy.
7. `local_governance_path` rows carry `local_core_continuity_explicit: true`.
8. `directory_provider_card` rows carry a non-empty `fallback_manual_path_label`.
9. Rows requiring admin action/result lineage carry an [`AdminActionLineage`] block with action, result, actor ref, and transaction id.

## Hard guardrails — withdrawal condition

One condition forces `Withdrawn` immediately and cannot be overridden:

- A row with `raw_secret_or_private_material_excluded: false`
  (narrow reason: `raw_secret_or_private_material_exposed`).

## Canonical paths

- Doc: `docs/enterprise/m4/harden-identity-and-admin-support-export-parity-audit.md`
- Runtime owner: `aureline_policy::harden_identity_and_admin_support_export_parity_audit`
- Fixtures: `fixtures/enterprise/m4/harden-identity-and-admin-support-export-parity-audit/`
- Schema: `schemas/enterprise/harden-identity-and-admin-support-export-parity-audit.schema.json`
