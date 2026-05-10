# Hot-path trace suite

Unattended proof lane that turns the protected hot paths (warm first
paint, interactive shell, file open, file switch, quick open,
keystroke-to-paint, save) into measurable trace packets bound to the
council-approved reference-hardware rows.

## Inputs

- `scenarios.yaml` — canonical scenario register; binds each scenario
  to a `journey_segment_id` set, an `event_class` / `protected_journey` /
  `dispatch_layer` triple, a `budget_ref`, and at least one `metric_id`
  drawn from
  [`artifacts/perf/latency_budget_ledger.yaml`](../../../artifacts/perf/latency_budget_ledger.yaml).
- [`artifacts/perf/m1/reference_hardware_matrix.yaml`](../../../artifacts/perf/m1/reference_hardware_matrix.yaml)
  — reference-hardware rows the bench lab is allowed to quote.
- [`fixtures/perf/hot_path_trace_reference.json`](../../../fixtures/perf/hot_path_trace_reference.json)
  — the canonical reference trace shape; the runner replays each
  scenario against this fixture so the lane is byte-stable across hosts.
- [`artifacts/build/build_identity.json`](../../../artifacts/build/build_identity.json)
  — exact-build identity stamped onto every emitted trace packet.

## Run locally

```sh
python3 benchmarks/m1/hot_path_trace_suite/run_hot_path_trace_suite.py --repo-root .
```

The runner emits its durable JSON capture to
[`artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json`](../../../artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json)
and exits non-zero if any required scenario is uncovered, any reference
row references an unknown `hardware_definition_id` or `scenario_id`, or
the named failure drill does not reproduce a typed missing-metric
failure.

## Failure drill

`scenarios.yaml#failure_drill` drops one protected
`journey_segment_id` from the reference trace and asserts the trace
packet is reported as `qualification_status = failed` instead of
silently passing on a partial trace. The runner replays this drill on
every run and records the verdict under `failure_drill_replay` in the
capture.

## Reviewer entry point

[`docs/perf/m1_hot_path_trace_readme.md`](../../../docs/perf/m1_hot_path_trace_readme.md)
is the human-readable landing page; it anchors the protected walk, the
failure drill, and the M1 dashboard hand-off contract.
