# System-Browser Auth, Passkey-Capable Step-Up, and Recovery Flows — Stable Packet

- Packet: `auth:system_browser_auth_stabilize:stable:0001`
- Schema version: `1`
- Contract ref: `auth:system_browser_auth_stabilize:v1`
- Qualification: `stable` (derived, not asserted)
- Return-paths beta page defects: 0
- Passkey step-up beta page defects: 0
- Stabilize defects: 0
- Withdrawn rows: 0
- Stable rows: all

## Lane coverage

| Lane | Passkey step-up | Fallback typed | Identity preserved |
|------|----------------|----------------|--------------------|
| Primary sign-in | required (claimed_capable) | yes | n/a |
| Reauth | required (claimed_capable) | yes | yes |
| Recovery | required (claimed_capable) | yes | yes |
| Account-free local | not claimed | n/a | n/a |

## Evidence sources

- System-browser return-paths beta audit:
  `auth:system_browser_return_paths_beta:v1`
  — `docs/auth/system_browser_callback_packet.md`
- Passkey step-up beta audit:
  `auth:passkey_step_up_beta:v1`
  — `docs/auth/managed_auth_and_session_continuity_contract.md`

## Key invariants verified

1. Both upstream beta pages audit with zero defects.
2. Every return-path row that claims an identity falls back to
   `system_browser` as the default auth mode, or quotes a closed
   `SystemBrowserPolicyExceptionClass` token.
3. Every row that claims passkey capability names a closed
   `PasskeyStepUpPostureClass` token from the safe set.
4. Every reauth and recovery passkey lane carries a
   `target_action_preservation` token of `preserved` — not `widened` or
   `rerouted`.
5. Every passkey lane whose lifecycle or outcome leaves it unsatisfied
   names a typed fallback path.

## Hard guardrails — withdrawal conditions

Both of the following force `Withdrawn` and cannot be overridden:

- Any `ReturnWidensAuthorityScope` or `GrantedAuthorityWidensRequested`
  defect in either beta page.
- Any `ReauthOrRecoveryWidened` defect in the passkey step-up beta page.

## Canonical paths

- Doc: `docs/enterprise/m4/stabilize-system-browser-auth-passkey-capable-step-up.md`
- Runtime owner: `aureline_auth::stabilize_system_browser_auth_passkey_capable_step_up`
