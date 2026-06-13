# Fixtures: networked-surface transport matrix

These fixtures document the stable networked-surface transport, endpoint,
route, and trust matrix proof packet. The canonical source of truth is the
seeded packet produced by
`aureline_remote::networked_surface_transport_matrix::seeded_networked_surface_matrix_page()`.

Regenerate every file with the headless dump example (do not hand-edit):

```sh
cargo run -q -p aureline-remote \
  --example dump_networked_surface_transport_matrix_fixtures -- <subcommand>
```

## Files

| File | Subcommand | Content |
|------|------------|---------|
| `page.json` | `page` | Full `NetworkedSurfaceTransportMatrixPage` proof packet (stable, zero defects) |
| `rows.json` | `rows` | Per-surface `NetworkedSurfaceMatrixRow` records (all required surfaces) |
| `defects.json` | `defects` | Empty defect list (clean stable packet) |
| `summary.json` | `summary` | `NetworkedSurfaceMatrixSummary` with counts and overall qualification |
| `support_export.json` | `support-export` | `NetworkedSurfaceMatrixSupportExport` envelope for support/diagnostics |
| `drills/drill_missing_surface_preview.json` | `drill-missing-surface-preview` | Missing `ai_gateway` surface narrows to `preview` |
| `drills/drill_raw_material_withdrawn.json` | `drill-raw-material-withdrawn` | Raw private material on `request_api_client` withdraws the packet |
| `drills/drill_silent_public_fallback_withdrawn.json` | `drill-silent-public-fallback-withdrawn` | A silent mirror→public fall-through on `registry_read` withdraws the packet |
| `drills/drill_non_idempotent_replay_withdrawn.json` | `drill-non-idempotent-replay-withdrawn` | Non-idempotent replay queuing on `sync_offboarding` withdraws the packet |
| `drills/drill_stale_proof_beta.json` | `drill-stale-proof-beta` | A stale-beyond-window proof on `docs_browser_fetcher` narrows that row to `beta` |

## Required surfaces

All of the following network-capable surfaces must be present for a stable
claim: `ai_gateway`, `docs_browser_fetcher`, `request_api_client`,
`database_cloud_connector`, `registry_read`, `companion_handoff`,
`provider_mutation`, `sync_offboarding`, `remote_preview_route`.

## Schema

`schemas/network/networked_surface_transport_matrix.schema.json`

## Contract ref

`remote:networked_surface_transport_matrix:v1`
