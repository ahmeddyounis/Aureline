# Nightly hot-path fitness gate

Unattended proof lane that promotes the protected startup, typing,
quick-open, save, and recovery measurements from one-off captures into
automated release controls.

## Inputs

- `nightly_hot_path.yml` — canonical gate definition; pins one
  `protected_metric_row` per (fitness function, scenario, metric)
  triple, plus the failure drill.
- [`../../benchmarks/m1/hot_path_trace_suite/scenarios.yaml`](../../benchmarks/m1/hot_path_trace_suite/scenarios.yaml)
  — scenario register the trace-suite emits for.
- [`../../artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json`](../../artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json)
  — latest trace-suite capture the gate joins against.
- [`../../artifacts/perf/latency_budget_ledger.yaml`](../../artifacts/perf/latency_budget_ledger.yaml)
  — `metric_id` truth for every protected row.
- [`../../artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — fitness-function row identity.
- [`../../artifacts/build/build_identity.json`](../../artifacts/build/build_identity.json)
  — exact-build identity stamped onto every dashboard cell.

## Run locally

```sh
python3 ci/perf/run_nightly_hot_path.py --repo-root .
```

The runner writes:

- `artifacts/milestones/m1/captures/nightly_hot_path_validation_capture.json`
  — durable JSON capture.
- `dashboards/m1/hot_path_fitness.json` — dashboard snapshot.

It exits non-zero if any `protected_metric_row` crosses its
regression floor without an active waiver, if any `upstream_contracts`
ref does not resolve, if the trace-suite capture is missing / not PASS,
if any `required_gate_row_coverage` id is unrendered, or if a row
references an unknown waiver state.

## Failure drill

```sh
python3 ci/perf/run_nightly_hot_path.py --repo-root . --force-drill
```

Forces the keystroke-to-paint p95 observation to 12 ms on the macOS
reference row and confirms the runner emits:

- `regression_status = fail` for the named gate row;
- a durable-capture finding with
  `check_id = nightly_hot_path.regression_floor.exceeded`;
- `expected_overall_status = FAIL`;
- an actionable next action naming the row's owner DRI.

Exits 0 when the drill reproduces all four; non-zero when any are
missing — i.e. the gate is no longer protecting against the named
regression.

## Reviewer entry point

[`../../docs/perf/m1_fitness_gate_policy.md`](../../docs/perf/m1_fitness_gate_policy.md)
is the human-readable landing page; it anchors the protected walk, the
failure drill, the regression-status vocabulary, and the rollback /
downgrade lever.

## Proof packet

[`../../artifacts/milestones/m1/proof_packets/nightly_hot_path.md`](../../artifacts/milestones/m1/proof_packets/nightly_hot_path.md)
— canonical sources, refresh rule, closure rule.
