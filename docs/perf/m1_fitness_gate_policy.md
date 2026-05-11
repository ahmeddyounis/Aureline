# Nightly hot-path fitness-gate policy (reviewer entry point)

This is the reviewer-facing landing page for the nightly hot-path
fitness gate that promotes the protected startup, typing, quick-open,
save, and recovery measurements from one-off captures into automated
release controls.

It is the human-readable companion to the machine-readable seeds under
[`ci/perf/nightly_hot_path.yml`](../../ci/perf/nightly_hot_path.yml),
[`ci/perf/run_nightly_hot_path.py`](../../ci/perf/run_nightly_hot_path.py),
and the dashboard projection at
[`dashboards/m1/hot_path_fitness.json`](../../dashboards/m1/hot_path_fitness.json).

## What this lane proves

The hot-path trace suite
([`docs/perf/m1_hot_path_trace_readme.md`](m1_hot_path_trace_readme.md))
emits one trace packet per (hardware row, scenario). That lane proves
the trace shape and the journey-segment binding. It does **not** by
itself decide whether a regression should block a release.

The fitness gate joins those trace packets against an explicit
threshold + warning band + waiver register for the protected hot paths:

| Hot path | Gate row id | Target |
|---|---|---|
| Warm startup → first paint | `gate.startup.warm_first_paint` | ≤ 150 ms |
| Warm startup → first useful chrome | `gate.startup.first_useful_chrome` | ≤ 400 ms |
| Placeholder file open → first paint | `gate.editor.file_open` | ≤ 70 ms |
| File switch → first paint | `gate.editor.file_switch` | ≤ 50 ms |
| Quick-open → ranked results (p95) | `gate.quick_open.invoke_p95` | ≤ 75 ms |
| Keystroke → paint (p95) | `gate.input_to_paint.keystroke_p95` | ≤ 8 ms |
| Save → durable completion (p95) | `gate.save.pipeline_p95` | ≤ 220 ms |
| Workspace snapshot restore (warm) | `gate.recovery.workspace_snapshot_restore` | ≤ 120 ms |

Each row carries an owner, an owning lane, a warning band, an explicit
regression floor, and a waiver record reference. The runner refuses to
treat a regression-floor breach as anything other than `fail` unless
the row cites a non-null `waiver_record_ref` minted through the
performance council.

## Canonical sources

- [`ci/perf/nightly_hot_path.yml`](../../ci/perf/nightly_hot_path.yml)
  — gate definition; one row per (fitness function, scenario, metric)
  triple plus the failure drill.
- [`ci/perf/run_nightly_hot_path.py`](../../ci/perf/run_nightly_hot_path.py)
  — unattended runner that joins the trace-suite capture against the
  gate definition and writes the durable JSON capture and dashboard.
- [`ci/perf/README.md`](../../ci/perf/README.md) — engineer-facing run
  notes and CI wiring.
- [`dashboards/m1/hot_path_fitness.json`](../../dashboards/m1/hot_path_fitness.json)
  — dashboard snapshot the exit packet embeds.
- [`artifacts/milestones/m1/captures/nightly_hot_path_validation_capture.json`](../../artifacts/milestones/m1/captures/nightly_hot_path_validation_capture.json)
  — durable JSON capture; pinned to the exact-build identity, the
  trace-suite capture, the fitness-function catalog rows, and the gate
  revision.
- [`artifacts/milestones/m1/proof_packets/nightly_hot_path.md`](../../artifacts/milestones/m1/proof_packets/nightly_hot_path.md)
  — proof packet (canonical sources, refresh rule, closure rule).

The gate inherits its identity vocabulary from:

