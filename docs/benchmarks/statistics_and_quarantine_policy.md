# Benchmark statistics, warm-up, variance, and quarantine policy

This document is the **normative** policy for the statistical method,
warm-up posture, variance-class selection, run-to-run comparability
rules, quarantine triggers, and the machine-readable report fields
every benchmark-lab run-result record, every dashboard snapshot,
every benchmark-report governance packet, and every public
benchmark publication pack must carry.

If this document disagrees with either companion artifact, this
document wins and the YAML must be updated in the same change.

Companion artifacts:

- [`/artifacts/benchmarks/variance_classes.yaml`](../../artifacts/benchmarks/variance_classes.yaml)
  — machine-readable register of variance classes, run counts,
  warm-up posture, discard rules, aggregation rules, confidence
  levels, and report-field bindings.
- [`/artifacts/benchmarks/quarantine_rules.yaml`](../../artifacts/benchmarks/quarantine_rules.yaml)
  — machine-readable quarantine rules, mechanical comparability
  verdict table, and regression-review rubric.
- [`/fixtures/benchmarks/stat_method_examples/`](../../fixtures/benchmarks/stat_method_examples/)
  — worked examples a reviewer or tool reads to confirm the
  mechanical verdicts this policy declares.
- [`/schemas/benchmarks/run_result.schema.json`](../../schemas/benchmarks/run_result.schema.json)
  — boundary schema every run-result record conforms to. The
  `comparability_envelope`, `run_context_class`, and
  `quarantine_reason_class` vocabularies the schema already freezes
  are the same vocabularies this policy resolves against.
- [`/docs/benchmarks/benchmark_lab_run_results.md`](./benchmark_lab_run_results.md)
  — run-result envelope companion document; names the closed
  vocabularies for run context, lane class, trigger kind, row result,
  regression trigger kind, and host-platform class this policy
  assumes without restating.
- [`/docs/benchmarks/corpus_governance.md`](./corpus_governance.md)
  — corpus-governance, protected-metrics change control, and
  threshold-easing rules. Change-control decisions that trip the
  quarantine rules below land on the governance asset matrix there.
