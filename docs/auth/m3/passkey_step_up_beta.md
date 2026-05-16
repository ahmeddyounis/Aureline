# Passkey-capable step-up, reauth, and recovery lanes (beta)

This page is the reviewer landing page for the beta projection that proves,
on every claimed beta row participating in a passkey-bound lane, that:

1. **The lane is named.** Every row quotes a closed
   [`PasskeyBetaLaneClass`][lane] token — `step_up_lane`, `reauth_lane`,
   `recovery_lane`, or `not_applicable_account_free_local` — and binds it to a
   target / requested-action label.
2. **The lifecycle state and the client scope are disclosed.** Every row
   names the user-visible passkey lifecycle state (e.g.
   `active_on_this_device`, `unavailable_this_platform`, `revoked`,
   `expired_attestation_required`) plus the client scope of the assertion
   (`step_up_scope_risky_write_action`, `reauth_scope_refresh_session`,
   `recovery_scope_rebind_authenticator`, etc.).
3. **Step-up is satisfied or a typed fallback is named.** Rows whose outcome
   is not `step_up_satisfied` MUST name a typed fallback path from
   [`PasskeyFallbackClass`][fallback]. The audit blocks silent drops — a
   `no_fallback_required` token on an unsatisfied row, an unsupported
   platform without a named fallback, or an admin-policy deny without a
   `policy_denied_no_fallback_available` token are all typed defects.
4. **Reauth and recovery preserve the originating target / action.** Rows
   that claim a reauth or recovery lane MUST quote a preservation token from
   the safe set (`target_action_preserved_exact` or
   `target_action_downscoped`). `target_action_rerouted` and
   `target_action_widened` are typed defects on those lanes — the lane MUST
   NOT silently widen or reroute the request.
5. **Granted authority MUST NOT widen the requested authority.** The row's
   `granted_authority_scope_token` is rank-checked against the
   `requested_authority_scope_token`; widening is a typed defect even when
   the lane completed.
6. **The support-export row reuses the same closed-vocabulary tokens.**
   Drift between the live row and the support row is a contract bug.

## Where the truth lives

- Module: [`/crates/aureline-auth/src/passkey/mod.rs`](../../../crates/aureline-auth/src/passkey/mod.rs)
- Shell consumer: [`/crates/aureline-shell/src/passkey_step_up_beta/mod.rs`](../../../crates/aureline-shell/src/passkey_step_up_beta/mod.rs)
- Headless inspector: [`/crates/aureline-shell/src/bin/aureline_shell_passkey_step_up_beta.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_passkey_step_up_beta.rs)
- Schema: [`/schemas/auth/passkey_step_up_beta.schema.json`](../../../schemas/auth/passkey_step_up_beta.schema.json)
- Fixtures: [`/fixtures/auth/m3/passkey_step_up/`](../../../fixtures/auth/m3/passkey_step_up/)
- Integration test: [`/crates/aureline-shell/tests/passkey_step_up_beta_fixtures.rs`](../../../crates/aureline-shell/tests/passkey_step_up_beta_fixtures.rs)

## Headless inspector commands

```sh
cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- page
cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- rows
cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- support-rows
cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- defects
cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- summary
cargo run -q -p aureline-shell --bin aureline_shell_passkey_step_up_beta -- validate
```

The `validate` subcommand exits 0 when the seeded page has zero defects and
exits 3 with a typed defect list otherwise.

## Lane vocabulary

| Token | Meaning |
| --- | --- |
| `step_up_lane` | Passkey step-up before a single risky write or admin action on an already signed-in row. |
| `reauth_lane` | Passkey-capable reauthentication after a session expiry or sensitive posture change. |
| `recovery_lane` | Passkey-bound recovery after a lost device, revoked authenticator, or admin-initiated unbind. |
| `not_applicable_account_free_local` | Account-free local row; no passkey lane is claimed. |

## Lifecycle state vocabulary

| Token | Meaning |
| --- | --- |
| `not_enrolled` | User has not enrolled a passkey on this account. |
| `enrollment_pending` | User started enrollment but has not completed it. |
| `active_on_this_device` | Passkey active and bound to the current device's authenticator. |
| `active_on_other_device_only` | Passkey active on another device only; cross-device flow required. |
| `revoked` | Passkey revoked by admin or user; fallback required. |
| `expired_attestation_required` | Passkey expired; reattestation required. |
| `unavailable_this_platform` | Current platform / browser does not support WebAuthn. |
| `not_applicable_account_free_local` | Account-free local row. |

## Outcome and fallback vocabulary

| Outcome | Fallback expectation |
| --- | --- |
| `step_up_satisfied` | `no_fallback_required` is allowed (and is the default). |
| `step_up_pending_user_action` | A typed fallback MUST be named in case the user does not complete the prompt. |
| `step_up_denied_by_policy` | A typed fallback MUST be named (or `policy_denied_no_fallback_available` when no fallback exists). |
| `step_up_denied_authenticator_missing` | A typed fallback MUST be named. |
| `step_up_user_canceled` | A typed fallback MUST be named. |
| `fallback_engaged` | A real fallback token (not `no_fallback_required`) MUST be quoted. |
| `not_applicable_account_free_local` | `not_applicable_account_free_local` is the only valid fallback token. |

## Acceptance gates this page proves

- **Claimed rows can request step-up with passkey-capable flows and disclose
  fallback when the platform or policy does not support them.** The seeded
  page includes a step-up row satisfied by an active passkey, a reauth row
  that names a typed fallback when WebAuthn is unsupported, and a recovery
  row whose admin policy denies passkey and names `contact_admin_for_recovery`
  as the fallback.
- **Reauth and recovery preserve the original target / action identity
  instead of silently widening or rerouting the request.** The audit's
  fourth axis verifies the preservation token on every reauth / recovery
  row; `target_action_rerouted` and `target_action_widened` are typed
  defects.
- **Passkey flows are labeled by lifecycle state and client scope in UI and
  docs.** Every non-local row names both a lifecycle state token (UI cue)
  and a client scope token (action binding). The same two tokens appear on
  the export-safe support row.

## Guardrails

- The audit **fails closed** before widening authority. The
  `granted_authority_widens_requested` defect fires whenever the granted
  scope ranks above the requested scope.
- Admin-policy denials cannot silently drop. Either the row names a typed
  fallback or it quotes `policy_denied_no_fallback_available`; otherwise
  the validator emits `policy_denies_passkey_without_fallback`.
- The shell does not mint a parallel passkey model — it consumes the
  auth-owned projection so audit truth has one source.

[lane]: ../../../crates/aureline-auth/src/passkey/mod.rs
[fallback]: ../../../crates/aureline-auth/src/passkey/mod.rs
