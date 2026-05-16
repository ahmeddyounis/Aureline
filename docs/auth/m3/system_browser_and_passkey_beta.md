# System-browser default + passkey step-up + return-path labeling (beta)

The beta system-browser audit promotes the existing alpha
[`ClaimedIdentityRow`](../../../crates/aureline-auth/src/system_browser/mod.rs)
seed to a page-level projection that, on every claimed identity row, proves
three M3 promises:

1. **System-browser default is the default.** Every claimed identity row
   defaults to `open_system_browser` unless the row quotes a closed
   `SystemBrowserPolicyExceptionClass` token that explicitly justifies a
   different default. The `system_browser_default_no_exception` token is
   reserved for the default; no row that picks a non-`open_system_browser`
   default may quote it.
2. **Return paths preserve workspace, target, and requested-action
   identity.** Every row quotes the workspace label, the target label, the
   plain-language requested-action label, the return-mode token, the
   return-anchor ref, the origin-validation token, and the
   tenant/workspace match-rule token used by the
   [`BrowserCallbackPacket`](../../../crates/aureline-auth/src/browser_callback/mod.rs)
   validator on return. The granted authority scope MUST NOT widen the
   requested authority scope.
3. **Passkey-capable reauth is honest.** Rows that claim passkey
   capability quote a closed `PasskeyStepUpPostureClass` token from the
   safe set (`passkey_required`, `passkey_capable_offered`,
   `passkey_unavailable_with_fallback`) and, when the platform cannot
   satisfy the passkey path right now, name a typed fallback retry-path
   token. Rows that do not claim passkey capability quote
   `passkey_not_applicable`.

The page yields a typed
[`SystemBrowserReturnPathBetaDefect`](../../../crates/aureline-auth/src/system_browser/beta.rs)
list. The seeded page seeds zero defects; the validator and the headless
inspector are what surface a regression when a row drops a required
field, drifts vocabulary across the live row and the support row, or
widens authority without an explicit exception.

## Contract surface

The beta projection ships five record kinds, all under the shared
contract ref `auth:system_browser_return_paths_beta:v1`:

- `auth_system_browser_return_paths_beta_row_record` — one audited row
  per claimed identity scenario. Each row carries a stable `case_id` and
  `row_id`, the source claim row ref, account-boundary class, identity
  mode, trust state, provider/domain/scope disclosure, the chosen
  `default_action_token`, the `policy_exception_token` plus a plain-
  language `policy_exception_label`, the
  [`ReturnPathLabel`](../../../crates/aureline-auth/src/system_browser/beta.rs)
  block (workspace label, target label, requested-action label, return-
  mode token, return-anchor ref, origin-validation token, tenant /
  workspace match-rule token), the requested vs granted authority
  scope, and the
  [`PasskeyStepUpBlock`](../../../crates/aureline-auth/src/system_browser/beta.rs)
  (posture token, reason label, optional fallback retry-path token +
  label).
- `auth_system_browser_return_paths_beta_support_row_record` — export-
  safe support row aligned 1:1 with the live row by `row_id`. The
  support row reuses the same closed-vocabulary tokens the live row
  paints; drift is a contract bug.
- `auth_system_browser_return_paths_beta_defect_record` — typed defect
  emitted by the validator when a row drops a required field, drifts
  vocabulary across live & support rows, weakens the system-browser
  default without an explicit exception, or widens authority on return.
- `auth_system_browser_return_paths_beta_page_record` — top-level page
  with the aggregate summary banner (return-mode coverage, policy-
  exception coverage, passkey-posture coverage), the live rows, the
  support rows, and the defects.
- `auth_system_browser_return_paths_beta_support_export_record` —
  support-export wrapper that quotes the page plus a metadata-safe
  defect roll-up (`defect_kinds_present`, `defect_counts_by_kind`,
  `raw_private_material_excluded=true`).

The frozen JSON schema lives at
[`/schemas/auth/system_browser_return_paths_beta.schema.json`](../../../schemas/auth/system_browser_return_paths_beta.schema.json).

## Audit axes

The audit checks every row against the closed
[`SystemBrowserReturnPathBetaAxis`](../../../crates/aureline-auth/src/system_browser/beta.rs)
vocabulary:

| Axis | What the row must show |
| --- | --- |
| `system_browser_default_unless_explicit_exception` | Row defaults to `open_system_browser` or quotes a `SystemBrowserPolicyExceptionClass` token from `{admin_policy_device_code_required, admin_policy_manual_resume_required, browser_launch_unavailable_use_device_code, browser_launch_offline_use_stay_local, account_free_local_no_auth_required}`. |
| `exact_return_path_label_preserved` | Row quotes workspace label, target label, requested-action label, return-mode token, return-anchor ref, origin-validation token, and tenant/workspace match-rule token. |
| `workspace_target_action_identity_preserved` | Granted authority scope does not widen requested authority scope. |
| `passkey_capable_step_up_when_claimed` | Row that claims passkey capability quotes a closed `PasskeyStepUpPostureClass` token from `{passkey_required, passkey_capable_offered, passkey_unavailable_with_fallback}`; rows that do not claim it quote `passkey_not_applicable`; rows that quote `passkey_capable_offered` or `passkey_unavailable_with_fallback` MUST name a fallback retry-path token. |
| `support_export_vocabulary_parity` | Support row reuses the same closed-vocabulary tokens (default action, policy exception, return-mode, return-origin-validation, return-tenant-or-workspace-match-rule, return anchor ref, workspace / target / requested-action label, requested + granted scope, passkey posture, fallback retry-path). |

