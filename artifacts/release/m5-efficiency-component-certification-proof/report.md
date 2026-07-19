# M5 Adaptive-Efficiency Component Surface Certification

- Packet: `m5-efficiency-component-certification:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-efficiency-components-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:shell-status-bar** — surface=shell_status_bar claimed=full_truth certified=full_truth status=green narrowed_axes=0
- **cert:activity-center** — surface=activity_center claimed=full_truth certified=full_truth status=green narrowed_axes=0
- **cert:support-export** — surface=support_export claimed=resolved_truth certified=resolved_truth status=green narrowed_axes=0
- **cert:docs-help** — surface=docs_help claimed=resolved_truth certified=resolved_truth status=green narrowed_axes=0
- **cert:work-content-canvas** — surface=work_content_canvas claimed=full_truth certified=degraded status=yellow narrowed_axes=1
- **cert:policy-aware-settings** — surface=policy_aware_settings claimed=full_truth certified=policy_blocked status=yellow narrowed_axes=1
- **cert:incident-diagnostics** — surface=incident_diagnostics claimed=full_truth certified=deferred status=yellow narrowed_axes=1
- **cert:cli-headless** — surface=cli_headless claimed=resolved_truth certified=stale_shown status=yellow narrowed_axes=1
