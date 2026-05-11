# Proof packet: nightly hot-path fitness gate

Purpose: anchor proof captures for the unattended nightly hot-path
fitness gate that joins the latest hot-path trace-suite capture against
the protected gate definition and emits a durable pass/fail artifact
plus a dashboard snapshot for the exit packet.

Reviewer entry point:
[`/docs/perf/m1_fitness_gate_policy.md`](../../../docs/perf/m1_fitness_gate_policy.md).

Canonical sources (non-exhaustive):

- `ci/perf/nightly_hot_path.yml` — gate definition; one
  `protected_metric_row` per (fitness function, scenario, metric)
  triple, plus the failure drill.
- `ci/perf/run_nightly_hot_path.py` — unattended runner that emits the
  durable JSON capture and the dashboard snapshot.
- `ci/perf/README.md` — engineer-facing run notes.
- `dashboards/m1/hot_path_fitness.json` — dashboard snapshot bound to
  the exact-build identity and the latest trace-suite capture.
- `artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json`
  — upstream trace-suite capture the gate joins against.
- `benchmarks/m1/hot_path_trace_suite/scenarios.yaml` — scenario
  register the trace suite emits for.
- `artifacts/perf/m1/reference_hardware_matrix.yaml` — reference-
  hardware rows.
- `artifacts/perf/latency_budget_ledger.yaml` — `metric_id` truth and
  per-path threshold publication.
- `artifacts/perf/protected_path_ledger.yaml` — `budget_ref` register.
- `artifacts/bench/fitness_function_catalog.yaml` — protected fitness
  functions by stable id.
- `artifacts/build/build_identity.json` — exact-build identity stamped
  onto every dashboard cell and the durable capture.
- `artifacts/qe/test_lane_registry.yaml#fixture_repo_integration` —
  registered validation lane.

Live runtime consumer (read-only):

- `crates/aureline-telemetry/src/hot_path_metrics.rs` — the emitter
  whose `validate_minimum_required` segment set the trace suite stays
  aligned with; the gate inherits its observation truth from the same
  segment register.

Validation captures:

- `artifacts/milestones/m1/captures/nightly_hot_path_validation_capture.json`

Refresh: re-run the validation lane after a change to the gate
definition, any threshold or warning band, the waiver vocabulary, the
trace-suite scenarios register, the reference-hardware matrix, the
fitness-function catalog, the build identity, or the protected-path /
latency-budget ledgers the gate quotes.

Closure rule: the lane stays open until the latest capture lands under
the governed proof root with `status = PASS`, every entry in
`gate.required_gate_row_coverage` resolves to at least one rendered
dashboard cell, the dashboard cells embed the exact-build identity
token derived from `artifacts/build/build_identity.json`, the failure
drill (`--force-drill`) reproduces `nightly_hot_path.regression_floor.exceeded`
on the named row, and the lane registration in
`artifacts/milestones/m1/artifact_index.yaml` points at the most recent
capture.
