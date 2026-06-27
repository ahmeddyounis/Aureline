# M5 Runbook Control-Plane Handoff Register

- Register: `m5-runbook-handoff-register:stable:0001`
- Label: `M5 runbook control-plane handoff register`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Handoffs: 4
- True control plane (handoff-required): 3 · Reference-only: 1
- Exposed on: incident workspace, operator history, support exports, docs/help

## Governed handoffs

| Handoff | Destination | Reason | Reference plane | Returns | Return anchor |
|---------|-------------|--------|-----------------|---------|---------------|
| `vendor-scale` | `vendor_console` | `execute_out_of_plane_action` | `handoff_required` | yes | `vendor-console-handoff` |
| `vendor-scaling-docs` | `browser_reference_doc` | `consult_reference_documentation` | `reference_only` | yes | `vendor-console-handoff:step:vendor.console` |
| `hosted-status-dashboard` | `browser_app_surface` | `inspect_vendor_state` | `handoff_required` | yes | `incident:vendor-scale:0014` |
| `identity-provider-sso` | `external_auth_authority` | `complete_auth_challenge` | `handoff_required` | yes | `vendor-console-handoff` |

## Reference-plane catalog

A reference-only destination can never present as in-product control.

| Destination | Class | Reference plane | In-product control |
|-------------|-------|-----------------|--------------------|
| `ref:vendor-scaling-console` | `vendor_console` | `handoff_required` | no |
| `ref:vendor-scaling-docs` | `browser_reference_doc` | `reference_only` | no |
| `ref:hosted-status-dashboard` | `browser_app_surface` | `handoff_required` | no |
| `ref:identity-provider-sso` | `external_auth_authority` | `handoff_required` | no |
