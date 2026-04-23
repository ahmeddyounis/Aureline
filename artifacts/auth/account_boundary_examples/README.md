# Account-boundary example artifacts

These artifacts anchor the account-boundary record frozen in
[`/docs/auth/system_browser_callback_packet.md`](../../../docs/auth/system_browser_callback_packet.md)
and validated by
[`/schemas/auth/auth_callback_state.schema.json`](../../../schemas/auth/auth_callback_state.schema.json).

Every file is an `account_boundary_record` instance. Together they seed
the local-only, self-hosted, managed, restricted-managed-only, and
grace-degraded-managed posture rows so the first auth / marketplace /
support / release-evidence surfaces can quote them by stable id instead
of minting local labels.

Each artifact:

- names the `boundary_class` and `identity_mode` rows from ADR-0001;
- pins a deployment-profile scope;
- carries an export-safe `boundary_label` and `plain_language_summary`;
- declares the visible downgrade path (trigger class + whether local
  work is preserved);
- carries the preserved-local-work block; and
- reserves the four passkey-capable / reauth-required / seat-loss /
  deprovision-preserves-local-work rows so later managed-auth work
  extends the same packet instead of replacing it.

| Artifact                                                                                 | Boundary class              | Identity mode         | What it seeds                                                                                                                                |
|------------------------------------------------------------------------------------------|-----------------------------|-----------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| [`local_only_account_free.json`](./local_only_account_free.json)                         | `local_only`                | `account_free_local`  | The default. No account, no hosted dependency, no embedded-vendor auth flow. Local work intact; downgrade trigger not_applicable.             |
| [`self_hosted_org_idp.json`](./self_hosted_org_idp.json)                                 | `self_hosted`               | `self_hosted_org`     | Customer-run IdP over the system browser; managed convenience absent. Local work intact with self-hosted narrowing on IdP unreachable.       |
| [`managed_convenience_active.json`](./managed_convenience_active.json)                   | `managed`                   | `managed_workspace`   | Vendor-hosted convenience layer active. Local work intact with managed narrowing on managed-service unreachable.                              |
| [`restricted_managed_only_policy.json`](./restricted_managed_only_policy.json)           | `restricted_managed_only`   | `managed_workspace`   | Narrow policy where one capability surface is managed-sign-in-required. Local work stays truthful; policy_narrowed downgrade preserves it.    |
| [`grace_degraded_managed_outage.json`](./grace_degraded_managed_outage.json)             | `grace_degraded_managed`    | `managed_workspace`   | Bounded grace posture when managed services are unreachable. Local + self-hosted paths stay usable; visible recovery copy is explicit.        |
