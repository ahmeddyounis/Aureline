# System-browser auth callback seed and local-versus-managed shell vocabulary

This document is the reviewer-facing landing page for the M1 seed that wires
system-browser auth callbacks and the shell vocabulary distinguishing
account-free local mode from signed-in managed / self-hosted mode. It points
at the canonical Rust seed object, the validation rules the seed enforces on
return from the system browser, the shell-chip projection a protected
terminal-pane row consumes, the failure-drill fixture, and the cross-tool
boundary vocabulary the seed grows into without forking truth.

## What this seed owns

- one inspectable [`BrowserCallbackPacket`](../../crates/aureline-auth/src/browser_callback/mod.rs)
  Rust object that carries:
  - `auth_flow_class` (`system_browser`, `device_code`,
    `platform_authenticator_native`, `not_applicable`),
  - `browser_launch_policy_class` and `embedded_fallback_posture`
    (system-default-browser is the default and required posture; embedded
    webview fallback is forbidden),
  - `pending_session_state` (`staged`, `awaiting_browser_return`,
    `completed`, `return_denied`, `session_superseded`, `session_expired`),
  - `account_boundary_class` (`local_only`, `self_hosted`, `managed`,
    `restricted_managed_only`, `grace_degraded_managed`, `unknown_boundary`)
    and the joined `identity_mode` re-exported from
    [`aureline_runtime::IdentityMode`](../../crates/aureline-runtime/src/execution_context/mod.rs),
  - `callback_correlation` aliases (correlation id, pending session id,
    state token alias, nonce alias, optional PKCE alias, bound workspace /
    tenant / actor refs, issued / expires timestamps),
  - `return_route` (return-mode class, return anchor ref, target label,
    origin-validation class, tenant / workspace match rule, optional policy
    check refs),
  - `preserved_local_work` (posture class, reviewable note, retained and
    blocked capability lists),
  - `recovery_path` (typed primary and fallback retry actions, recovery
    copy label, optional repair-hook ref, and a visible-recovery flag that
    the chrome reads verbatim).
- one [`BrowserCallbackHandoff`](../../crates/aureline-auth/src/browser_callback/mod.rs)
  validator that mints the packet through `stage_account_free_local` or
  `stage_system_browser_handoff`, runs typed validation on the returning
  browser state, and fails closed with a [`PendingSessionDeniedReason`]
  rather than silently widening the local boundary.
- one [`ShellAuthChip`](../../crates/aureline-auth/src/browser_callback/mod.rs)
  projection that compresses the packet to a single
  [`ShellAuthVocabulary`] chip the shell consumer renders next to the
  terminal pane (`Local only`, `Managed`, `Self-hosted`,
  `Managed (restricted)`, `Managed (degraded)`, `Sign in again`,
  `Sign-in not configured`, `Identity unknown`).

## Cross-tool boundary

The seed is a deliberate subset of the frozen cross-tool boundary
vocabulary in
[`docs/auth/system_browser_callback_packet.md`](system_browser_callback_packet.md)
and
[`schemas/auth/auth_callback_state.schema.json`](../../schemas/auth/auth_callback_state.schema.json).
Tokens shared between the seed and the boundary include:

- the identity-mode tokens `account_free_local`, `self_hosted_org`, and
  `managed_workspace` re-exported from ADR-0001 (the seed maps the latter
  two through [`IdentityMode::SelfHostedOrg`] and
  [`IdentityMode::ManagedConvenience`]);
- the account-boundary tokens `local_only`, `self_hosted`, `managed`,
  `restricted_managed_only`, `grace_degraded_managed`, `unknown_boundary`;
- the browser-launch policy tokens
  `system_default_browser_required`, `managed_approved_browser_allowed`,
  `separately_approved_boundary_contract`, `browser_launch_policy_blocked`;
- the embedded-fallback posture token `embedded_fallback_forbidden`;
- the return-mode tokens `loopback_http_return`, `platform_deep_link_return`,
  `device_code_poll_return`, `manual_return_resume`, `not_applicable`;
