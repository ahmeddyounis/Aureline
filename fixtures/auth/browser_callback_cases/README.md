# Browser callback seed cases

Worked examples for the M1 system-browser auth callback seed in
`crates/aureline-auth/src/browser_callback/`. Each fixture exercises the seed
[`BrowserCallbackPacket`](../../../crates/aureline-auth/src/browser_callback/mod.rs)
shape: `auth_flow_class`, `account_boundary_class`, `pending_session_state`,
`callback_correlation`, `return_route`, `preserved_local_work`, and
`recovery_path`.

The seed packet is a deliberate subset of the frozen cross-tool boundary
vocabulary in
[`/docs/auth/system_browser_callback_packet.md`](../../../docs/auth/system_browser_callback_packet.md)
and
[`/schemas/auth/auth_callback_state.schema.json`](../../../schemas/auth/auth_callback_state.schema.json).
Seed and packet share identity-mode tokens, account-boundary tokens, browser-
launch policy tokens, return-mode and origin-validation tokens, retry-path
tokens, embedded-fallback posture tokens, preserved-local-work posture tokens,
and pending-session-denied reason tokens — so the seed grows additively into
the full packet without forking truth.

## Cases

- `account_free_local_no_sign_in.json` — no-account local path. Editing,
  save, undo, search, local Git, local tasks, and BYOK AI all stay usable
  without sign-in. Acceptance: the shell still exposes a no-account local
  path when no managed identity exists.
- `managed_sign_in_outbound_browser_handoff.json` — a managed sign-in
  handoff routed through the system default browser, with loopback HTTP
  return, strict tenant + workspace match, and visible recovery copy.
  Acceptance: managed identity flows use the system browser on supported
  platforms and return to a reviewed product surface.
- `failure_drill_app_partially_unavailable.json` — the named failure drill:
  the user completes auth in the system browser while the app is partially
  unavailable (an embedded fallback was attempted). The packet fails closed
  with `callback_embedded_fallback_attempted`, preserves the
  preserved-local-work block, and routes to a typed visible-recovery row so
  the no-account local path stays truthful.

## How to verify

```
cargo test -p aureline-auth
```

The crate's integration test
[`tests/browser_callback_cases.rs`](../../../crates/aureline-auth/tests/browser_callback_cases.rs)
parses every fixture in this directory and asserts the seed contract for the
`auth_flow_class`, `account_boundary_class`, `pending_session_state`, the
preserved-local-work block, and the typed denial reason on the failure-drill
fixture.
