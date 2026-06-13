# Fixtures: networked-surface mirror/offline continuity

These fixtures document the stable networked-surface mirror/offline continuity
proof packet — the capstone of the
[networked-surface transport-governance lane](../networked_surface_transport_matrix/README.md)
that makes mirror-only, local-file-bundle, public-direct, blocked, and deferred
route handling explicit across the claimed M5 artifact families. The canonical
source of truth is the seeded packet produced by
`aureline_remote::networked_surface_mirror_offline_continuity::seeded_mirror_offline_continuity_page()`.

Regenerate every file with the headless dump example (do not hand-edit):

```sh
cargo run -q -p aureline-remote \
  --example dump_networked_surface_mirror_offline_continuity_fixtures -- <subcommand>
```

## Files

| File | Subcommand | Content |
|------|------------|---------|
| `page.json` | `page` | Full `MirrorOfflineContinuityPage` proof packet (stable, zero defects) |
| `rows.json` | `rows` | Per-family `MirrorOfflineContinuityRow` records (all required families) |
| `defects.json` | `defects` | Empty defect list (clean stable packet) |
| `summary.json` | `summary` | `MirrorOfflineContinuitySummary` with counts, route-handling roll-up, and overall qualification |
| `records.json` | `records` | `ContinuityRecord` records: route handling, stale-mirror warning, public-fallback rule per family |
| `continuity_cli_view.txt` | `continuity-cli` | Headless CLI continuity view (`key=value` per family), proving CLI/support/product parity |
| `support_export.json` | `support-export` | `MirrorOfflineContinuitySupportExport` envelope for support/diagnostics |
| `drills/drill_missing_family_preview.json` | `drill-missing-family-preview` | Missing `docs_pack` coverage narrows to `preview` |
| `drills/drill_raw_material_withdrawn.json` | `drill-raw-material-withdrawn` | Raw private material on `registry` withdraws the packet |
| `drills/drill_silent_fallback_withdrawn.json` | `drill-silent-fallback-withdrawn` | A mirror-only `registry` flipping to `public_direct` withdraws the packet |
| `drills/drill_non_idempotent_withdrawn.json` | `drill-non-idempotent-withdrawn` | A non-idempotent `companion_handoff` queued for replay withdraws the packet |
| `drills/drill_blocked_no_reason_beta.json` | `drill-blocked-no-reason-beta` | A `blocked` `model_pack` with no reason narrows that row to `beta` |
| `drills/drill_stale_mirror_beta.json` | `drill-stale-mirror-beta` | A stale-beyond-grace mirror still served on `registry` narrows that row to `beta` |

## Required artifact families

All of the following M5 artifact families must have a continuity record for a
stable claim: `docs_pack`, `registry`, `model_pack`, `request_workspace`,
`companion_handoff`.

## Route-handling vocabulary

Each family distinguishes exactly one of: `mirror_route`, `local_file_bundle`,
`public_direct`, `blocked`, `deferred`.

## Schema

`schemas/network/networked_surface_mirror_offline_continuity.schema.json`

## Contract ref

`remote:networked_surface_mirror_offline_continuity:v1`
