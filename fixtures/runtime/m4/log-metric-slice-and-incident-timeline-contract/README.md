# Log/Metric Slice and Incident Timeline Contract Fixtures

These fixtures exercise the stable operational evidence contract implemented by
`crates/aureline-runtime/src/log_metric_slice_and_incident_timeline_contract/`.

The manifest keeps drills compact by naming a mutation applied to the canonical
baseline packet:

- `none` keeps the baseline stable packet unchanged.
- `collapse_freshness` removes required freshness states from a stable consumer.
- `missing_timeline_timezone` removes timezone chronology from a timeline row.
- `missing_mutation_approval` removes approval provenance from a mutating
  runbook execution.

Stable consumers must downgrade or block stable promotion for every negative
case.
