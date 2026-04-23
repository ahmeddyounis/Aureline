# System-browser callback and credential-store lock-state example fixtures

These fixtures anchor the auth callback, credential-store lock-state,
and account-boundary contract frozen in
[`/docs/auth/system_browser_callback_packet.md`](../../../docs/auth/system_browser_callback_packet.md)
and validated by
[`/schemas/auth/auth_callback_state.schema.json`](../../../schemas/auth/auth_callback_state.schema.json).

They reuse the identity-mode, deployment-profile, workspace-trust,
credential-store, and browser-handoff vocabulary already frozen by
ADR-0001, ADR-0007, ADR-0010, and ADR-0015 rather than minting a second
identity or handoff model.

**Scope rules**

- Every fixture validates as one of `auth_callback_packet_record`,
  `credential_store_lock_state_record`, or
  `account_boundary_record`.
- Every callback-packet fixture exercises the four reserved rows
  (`passkey_capable`, `reauth_required`, `seat_loss`,
  `deprovision_preserves_local_work`) so later managed-auth work can
  extend the same packet.
- Raw tokens, raw URLs, raw cookies, raw codes, raw PKCE verifiers,
  raw nonces, raw passkey material, and raw provider query strings
  never appear. Every correlation / nonce / PKCE value is an opaque
  alias.

**Index**

| Fixture                                                                                                                     | Record kind                             | What it proves                                                                                                                                  |
|-----------------------------------------------------------------------------------------------------------------------------|-----------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| [`account_free_local_no_sign_in.json`](./account_free_local_no_sign_in.json)                                                | `auth_callback_packet_record`           | Account-free local mode representable without implying network or account; no pending browser hop; preserved-local-work intact.                 |
| [`managed_sign_in_required_outbound.json`](./managed_sign_in_required_outbound.json)                                        | `auth_callback_packet_record`           | Managed sign-in-required outbound handoff: system-browser default, loopback return, bound workspace + tenant, visible preserved-local-work.     |
| [`restricted_managed_only_grace_degraded.json`](./restricted_managed_only_grace_degraded.json)                              | `auth_callback_packet_record`           | Grace / restricted-managed-only posture preserves local continuity while managed services are narrowed; visible recovery is required.           |
| [`callback_tenant_mismatch_denied.json`](./callback_tenant_mismatch_denied.json)                                            | `auth_callback_packet_record`           | Returning browser state that doesn't match the pending tenant / workspace binding fails closed with a typed denial reason and visible recovery. |
| [`credential_store_locked_on_launch.json`](./credential_store_locked_on_launch.json)                                        | `credential_store_lock_state_record`    | OS keychain locked on launch surfaces one typed retry path; local editing, save, and BYOK AI remain representable.                              |
| [`return_from_browser_bound_workspace.json`](./return_from_browser_bound_workspace.json)                                    | `auth_callback_packet_record`           | Returning from the system browser into the bound workspace passes origin / tenant / replay validation and records the callback receipt ref.     |
