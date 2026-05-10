# Proof packet: hot-path trace suite

Purpose: anchor proof captures for the unattended hot-path trace suite
that turns the protected startup, file-open, file-switch, quick-open,
keystroke-to-paint, and save loops into measurable trace packets bound
to the council-approved reference-hardware rows.

Reviewer entry point:
[`/docs/perf/m1_hot_path_trace_readme.md`](../../../docs/perf/m1_hot_path_trace_readme.md).

Canonical sources (non-exhaustive):

- `benchmarks/m1/hot_path_trace_suite/scenarios.yaml` — scenario
  register, required-coverage list, and failure drill.
- `benchmarks/m1/hot_path_trace_suite/run_hot_path_trace_suite.py` —
  unattended runner that emits the durable JSON capture.
- `artifacts/perf/m1/reference_hardware_matrix.yaml` — reference-
  hardware rows × covered scenarios.
- `artifacts/perf/reference_hardware_manifest.yaml` — frozen
  `hardware_definition_id` register the matrix resolves against.
- `artifacts/perf/protected_path_ledger.yaml` and
  `artifacts/perf/latency_budget_ledger.yaml` — `budget_ref` and
  `metric_id` truth.
- `artifacts/benchmarks/journey_segment_ids.yaml` — `journey_segment_id`
  register; the runner refuses any scenario that names ids not in the
  register.
- `fixtures/perf/hot_path_trace_reference.json` — reference trace
  fixture the runner replays each scenario against.

Live runtime consumer (read-only):

- `crates/aureline-telemetry/src/hot_path_metrics.rs` — the emitter
  whose `validate_minimum_required` segment set the trace suite stays
  aligned with.
- `crates/aureline-shell/src/bootstrap/native_shell.rs` — live wiring
  for the milestone marks and span boundaries.

Validation captures:

- `artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json`

Refresh: re-run the validation lane after a change to the hot-path
metrics emitter, the journey-segment register, the reference-hardware
manifest, the reference-hardware matrix, the reference trace fixture,
or any latency-budget ledger threshold the trace suite quotes.

Closure rule: the lane stays open until the latest capture lands under
the governed proof root with `status = PASS`, every entry in
`scenarios.required_scenario_coverage` covered by at least one passing
trace packet, and the `failure_drill_replay` records `replay_passed =
true`.
