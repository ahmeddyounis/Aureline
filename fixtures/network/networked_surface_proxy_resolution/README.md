# Fixtures: networked-surface proxy-resolution governance

These fixtures document the stable networked-surface proxy-resolution governance
proof packet — the layer that makes the proxy-resolution step a first-class
governed object alongside the
[networked-surface transport decision log](../networked_surface_transport_decision/README.md).
The canonical source of truth is the seeded packet produced by
`aureline_remote::networked_surface_proxy_resolution::seeded_proxy_resolution_page()`.

Regenerate every file with the headless dump example (do not hand-edit):

```sh
cargo run -q -p aureline-remote \
  --example dump_networked_surface_proxy_resolution_fixtures -- <subcommand>
```

## Files

| File | Subcommand | Content |
|------|------------|---------|
| `page.json` | `page` | Full `ProxyResolutionGovernancePage` proof packet (stable, zero defects) |
| `rows.json` | `rows` | Per-record `ProxyResolutionRow` records (all required surfaces) |
| `defects.json` | `defects` | Empty defect list (clean stable packet) |
| `summary.json` | `summary` | `ProxyResolutionSummary` with counts, tier/outcome roll-up, and overall qualification |
| `support_export.json` | `support-export` | `ProxyResolutionSupportExport` envelope for support/diagnostics |
| `cli_view.txt` | `cli-view` | Stable CLI/headless rendering for terminal parity |
| `drills/drill_missing_surface_preview.json` | `drill-missing-surface-preview` | Missing `ai_gateway` record narrows to `preview` |
| `drills/drill_raw_material_withdrawn.json` | `drill-raw-material-withdrawn` | Raw private material on `request_api_client` withdraws the packet |
| `drills/drill_private_stack_withdrawn.json` | `drill-private-stack-withdrawn` | A private proxy stack on `docs_browser_fetcher` withdraws the packet |
| `drills/drill_ca_override_withdrawn.json` | `drill-ca-override-withdrawn` | A `no_direct_ca_override: false` record on `database_cloud_connector` withdraws the packet |
| `drills/drill_silent_fallback_withdrawn.json` | `drill-silent-fallback-withdrawn` | A silent direct-to-public fallback on `registry_read` withdraws the packet |
| `drills/drill_denied_no_reason_beta.json` | `drill-denied-no-reason-beta` | A `denied_proxy_resolution` record on `sync_offboarding` with no reason narrows that row to `beta` |
| `drills/drill_precedence_not_respected_beta.json` | `drill-precedence-not-respected-beta` | Selecting the lower system proxy over the available environment proxy on `request_api_client` narrows that row to `beta` |

## Required surfaces

All of the following network-capable surfaces must have a proxy-resolution
record for a stable claim: `ai_gateway`, `docs_browser_fetcher`,
`request_api_client`, `database_cloud_connector`, `registry_read`,
`companion_handoff`, `provider_mutation`, `sync_offboarding`,
`remote_preview_route`.

## Precedence ladder

`pac_script` > `manual_pinned` > `environment_proxy` > `system_proxy` >
`direct_no_proxy`. The `mirror_pinned` and `offline_no_route` tiers sit outside
the ladder.

## Schema

`schemas/network/networked_surface_proxy_resolution.schema.json`

## Contract ref

`remote:networked_surface_proxy_resolution:v1`
