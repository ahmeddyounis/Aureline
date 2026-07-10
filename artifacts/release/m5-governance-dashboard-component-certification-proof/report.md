# M5 Governance-Dashboard Component Surface Certification

- Packet: `m5-governance-dashboard-component-certification:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-governance-dashboard-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:assurance-center** — surface=assurance_center claimed=governed_pass certified=governed_pass status=green narrowed_axes=0
- **cert:release-center** — surface=release_center claimed=governed_pass certified=governed_pass status=green narrowed_axes=0
- **cert:support-export** — surface=support_export claimed=governed_resolved certified=governed_resolved status=green narrowed_axes=0
- **cert:docs-help** — surface=docs_help claimed=governed_resolved certified=governed_resolved status=green narrowed_axes=0
- **cert:operator-overview** — surface=operator_overview claimed=governed_pass certified=provisional status=yellow narrowed_axes=1
- **cert:service-health** — surface=service_health claimed=governed_pass certified=degraded status=yellow narrowed_axes=1
- **cert:shiproom** — surface=shiproom claimed=governed_pass certified=degraded status=yellow narrowed_axes=1
- **cert:cli-headless** — surface=cli_headless claimed=governed_resolved certified=waiver_gated status=yellow narrowed_axes=1
