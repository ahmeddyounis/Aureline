# M5 historical-reference, archived-snapshot, imported/offline-evidence, and live-target-handoff operations

This is the human-readable companion to the frozen **M5 historical-reference matrix** that opens
batch **B149**. The canonical, machine-checked source of truth is the Rust module
`m5_historical_reference_matrix` in `crates/aureline-ui` and the export-safe packet it mints; this
document exists so support, help/docs, review, runbook-archive, and companion/export owners can read
the same non-live-evidence contract without re-deriving it.

- Combined matrix schema: `schemas/program/m5-historical-reference-matrix.schema.json`
- Domain schemas:
  - `schemas/program/m5-historical-snapshot-descriptor.schema.json`
  - `schemas/program/m5-live-target-handoff.schema.json`
  - `schemas/program/m5-imported-offline-evidence-state.schema.json`
- Checked support export: `artifacts/support/m5-historical-evidence/support_export.json`
- Machine-readable CSV: `artifacts/support/m5-historical-evidence/matrix.csv`
- Design report: `artifacts/program/m5-historical-reference-matrix.md`
- Health dashboard: `dashboards/m5-historical-evidence-health.json`
- Narrowed fixtures: `fixtures/recovery/m5-historical-snapshots/`

## What this matrix locks

It freezes the object classes that Aureline presents as **preserved / non-live evidence** rather than
as ordinary editable live objects, and the required visible state each must carry so an archived
snapshot or an imported/offline packet can never be mistaken for a current live object or reopen
through an ambiguous route.

### Covered historical-reference object classes

| Object class | Canonical domain schema | Owner (backup) |
| --- | --- | --- |
| `retirement_snapshot` | historical-snapshot-descriptor | Retirement-snapshot evidence owner (Release-governance) |
| `support_export_evidence` | historical-snapshot-descriptor | Support / export evidence owner (Support-governance) |
| `archived_runbook_packet` | live-target-handoff | Archived-runbook evidence owner (Runbook-governance) |
| `imported_offline_route_evidence` | imported-offline-evidence-state | Imported / offline route-evidence owner (Continuity-governance) |
| `review_incident_snapshot` | live-target-handoff | Review / incident snapshot owner (Incident-governance) |

### Required visible state (every class)

Each row carries `snapshot label`, `capture time`, `provenance`, `live-target availability`,
`imported/offline status`, `mutation-blocked posture`, `expiry/removed handling`, and an explicit
`live-target handoff or metadata-only exit`. The `evidence_state` field makes captured/archived and
imported/offline evidence mechanically distinct from ordinary live objects, read-only cached current
state, and restore-capable workspaces.

### Hard invariants (all MUST be false on every row)

1. Lets archived or imported/offline evidence look live, writable, or current by omission.
2. Reopens a live target from a snapshot without validating target identity, trust, route, and
   authority first.
3. Dead-links an expired/removed artifact instead of showing metadata, provenance, or cleanup state.
4. Leaves non-live evidence unjoined to capture time, provenance, retention/removal state, or any
   current live-target mismatch.
5. Presents a snapshot or imported/offline packet as a current live object or reopens through an
   ambiguous route.

## Regenerating the checked-in artifacts

The headless emitter is the only mint-from-truth path:

```text
cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- support-export
cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- csv
cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- report
cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- dashboard
cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- fixture-imported-offline-route-evidence-beta-narrowed
cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- fixture-review-incident-snapshot-preview-narrowed
cargo run -p aureline-ui --example dump_m5_historical_reference_matrix -- validate
```

The inline tests assert the checked-in support export, dashboard, and fixtures never drift from the
seed builder, so the emitter output is deterministic and export-safe (metadata only; no secrets or
private endpoints cross the boundary).

## Scope

This row is matrix-only: it defines the canonical object classes, vocabulary, and mandatory state
transitions. It does not build every consumer — later B149 rows implement the archive viewers,
compare / open-live-target handoff plumbing, and support / help / review / retirement consumers that
adopt this contract.
