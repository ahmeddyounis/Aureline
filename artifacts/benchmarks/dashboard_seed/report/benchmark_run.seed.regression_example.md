# Benchmark run: `benchmark_run.seed.regression_example`

| Field                              | Value                                                                  |
|------------------------------------|------------------------------------------------------------------------|
| Run context                        | `self_capture`                                                         |
| Lane                               | `developer_local`                                                      |
| Trigger                            | `developer_invocation`                                                 |
| Measured on                        | 1970-01-01                                                             |
| Build identity                     | `exact_build_identity.seed.regression_example`                         |
| Release channel                    | `dev_local`                                                            |
| Workspace version                  | 0.0.0                                                                  |
| Corpus manifest revision           | 1                                                                      |
| Protected metrics revision         | 1                                                                      |
| Fitness-function catalog revision  | 1                                                                      |
| Hardware definition                | `hardware_definition.self_capture.current_machine_reported` (council-approved: no)   |
| Environment definition             | `environment_definition.self_capture.current_machine_default`                                    |
| Display class                      | `display_class.self_capture.current_machine_reported`                                   |
| Lab image                          | `lab_image.self_capture.unmanaged_local.rev1` @ rev 1                 |
| Power / thermal posture            | `power_posture.self_capture.reported_out_of_band` / `thermal_posture.self_capture.reported_out_of_band`  |
| Comparability                      | `not_yet_comparable` (no quarantine reasons)                           |

**This run intentionally demonstrates the regression path.** The
lane emits it so the benchmark-lab wrappers and the nightly workflow
can show a non-zero exit code end to end, with a named fitness row,
a named regression-trigger kind, and a pointer back to the raw
artifact. It is `self_capture`, not a reference capture — the
fail verdict below is a harness demonstration, not a release signal.

## Row results

| Fitness row                 | Result | Trend        | Threshold mode | Regression trigger    | Notes                                                                                   |
|-----------------------------|--------|--------------|----------------|-----------------------|-----------------------------------------------------------------------------------------|
| `ff.benchmark_lab_health`   | `fail` | `regressing` | `boolean_gate` | `corpus_row_missing`  | The harness pretended a cited corpus id failed to resolve; the gate reported a fail.    |

Row-count totals: 0 pass, 0 warn, **1 fail**, 0 not_measured, 0 waived,
0 provisional. The regression-trigger bucket increments
`corpus_row_missing` by one.

## Why this is the regression demonstration

`ff.benchmark_lab_health` is the fitness row whose threshold reads
"every fixture cited by a benchmark report resolves to an id in the
corpus manifest". The fixture id this run cites
(`corpus.workflow.startup_warm_to_first_paint`) resolves cleanly
against the real manifest revision — the `fail` here comes from the
harness deliberately flipping the boolean outcome on emit to exercise
the wiring. A real nightly lane that hit the same regression would
fail the run, record the `corpus_row_missing` trigger, and flag the
row on the dashboard under the same summary_ref that this file
carries.

## Links

- Raw artifact: [`artifacts/benchmarks/dashboard_seed/raw/benchmark_run.seed.regression_example.json`](../raw/benchmark_run.seed.regression_example.json)
- Dashboard snapshot: [`artifacts/benchmarks/dashboard_seed/dashboard.json`](../dashboard.json)
- Protected metrics: [`artifacts/bench/protected_metrics.yaml`](../../../bench/protected_metrics.yaml)
- Fitness-function catalog row: `ff.benchmark_lab_health` in [`artifacts/bench/fitness_function_catalog.yaml`](../../../bench/fitness_function_catalog.yaml)
- Corpus manifest: [`fixtures/benchmarks/corpus_manifest.yaml`](../../../../fixtures/benchmarks/corpus_manifest.yaml)
- Reference hardware manifest: [`artifacts/perf/reference_hardware_manifest.yaml`](../../../perf/reference_hardware_manifest.yaml)
- Lab-image manifest: [`artifacts/perf/lab_image_manifest.yaml`](../../../perf/lab_image_manifest.yaml)
- Self-capture parity guidance: [`docs/perf/self_capture_parity.md`](../../../../docs/perf/self_capture_parity.md)
