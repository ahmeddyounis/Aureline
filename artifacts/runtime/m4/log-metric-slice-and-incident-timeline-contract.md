# Log/Metric Slice and Incident Timeline Contract Artifact

This artifact records the stable runtime evidence packet family for
observability slices, incident chronology, runbook execution, and
support/incident export continuity.

## Produced Artifacts

- `crates/aureline-runtime/src/log_metric_slice_and_incident_timeline_contract/`
- `schemas/runtime/log-metric-slice-and-incident-timeline-contract.schema.json`
- `docs/runtime/m4/log-metric-slice-and-incident-timeline-contract.md`
- `fixtures/runtime/m4/log-metric-slice-and-incident-timeline-contract/`

## Stable Claims

- Log, metric, and trace slices carry source identity, target scope, query or
  filter identity, timezone-aware windows, freshness, sampling/truncation
  posture, export/redaction posture, and linked incident timeline refs.
- Incident timeline entries preserve event ids, actor lineage, timezone-aware
  chronology, affected scope, outcome, and typed links to slices, runs,
  artifacts, provider events, repairs, approvals, and runbook steps.
- Runbook step executions preserve status, actor, target, approvals,
  deviations, external-console handoffs, evidence refs, rollback refs, and
  export continuity across UI, CLI/headless, and support packets.
- Stable consumer projections must expose `live`, `buffering`, `cached`,
  `imported`, `stale`, `partial`, `offline`, `truncated`, and `exported_copy`
  without collapsing imported, stale, truncated, or exported evidence into live
  truth.
- Export bundles distinguish embedded evidence from by-reference evidence and
  declare redactions or omissions explicitly.

## Fixture Coverage

- Baseline stable packet covering log, metric, and trace slices; incident
  chronology; one approved mutating runbook execution; support export; and
  incident export projection.
- Negative drill for a stable projection that collapses freshness vocabulary.
- Negative drill for a timeline entry with missing timezone chronology.
- Negative drill for a mutating runbook execution missing approval provenance.
