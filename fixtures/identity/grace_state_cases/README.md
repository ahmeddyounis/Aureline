# Policy-bundle, entitlement-snapshot, grace-state, and admin-audit fixtures

These fixtures anchor the policy-bundle, entitlement-snapshot, grace-
state, and admin-audit-packet contract frozen in
[`/docs/identity/offline_entitlement_and_policy_seed.md`](../../../docs/identity/offline_entitlement_and_policy_seed.md)
and validated by the three schemas in
[`/schemas/identity/`](../../../schemas/identity/).

They reuse the identity-mode, deployment-profile, workspace-trust,
signer-continuity, and distribution-freshness vocabulary already
frozen by ADR-0001, ADR-0007, ADR-0010, ADR-0015, and the security
emergency-action / revocation contract rather than minting a second
identity, policy, or audit model.

**Scope rules**

- Every fixture validates as one of `policy_bundle_record`,
  `entitlement_snapshot_record`, or `admin_audit_packet_record`.
- No fixture declares a `managed_only_requires_fresh_refresh` feature
  as `available_*` while `last_refresh.freshness_class` is
  `stale_past_grace` or `offline_snapshot_expired`. Stale snapshots
  never grant new managed privilege.
- No fixture uses `authoritative_live` freshness together with
  `verification_failed` validation.
- Raw tenant names, raw user emails, raw directory attribute values,
  raw policy rule text, raw signatures, raw billing strings, and raw
  device fingerprints never appear. Every binding is an opaque ref or
  an export-safe label.

**Index**

| Fixture                                                                                                                                             | Record kind                       | What it proves                                                                                                                                                                      |
|-----------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [`account_free_local_inspectable_policy.json`](./account_free_local_inspectable_policy.json)                                                        | `policy_bundle_record`            | Account-free local mode carries an inspectable policy bundle (local_advisory_scope_only) that never gates managed-only features; desktop-core stays on ADR-0001 defaults.           |
| [`managed_policy_stale_past_grace.json`](./managed_policy_stale_past_grace.json)                                                                    | `policy_bundle_record`            | A tenant policy bundle past its mirror freshness grace names the stale posture honestly and falls back to last-known-good narrowing; no new managed narrowing is introduced.        |
| [`entitlement_grace_managed_service_unreachable.json`](./entitlement_grace_managed_service_unreachable.json)                                        | `entitlement_snapshot_record`     | Grace-state posture while managed services are unreachable: desktop-core stays local-safe; managed-only features visibly narrow; no new managed privilege is granted.               |
| [`entitlement_grace_expired_narrows_to_local_safe.json`](./entitlement_grace_expired_narrows_to_local_safe.json)                                    | `entitlement_snapshot_record`     | Grace expired with stale_past_grace refresh; every managed_only_requires_fresh_refresh feature marked unavailable_pending_entitlement_refresh; desktop-core remains available.      |
| [`admin_audit_policy_rollback_last_known_good.json`](./admin_audit_policy_rollback_last_known_good.json)                                            | `admin_audit_packet_record`       | Admin rollback to last-known-good policy bundle. Before / after state, typed decision reason, and explainability are renderable locally.                                            |
| [`admin_audit_entitlement_refresh_declined_stale_snapshot.json`](./admin_audit_entitlement_refresh_declined_stale_snapshot.json)                    | `admin_audit_packet_record`       | Client refusal to grant managed privilege from a stale snapshot; distribution rows carry stale freshness, proving the refusal cannot be read as an authoritative-live refresh.      |
| [`admin_audit_org_switch_degraded_into_local.json`](./admin_audit_org_switch_degraded_into_local.json)                                              | `admin_audit_packet_record`       | Org switch that degrades into account_free_local when both control planes are unreachable; desktop-core capabilities remain invariant and the audit row is recorded locally.        |