- [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml),
  [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — protected-metrics file and fitness-function catalog every
  run-result row cites.
- [`/artifacts/perf/lab_image_manifest.yaml`](../../artifacts/perf/lab_image_manifest.yaml),
  [`/artifacts/perf/reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml)
  — lab-image, environment, and hardware registers whose revision
  bumps are the primary quarantine triggers.

## 1. Why this policy exists

The benchmark lab is cheap to misread. A fast-looking number with a
small sample, no warm-up, a one-off hardware swap, or a quietly
re-rolled corpus revision looks identical to a real engineering win.
If any of those shortcuts are admissible, the lab stops being an
audit trail and becomes an optimisation game.

This policy exists to prevent a benchmark packet from being
numerically precise and methodologically weak. It binds every
run-result row to:

1. a declared **variance class** that fixes run count, warm-up, and
   aggregation before the row is admissible;
2. a declared **comparability posture** that resolves mechanically
   from the record's environment, corpus, and catalog fields, not
   from narrative prose;
3. a declared **quarantine status** that drives the dashboard, the
   benchmark-report packet, and the publication packet from one
   shared field rather than three re-derivations; and
4. an explicit **baseline lineage** block so every claim can be
   traced back to the exact baseline it was scored against.

Adding a rule here is additive-minor. Renaming a rule id,
repurposing a quarantine trigger, or weakening a clearance condition
is **breaking** and opens a decision row in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

## 2. Scope

### In scope

- Run-count, warm-up, discard, and aggregation rules for
  microbenchmarks, protected-journey captures, restore-fidelity
  ratios, power/thermal windowed captures, and contract/correctness
  gates.
- Variance classes, admissible threshold modes per class, and the
  confidence level every class defaults to.
- Comparability-dimension table and the mechanical verdict for each
  axis delta (build identity, toolchain pin, corpus revision,
  protected-metrics revision, fitness-catalog revision, hardware
  definition, lab-image revision, display class, power posture,
  thermal posture, calibration rule set, capture path, run context).
- Quarantine rules for lab-image changes, power/thermal posture
  changes, hardware replacements, capture-path shifts, corpus-
  revision drift, and harness self-health failures, with declared
  clearance conditions.
- Report-field bindings: comparability note, confidence level,
  baseline lineage, quarantine status, sample count observed, and
  discarded sample count.
- Regression-review rubric distinguishing noise, suspect environment
  drift, and likely code regression.

### Out of scope

- Marketing benchmark narratives. This policy governs the evidence
  packet those surfaces would have to quote; it does not produce
  public copy.
- A general "statistical perfection on all future workloads"
  program. Classes and thresholds evolve per workload under the
  benchmark council charter; this policy freezes the vocabulary
  they evolve inside.
- Replacing the fitness-function catalog or the protected-metrics
  file. Those remain the canonical registers; this policy binds
  how runs against them are captured and compared.

## 3. Vocabulary and authority

All vocabularies cited below are **closed**. A run-result row, a
dashboard snapshot, or a publication packet that introduces a value
outside these sets is non-conforming. The full vocabularies live in
the companion YAMLs; this document names them so the same closure
holds across the human and machine forms.

- Variance classes: see
  `artifacts/benchmarks/variance_classes.yaml#classes[]`.
- Warm-up postures: see
  `artifacts/benchmarks/variance_classes.yaml#warm_up_postures[]`.
- Discard rules: see
  `artifacts/benchmarks/variance_classes.yaml#discard_rules[]`.
- Aggregation rules: see
  `artifacts/benchmarks/variance_classes.yaml#aggregation_rules[]`.
- Confidence levels: see
  `artifacts/benchmarks/variance_classes.yaml#confidence_levels[]`.
- Comparability dimensions: see
  `artifacts/benchmarks/quarantine_rules.yaml#comparability_dimensions[]`.
- Quarantine status values: see
  `artifacts/benchmarks/quarantine_rules.yaml#quarantine_status_values[]`.
- Run context, comparability class, quarantine reason class, row
  result, regression trigger kind, threshold mode, SLI kind,
  data-source kind, lane class, trigger kind: frozen at
  `schemas/benchmarks/run_result.schema.json`.

### 3.1 Authority

- **Benchmark council** owns variance-class bindings, threshold
  modes per row, the quarantine rules table, and any clearance
  condition that requires a recorded decision row. Charter:
  [`/docs/governance/benchmark_council_charter.md`](../governance/benchmark_council_charter.md).
- **Performance council** is the waiver authority on fitness-catalog
  rows. A quarantine rule does **not** waive a row; it downgrades
  comparability until a clearance condition is met.
- **Performance owner and lane DRI** may add a new variance class
  row (additive-minor) when a lane acquires a workload no existing
  class fits. Removing a class or repurposing an id requires a
  decision row.

## 4. Run counts, warm-up, and discard

Every fitness-catalog row MUST cite exactly one variance class id
from `variance_classes.yaml#classes[]`. The variance class fixes
the statistical posture; row-level overrides are not admissible.

### 4.1 Variance-class summary

| Variance class                                               | Min runs | Recommended | Warm-up posture                  | Discard rule                  | Aggregation                | Default confidence              |
| ------------------------------------------------------------ | -------: | ----------: | -------------------------------- | ----------------------------- | -------------------------- | ------------------------------- |
| `variance_class.microbench.low_noise`                        |       30 |          50 | `fixed_iteration_warm_up`        | `discard_first_n`             | `p50_and_p95`              | `directional`                   |
| `variance_class.microbench.io_or_storage_bound`              |       60 |         100 | `fixed_duration_warm_up`         | `discard_first_n_and_outliers`| `p50_p95_and_p99`          | `directional`                   |
| `variance_class.protected_journey.cold_start`                |        5 |          10 | `no_warm_up`                     | `no_discard`                  | `p50_and_p95`              | `bounded_with_quarantine_rule`  |
| `variance_class.protected_journey.warm_steady_state`         |       10 |          20 | `workflow_warm_and_settle`       | `discard_first_n`             | `p50_and_p95`              | `bounded_with_quarantine_rule`  |
| `variance_class.protected_journey.save_pipeline_latency`     |       10 |          20 | `workflow_warm_open`             | `discard_first_n`             | `p50_p95_and_p99`          | `bounded_with_quarantine_rule`  |
| `variance_class.restore_fidelity.ratio_gate`                 |       50 |         200 | `no_warm_up`                     | `no_discard`                  | `ratio_with_bootstrap_ci`  | `bounded_with_quarantine_rule`  |
| `variance_class.power_thermal.window_capture`                |        3 |           5 | `workflow_warm_and_settle`       | `no_discard`                  | `p50_and_p95`              | `trend_only`                    |
| `variance_class.contract_gate.single_pass_fail`              |        1 |           1 | `no_warm_up`                     | `no_discard`                  | `boolean_all_must_pass`    | `contract_verified`             |
| `variance_class.structural_digest.identity_only`             |        1 |           2 | `no_warm_up`                     | `no_discard`                  | `structural_digest_identity` | `contract_verified`            |
| `variance_class.harness_self_health.boolean_gate`            |        1 |           1 | `no_warm_up`                     | `no_discard`                  | `boolean_all_must_pass`    | `contract_verified`             |

### 4.2 Microbenchmarks

Microbenchmark rows (`applies_to: microbenchmark_scenario`) run
**at least 30 post-warm-up samples** by default. Warm-up runs a
**fixed iteration count** or a **fixed duration** before recording
begins, and the class's `discard_rule` is applied to the recorded
samples before aggregation. The aggregation rule names which
percentiles land on the run-result row; microbenchmark rows report
**both p50 and p95** (p99 additionally on IO-bound classes).

An observed coefficient of variation outside the class's
`coefficient_of_variation_budget` downgrades the emission to
`confidence_level = trend_only`, even when the point estimate
passes the protected-metrics threshold. See the worked example at
[`/fixtures/benchmarks/stat_method_examples/warm_up_and_discard_microbench.json`](../../fixtures/benchmarks/stat_method_examples/warm_up_and_discard_microbench.json).

### 4.3 Protected-journey captures

Protected-journey rows (cold-start, warm steady state, save
pipeline) run fewer iterations than a microbenchmark — each run is
itself the signal. The minimum and recommended counts are set so
that a nightly lane can realistically land them without the
noise-suppression heuristics a microbench can afford.

- Cold-start journeys (`variance_class.protected_journey.cold_start`)
  use **no warm-up** between measured runs; the "warm" in
  warm-start refers to the OS filesystem cache being primed, not
  the process. The harness primes the OS cache with one discarded
  open of the reference workspace before the first measured run.
- Warm-steady-state journeys
  (`variance_class.protected_journey.warm_steady_state`) open the
  workspace once, **wait for a quiet settle window**
  (`workflow_warm_and_settle`, default 1500 ms), and then sample.
- Save-pipeline journeys report **p99 in addition to p50 and p95**
  because the tail is the protected claim; regressions commonly
  appear on p99 before p50 on filesystem-bound paths.

### 4.4 Ratio gates and structural digests

- Restore-fidelity rows
  (`variance_class.restore_fidelity.ratio_gate`) emit a point
  estimate in [0, 1] and a 95% bootstrap confidence interval over
  **1 000 resamples**. A confidence interval wider than
  (1 - threshold) downgrades the emission to `trend_only`
  regardless of the point estimate; see
  [`/fixtures/benchmarks/stat_method_examples/restore_fidelity_bootstrap_ci_wide.json`](../../fixtures/benchmarks/stat_method_examples/restore_fidelity_bootstrap_ci_wide.json).
- Structural-digest rows
  (`variance_class.structural_digest.identity_only`) are not
  numeric. The aggregation is "the digest matches the committed
  seed". A mismatch is a validation failure on the
  benchmark-lab-health row, not a performance regression.

### 4.5 Contract gates and harness self-health

Contract-gate rows (`variance_class.contract_gate.single_pass_fail`)
and harness self-health rows
(`variance_class.harness_self_health.boolean_gate`) run **one
sample per corpus row** because the claim is boolean. A failure is
release-blocking regardless of `confidence_level`, and environment-
drift heuristics MUST NOT mute them — see
`variance_classes.yaml#invariants.contract_gate_failures_are_not_noise`.

## 5. Aggregation and what the row reports

Every run-result row MUST report at least these fields alongside
the schema-required block:

- `variance_class_id` and `variance_class_revision`.
- `confidence_level` (from the variance class, possibly
  downgraded).
- `comparability_note_ref` — opaque ref resolving to the
  human-readable comparability note on the dashboard.
- `baseline_lineage` — block carrying `baseline_run_ref`,
  `baseline_build_identity_ref`, `baseline_corpus_manifest_revision`,
  `baseline_protected_metrics_revision`,
  `baseline_fitness_catalog_revision`,
  `baseline_hardware_definition_ref`,
  `baseline_environment_definition_ref`, and
  `baseline_confidence_level`.
- `quarantine_status` from
  `quarantine_rules.yaml#quarantine_status_values[]`.
- `sample_count_observed` (integer, after discard).
- `discarded_sample_count` (integer, sum of warm-up discard and
  outlier discard).

Optional fields are listed in
`variance_classes.yaml#report_fields.optional` and include
per-percentile confidence intervals or standard errors, bootstrap
resample count (required for `ratio_with_bootstrap_ci`), and a
variance-class drift note ref when a run used a narrower class than
the row's declared class.

### 5.1 Field: comparability note

The comparability note is a **short, human-readable line** that
resolves through `comparability_note_ref`. It never carries raw
URLs, raw log bodies, or raw fixture bytes; it narrates what changed
and which rule it tripped. The dashboard and publication packs
quote it verbatim.

### 5.2 Field: confidence level

Every row inherits the variance class's `default_confidence_level`.
Tooling MUST downgrade it when:

- `run_context_class != reference_capture` — no row may claim
  `reference_grade` on a `provisional_capture`, `self_capture`, or
  `smoke_subset`.
- Observed CoV exceeds the class's
  `coefficient_of_variation_budget`.
- The row's bootstrap confidence interval is wider than the
  threshold headroom.
- `quarantine_status != comparable`.

A downgrade never silently flips the numeric result, but it binds
the confidence label the dashboard and the publication pack
render.

### 5.3 Field: baseline lineage

Tooling MUST resolve `baseline_lineage` before it renders a trend.
Two runs whose baseline lineages disagree on any dimension are
routed through the mechanical comparability-verdict table before a
trend direction is computed. `baseline_lineage.baseline_run_ref`
MUST be null when `quarantine_status` is
`not_yet_comparable`, `quarantine_clearing`, or `quarantined`.

### 5.4 Field: quarantine status

`quarantine_status` is one of `comparable`, `under_observation`,
`not_yet_comparable`, `quarantine_clearing`, or `quarantined`. A
run whose status is `not_yet_comparable`, `quarantine_clearing`,
or `quarantined` MUST NOT show a trend arrow on the dashboard and
MUST NOT be cited as evidence on a benchmark-report packet without
the comparability note explaining why.

## 6. Comparability, mechanical verdicts

Given a candidate run and a baseline run, the dashboard generator
resolves comparability through the table in
`quarantine_rules.yaml#mechanical_comparability_verdicts`. The
axes are:

- `build_identity` — different builds are the **signal**; a
  build-identity delta is comparable by construction.
- `toolchain_pin` — a Rust channel, rustc version, or Cargo.lock
  digest change is `not_yet_comparable` until a re-baseline.
- `corpus_manifest_revision` — `not_yet_comparable`; the corpus
  redefines "the same input". See
  [`/fixtures/benchmarks/stat_method_examples/not_comparable_lab_image_and_corpus_both_changed.json`](../../fixtures/benchmarks/stat_method_examples/not_comparable_lab_image_and_corpus_both_changed.json).
- `protected_metrics_revision` — `not_yet_comparable`; threshold
  snapshots and comparability notes changed.
- `fitness_function_catalog_revision` — `not_yet_comparable`; row
  identity or data source may have shifted.
- `hardware_definition` — `quarantined`; a new hardware row is a
  new machine. See
  [`/fixtures/benchmarks/stat_method_examples/not_comparable_hardware_replaced.json`](../../fixtures/benchmarks/stat_method_examples/not_comparable_hardware_replaced.json).
- `lab_image_revision` — `quarantined`; treat a lab-image bump as
  a hardware replacement until the council widens comparability.
- `display_class`, `power_posture`, `thermal_posture`,
  `calibration_rule_set` — each triggers `not_yet_comparable`
  until a reviewer attaches a comparability note and the next
  reference capture becomes the new baseline for that axis.
- `capture_path` — trace-schema or harness-wrapper version delta
  triggers `not_yet_comparable` with
  `quarantine_reason_class = trace_schema_nonconformance`.
- `run_context` — a `reference_capture` compared against a
  `self_capture`, `provisional_capture`, or `smoke_subset` is
  `not_yet_comparable`; reports keep the distinction even when
  the numbers are on the same axis.

Every axis delta MUST be carried on the candidate's comparability
envelope with the matching `quarantine_reason_class` from
`schemas/benchmarks/run_result.schema.json#quarantine_reason_class`.

### 6.1 Multiple axes change in one window

When more than one axis changes between baseline and candidate, the
candidate's `comparability_envelope.quarantine_reasons[]` MUST list
**every** matching reason. Clearance requires **every** triggered
rule's clearance condition to be satisfied. The worked example at
[`/fixtures/benchmarks/stat_method_examples/not_comparable_lab_image_and_corpus_both_changed.json`](../../fixtures/benchmarks/stat_method_examples/not_comparable_lab_image_and_corpus_both_changed.json)
demonstrates two rules firing simultaneously and refusing to render
a single-axis trend line until both clear.

## 7. Quarantine rules

The machine-readable rules live in
`artifacts/benchmarks/quarantine_rules.yaml#rules`. Every rule
names its trigger event, the `quarantine_reason_class` it binds,
the status it drives (`under_observation`, `not_yet_comparable`, or
`quarantined`), the comparability dimensions it affects, the run
contexts it applies to, the minimum clearance-run count, and the
clearance condition itself.

### 7.1 Rules summary

| Rule id                                                  | Trigger event                                             | Reason class                            | Status               | Min clearance runs |
| -------------------------------------------------------- | --------------------------------------------------------- | --------------------------------------- | -------------------- | -----------------: |
| `rules.lab_image_revision_changed`                       | `lab_image_revision_bumped`                                | `hardware_drift_observed`                | `quarantined`         |                  3 |
| `rules.power_policy_changed`                             | `power_posture_id_changed`                                 | `hardware_drift_observed`                | `not_yet_comparable`  |                  1 |
| `rules.thermal_posture_changed`                          | `thermal_posture_id_changed`                               | `hardware_drift_observed`                | `not_yet_comparable`  |                  1 |
| `rules.screen_configuration_changed`                     | `display_class_id_changed`                                 | `hardware_drift_observed`                | `not_yet_comparable`  |                  1 |
| `rules.calibration_rule_set_changed`                     | `calibration_rule_set_id_or_revision_changed`              | `hardware_drift_observed`                | `not_yet_comparable`  |                  2 |
| `rules.hardware_row_replaced_or_added`                   | `hardware_definition_id_or_revision_changed`               | `hardware_drift_observed`                | `quarantined`         |                  3 |
| `rules.capture_path_changed`                             | `capture_path_axis_changed`                                | `trace_schema_nonconformance`            | `not_yet_comparable`  |                  1 |
| `rules.corpus_manifest_revision_changed`                 | `corpus_manifest_revision_bumped`                          | `corpus_revision_mismatch`               | `not_yet_comparable`  |                  1 |
| `rules.protected_metrics_revision_changed`               | `protected_metrics_revision_bumped`                        | `fitness_catalog_revision_mismatch`      | `not_yet_comparable`  |                  1 |
| `rules.fitness_catalog_revision_changed`                 | `fitness_catalog_revision_bumped`                          | `fitness_catalog_revision_mismatch`      | `not_yet_comparable`  |                  1 |
| `rules.toolchain_pin_drift`                              | `toolchain_pin_digest_or_lockfile_digest_changed`          | `toolchain_pin_drift_observed`           | `not_yet_comparable`  |                  1 |
| `rules.corpus_id_resolution_incomplete`                  | `missing_corpus_id_resolution_observed`                    | `missing_corpus_id_resolution`           | `quarantined`         |                  1 |
| `rules.trace_schema_nonconformance`                      | `trace_schema_conformance_failed`                          | `trace_schema_nonconformance`            | `quarantined`         |                  1 |
| `rules.missing_build_identity_record`                    | `exact_build_identity_ref_unresolvable`                    | `missing_build_identity_record`          | `quarantined`         |                  1 |
| `rules.harness_self_health_failed`                       | `benchmark_lab_health_boolean_gate_failed`                 | `manual_benchmark_council_hold`          | `quarantined`         |                  1 |
| `rules.manual_benchmark_council_hold`                    | `benchmark_council_raised_hold`                            | `manual_benchmark_council_hold`          | `quarantined`         |                  1 |
| `rules.suspect_environment_drift_observed`               | `environment_drift_heuristic_tripped`                      | `hardware_drift_observed`                | `under_observation`   |                  2 |

### 7.2 Clearance

A rule's clearance condition names exactly what must happen before
the rule stops applying. Common shapes:

- **One reference-capture run returns to threshold** — used when
  the rule documents drift but does not invalidate the baseline
  (e.g. display-class change with a narrower claim).
- **Two consecutive reference captures agree** — used when the
  rule flags a soft drift that may be noise (e.g. suspect
  environment drift) or a calibration checklist change.
- **Three consecutive reference captures + council decision row**
  — used for hardware replacements and lab-image revision bumps;
  the council records a re-baseline decision in
  `artifacts/governance/decision_index.yaml` before trend
  comparison resumes.
- **Manual council release** — the only admissible clearance for
  `rules.manual_benchmark_council_hold`.

Runs in the clearance window carry `quarantine_status =
quarantine_clearing`. The dashboard MUST render them as
"clearing" and MUST NOT roll them into the comparable trend
until the window completes.

### 7.3 Suspect environment drift is a soft flag

`rules.suspect_environment_drift_observed` sets
`quarantine_status = under_observation`, not `quarantined`. The
heuristic fires when multiple unrelated fitness rows regress
together on the same run with no build or corpus delta — see
[`/fixtures/benchmarks/stat_method_examples/suspect_environment_drift.json`](../../fixtures/benchmarks/stat_method_examples/suspect_environment_drift.json).
The intent is that the dashboard prompts the reviewer before a
single row is filed as a code regression; two consecutive clean
runs auto-clear the flag.

## 8. Regression review rubric

`quarantine_rules.yaml#regression_review_rubric` names three
mutually-reinforcing triage entries plus a hold-review entry for
non-comparable pairs.

### 8.1 Likely code regression

All of:

- mechanical comparability verdict is `comparable_to_baseline`;
- quarantine status is `comparable`;
- observed delta exceeds the protected-metrics threshold band;
- no environment axis changed.

Action: open a regression issue citing the fitness row, the
`regression_trigger_ref.kind`, and the baseline lineage. A waiver
requires the row's co-waiver authority per the fitness catalog.
See
[`/fixtures/benchmarks/stat_method_examples/likely_code_regression.json`](../../fixtures/benchmarks/stat_method_examples/likely_code_regression.json).

### 8.2 Suspect environment drift

Any of:

- `quarantine_status = under_observation`;
- multiple unrelated rows regressed together on one run with no
  code delta;
- environment-drift heuristic tripped.

Action: hold the regression review, rerun the same run context on
the same hardware row and environment row, and only file a code
regression if the drift does not reproduce. See
[`/fixtures/benchmarks/stat_method_examples/suspect_environment_drift.json`](../../fixtures/benchmarks/stat_method_examples/suspect_environment_drift.json).

### 8.3 Noise within variance budget

All of:

- mechanical comparability verdict is `comparable_to_baseline`;
- quarantine status is `comparable`;
- observed delta is inside the protected-metrics threshold band;
- observed CoV is inside the variance-class
  `coefficient_of_variation_budget`.

Action: mark the row unchanged on the dashboard; do not open a
regression issue. See
[`/fixtures/benchmarks/stat_method_examples/noise_within_variance_budget.json`](../../fixtures/benchmarks/stat_method_examples/noise_within_variance_budget.json).

### 8.4 Not comparable — hold review

Any of:

- `quarantine_status = not_yet_comparable`;
- `quarantine_status = quarantine_clearing`;
- `quarantine_status = quarantined`.

Action: do not interpret the numeric delta as a regression or
improvement; follow the clearance condition on the rule that set
the status before re-attempting trend comparison. See
[`/fixtures/benchmarks/stat_method_examples/not_comparable_hardware_replaced.json`](../../fixtures/benchmarks/stat_method_examples/not_comparable_hardware_replaced.json)
and
[`/fixtures/benchmarks/stat_method_examples/not_comparable_lab_image_and_corpus_both_changed.json`](../../fixtures/benchmarks/stat_method_examples/not_comparable_lab_image_and_corpus_both_changed.json).

## 9. How consumers read this policy

One shape; four consumers.

### 9.1 Nightly CI lane

The nightly lane emits run-result records conforming to
`schemas/benchmarks/run_result.schema.json` and carrying the
additional report fields declared here
(`variance_class_id`, `confidence_level`, `comparability_note_ref`,
`baseline_lineage`, `quarantine_status`, `sample_count_observed`,
`discarded_sample_count`). The lane's verify-seed gate runs before
the lab; a seed drift trips
`rules.harness_self_health_failed` and quarantines the run before
any benchmark work happens.

### 9.2 Dashboard generator

The dashboard resolves comparability through
`quarantine_rules.yaml#mechanical_comparability_verdicts`, applies
rules in order of `quarantine_trigger_event`, and renders the
row with the resulting `quarantine_status` and the matching
`trend_direction_class`. It MUST NOT recompute comparability from
narrative notes.

### 9.3 Benchmark-report governance packet

Benchmark-report packets
(`schemas/governance/governance_packet.schema.json` with
`packet_family = benchmark_report`) consume the same fields the
dashboard reads, plus the
`fitness_function_catalog.yaml#packet_export_shape.fitness_function_snapshot`
block. A packet that omits `quarantine_status` or
`baseline_lineage` is non-conforming.

### 9.4 Public benchmark publication pack

Public publication packs
(`docs/benchmarks/benchmark_publication_pack_template.md`) compose
the same fields. A row whose `quarantine_status` is not
`comparable` MUST NOT be quoted as a claim-bearing result in a
publication pack; the publication rules in
[`/docs/benchmarks/public_comparison_rules.md`](./public_comparison_rules.md)
already forbid it, and this policy binds the mechanical check that
enforces the rule.

## 10. Worked examples

The files in
[`/fixtures/benchmarks/stat_method_examples/`](../../fixtures/benchmarks/stat_method_examples/)
are the canonical worked examples tooling and reviewers read when
they need to confirm that a given run-pair is handled the way this
policy declares:

- `noise_within_variance_budget.json` — noise path.
- `likely_code_regression.json` — code-regression path.
- `suspect_environment_drift.json` — environment-drift path.
- `not_comparable_hardware_replaced.json` — mechanical quarantine
  on a hardware row change; the policy's acceptance-language
  reference point for "two benchmark packets from non-comparable
  environments can be marked non-comparable mechanically".
- `not_comparable_lab_image_and_corpus_both_changed.json` —
  two-axis quarantine.
- `restore_fidelity_bootstrap_ci_wide.json` — bootstrap CI
  downgrade.
- `warm_up_and_discard_microbench.json` — warm-up and discard
  pipeline.

Adding a worked example is additive-minor; renaming or removing
one is breaking because this document cites each file by name.

## 11. Change control

Changes that fall under this policy are governed by
[`/docs/benchmarks/corpus_governance.md`](./corpus_governance.md).
Specifically:

- Adding a variance class or a quarantine rule is additive-minor;
  landed here, in
  `artifacts/benchmarks/variance_classes.yaml` or
  `artifacts/benchmarks/quarantine_rules.yaml`, and in the
  control-artifact index in the same change.
- Renaming a class id, repurposing a trigger event, or relaxing
  a clearance condition is breaking and opens a decision row in
  `artifacts/governance/decision_index.yaml`.
- Threshold easing or corpus removal that moves a row into a new
  variance class carries the threshold-easing requirements from
  `artifacts/bench/protected_metrics.yaml#threshold_easing_requirements`
  and a comparability note referencing this policy.
- Public-comparison changes inherit the rules in
  [`/docs/benchmarks/public_comparison_rules.md`](./public_comparison_rules.md).

## 12. Acceptance criteria covered

This policy satisfies the acceptance language that introduced it:

- **Two benchmark packets from non-comparable environments can be
  marked non-comparable mechanically.** Section 6 names the axes,
  `quarantine_rules.yaml#mechanical_comparability_verdicts` encodes
  the table, and the two worked examples at
  `not_comparable_hardware_replaced.json` and
  `not_comparable_lab_image_and_corpus_both_changed.json`
  demonstrate the mechanical verdict on one-axis and two-axis
  deltas.
- **Regression review can distinguish noise, suspect environment
  drift, and likely code regression.** Section 8 names the three
  rubric entries, `quarantine_rules.yaml#regression_review_rubric`
  encodes the machine form, and the three worked examples at
  `noise_within_variance_budget.json`,
  `suspect_environment_drift.json`, and
  `likely_code_regression.json` demonstrate each path.
- **Dashboard and publication packets can consume the same
  comparability and quarantine fields.** Section 5 names the
  report fields, `variance_classes.yaml#report_fields` encodes
  the binding, and Section 9 describes how the nightly lane, the
  dashboard generator, the benchmark-report packet, and the
  publication pack all consume the one set of fields.
