# Managed session continuity fixtures

These fixtures anchor the managed-authentication and session-continuity
contract frozen in
[`/docs/auth/managed_auth_and_session_continuity_contract.md`](../../../docs/auth/managed_auth_and_session_continuity_contract.md)
and validated by
[`/schemas/auth/managed_session_state.schema.json`](../../../schemas/auth/managed_session_state.schema.json)
plus
[`/schemas/auth/reauth_requirement.schema.json`](../../../schemas/auth/reauth_requirement.schema.json).

They extend the system-browser callback and account-boundary packet
rather than minting a second identity model.

**Scope rules**

- Every fixture is a `managed_session_state_record`.
- Reauth, expiring, and managed-blocked examples include a nested
  `reauth_requirement_record` with exact reason, affected scope,
  fallback, and local-continuity fields.
- Every fixture keeps local edit, save, undo/redo, diagnostics, and
  user-owned export explicitly available.
- Raw tokens, raw URLs, raw cookies, raw IdP error bodies, raw
  directory attributes, raw passkey material, and account identifiers
  never appear.

**Index**

| Fixture | State | What it proves |
|---|---|---|
| [`local_only_mode.yaml`](./local_only_mode.yaml) | `local_only` | Account-free local use is a complete state with no managed dependency. |
| [`passkey_capable_sign_in.yaml`](./passkey_capable_sign_in.yaml) | `signed_in` | Managed sign-in can be passkey-capable while staying system-browser-first. |
| [`accessible_fallback_device_code.yaml`](./accessible_fallback_device_code.yaml) | `managed_blocked` | Authenticator incompatibility names an accessible device-code or equivalent fallback instead of a generic auth error. |
| [`dirty_buffer_session_expiry.yaml`](./dirty_buffer_session_expiry.yaml) | `session_expiring` | Dirty buffers remain editable/saveable when managed session refresh is due. |
| [`org_switch_preserves_local.yaml`](./org_switch_preserves_local.yaml) | `reauth_required` | Org switch requires fresh auth for managed scope while preserving local workspace authority. |
| [`seat_transfer.yaml`](./seat_transfer.yaml) | `reauth_required` | Seat transfer pauses seat-bound managed actions without blocking local artifacts. |
| [`seat_removal_deprovisioning_preserved_local_artifacts.yaml`](./seat_removal_deprovisioning_preserved_local_artifacts.yaml) | `deprovisioned` | Seat removal and account deprovisioning preserve local artifacts and offboarding export. |
