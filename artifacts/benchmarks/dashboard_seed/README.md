# Benchmark-lab dashboard seed

This directory is the committed seed for the nightly benchmark lab.
Every file below resolves through one of the boundary schemas and
stable registers the benchmark council already owns:

- Raw run records under `raw/` conform to
  [`/schemas/benchmarks/run_result.schema.json`](../../../schemas/benchmarks/run_result.schema.json)
  and key every result by `exact_build_identity_ref`, the corpus
  manifest revision, the fitness-function catalog revision, and a
  hardware-definition ref rather than a free-text label.
- `dashboard.json` is the rolled-up snapshot the lane surfaces on the
  dashboard. It indexes the raw runs and carries trend counts per
  fitness row.
- `report/<run_id>.md` is the human-readable summary the lane emits
  for one run. It links back to the raw artifact, names the protected
  metric, corpus row, or threshold that triggered any regression, and
  is distinct per `run_context` class (`reference_capture`,
  `provisional_capture`, `self_capture`, `smoke_subset`).

The seed is deliberately frozen: regenerating it locally uses
[`tools/benchmark_lab.sh --emit-seed`](../../../tools/benchmark_lab.sh)
with `SOURCE_DATE_EPOCH`, `TZ`, and `LC_ALL` pinned the same way as
the other prototype wrappers so the committed bytes stay stable
across hosts. The nightly CI lane
([`.github/workflows/nightly_benchmark.yml`](../../../.github/workflows/nightly_benchmark.yml))
reads the same script; a run that cannot reproduce one of these seeds
is a validation failure on the `benchmark_lab_health` fitness row.

## Files

- `raw/benchmark_run.seed.self_capture.json` — one full run-result
  record in `self_capture` context, covering every seeded fitness
  row. Every row lands `pass` under the seeded harness inputs.
- `raw/benchmark_run.seed.regression_example.json` — a companion
  run where one protected row trips a `boolean_gate_failed`
  regression condition on purpose. The lane returns a non-zero exit
  code when it emits this shape, so the dashboard can demonstrate a
  named regression path end to end.
- `dashboard.json` — rolled-up snapshot over the two seeded runs.
  Every row resolves back to a `fitness_row_id` in
  [`/artifacts/bench/fitness_function_catalog.yaml`](../../bench/fitness_function_catalog.yaml).
- `report/benchmark_run.seed.self_capture.md` — human-readable
  summary for the clean run.
- `report/benchmark_run.seed.regression_example.md` — human-readable
  summary for the regression run, naming the fitness row, the corpus
  ids, and the regression-trigger kind that caused the failure.

No numeric SLO thresholds are promoted here. Every row that carries
`threshold_mode = to_be_set_by_benchmark_council` in the fitness
catalog either records `result = pass` against a counts-only digest
or `result = provisional` with `trend_direction = unknown_insufficient_history`.
Promotion is explicitly reserved to the benchmark council.
