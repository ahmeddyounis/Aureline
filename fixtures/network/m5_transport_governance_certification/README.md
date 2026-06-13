# Fixtures: M5 transport-governance certification

These fixtures document the stable M5 transport-governance certification proof
packet — the milestone-exit truth source of the
[networked-surface transport-governance lane](../networked_surface_transport_matrix/README.md)
that binds the six transport-governance lanes (shared decision, proxy
resolution, trust store, host proof, mirror/offline continuity, and denial
vocabulary) into one certification verdict per named deployment profile and
auto-narrows any profile whose proof is missing, stale, or partial. The
canonical source of truth is the seeded packet produced by
`aureline_remote::m5_transport_governance_certification::seeded_m5_transport_governance_certification_page()`.

Regenerate every file with the headless dump example (do not hand-edit):

```sh
cargo run -q -p aureline-remote \
  --example dump_m5_transport_governance_certification_fixtures -- <subcommand>
```

## Files

| File | Subcommand | Content |
|------|------------|---------|
| `page.json` | `page` | Full `M5TransportGovernanceCertificationPage` proof packet (certified, zero defects) |
| `rows.json` | `rows` | Per-profile `CertificationRow` verdict rows |
| `defects.json` | `defects` | Empty defect list (clean certified packet) |
| `summary.json` | `summary` | `CertificationSummary` with counts, dimensions, denial vocabulary, and overall verdict |
| `bindings.json` | `bindings` | `DimensionBinding` evidence index, one per dimension |
| `profiles.json` | `profiles` | `ProfileCertificationSnapshot` records: cells and guardrail flags per profile |
| `support_export.json` | `support-export` | `CertificationSupportExport` envelope for support/diagnostics |
| `cells_cli_view.txt` | `cells-cli` | Headless CLI cell view (`key=value` per cell), proving CLI/support/product parity |
| `drills/drill_stale_narrowed.json` | `drill-stale-narrowed` | Stale `self_hosted` trust proof narrows that row to `narrowed` |
| `drills/drill_missing_continuity_held.json` | `drill-missing-continuity-held` | Missing `managed` mirror/offline cell holds that row back |
| `drills/drill_missing_profile_held.json` | `drill-missing-profile-held` | Missing `air_gapped` profile holds the packet back |
| `drills/drill_raw_material_withdrawn.json` | `drill-raw-material-withdrawn` | Raw private material withdraws the packet |
| `drills/drill_fallthrough_withdrawn.json` | `drill-fallthrough-withdrawn` | A silent mirror-to-public fallthrough withdraws the packet |

## Required profiles

All four named deployment profiles must have a certification record for a clean
certified packet: `local_oss`, `self_hosted`, `managed`, `air_gapped`.

## Required dimensions

Each profile must certify all six transport-governance dimensions:
`transport_decision`, `proxy_resolution`, `trust_store`, `host_proof`,
`mirror_offline`, `denial_vocabulary`. A dimension that does not apply to a
profile (e.g. proxy resolution and host proof on the no-egress `local_oss`
profile) is `waived` rather than asserted.

## Schema

`schemas/network/m5_transport_governance_certification.schema.json`

## Contract ref

`remote:m5_transport_governance_certification:v1`
