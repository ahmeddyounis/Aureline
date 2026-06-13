# Fixtures: networked-surface transport automation

These fixtures document the stable networked-surface transport-automation proof
packet — the audit and automation truth source of the
[networked-surface transport-governance lane](../networked_surface_transport_matrix/README.md)
that gives M5 network activity history, support exports, and headless automation
one canonical denial vocabulary, one set of network-activity filters, and one
redaction-safe route/origin history join. The canonical source of truth is the
seeded packet produced by
`aureline_remote::networked_surface_transport_automation::seeded_transport_automation_page()`.

Regenerate every file with the headless dump example (do not hand-edit):

```sh
cargo run -q -p aureline-remote \
  --example dump_networked_surface_transport_automation_fixtures -- <subcommand>
```

## Files

| File | Subcommand | Content |
|------|------------|---------|
| `page.json` | `page` | Full `TransportAutomationPage` proof packet (stable, zero defects) |
| `rows.json` | `rows` | Per-record `TransportAutomationRow` stability rows (all required surfaces) |
| `defects.json` | `defects` | Empty defect list (clean stable packet) |
| `summary.json` | `summary` | `TransportAutomationSummary` with counts, denial vocabulary, and overall qualification |
| `records.json` | `records` | `NetworkActivityRecord` records: surface, origin scope, route choice, disposition, denial code |
| `route_origin_joins.json` | `joins` | `RouteOriginJoinRow` aggregates keyed by `(route_choice, origin_scope)` |
| `filter_facets.json` | `facets` | `ActivityFilterFacet` values per filter dimension |
| `activity_cli_view.txt` | `activity-cli` | Headless CLI activity view (`key=value` per record), proving CLI/support/product parity |
| `denied_filter_view.txt` | `denied-filter` | Headless `ActivityFilter` result: every denied action with its canonical denial code |
| `support_export.json` | `support-export` | `TransportAutomationSupportExport` envelope for support/diagnostics |
| `drills/drill_missing_surface_preview.json` | `drill-missing-surface-preview` | Missing `ai_gateway` coverage narrows to `preview` |
| `drills/drill_raw_material_withdrawn.json` | `drill-raw-material-withdrawn` | Raw private material on `docs_browser_fetcher` withdraws the packet |
| `drills/drill_bypass_withdrawn.json` | `drill-bypass-withdrawn` | A `request_api_client` bypass of shared governance withdraws the packet |
| `drills/drill_non_idempotent_withdrawn.json` | `drill-non-idempotent-withdrawn` | A non-idempotent `companion_handoff` queued for replay withdraws the packet |
| `drills/drill_denied_no_code_beta.json` | `drill-denied-no-code-beta` | A `denied` `provider_mutation` with no canonical code narrows that row to `beta` |
| `drills/drill_disposition_mismatch_beta.json` | `drill-disposition-mismatch-beta` | An `allowed` `ai_gateway` carrying a denial code narrows that row to `beta` |

## Required surfaces

All nine M5 networked surfaces must have an activity record for a stable claim:
`ai_gateway`, `docs_browser_fetcher`, `request_api_client`,
`database_cloud_connector`, `registry_read`, `companion_handoff`,
`provider_mutation`, `sync_offboarding`, `remote_preview_route`.

## Canonical denial vocabulary

All eight required denial codes must be surfaced across the activity history:
`proxy_misconfigured`, `proxy_auth_required`, `ca_untrusted`,
`ssh_host_key_unknown`, `egress_blocked_policy`, `mirror_unreachable`,
`offline_mode`, `origin_scope_ambiguous`. `none` is the allowed-action sentinel.

## Schema

`schemas/network/networked_surface_transport_automation.schema.json`

## Contract ref

`remote:networked_surface_transport_automation:v1`
