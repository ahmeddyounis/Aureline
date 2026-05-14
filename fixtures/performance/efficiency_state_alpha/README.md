# Efficiency-State Alpha Fixtures

These fixtures exercise the alpha shell efficiency-state runtime hooks:

- active efficiency state and pressure source projection;
- throttled capability rows for indexing, AI/background work, uploads, and
  remote/session helpers;
- hidden-pane render suppression before audit samples are recorded; and
- the durable activity-row adapter for paused or reduced indexing work.

The raw capture fixture also conforms to
`schemas/benchmarks/power_thermal_capture.schema.json` and is audited by
`tools/perf/power_thermal_audit/audit_capture.py`.

