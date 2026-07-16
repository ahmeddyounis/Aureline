# M5 Historical-Evidence Drill Corpus

The historical-evidence drill corpus is the B149 fixture-corpus + regression-drill lane over the five non-live-evidence
object classes frozen in the historical-reference matrix. It seeds the reusable corpus QA, release, and support pull to
prove that the archived-snapshot, imported / offline evidence, and live-target handoff loops stay honest under failure:
missing targets, retired lines, stale imports, expired snapshots, and evidence-only reopen paths.

- Boundary schema: `schemas/program/m5-historical-evidence-drill-corpus.schema.json`
- Support export: `artifacts/support/m5-historical-evidence-drills/support_export.json`
- Matrix CSV: `artifacts/support/m5-historical-evidence-drills/matrix.csv`
- Markdown summary: `artifacts/support/m5-historical-evidence-drills/summary.md`
- Health dashboard: `dashboards/m5-historical-evidence-drill-health.json`
- Narrowed fixtures: `fixtures/recovery/m5-historical-evidence-drills/`

## Seeded fixtures

The corpus seeds four fixture families across the five historical-reference object classes, each with known provenance
and handoff expectations:

- **Last-supported retirement snapshot** (`retirement_snapshot`).
- **Captured support / export evidence bundle** (`support_export_evidence`).
- **Runbook / incident archived packet** (`archived_runbook_packet` and `review_incident_snapshot`).
- **Imported / offline route packet** (`imported_offline_route_evidence`).

Each fixture carries the controlled non-live grammar — a frozen historical-role word plus snapshot-label, capture-time,
provenance, and mutation-blocked-posture words — identical across every surface that renders the same fixture, and joins
its provenance back to a source snapshot descriptor.

## Drills

Six drills exercise each fixture. One clears the live-target handoff; the other five block it with an exact, named
blocker and fall back to a satisfy-prerequisite or metadata-only exit rather than a dead end:

| Drill | Historical-reference state | Handoff outcome | Exact blocker | Fallback |
| --- | --- | --- | --- | --- |
| `preserved_live_target_handoff` | `preserved_live_target_joinable` | `handoff_cleared` | `none_cleared` | open current live object |
| `missing_live_target` | `missing_live_target_metadata_only` | `blocked_target_unavailable` | `missing_target` | metadata-only exit |
| `retired_line_reopen` | `retired_line_no_live_counterpart` | `blocked_needs_prerequisite` | `route_unavailable` | satisfy prerequisite (migration) |
| `stale_imported_evidence` | `stale_imported_evidence` | `blocked_needs_prerequisite` | `trust_block` | satisfy prerequisite (re-import) |
| `expired_snapshot_metadata_only_fallback` | `expired_snapshot_metadata_fallback` | `blocked_by_policy` | `expired_snapshot` | metadata-only exit (content gone) |
| `evidence_only_reopen_after_version_schema_drift` | `imported_offline_evidence_only` | `blocked_target_unavailable` | `imported_offline_evidence_only` | metadata-only exit |

Each exact blocker maps into the live-target-handoff module's own `HandoffBlockerReason` vocabulary, so QA / support
automation can mechanically separate failure modes rather than seeing a single generic failure.

## Acceptance criteria coverage

1. **At least four distinct historical-reference states and two distinct live-target handoff outcomes.** The seeded
   corpus covers all six states and all four outcomes (`handoff_cleared` plus three blocked outcomes).
2. **Exact blockers are distinguishable.** All six blockers (`none_cleared`, `missing_target`, `trust_block`,
   `route_unavailable`, `expired_snapshot`, `imported_offline_evidence_only`) appear, and each blocker's required
   handoff outcome is validated against its binding.
3. **The corpus is referenced by release and support evidence, not an ad hoc sample set.** Every binding binds back to a
   screenshot, an accessibility check, the CLI / support export, and the health dashboard; the packet points at the
   canonical matrix and per-domain schemas.

## Regeneration

The seed builders in `crates/aureline-ui/src/m5_historical_evidence_drill_corpus/` are the only mint-from-truth path.
Regenerate the checked-in artifacts with the headless emitter:

```text
cargo run -p aureline-ui --example dump_m5_historical_evidence_drill_corpus -- support-export > artifacts/support/m5-historical-evidence-drills/support_export.json
cargo run -p aureline-ui --example dump_m5_historical_evidence_drill_corpus -- csv > artifacts/support/m5-historical-evidence-drills/matrix.csv
cargo run -p aureline-ui --example dump_m5_historical_evidence_drill_corpus -- report > artifacts/support/m5-historical-evidence-drills/summary.md
cargo run -p aureline-ui --example dump_m5_historical_evidence_drill_corpus -- dashboard > dashboards/m5-historical-evidence-drill-health.json
cargo run -p aureline-ui --example dump_m5_historical_evidence_drill_corpus -- fixture-missing-target-narrowed > fixtures/recovery/m5-historical-evidence-drills/missing_target_narrowed.json
cargo run -p aureline-ui --example dump_m5_historical_evidence_drill_corpus -- fixture-expired-snapshot-narrowed > fixtures/recovery/m5-historical-evidence-drills/expired_snapshot_narrowed.json
```