## Defect vocabulary

The audit emits one of the following typed defects when an axis is
violated:

| Defect kind | When the validator emits it |
| --- | --- |
| `system_browser_not_default_without_explicit_exception` | Row picked a non-`open_system_browser` default but still quotes `system_browser_default_no_exception`. |
| `policy_exception_label_missing` | Row quotes a non-default exception token without a plain-language label. |
| `return_path_workspace_drift` | Non-local row missing a workspace label on the return path. |
| `return_path_target_drift` | Row missing a target label. |
| `return_path_action_drift` | Row missing a requested-action label. |
| `return_anchor_ref_missing` | Non-local row missing a return-anchor ref. |
| `return_widens_authority_scope` | Granted scope widens the requested scope (ranked by `AuthorityScopeClass::rank`). |
| `passkey_claimed_without_step_up_block` | Row claimed passkey capability but the posture is not in the safe set. |
| `passkey_unavailable_without_honest_fallback` | `passkey_capable_offered` or `passkey_unavailable_with_fallback` without a fallback retry-path token. |
| `passkey_not_applicable_mislabeled` | Row that does not claim passkey capability quoted a posture other than `passkey_not_applicable`. |
| `support_row_vocabulary_drift` | Support row drifted from the live row on a closed-vocabulary token (default action, policy exception, return-mode, origin-validation, match-rule, return-anchor ref, workspace / target / requested-action label, requested + granted scope, passkey posture, fallback retry-path). |
| `system_browser_default_inconsistent_with_blocked_launch` | Row claims system-browser default while declaring `browser_launch_policy_blocked`. |

## Seeded coverage

The seeded page exercises three claimed identity rows that span the
default and the explicit-exception postures:

| Case | Default action | Policy exception | Return mode | Passkey posture |
| --- | --- | --- | --- | --- |
| `system_browser_default_with_passkey_step_up` | `open_system_browser` | `system_browser_default_no_exception` | `loopback_http_return` | `passkey_capable_offered` (fallback `resume_after_step_up`) |
| `admin_policy_device_code_required` | `use_device_code` | `admin_policy_device_code_required` | `device_code_poll_return` | `passkey_not_applicable` |
| `account_free_local_no_auth_required` | `no_auth_required` | `account_free_local_no_auth_required` | `not_applicable` | `passkey_not_applicable` |

The seeded page seeds zero defects.

## Failure drills

The unit tests in
[`crates/aureline-auth/src/system_browser/beta.rs`](../../../crates/aureline-auth/src/system_browser/beta.rs)
include one drill per defect kind. The integration test in
[`crates/aureline-shell/tests/system_browser_return_paths_beta_fixtures.rs`](../../../crates/aureline-shell/tests/system_browser_return_paths_beta_fixtures.rs)
also replays three named drills against the seeded page (granted scope
widens beyond requested, passkey posture without fallback retry-path,
non-`open_system_browser` default without explicit exception) so a
regression in any of these surfaces trips the build.

The drill fixtures live under
[`fixtures/auth/m3/system_browser_return_paths/`](../../../fixtures/auth/m3/system_browser_return_paths/)
and are generated by the same headless inspector that emits the seeded
page.

## Reproduce locally

```sh
# Generate or refresh fixtures (a clean checkout already has them):
cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- page \
  > fixtures/auth/m3/system_browser_return_paths/page.json
cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- support-export \
  > fixtures/auth/m3/system_browser_return_paths/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_system_browser_return_paths -- summary \
  > fixtures/auth/m3/system_browser_return_paths/render_summary.json

# Replay all unit + integration tests:
cargo test -p aureline-auth --lib system_browser::beta
cargo test -p aureline-shell --lib system_browser_return_paths
cargo test -p aureline-shell --test system_browser_return_paths_beta_fixtures
```

## Joined with the rest of the auth lane

The beta page does not re-derive any claim truth — the
[`ClaimedIdentityRow`](../../../crates/aureline-auth/src/system_browser/mod.rs)
alpha seed remains the source of the default-action, fallback-set,
provider-scope, and recovery posture. The beta projection layers, on
top of those rows, the policy-exception explicitness, the exact
return-path labeling, and the passkey step-up block.

The return-mode, origin-validation, and tenant/workspace match-rule
tokens are the same closed vocabulary the
[`BrowserCallbackHandoff`](../../../crates/aureline-auth/src/browser_callback/mod.rs)
validator already enforces on the returning browser callback. Adding
them to the audit row gives reviewers and support a single artifact to
inspect: the auth lane and the audit lane never invent parallel return-
mode, origin, or scope vocabularies.

## Out of scope (M3)

- Live OAuth / OIDC / SAML provider adapters and their wire protocols.
- Provider-specific WebAuthn / FIDO2 attestation policies.
- Managed-admin policy-bundle formats (the audit names the explicit
  exception tokens; the bundle that carries them stays out of scope).
- M4-only breadth such as cross-device passkey bridge UX or full
  guided-classroom productization.
