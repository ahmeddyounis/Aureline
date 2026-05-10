# Hot-path trace suite (reviewer entry point)

This is the reviewer-facing landing page for the unattended hot-path
trace suite that proves the protected startup, file-open, file-switch,
quick-open, keystroke-to-paint, and save loops on the council-approved
reference hardware. It is the human-readable companion to the
machine-readable seeds under
[`benchmarks/m1/hot_path_trace_suite/`](../../benchmarks/m1/hot_path_trace_suite/)
and
[`artifacts/perf/m1/reference_hardware_matrix.yaml`](../../artifacts/perf/m1/reference_hardware_matrix.yaml).

## Why this lane exists

The performance / dashboard / exit-evidence path needs the same named
scenarios and metrics vocabulary on every host so cross-run comparisons
stay apples-to-apples. The trace suite turns the M1 hot paths into a
durable JSON packet per (hardware row, scenario), each carrying:

- the exact-build identity (commit short, profile, target, workspace
  version) so traces are tied to a build, not a calendar day;
- the council-approved `hardware_definition_id` plus `power_profile_id`
  and `capture_posture_id`;
- the canonical `scenario_id` plus the bound `event_class`,
  `protected_journey`, `dispatch_layer`, `budget_ref`, and the required
  `journey_segment_id` set; and
- a `timing_bucket` value drawn from a closed vocabulary
  (`under_target`, `within_p50`, `within_p95`, `over_target`,
  `missing`) so the M1 performance dashboard ingests the packets
  without manual spreadsheet cleanup.

## Canonical sources

- [`benchmarks/m1/hot_path_trace_suite/scenarios.yaml`](../../benchmarks/m1/hot_path_trace_suite/scenarios.yaml)
  — scenario register and required-coverage list.
- [`benchmarks/m1/hot_path_trace_suite/run_hot_path_trace_suite.py`](../../benchmarks/m1/hot_path_trace_suite/run_hot_path_trace_suite.py)
  — unattended runner.
- [`artifacts/perf/m1/reference_hardware_matrix.yaml`](../../artifacts/perf/m1/reference_hardware_matrix.yaml)
  — reference-hardware rows × covered scenarios.
- [`artifacts/perf/reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml)
  — frozen `hardware_definition_id` register.
- [`artifacts/perf/protected_path_ledger.yaml`](../../artifacts/perf/protected_path_ledger.yaml)
  and
  [`artifacts/perf/latency_budget_ledger.yaml`](../../artifacts/perf/latency_budget_ledger.yaml)
  — `budget_ref` and `metric_id` truth.
- [`artifacts/benchmarks/journey_segment_ids.yaml`](../../artifacts/benchmarks/journey_segment_ids.yaml)
  — `journey_segment_id` register.
- [`fixtures/perf/hot_path_trace_reference.json`](../../fixtures/perf/hot_path_trace_reference.json)
  — reference trace fixture.

## Live runtime consumer

- [`crates/aureline-telemetry/src/hot_path_metrics.rs`](../../crates/aureline-telemetry/src/hot_path_metrics.rs)
  — emitter that the runner's required-segment set must stay aligned
  with (`HotPathMetricsRecord::validate_minimum_required`).
- [`crates/aureline-shell/src/bootstrap/native_shell.rs`](../../crates/aureline-shell/src/bootstrap/native_shell.rs)
  — live wiring for the milestone marks and span boundaries.

## Protected walk

1. The runner loads scenarios + the reference-hardware matrix and
   verifies every upstream contract reference resolves.
2. For each `(hardware_row, scenario)` pair it emits one trace packet
   stamped with the exact-build identity, the resolved hardware
   definition, the `journey_segment_id` requirement set, and a
   `timing_bucket` for each declared metric.
3. The runner refuses to PASS unless every entry in
   `scenarios.required_scenario_coverage` lands at least one `passed`
   trace packet on at least one reference row.
4. The capture lands at
   [`artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json`](../../artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json),
   already shaped for direct dashboard ingestion.

## Failure drill

`scenarios.yaml#failure_drill` removes one protected
`journey_segment_id` (`seg.first_paint.renderer_work.submit`) from the
reference trace and re-runs qualification. The runner asserts the
trace packet reports `qualification_status = failed` and that
`missing_journey_segment_ids` names the dropped id verbatim. A silent
pass on a partial trace would be a regression of the lane itself.

## Refresh triggers

Re-run the validation lane after any of:

- a change to the canonical `journey_segment_id` register or to the
  `hot_path_metrics` emitter;
- a change to the reference-hardware manifest or the per-row coverage
  in
  [`artifacts/perf/m1/reference_hardware_matrix.yaml`](../../artifacts/perf/m1/reference_hardware_matrix.yaml);
- a change to the reference trace fixture; or
- a budget threshold change in the latency-budget ledger that any
  scenario quotes.

## How to run locally

```sh
python3 benchmarks/m1/hot_path_trace_suite/run_hot_path_trace_suite.py --repo-root .
```

The same command runs in CI / nightly; the produced capture is
byte-stable when the inputs are unchanged so reviewers can diff
regressions cleanly.