- [`artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — protected fitness functions by stable id.
- [`artifacts/perf/latency_budget_ledger.yaml`](../../artifacts/perf/latency_budget_ledger.yaml)
  — `metric_id` truth and per-path threshold publication.
- [`artifacts/perf/protected_path_ledger.yaml`](../../artifacts/perf/protected_path_ledger.yaml)
  — `budget_ref` register.
- [`artifacts/perf/m1/reference_hardware_matrix.yaml`](../../artifacts/perf/m1/reference_hardware_matrix.yaml)
  — reference-hardware rows.
- [`benchmarks/m1/hot_path_trace_suite/scenarios.yaml`](../../benchmarks/m1/hot_path_trace_suite/scenarios.yaml)
  — scenario register.

The runner refuses any reference whose path does not resolve and any
gate row whose `coverage_state` is `live` but whose scenario does not
appear in the trace-suite capture.

## Regression-status vocabulary

The runner projects one dashboard cell per (gate row, hardware row).
Each cell carries one closed-vocabulary status:

| Status | Meaning |
|---|---|
| `pass` | Observed value is at or below the warning band. Release-blocking on no metric. |
| `warn` | Observed value is over the warning band but at or below the regression floor. Dashboard signal; does not block. |
| `fail` | Observed value crosses the regression floor and the row carries no active waiver. Lane exits non-zero. |
| `waived` | Observed value crosses the regression floor but a non-null `waiver_record_ref` covers the row (active waiver minted through `waiver_authority_ref`). Lane stays green; the waiver expiry is rendered. |
| `missing_observation` | The trace-suite capture does not yet emit an observed value for this (scenario, metric) cell. Surfaces honestly on the dashboard; not a failure on its own. |
| `pending_scenario_seed` | The gate row's `scenario_id` is null because no hot-path trace-suite scenario emits the metric yet. The threshold is published in the latency-budget ledger; the row is advisory until the trace-suite scenario lands. |

## Waiver-state vocabulary

Waiver states mirror the closed vocabulary in
[`docs/perf/benchmark_waiver_dashboard_contract.md`](benchmark_waiver_dashboard_contract.md)
so this gate, the dashboard, and the waiver register share one truth:

- `no_active_waiver` — admissible only when no row is over the
  regression floor.
- `active_waiver` — covers a row over the regression floor; requires
  non-null `waiver_record_ref` and a non-null `expiry_at`.
- `expired_waiver` — surfaces honestly; rows in this state cannot mask
  a regression-floor breach.
- `threshold_provisional_pending_council` — used only when the latency-
  budget ledger flags the threshold as
  `to_be_set_by_benchmark_council`; never used to silence a real
  regression.

## Protected walk

1. Re-run the trace-suite lane and check the durable capture is `PASS`:
   ```sh
   python3 benchmarks/m1/hot_path_trace_suite/run_hot_path_trace_suite.py --repo-root .
   ```
2. Run the nightly hot-path gate against the latest trace-suite capture:
   ```sh
   python3 ci/perf/run_nightly_hot_path.py --repo-root .
   ```
3. Inspect the dashboard
   ([`dashboards/m1/hot_path_fitness.json`](../../dashboards/m1/hot_path_fitness.json))
   for startup, typing, quick-open, save, and recovery cells. Every
   `fail` row MUST point at a named owner, a regression floor, and a
   missing or expired waiver.
4. Confirm the durable capture under
   `artifacts/milestones/m1/captures/nightly_hot_path_validation_capture.json`
   carries `status: PASS`, no findings, and the exact-build identity
   token derived from `artifacts/build/build_identity.json`.
5. Run the failure drill and confirm it reproduces the expected
   check_id rather than emitting a passive dashboard warning:
   ```sh
   python3 ci/perf/run_nightly_hot_path.py --repo-root . --force-drill
   ```
   The drill exits 0 only when the gate emits
   `nightly_hot_path.regression_floor.exceeded` for the named row, the
   overall status is `FAIL`, and the actionable owner reference matches
   the gate row's `owner_dri`.

## Failure-drill contract

The gate's failure drill (defined in
[`ci/perf/nightly_hot_path.yml#failure_drill`](../../ci/perf/nightly_hot_path.yml))
forces the keystroke-to-paint p95 observation to 12 ms on the macOS
reference row. The expected outcome is:

- a dashboard cell with `regression_status = fail`;
- a durable-capture finding with
  `check_id = nightly_hot_path.regression_floor.exceeded`;
- an `expected_overall_status = FAIL`;
- an actionable next action that names the owning DRI for bisect /
  waiver work.

When the drill reproduces all four, the lane exits 0 in `--force-drill`
mode. When any are missing, the lane exits non-zero — the gate is no
longer protecting against the named regression and must be repaired
before the next nightly run.

## Rollback / downgrade lever

The lane's threshold and waiver vocabularies live entirely in the gate
file and are versioned through `gate_revision`. To downgrade a row from
release-blocking to advisory without deleting it:

1. Mint a waiver record under the row's `waiver_authority_ref`.
2. Set the row's `waiver_state = active_waiver`, populate
   `waiver_record_ref` and `expiry_at`.
3. Re-run the lane and confirm the row's `regression_status` flips to
   `waived` (the dashboard cell still shows current value, target,
   warning band, regression floor, and the waiver expiry).

The gate never deletes a row — historical durable captures stay
comparable and reviewers can trace the regression history through the
captures directory.

## Refresh / freshness rule

Refresh the dashboard and capture whenever any of the following change:

- a row threshold (target, warning band, regression floor);
- the gate row vocabulary (`protected_metric_rows`);
- the waiver vocabulary or `waiver_state_vocabulary` membership;
- the trace-suite scenarios register or the reference-hardware matrix;
- the build identity (`artifacts/build/build_identity.json`);
- the hardware row set the trace suite emits for.

Stale captures stay readable but the lane re-renders the dashboard on
every run.

## Closure rule

The lane stays open until:

1. `artifacts/milestones/m1/captures/nightly_hot_path_validation_capture.json`
   reports `status: PASS` against the latest trace-suite capture;
2. every `required_gate_row_coverage` id is rendered;
3. the failure drill (`--force-drill`) reproduces
   `nightly_hot_path.regression_floor.exceeded` on the named row;
4. the dashboard cells embed the exact-build identity token from
   `artifacts/build/build_identity.json`; and
5. the lane registration in
   [`artifacts/milestones/m1/artifact_index.yaml`](../../artifacts/milestones/m1/artifact_index.yaml)
   points at the most recent capture.
