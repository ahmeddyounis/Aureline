# System-Browser Fallback Alpha

This page is the reviewer entry point for claimed identity rows that need
managed, self-hosted, or provider-linked auth.

## Runtime Contract

The canonical row is
[`ClaimedIdentityRow`](../../crates/aureline-auth/src/system_browser/mod.rs).
It records:

- provider/domain and tenant, org, workspace, or provider scope;
- browser-launch policy and embedded-auth posture;
- auth timeout or expiry;
- default action selected by the resolver;
- device-code and stay-local alternatives;
- preserved local-work capabilities and narrowed managed capabilities;
- refs to callback, browser-handoff, native boundary, embedded boundary, and
  managed-session records.

Claimed managed/provider rows default to `open_system_browser` when policy
allows system-browser auth. If browser launch is blocked but device-code is
supported, the default becomes `use_device_code`. If auth is denied or expired,
the default becomes `continue_local_without_sign_in` when local continuation is
available.

## Shell Consumer

The first shell consumer is
[`TerminalPaneSnapshot`](../../crates/aureline-shell/src/terminal_pane/mod.rs).
It attaches [`ClaimedIdentitySurfaceRow`](../../crates/aureline-auth/src/system_browser/mod.rs)
records from a `SystemBrowserAlphaPacket` and renders provider/domain scope,
expiry, default action, available alternatives, local-continuity copy, and
native-boundary refs next to the existing shell-auth chip.

The shell does not infer a generic signed-in state from these rows. It quotes
the auth packet and surface rows so support/export evidence sees the same
scope, expiry, and fallback truth as the live chrome.

## Protected Fixtures

Fixtures live under
[`fixtures/auth/system_browser_alpha`](../../fixtures/auth/system_browser_alpha/):

- `managed_claim_prefers_system_browser.json` proves the normal managed row
  prefers system-browser auth while exposing device-code and stay-local
  alternatives.
- `browser_blocked_device_code_or_stay_local.json` proves browser-launch policy
  denial defaults to device-code and keeps stay-local visible.
- `scope_denied_stay_local.json` proves denied auth does not create a dead end;
  the row defaults to local continuation and still exposes retry paths.

All fixtures use refs and aliases only. Raw tokens, raw device codes, raw
cookies, raw provider error bodies, and raw PKCE material are excluded.

## Verification

```sh
cargo test -p aureline-auth system_browser --no-fail-fast
cargo test -p aureline-auth --test system_browser_alpha_cases --no-fail-fast
cargo test -p aureline-shell terminal_pane::tests::snapshot_surfaces_claimed_identity_scope_expiry_and_fallbacks --no-fail-fast
```
