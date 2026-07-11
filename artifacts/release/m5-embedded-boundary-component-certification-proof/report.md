# M5 Embedded-Boundary Component Surface Certification

- Packet: `m5-embedded-boundary-component-certification:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-embedded-boundary-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:docs-help-pane** — surface=docs_help_pane claimed=full_truth certified=full_truth status=green narrowed_axes=0
- **cert:marketplace-pane** — surface=marketplace_pane claimed=resolved_truth certified=resolved_truth status=green narrowed_axes=0
- **cert:account-pane** — surface=account_pane claimed=resolved_truth certified=resolved_truth status=green narrowed_axes=0
- **cert:support-export** — surface=support_export claimed=resolved_truth certified=resolved_truth status=green narrowed_axes=0
- **cert:embedded-webview** — surface=embedded_webview claimed=full_truth certified=degraded status=yellow narrowed_axes=1
- **cert:remote-service-dashboard** — surface=remote_service_dashboard claimed=full_truth certified=stale status=yellow narrowed_axes=1
- **cert:auth-handoff** — surface=auth_handoff claimed=resolved_truth certified=offline status=yellow narrowed_axes=1
- **cert:cli-headless** — surface=cli_headless claimed=resolved_truth certified=provider_blocked status=yellow narrowed_axes=1