- the origin-validation tokens `strict_origin_match_required`,
  `loopback_port_pinned`, `deep_link_scheme_pinned`,
  `device_code_poll_only`, `manual_resume_only`;
- the tenant / workspace match-rule tokens
  `must_match_bound_workspace_and_tenant`, `must_match_bound_tenant`,
  `must_match_bound_workspace`, `no_tenant_or_workspace_binding`;
- the retry-path tokens `retry_in_system_browser`, `switch_to_device_code`,
  `resume_after_step_up`, `resume_after_credential_store_unlock`,
  `request_admin_policy_change`, `continue_local_without_sign_in`,
  `import_signed_session_snapshot`, `return_to_account_free_local`,
  `contact_support_with_export`, `no_recovery_without_superseding_action`;
- the preserved-local-work posture tokens
  `local_work_intact`, `local_work_intact_with_managed_narrowed`,
  `local_work_intact_with_self_hosted_narrowed`,
  `local_work_narrowed_by_workspace_trust`, `local_work_blocked_by_policy`;
  and
- the typed denial-reason tokens
  `callback_origin_mismatch`, `callback_replay_or_state_mismatch`,
  `callback_tenant_or_workspace_mismatch`,
  `callback_embedded_fallback_attempted`, `callback_policy_blocked`,
  `callback_pending_session_expired`.

The seed Rust object covers a subset of the boundary schema's fields.
Adding fields is additive-minor and does not bump the seed's
[`BROWSER_CALLBACK_PACKET_SCHEMA_VERSION`](../../crates/aureline-auth/src/browser_callback/mod.rs);
widening a vocabulary is additive-minor; repurposing a token is breaking and
requires a new decision row.

## Joined with the execution-context lane

