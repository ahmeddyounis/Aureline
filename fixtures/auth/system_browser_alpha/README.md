# System Browser Alpha Fixtures

These fixtures prove the claimed-identity row contract consumed by the auth
crate and shell projection.

- `managed_claim_prefers_system_browser.json` covers a managed/provider-linked
  row where the default action is system-browser auth and device-code plus
  stay-local alternatives remain visible.
- `browser_blocked_device_code_or_stay_local.json` covers browser-launch policy
  denial where the default action becomes device-code and the stay-local path
  remains visible.
- `scope_denied_stay_local.json` covers auth scope denial where the row does
  not strand the user and defaults to local continuation.

The fixtures deliberately store aliases and refs only. Raw tokens, raw device
codes, raw cookies, raw URLs, raw PKCE verifiers, and raw provider error bodies
do not appear.
