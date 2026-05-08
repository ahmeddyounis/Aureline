# Proof dashboards review workflow (seed)

Canonical dashboard definition: `artifacts/dashboards/m1_protected_functions.json`.
Canonical proof index: `artifacts/milestones/m1/artifact_index.yaml`.

This workflow keeps the protected-functions proof dashboard exportable into
milestone packets, so reviewers can reference a checked-in snapshot instead of
depending on a live dashboard.

## When to use this workflow

Use this workflow when a change:

- edits the protected-functions dashboard definition (rows, links, thresholds); or
- changes proof-lane freshness / captures in `artifacts/milestones/m1/artifact_index.yaml` and the dashboard snapshot should be refreshed.

## Export a snapshot (fail closed on structural errors)

Run:

`python3 tools/ci/publish_m1_dashboard_snapshot.py --repo-root . --report artifacts/milestones/m1/captures/proof_dashboards_validation_capture.json --snapshot artifacts/milestones/m1/captures/proof_dashboards_snapshot.json`

Then review:

- Validation capture: `artifacts/milestones/m1/captures/proof_dashboards_validation_capture.json`
- Snapshot: `artifacts/milestones/m1/captures/proof_dashboards_snapshot.json`

## Review steps

1. Confirm each dashboard row deep-links to at least one `lane_id` in
   `artifacts/milestones/m1/artifact_index.yaml` and that the lane’s owning
   proof packet is correct.
2. Confirm “planned not yet seeded” lanes render as gaps (not a pass) in the
   exported snapshot.
3. If a proof lane is marked current/needs_refresh/stale, confirm it carries a
   `latest_capture` with a real `report_ref` so the snapshot can deep-link to
   evidence.

## Failure drill (guardrail)

To confirm the wiring degrades with visible gaps and errors rather than false
green:

1. Temporarily change one dashboard row’s `proof_lane_ids` to a non-existent
   value.
2. Rerun the export command; it must fail with an actionable error.
3. Undo the change and rerun; it must pass and re-emit the snapshot.

