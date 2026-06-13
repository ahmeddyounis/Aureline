# Fixtures: networked-surface transport-explainability

These fixtures document the stable networked-surface transport-explainability
proof packet — the product-grade layer over the
[networked-surface transport-decision log](../networked_surface_transport_decision/README.md).
The canonical source of truth is the seeded packet produced by
`aureline_remote::networked_surface_transport_explainability::seeded_transport_explainability_page()`.

Regenerate every file with the headless dump example (do not hand-edit):

```sh
cargo run -q -p aureline-remote \
  --example dump_networked_surface_transport_explainability_fixtures -- <subcommand>
```

## Files

| File | Subcommand | Content |
|------|------------|---------|
| `page.json` | `page` | Full `TransportExplainabilityPage` proof packet (stable, zero defects) |
| `rows.json` | `rows` | Per-surface `TransportExplainabilityRow` records (all required surfaces) |
| `defects.json` | `defects` | Empty defect list (clean stable packet) |
| `summary.json` | `summary` | `TransportExplainabilitySummary` with counts, disposition roll-up, and overall qualification |
| `posture_inspectors.json` | `posture-inspectors` | `TransportPostureInspector` records: effective proxy mode, trust source, mirror/offline state per surface |
| `event_ledger.json` | `event-ledger` | `NetworkEventLedger` with one filterable event per recent decision |
| `explain_sheets.json` | `explain-sheets` | `ActionExplainSheet` records rendered through the shared field catalog |
| `explain_cli_view.txt` | `explain-cli` | Headless CLI explain view (`key=value` per action), proving CLI/support/product parity |
| `support_export.json` | `support-export` | `TransportExplainabilitySupportExport` envelope for support/diagnostics |
| `drills/drill_missing_surface_preview.json` | `drill-missing-surface-preview` | Missing `ai_gateway` coverage narrows to `preview` |
| `drills/drill_raw_material_withdrawn.json` | `drill-raw-material-withdrawn` | Raw private material on `database_cloud_connector` withdraws the packet |
| `drills/drill_bypass_withdrawn.json` | `drill-bypass-withdrawn` | A `no_bypass: false` decision on `provider_mutation` withdraws the packet |
| `drills/drill_denied_no_reason_beta.json` | `drill-denied-no-reason-beta` | A `denied` event on `provider_mutation` with no reason narrows that row to `beta` |
| `drills/drill_stale_proof_beta.json` | `drill-stale-proof-beta` | A stale-beyond-window trust proof on `docs_browser_fetcher` narrows that row to `beta` |

## Required surfaces

All of the following network-capable surfaces must have a posture inspector and
an explain sheet for a stable claim: `ai_gateway`, `docs_browser_fetcher`,
`request_api_client`, `database_cloud_connector`, `registry_read`,
`companion_handoff`, `provider_mutation`, `sync_offboarding`,
`remote_preview_route`.

## Schema

`schemas/network/networked_surface_transport_explainability.schema.json`

## Contract ref

`remote:networked_surface_transport_explainability:v1`
