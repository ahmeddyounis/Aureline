# Benchmark-lab run results and dashboard baseline

This document is the **normative** companion to the benchmark-lab run-
result boundary schema at
[`/schemas/benchmarks/run_result.schema.json`](../../schemas/benchmarks/run_result.schema.json)
and the seeded dashboard baseline under
[`/artifacts/benchmarks/dashboard_seed/`](../../artifacts/benchmarks/dashboard_seed/).
It defines the closed vocabularies the schema, the nightly CI lane, the
benchmark-report governance packet family, and every consumer of a
machine-readable benchmark run resolve against when they name a run
context, a comparability class, a regression-trigger kind, a lane
class, or a trigger kind.

If this document disagrees with the schema, this document wins and the
schema must be updated in the same change. Renaming any token defined
here is **breaking** and opens a decision row in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml);
adding a value is additive-minor and lands in this document and the
schema in the same change.

Companion artifacts:

- [`/schemas/benchmarks/run_result.schema.json`](../../schemas/benchmarks/run_result.schema.json)
  — boundary schema every run-result record conforms to.
- [`/artifacts/benchmarks/dashboard_seed/`](../../artifacts/benchmarks/dashboard_seed/)
  — seeded dashboard snapshot and two reference run records: the
  self-capture that covers every fitness-catalog row, and the
  regression-demonstration record that intentionally trips
  `ff.benchmark_lab_health`.
- [`/docs/benchmarks/benchmark_publication_pack_template.md`](./benchmark_publication_pack_template.md)
  — public benchmark/public-proof packet template that composes over
  the raw run-result record when results leave the internal dashboard
  context.
- [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml)
  — revisioned protected-metrics file every run cites alongside the
  corpus manifest and fitness catalog.
- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — protected register whose `rows[].id` every run-result row cites.
- [`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml)
  — protected corpus manifest whose `fixtures[].id` every row's
  `data_source.corpus_refs` resolves against.
- [`/tools/benchmark_lab.sh`](../../tools/benchmark_lab.sh) and
  [`/tools/benchmark_lab_emit.py`](../../tools/benchmark_lab_emit.py)
  — the wrapper the nightly CI lane and a developer running the lab
  locally both invoke.
- [`/.github/workflows/nightly_benchmark.yml`](../../.github/workflows/nightly_benchmark.yml)
  — the nightly CI lane. Runs the verify-seed gate before the lab so
  committed-seed drift fails fast.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — index rows `benchmark_run_result_schema`,
  `benchmark_lab_dashboard_seed`, and `nightly_benchmark_ci_lane`
  name this document as the `overview_page`.

## 1. Why a run-result schema and dashboard seed exist

1. Release claims of "fast" and "safe" have to resolve against one
   register of evidence. Without a typed run-result record, every
   subsystem lane emits a private CSV shape, every regression report
   cites a private metric name, and shiproom has no comparable input.
   This schema freezes one machine-readable envelope so the benchmark
   council, the release-evidence shiproom, and the performance-council
   waiver log consume one artifact family rather than parallel
   scoreboards.
2. Regressions have to be comparable across runs. The schema pins every
   record to a single exact-build identity, a single corpus-manifest
   revision, a single protected-metrics revision, a single
   fitness-catalog revision, and a single hardware definition. If any
   of those drift, the record's `comparability_class` downgrades before
   the dashboard compares trends.
3. A regression has to be attributable to a named fitness row, a named
   corpus row (where applicable), a named threshold mode, and one
   typed `regression_trigger_ref.kind`. The schema forces a row-level
   `regression_trigger_ref` for every `fail`/`warn` result so later
   support-bundle, shiproom, and waiver-register consumers do not
   retrofit triggers ad-hoc.
4. Reports have to distinguish reference captures from provisional and
   self captures. The schema carries `run_context_class` at the record
   root and every human-readable report quotes it verbatim on a
   banner before any numeric value appears.

## 2. Scope

### In scope

- Closed vocabularies for: run context, comparability class,
  quarantine reason, measurement status, row result, trend direction,
  threshold mode, SLI kind, data-source kind, lane class, trigger
  kind, host-platform class, host-OS class, and
  `regression_trigger_ref.kind`.
- Record envelopes for `hardware_definition_reference`,
  `build_identity_reference`, `corpus_manifest_reference`,
  `protected_metrics_reference`,
  `fitness_function_catalog_reference`, `toolchain_pin_reference`,
  `comparability_envelope`, `summary_envelope`, and
  `row_measurement_record`.
- Reproducibility posture: `SOURCE_DATE_EPOCH`, `TZ=UTC`, `LC_ALL=C`.
- Seed dashboard shape under `artifacts/benchmarks/dashboard_seed/`
  covering the two reference run records and the rolled-up
  `dashboard.json` snapshot.
- Nightly CI lane entry point and verify-seed gate.

### Out of scope

- Full enterprise performance infrastructure (long-horizon retention,
  tenancy isolation, per-hardware comparability clustering). Those
  lanes cite this schema; they do not reshape it.
- Public publication of release-evidence comparisons. Public surfaces
  compose over the `packet_export_shape.fitness_function_snapshot`
  block in the fitness catalog, not over this file.
- Storage backend selection (object store, S3 class, retention
  bucket). Those are deployment concerns for the release-council and
  are framed by the control-artifact index, not this schema.

## 3. Run context vocabulary

Every run record MUST carry exactly one `run_context_class`. Every
human-readable report MUST quote the class verbatim on a banner
before any numeric value appears.

| Class | Meaning | Admissible as release-evidence input? |
|---|---|---|
| `reference_capture` | Full nightly lane on the benchmark-council-approved hardware baseline. | **Yes** — the only class admissible as protected-evidence input to a shiproom packet. |
| `provisional_capture` | Row captured before the benchmark council has ratified a threshold or data source. | No — admissible for trend inspection; MUST NOT fail a release gate. |
| `self_capture` | Developer-local or contributor-machine capture. | No — reports MUST mark it distinct from `reference_capture` in every surface. |
| `smoke_subset` | Fast subset the nightly lane runs before the full corpus. | No — admissible for keeping the lane green; MUST NOT be confused with a reference capture. |

## 4. Comparability and quarantine

`comparability_class` scopes what a trend inspection can claim.

| Class | Meaning |
|---|---|
| `comparable_to_baseline` | Matches the approved hardware definition AND the same exact-build-identity axes as the baseline. |
| `comparable_to_prior_run_same_host` | Comparability guaranteed only to the immediately prior nightly on the same host. |
| `not_yet_comparable` | Default when a new corpus revision or threshold has just landed; no trend claim is admissible until the next run. |
| `quarantined` | Excluded from trend comparisons for a named reason. |

A `quarantined` record MUST carry at least one
`quarantine_reason_class`:

- `hardware_drift_observed`
- `toolchain_pin_drift_observed`
- `corpus_revision_mismatch`
- `fitness_catalog_revision_mismatch`
- `trace_schema_nonconformance`
- `suspected_host_contention`
- `test_plan_changed_in_flight`
- `benchmark_council_directive`

A new quarantine reason is additive-minor and lands in this document
and the schema in the same change.

## 5. Row result and trend direction

`row_result_class`:

- `pass` — threshold met; baseline holds.
- `warn` — threshold crossed a warning band but has not tripped the
  fail gate. A `regression_trigger_ref` is REQUIRED.
- `fail` — threshold tripped the fail gate. A `regression_trigger_ref`
  is REQUIRED.
- `not_measured` — row reserved by the catalog but the data source
  is not yet wired. Row carries `measurement_status = not_measured`.
- `waived` — the performance council has an active waiver against
  this row; the record MUST carry a waiver ref.
- `provisional` — the row is `ready_for_instrumentation` in the
  fitness catalog (threshold is `to_be_set_by_benchmark_council`);
  admissible for trend inspection only.

`trend_direction_class`:

- `improving`
- `unchanged`
- `regressing`
- `unknown_insufficient_history` — fewer than the catalog's
  `trend_window` runs available.
- `unknown_not_comparable` — `comparability_class` is
  `not_yet_comparable` or `quarantined`.

## 6. Regression trigger vocabulary

Every `fail` or `warn` row MUST carry exactly one
`regression_trigger_ref.kind`. The kind names the axis on which the
row tripped so later consumers can bucket regressions without
re-deriving intent from prose.

| Kind | Meaning |
|---|---|
| `threshold_exceeded` | A numeric SLI crossed the catalog-defined threshold. |
| `boolean_gate_failed` | A boolean-gate row reported false. |
| `ratio_below_floor` | A ratio-unit-interval SLI fell below the floor. |
| `corpus_row_missing` | A cited `corpus_refs` id did not resolve against the manifest revision pinned on this record. |
| `trace_schema_nonconforming` | Trace bundle failed to validate against its producer schema. |
| `ad_hoc_metric_name_observed` | Row emitted a metric name not registered in the fitness catalog. |
| `toolchain_pin_drift` | Toolchain pin observed on the runner did not match the pin the record declares. |
| `hardware_definition_mismatch` | Host signature did not match the hardware-definition ref. |
| `fitness_catalog_row_status_provisional` | Row is `provisional` in the catalog but a release-gate caller tried to treat it as admissible. |

## 7. Threshold mode and SLI kind

`threshold_mode_class` (MUST match the fitness catalog's vocabulary):

- `absolute_p50_and_p95`
- `absolute_p50_p95_p99`
- `ratio_unit_interval`
- `boolean_gate`
- `structural_conformance`
- `to_be_set_by_benchmark_council`

`sli_kind_class`:

- `latency_p50_ms`
- `latency_p95_ms`
- `latency_p99_ms`
- `ratio_unit_interval`
- `count_non_negative`
- `boolean_pass_fail`
- `structural_digest`

## 8. Data-source kind

`data_source_kind_class` (MUST match the fitness catalog):

- `hot_path_hook_counter_stream`
- `spike_timing_trace`
- `buffer_op_hook_counter_stream`
- `vfs_decision_log`
- `benchmark_lab_self_audit`
- `restore_harness_trace`
- `command_dispatch_trace`
- `to_be_wired_by_benchmark_council`

## 9. Lane and trigger vocabulary

`lane_class`:

- `ci_nightly`
- `ci_merge_queue`
- `ci_preview`
- `developer_local`

`trigger_kind_class`:

- `scheduled_nightly`
- `manual_dispatch`
- `commit_gated`
- `developer_invocation`

## 10. Reproducibility posture

Every run record is produced under pinned inputs so reruns on the same
commit yield byte-stable output:

- `SOURCE_DATE_EPOCH` — pinned to the commit timestamp if not set.
- `TZ=UTC`.
- `LC_ALL=C`.
- JSON written with `indent=2`, preserved key order, and a trailing
  newline.

The emitter in `tools/benchmark_lab_emit.py` ships with a
`--verify-seed` mode that re-emits the committed seed under a tempdir
and unified-diffs against the committed files. Seed drift fails the
nightly lane in the first job so drift is caught before any work is
wasted running the lab.

## 11. Report and dashboard rules

- Every raw run record under `raw/*.json` carries a
  `summary.human_readable_summary_ref` pointing at the markdown
  report under `report/*.md`.
- Every markdown report carries a banner naming the run context. A
  `self_capture` report MUST state explicitly that the record is not
  a reference capture before any numeric value appears.
- The `dashboard.json` snapshot aggregates `source_run_refs` with
  per-run `row_count_by_result` and
  `regression_trigger_count_by_kind` totals, and a `by_fitness_row`
  index keyed by `ff.<id>` pointing at the latest run for that row.
- The dashboard is a rolled-up seed, not a full history store; long-
  horizon retention is out of scope at this milestone and is framed
  by the control-artifact index.

## 12. Known holes carried forward

- The `hardware_definition` rows are reserved under the sentinel
  `hardware_definition.reserved.not_yet_seeded`. A benchmark-council-
  approved hardware baseline is required before any record can claim
  `comparability_class = comparable_to_baseline`.
- The `toolchain_pin_reference.toolchain_pin_source` is set to
  `tools/build/build.sh` on the seed; once the release-council pins
  a full toolchain manifest, the seed is refreshed in the same
  change that lands the manifest.
- `power_thermal_posture`, `restore_fidelity`, and `command_parity`
  are `provisional` on the seed because the fitness catalog reserves
  them pending their producer lanes landing; the lab emits them so
  the dashboard schema is exercised end to end, but release gates
  MUST NOT key off provisional rows.
