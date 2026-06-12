# Fixtures: managed_workspace_lifecycle

These fixtures document the stable managed-workspace lifecycle proof packet for
the M5 remote and companion flows. The canonical source of truth is the seeded
packet produced by
`aureline_remote::managed_workspace_lifecycle::seeded_managed_workspace_lifecycle_page()`.

Regenerate every file with the dump example:

```sh
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- page    > fixtures/remote/managed_workspace_lifecycle/page.json
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- rows    > fixtures/remote/managed_workspace_lifecycle/rows.json
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- defects > fixtures/remote/managed_workspace_lifecycle/defects.json
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- summary > fixtures/remote/managed_workspace_lifecycle/summary.json
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- support-export > fixtures/remote/managed_workspace_lifecycle/support_export.json
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- drill-missing-state-flagged          > fixtures/remote/managed_workspace_lifecycle/drills/missing_state_flagged.json
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- drill-raw-material-withheld          > fixtures/remote/managed_workspace_lifecycle/drills/raw_material_withheld.json
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- drill-continuity-overclaim-narrowed  > fixtures/remote/managed_workspace_lifecycle/drills/continuity_overclaim_narrowed.json
cargo run -q -p aureline-remote --example dump_managed_workspace_lifecycle_fixtures -- drill-expiry-no-local-safe-narrowed  > fixtures/remote/managed_workspace_lifecycle/drills/expiry_no_local_safe_narrowed.json
```

## Files

| File | Content |
|------|---------|
| `page.json` | Full `ManagedWorkspaceLifecyclePage` proof packet (truthful, zero defects) |
| `rows.json` | All ten `LifecycleMatrixRow` disposition rows |
| `defects.json` | Empty defect list (clean truthful packet) |
| `summary.json` | `LifecycleSummary` with counts and overall disposition |
| `support_export.json` | `LifecycleSupportExport` envelope for support/diagnostics |
| `drills/missing_state_flagged.json` | Drill: missing `resumed` state flags the packet |
| `drills/raw_material_withheld.json` | Drill: raw private material on the `ready` record withholds the packet |
| `drills/continuity_overclaim_narrowed.json` | Drill: `resumed` claims exact continuity over a changed image — narrowed |
| `drills/expiry_no_local_safe_narrowed.json` | Drill: `expired` offers no local-safe continuation — narrowed |

The `raw_material_withheld.json` drill intentionally sets
`raw_private_material_excluded: false` on a record to exercise the hard
withhold guardrail; it therefore does not validate against the record schema's
`const true` constraint. Only the canonical `page.json` (and the other clean
fixtures) validate against the schema.

## Required lifecycle states

All ten states must be present for a truthful claim, in canonical order:

`provision`, `warm`, `ready`, `suspended`, `resumed`, `reconnecting`,
`rebuild_required`, `recreate_required`, `expired`, `local_safe_continuation`.

## Required surfaces

Each record must reach every consuming surface:

`desktop`, `preview_route`, `companion_handoff`, `incident_packet`,
`support_export`.

## Schema

`schemas/remote/managed_workspace_lifecycle.schema.json`

## Contract ref

`remote:managed_workspace_lifecycle:v1`
