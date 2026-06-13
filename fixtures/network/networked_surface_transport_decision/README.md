# Fixtures: networked-surface transport-decision log

These fixtures document the stable networked-surface transport-decision log
proof packet — the runtime layer that pairs with the frozen
[networked-surface transport matrix](../networked_surface_transport_matrix/README.md).
The canonical source of truth is the seeded packet produced by
`aureline_remote::networked_surface_transport_decision::seeded_transport_decision_page()`.

Regenerate every file with the headless dump example (do not hand-edit):

```sh
cargo run -q -p aureline-remote \
  --example dump_networked_surface_transport_decision_fixtures -- <subcommand>
```

## Files

| File | Subcommand | Content |
|------|------------|---------|
| `page.json` | `page` | Full `TransportDecisionLogPage` proof packet (stable, zero defects) |
| `rows.json` | `rows` | Per-decision `TransportDecisionRow` records (all required surfaces) |
| `defects.json` | `defects` | Empty defect list (clean stable packet) |
| `summary.json` | `summary` | `TransportDecisionSummary` with counts, outcome roll-up, and overall qualification |
| `support_export.json` | `support-export` | `TransportDecisionSupportExport` envelope for support/diagnostics |
| `drills/drill_missing_surface_preview.json` | `drill-missing-surface-preview` | Missing `ai_gateway` decision narrows to `preview` |
| `drills/drill_raw_material_withdrawn.json` | `drill-raw-material-withdrawn` | Raw private material on `request_api_client` withdraws the packet |
| `drills/drill_bypass_withdrawn.json` | `drill-bypass-withdrawn` | A `no_bypass: false` decision on `database_cloud_connector` withdraws the packet |
| `drills/drill_silent_public_fallback_withdrawn.json` | `drill-silent-public-fallback-withdrawn` | A silent mirror→public fall-through on `registry_read` withdraws the packet |
| `drills/drill_non_idempotent_replay_withdrawn.json` | `drill-non-idempotent-replay-withdrawn` | A non-idempotent action deferred for replay on `sync_offboarding` withdraws the packet |
| `drills/drill_denied_no_reason_beta.json` | `drill-denied-no-reason-beta` | A `denied` decision on `provider_mutation` with no reason narrows that row to `beta` |
| `drills/drill_stale_proof_beta.json` | `drill-stale-proof-beta` | A stale-beyond-window trust proof on `docs_browser_fetcher` narrows that row to `beta` |

## Required surfaces

All of the following network-capable surfaces must have a decision for a stable
claim: `ai_gateway`, `docs_browser_fetcher`, `request_api_client`,
`database_cloud_connector`, `registry_read`, `companion_handoff`,
`provider_mutation`, `sync_offboarding`, `remote_preview_route`.

## Schema

`schemas/network/networked_surface_transport_decision.schema.json`

## Contract ref

`remote:networked_surface_transport_decision:v1`