The seed re-exports [`aureline_runtime::IdentityMode`] and
[`aureline_workspace::TrustState`] so the auth lane and the execution-context
lane stay on one identity-mode + trust-state vocabulary. Every seed packet
carries an optional `execution_context_ref` so a support export can join the
auth packet to the canonical
[`ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs)
record minted by the resolver in
[`aureline-runtime`](../../crates/aureline-runtime/src/lib.rs).

## Protected walk

1. Open a terminal session — the bottom-panel
   [`TerminalPaneSnapshot`](../../crates/aureline-shell/src/terminal_pane/mod.rs)
   consumes the seed [`BrowserCallbackPacket`] through
   `project_with_auth_packet` and renders the projected
   [`ShellAuthChip`](../../crates/aureline-auth/src/browser_callback/mod.rs).
2. Stage a managed sign-in — the chip flips to `Sign in again` (the
   `reauth_required` vocabulary), the `visible_recovery_required` flag is
   true, and the `recovery_copy_label` is quoted verbatim.
3. Browse to the system default browser through
   [`webbrowser`](../../crates/aureline-shell/Cargo.toml) — the seed
   handoff carries the bound workspace + tenant + actor refs, the loopback
   return-anchor ref, and the typed origin-validation class so the return
   path is verifiable on arrival.
4. Validate the returning browser state through
   [`BrowserCallbackHandoff::redeem`](../../crates/aureline-auth/src/browser_callback/mod.rs)
   — the packet flips to `completed` only when the state-token alias, the
   origin-validation class, and the bound tenant / workspace match the
   pending correlation; otherwise it fails closed with a typed
   [`PendingSessionDeniedReason`].
5. Confirm the no-account local path — even when the managed sign-in is
   denied the shell chip stays `local_path_available = true` and the
   [`PreservedLocalWork`](../../crates/aureline-auth/src/browser_callback/mod.rs)
   block remains readable so editing, save, undo, search, local Git, local
   tasks, and BYOK AI keep working honestly.

## Failure drill

The named failure drill from
[`.plans/M01-079.md`](../../.plans/M01-079.md) — _complete auth in the system
browser with the app partially unavailable and confirm callback handling
preserves local-versus-managed truth_ — is exercised end-to-end:

- fixture: [`fixtures/auth/browser_callback_cases/failure_drill_app_partially_unavailable.json`](../../fixtures/auth/browser_callback_cases/failure_drill_app_partially_unavailable.json)
- unit coverage:
  `aureline_auth::browser_callback::tests::embedded_fallback_attempt_fails_closed_without_widening_local_path`
- integration coverage:
  `aureline-auth/tests/browser_callback_cases.rs::failure_drill_fixture_denies_silent_embedded_fallback_and_preserves_local_work`

The fixture stages a managed sign-in handoff. A returning browser callback
attempts a silent embedded-webview fallback while the app is partially
unavailable. The validator denies the callback with
`callback_embedded_fallback_attempted`, keeps the
[`PreservedLocalWork`](../../crates/aureline-auth/src/browser_callback/mod.rs)
block intact, and the
[`ShellAuthChip`](../../crates/aureline-auth/src/browser_callback/mod.rs)
flips to `Sign in again` with `local_path_available = true` and
`visible_recovery_required = true`. The shell never silently flips into a
`Connected` badge, never widens the boundary into `managed`, and never
strands the user out of the no-account local path.

## Acceptance evidence

Spec acceptance from
[`.plans/M01-079.md`](../../.plans/M01-079.md):

- _A protected dogfood flow can exercise the behavior in the live shell
  without relying on a one-off demo or mock-only path._ Coverage:
  `crates/aureline-shell/src/terminal_pane/mod.rs::tests::snapshot_attaches_local_only_auth_chip_for_no_account_path`
  and `snapshot_attaches_reauth_required_chip_when_managed_callback_is_pending`.
- _Managed identity flows use the system browser on supported platforms and
  return to a reviewed product surface._ Coverage:
  `aureline_auth::browser_callback::tests::managed_outbound_handoff_uses_system_browser_and_forbids_embedded_fallback`
  and the fixture
  [`managed_sign_in_outbound_browser_handoff.json`](../../fixtures/auth/browser_callback_cases/managed_sign_in_outbound_browser_handoff.json).
- _The shell still exposes a no-account local path when no managed identity
  exists or the callback flow is unavailable._ Coverage:
  `aureline_auth::browser_callback::tests::account_free_local_packet_preserves_local_work_and_advertises_no_sign_in`,
  `account_free_local_cannot_stage_outbound_handoff`, and the fixture
  [`account_free_local_no_sign_in.json`](../../fixtures/auth/browser_callback_cases/account_free_local_no_sign_in.json).
- _Unit, integration, fixture, or deterministic-state coverage exists for
  nominal behavior and at least one degraded or error case._ Coverage: ten
  unit tests under `aureline_auth::browser_callback::tests` and four
  integration tests under
  `crates/aureline-auth/tests/browser_callback_cases.rs`, plus two
  consumer-side tests under `aureline-shell::terminal_pane::tests`.

## How to verify

```
cargo test -p aureline-auth
cargo test -p aureline-shell --lib terminal_pane
```

The `aureline-auth` crate runs ten unit tests (covering nominal local,
managed-outbound handoff, redeemed callback, and four typed denial paths
plus three chip projections) and four fixture-driven integration tests
(every fixture parses, the failure-drill validator denies the silent
embedded fallback while preserving local work).

The `aureline-shell` crate's terminal-pane suite adds two protected-row
tests that wire the seed packet onto the bottom-panel snapshot and assert
the projected chip.

## Out of scope (M1)

- Live OAuth / OIDC / SAML provider adapters and their wire protocols;
- Provider-specific flows (GitHub Actions check links, Jira tenant
  switchers, etc.);
- Device-code poll transport and rate-limit policy;
- Passkey / WebAuthn platform adapters; and
- Managed-admin policy-bundle formats.

The seed vocabulary above is what those integrations will land on.
