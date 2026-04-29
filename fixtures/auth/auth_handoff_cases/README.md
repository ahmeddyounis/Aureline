# Auth-handoff interstitial and embedded-auth exception register example fixtures

These fixtures anchor the auth-handoff interstitial and embedded-auth
exception register contract frozen in
[`/docs/auth/auth_handoff_interstitial_contract.md`](../../../docs/auth/auth_handoff_interstitial_contract.md)
and validated by
[`/schemas/auth/auth_handoff_interstitial.schema.json`](../../../schemas/auth/auth_handoff_interstitial.schema.json)
and
[`/schemas/auth/embedded_auth_exception.schema.json`](../../../schemas/auth/embedded_auth_exception.schema.json).

They reuse the identity-mode, deployment-profile, workspace-trust,
provider-class, surface-family, auth-flow-class, callback-origin,
preserved-local-work, recovery-path, and reserved-native-surface
vocabulary already frozen by ADR-0001, ADR-0010, ADR-0015, and the
existing system-browser callback packet plus managed-authentication
contract rather than minting a second review or boundary model.

## Scope rules

- Every fixture validates as one of `auth_handoff_interstitial_record`
  or `embedded_auth_exception_register_record`.
- Every interstitial fixture pre-discloses callback origin, requested
  action class, provider/domain, target scope, expiry/replay policy,
  availability rows (system browser, device code, passkey), native-
  approval boundary, and typed confirm/reject actions.
- Embedded auth paths (`embedded_session_refresh`,
  `embedded_password_exception`) MUST cite an active register entry by
  ref. System-browser, device-code, and platform-authenticator-native
  flows MUST NOT cite a register entry.
- Step-up authority, admin step-up, scope grant, switch org or tenant,
  and deprovision acknowledge actions all set
  `product_owned_native_required = true` and route the confirm action
  to a host-native review surface.
- Raw tokens, raw URLs, raw cookies, raw codes, raw nonces, raw PKCE
  verifiers, raw passkey material, raw passwords, and raw provider
  query strings never appear; every reference is opaque-id-aliased.

## Index

| Fixture                                                                                                                                       | Record kind                                       | What it proves                                                                                                                                                           |
|-----------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [`system_browser_managed_sign_in_interstitial.yaml`](./system_browser_managed_sign_in_interstitial.yaml)                                       | `auth_handoff_interstitial_record`                | System-browser-first sign-in: callback origin, target tenant scope, availability disclosure, and typed confirm/reject all reviewable before authority widens.            |
| [`device_code_fallback_interstitial.yaml`](./device_code_fallback_interstitial.yaml)                                                           | `auth_handoff_interstitial_record`                | Device-code fallback when system-browser launch is blocked by managed kiosk policy; preserved-local-work intact while the device code is redeemed.                       |
| [`passkey_step_up_interstitial.yaml`](./passkey_step_up_interstitial.yaml)                                                                     | `auth_handoff_interstitial_record`                | Platform-authenticator-native step-up: passkey-capable disclosure, host-native step-up requirement, and reserved-native-surface linkage on the confirm action.            |
| [`embedded_session_refresh_exception_register_entry.yaml`](./embedded_session_refresh_exception_register_entry.yaml)                           | `embedded_auth_exception_register_record`         | Active session-refresh-only register entry with lower-trust cues, host-native approval still required, and a fallback rule preferring system browser / device code / passkey first. |
| [`embedded_password_exception_register_entry.yaml`](./embedded_password_exception_register_entry.yaml)                                         | `embedded_auth_exception_register_record`         | Active password-exception register entry for a legacy IdP domain with `no_high_risk_actions` scope, no-password-persistence requirement, and visible lower-trust badge.   |
| [`scope_grant_rejected_by_user_interstitial.yaml`](./scope_grant_rejected_by_user_interstitial.yaml)                                           | `auth_handoff_interstitial_record`                | Scope-grant interstitial rejected by the user with a typed `continue_local_without_sign_in` outcome; preserved-local-work assertion remains true.                        |
| [`callback_origin_mismatch_denied_interstitial.yaml`](./callback_origin_mismatch_denied_interstitial.yaml)                                     | `auth_handoff_interstitial_record`                | Returning origin does not match the pre-disclosed loopback callback; the interstitial fails closed with `denied_origin_mismatch` and a typed `callback_origin_mismatch` denial reason. |

## Worked example: how surfaces compose

1. The host shell mints an `auth_handoff_interstitial_record` and
   binds it to an `auth_callback_packet_record` (and, for resume / step-
   up / refresh / scope-grant / org-switch / deprovision-acknowledge
   actions, a `managed_session_state_record`).
2. The interstitial pre-discloses the callback origin, requested
   action class, provider/domain, target scope, expiry/replay policy,
   availability rows, and native-approval boundary. The reviewer reads
   the visible cues and exercises the typed confirm or reject action.
3. For embedded paths only, the interstitial cites an active
   `embedded_auth_exception_register_record` by id. The boundary card
   on the embedded surface re-renders the register entry's lower-
   trust cues; the host shell never widens the embedded path beyond
   what the register entry authorises.
4. Audit events emit through the typed audit-event vocabulary on
   either schema. Support packets, admin exports, release evidence,
   and docs/help diagnostics quote the interstitial id, register entry
   id, and the bound callback / managed-session / browser-handoff refs;
   every label is reusable across surfaces without per-surface copy.
