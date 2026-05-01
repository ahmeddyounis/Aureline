# Callback-origin skew example fixtures

These fixtures anchor the system-browser auth drill packet at
[`/artifacts/auth/system_browser_auth_drill_packet.md`](../../../artifacts/auth/system_browser_auth_drill_packet.md)
and validate against
[`/schemas/auth/auth_handoff_interstitial.schema.json`](../../../schemas/auth/auth_handoff_interstitial.schema.json).
They reuse the identity-mode, deployment-profile, workspace-trust,
provider-class, callback-origin, availability, native-approval
boundary, preserved-local-work, recovery-path, and reserved-native-
surface vocabulary already frozen by ADR-0001, ADR-0010, ADR-0015,
the system-browser callback packet, the managed-authentication
contract, and the auth-handoff interstitial contract.

The corpus exists so callback-origin drift, replay, tenant mismatch,
expiry skew, browser/device-code fallback, and passkey
available/unavailable behaviors can be reviewed before any privileged
action resumes — without re-minting closed vocabularies per fixture.

## Scope rules

- Every fixture validates as one `auth_handoff_interstitial_record`.
- Every fixture's `__fixture__.exercised_axes` carries `skew_class`,
  `resulting_state`, `safe_recovery_action_class`, and
  `authority_change_class` so the drill packet can quote the row by
  stable id.
- Callback origin, requested action class, provider/domain, target
  scope, expiry/replay policy, availability rows (system browser,
  device code, passkey), native-approval boundary, and typed
  confirm/reject actions appear on every fixture.
- System-browser, device-code, and platform-authenticator-native
  flows MUST set `embedded_handoff_forbidden = true` and MUST carry
  `embedded_auth_exception_ref = null`.
- Denied / expired / superseded / rejected_by_policy rows MUST name
  one `denial_reason_class` from the closed set.
- `safe_recovery_action_class` resolves to one of the closed
  `retry_path_class` values on
  `schemas/auth/auth_callback_state.schema.json#/$defs/retry_path_class`.
- `authority_change_class` resolves to one of the six closed values
  in §3 of the system-browser auth drill packet.
- Raw tokens, raw URLs, raw cookies, raw codes, raw nonces, raw PKCE
  verifiers, raw passkey material, raw passwords, and raw provider
  query strings never appear; every reference is opaque-id-aliased.

## Index

| Fixture | Skew class | Resulting state | Safe recovery action | Authority change |
|---|---|---|---|---|
| [`baseline_loopback_origin_verified.yaml`](./baseline_loopback_origin_verified.yaml) | `no_skew_baseline` | `pending_review` | `continue_local_without_sign_in` | `authority_unchanged_local_continuity_preserved` |
| [`loopback_port_drift_denied.yaml`](./loopback_port_drift_denied.yaml) | `loopback_port_or_host_drift` | `denied_origin_mismatch` | `retry_in_system_browser` | `authority_denied_failed_closed_no_widening` |
| [`tenant_match_rule_violated_denied.yaml`](./tenant_match_rule_violated_denied.yaml) | `tenant_or_workspace_binding_mismatch` | `denied_native_boundary` | `contact_support_with_export` | `authority_narrowed_managed_only_revoked` |
| [`expired_callback_after_browser_idle.yaml`](./expired_callback_after_browser_idle.yaml) | `expiry_window_closed_before_review` | `expired_before_review` | `retry_in_system_browser` | `authority_unchanged_managed_held_until_revalidation` |
| [`stale_approval_replay_denied.yaml`](./stale_approval_replay_denied.yaml) | `stale_approval_replay_attempt` | `denied_replay` | `no_recovery_without_superseding_action` | `authority_denied_failed_closed_no_widening` |
| [`browser_blocked_device_code_offered.yaml`](./browser_blocked_device_code_offered.yaml) | `browser_launch_policy_blocked_device_code_offered` | `pending_review` | `switch_to_device_code` | `authority_narrowed_visible_recovery_required` |
| [`passkey_available_step_up_native.yaml`](./passkey_available_step_up_native.yaml) | `passkey_available_native_step_up` | `pending_review` | `resume_after_step_up` | `authority_widened_inside_native_review_only` |
| [`passkey_unavailable_system_browser_fallback.yaml`](./passkey_unavailable_system_browser_fallback.yaml) | `passkey_unavailable_system_browser_fallback` | `pending_review` | `retry_in_system_browser` | `authority_narrowed_visible_recovery_required` |

## How surfaces compose

1. The host shell mints an `auth_handoff_interstitial_record` and
   pre-discloses its callback origin, target scope, expiry/replay
   policy, availability rows, and native-approval boundary.
2. A reviewer reads the record before any state mutation. Origin
   drift between mint and return triggers `denied_origin_mismatch`;
   tenant or workspace mismatch triggers `denied_native_boundary`;
   replay against a consumed callback triggers `denied_replay`;
   expiry-window violation triggers `expired_before_review`. None of
   these widen the interstitial; all of them carry a typed
   `denial_reason_class` and emit a closed `audit_event_id`.
3. Browser blocked, device-code unavailable, and passkey unavailable
   rows narrow the path with a typed `retry_path_class` recovery
   instead of routing into an embedded webview without an active
   register entry.
4. Step-up authority, scope grant, switch org or tenant, admin step-
   up, and deprovision-acknowledge confirms land on a host-native
   review surface (`product_security_messaging`,
   `update_verification`, `workspace_trust_elevation`,
   `rollback_or_restore_confirmation`, `ai_apply_review`,
   `high_risk_approval_sheet`); OS notifications and companion
   shortcuts may not deliver the final approval.
5. Audit events emit through the typed `audit_event_id` vocabulary on
   `schemas/auth/auth_handoff_interstitial.schema.json#/$defs/audit_event_id`.
   Support packets, admin exports, release evidence, and docs/help
   diagnostics quote the interstitial id, the callback packet ref,
   the browser handoff packet ref, the device-code companion ref, the
   bound managed-session ref, and the audit event refs by stable id.
