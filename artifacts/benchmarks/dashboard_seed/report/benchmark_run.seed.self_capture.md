# Benchmark run: `benchmark_run.seed.self_capture`

| Field                              | Value                                                                 |
|------------------------------------|-----------------------------------------------------------------------|
| Run context                        | `self_capture`                                                        |
| Lane                               | `developer_local`                                                     |
| Trigger                            | `developer_invocation`                                                |
| Measured on                        | 1970-01-01                                                            |
| Build identity                     | `exact_build_identity.seed.self_capture`                              |
| Release channel                    | `dev_local`                                                           |
| Workspace version                  | 0.0.0                                                                 |
| Corpus manifest revision           | 1                                                                     |
| Fitness-function catalog revision  | 1                                                                     |
| Hardware definition                | `hardware_definition.reserved.not_yet_seeded` (council-approved: no)  |
| Comparability                      | `not_yet_comparable` (no quarantine reasons)                          |

**This run is `self_capture`, not a reference capture.** The numbers
below describe harness self-health on a seeded input set; they are
not admissible as release-evidence input. A reference capture MUST
run on the `ci_nightly` lane against a council-approved hardware
baseline.

## Row results

| Fitness row                          | Result        | Trend                         | Threshold mode                    | Notes                                                                                          |
|--------------------------------------|---------------|-------------------------------|-----------------------------------|------------------------------------------------------------------------------------------------|
| `ff.warm_start_to_first_paint`       | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Trace digest seeded from `artifacts/traces/examples/full_scene.json`.                          |
| `ff.first_paint`                     | `provisional` | `unknown_insufficient_history`| `absolute_p50_and_p95`            | Same trace digest; numeric SLO bars deferred to benchmark-council ratification.                |
| `ff.input_to_paint`                  | `provisional` | `unknown_insufficient_history`| `absolute_p50_and_p95`            | Eight hot-path marks from the fixture scene.                                                   |
| `ff.buffer_operations`               | `pass`        | `unchanged`                   | `boolean_gate`                    | Undo-class correctness gate reads `artifacts/buffer/buffer_metrics_seed.json`.                 |
| `ff.vfs_save_conflict_handling`      | `pass`        | `unchanged`                   | `boolean_gate`                    | Compare-before-write floor held against the frozen VFS decision examples.                      |
| `ff.benchmark_lab_health`            | `pass`        | `unchanged`                   | `boolean_gate`                    | Self-audit across governance-packet, corpus-manifest, and fitness-catalog resolution.          |
| `ff.power_thermal_posture`           | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Data source `to_be_wired_by_benchmark_council`; row reserved by the fitness catalog.           |
| `ff.restore_fidelity`                | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Entry-restore harness not yet wired; reserves the onboarding-metric name.                      |
| `ff.command_parity`                  | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Command-graph parity harness not yet wired; ADR landing required.                              |

Row-count totals: **3 pass**, 0 warn, 0 fail, 0 not_measured, 0 waived,
**6 provisional**.

## Links

- Raw artifact: [`artifacts/benchmarks/dashboard_seed/raw/benchmark_run.seed.self_capture.json`](../raw/benchmark_run.seed.self_capture.json)
- Dashboard snapshot: [`artifacts/benchmarks/dashboard_seed/dashboard.json`](../dashboard.json)
- Fitness-function catalog: [`artifacts/bench/fitness_function_catalog.yaml`](../../../bench/fitness_function_catalog.yaml)
- Corpus manifest: [`fixtures/benchmarks/corpus_manifest.yaml`](../../../../fixtures/benchmarks/corpus_manifest.yaml)
- Trace bundle seed: [`artifacts/traces/examples/full_scene.json`](../../../traces/examples/full_scene.json)
