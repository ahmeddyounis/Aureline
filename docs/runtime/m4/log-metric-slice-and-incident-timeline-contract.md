# Log/Metric Slice and Incident Timeline Contract

This contract is the stable evidence boundary for logs, metrics, traces,
incident timelines, runbook execution, and operational exports. The canonical
Rust contract lives in
`crates/aureline-runtime/src/log_metric_slice_and_incident_timeline_contract/`
and the boundary schema lives at
`schemas/runtime/log-metric-slice-and-incident-timeline-contract.schema.json`.

Stable observability and incident consumers must read this packet shape instead
of deriving local dashboard semantics. A surface that cannot prove slice
identity, timezone-aware chronology, freshness honesty, and runbook execution
provenance must narrow to inspect-only or handoff-only posture.

## Contract Objects

- `SignalSlice` is the canonical log, metric, or trace slice. It carries
  source identity, target scope, query/filter hash, timezone-aware time window,
  freshness state, sample/truncation posture, export/redaction class, collection
  time, and linked incident timeline refs.
- `IncidentTimelineEntry` is append-only chronology. It preserves event id,
  incident ref, event time, timezone, actor lineage, action class, affected
  scope, outcome, and links to slices, runs, artifacts, provider events,
  repairs, approvals, and runbook steps.
- `RunbookPacket` pins runbook identity, version, source class, step ids,
  approval requirements, expected evidence outputs, target selector rules, and
  deviation policy.
- `RunbookStepExecution` records step execution provenance across UI,
  CLI/headless, and support packets. Mutating steps require approval refs;
  deviations require durable deviation notes; external-console handoffs require
  stable handoff refs.
- `OperationalEvidenceBundle` records redaction profile, embedded evidence,
  by-reference evidence, omission summary, destination class, and creation time.

## Freshness Vocabulary

Stable consumers must preserve all of these states where the distinction
matters:

- `live`
- `buffering`
- `cached`
- `imported`
- `stale`
- `partial`
- `offline`
- `truncated`
- `exported_copy`

Imported, truncated, stale, partial, and exported evidence must not be flattened
into live runtime truth. Loading spinners or generic unavailable text are not a
substitute once evidence exists.

## Stable Consumer Requirements

Every stable consumer projection must prove:

- exact slice identifiers and reopen refs survive UI, CLI/headless, support,
  and incident export surfaces;
- event ordering uses timezone-aware chronology and preserves actor lineage;
- runbook step executions preserve action class, status, target scope, actor,
  approval refs, deviation notes, handoff refs, evidence refs, rollback refs,
  and export continuity refs;
- export bundles distinguish embedded evidence from by-reference evidence and
  declare omissions explicitly;
- all required freshness states remain visible instead of being collapsed into a
  smaller local vocabulary.

## Downgrade Rules

The validator blocks stable promotion when:

- a signal slice lacks source identity, target scope, query/filter hash,
  time-window timezone, collection time, or linked timeline refs;
- a timeline entry lacks event id, actor lineage, timezone-aware event time,
  affected scope, source/evidence links, or chronological ordering;
- a runbook packet or step execution lacks version, target, actor, status,
  evidence refs, approval refs for mutating actions, deviation notes for
  deviations, handoff refs for external-console pivots, or export continuity;
- a stable consumer projection collapses `live`, `buffering`, `cached`,
  `imported`, `stale`, `partial`, `offline`, `truncated`, or `exported_copy`;
- an export bundle omits the embedded/by-reference manifest or explicit omission
  summary.

When any of those proofs are unavailable, the correct posture is inspect-only or
handoff-only. The product must not imply live observability or incident
authority from cached dashboards, imported artifacts, truncated logs, or
free-text notes.

## References

- Boundary schema:
  `schemas/runtime/log-metric-slice-and-incident-timeline-contract.schema.json`
- Fixture corpus:
  `fixtures/runtime/m4/log-metric-slice-and-incident-timeline-contract/`
- Support runbook prior art:
  `schemas/support/incident_action_ledger.schema.json`
- Runbook source/step envelope:
  `docs/support/m4/stabilize-runbook-source-step-envelope-and-handoff-truth.md`
