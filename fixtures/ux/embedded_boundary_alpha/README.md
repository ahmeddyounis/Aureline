# Embedded Boundary Alpha Fixtures

These fixtures are the shell-consumable proof set for embedded-surface boundary
chrome. They complement the broader `fixtures/ux/embedded_boundary_cases/`
corpus by staying small enough for `aureline-shell` unit tests and runtime
support evidence.

- `extension_webview_alpha_card.json` covers an extension-owned webview with
  cross-origin limits and a host-owned browser handoff.
- `marketplace_account_alpha_card.json` covers marketplace/account content with
  current account scope, stale provider health, service boundary, and browser
  renewal handoff.
- `manifest.yaml` names the docs/help baseline card and the assertions the
  shell projection must keep true. It also lists the native handoff and native
  file affordance fixtures consumed by the shell runtime packet.
