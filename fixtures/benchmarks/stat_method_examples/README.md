# Benchmark stat-method worked examples

This directory carries worked examples for the policy at
[`/docs/benchmarks/statistics_and_quarantine_policy.md`](../../../docs/benchmarks/statistics_and_quarantine_policy.md).

Every JSON file is a small, self-describing record that:

- names the variance class(es) from
  [`/artifacts/benchmarks/variance_classes.yaml`](../../../artifacts/benchmarks/variance_classes.yaml)
  the example exercises;
- names the quarantine rule(s) from
  [`/artifacts/benchmarks/quarantine_rules.yaml`](../../../artifacts/benchmarks/quarantine_rules.yaml)
  the example trips or relies on;
- carries enough of the candidate and (where applicable) baseline
  run-result record for the mechanical verdict to be reconstructed
  without an external harness;
- reports the expected mechanical decision and the regression-review
  rubric entry the decision maps onto.

These files are fixtures, not real run records. They are the
committed reference a reviewer or tool reads to confirm that "two
benchmark packets from non-comparable environments can be marked
non-comparable mechanically" and that "regression review can
distinguish noise, suspect environment drift, and likely code
regression" — the acceptance language on the policy.

## Files

- `noise_within_variance_budget.json` — two comparable runs whose
  observed delta is inside the variance-class CoV budget and the
  protected-metrics threshold band. Expected decision:
  `rubric.noise_within_variance_budget`.
- `likely_code_regression.json` — two comparable runs with no
  environment delta whose observed delta exceeds the protected
  threshold. Expected decision: `rubric.likely_code_regression`.
- `suspect_environment_drift.json` — two comparable runs where four
  unrelated fitness rows regress together on the same run with no
  code or corpus delta. Expected decision:
  `rubric.suspect_environment_drift` and transition to `under_observation`.
- `not_comparable_hardware_replaced.json` — baseline and candidate on
  different hardware rows. Expected decision: mechanical quarantine
  via `rules.hardware_row_replaced_or_added`.
- `not_comparable_lab_image_and_corpus_both_changed.json` — baseline
  and candidate differ on lab-image revision and corpus-manifest
  revision simultaneously. Expected decision: mechanical quarantine
  via `rules.lab_image_revision_changed` and non-comparable verdict
  on `rules.corpus_manifest_revision_changed`.
- `restore_fidelity_bootstrap_ci_wide.json` — restore-fidelity ratio
  row where the bootstrap confidence interval is wider than the
  threshold headroom. Expected decision: confidence downgrade to
  `trend_only` even when the point estimate passes.
- `warm_up_and_discard_microbench.json` — low-noise microbenchmark
  with ten warm-up iterations and five discarded samples; expected
  aggregation is `p50_and_p95` over thirty post-discard samples.

## How tooling reads these

Each file carries:

- `$schema_reference` — repo-relative path to the closest boundary
  schema (`schemas/benchmarks/run_result.schema.json` for run-result
  excerpts).
- `fixture_metadata.name`, `fixture_metadata.scenario`,
  `fixture_metadata.policy_section`, and
  `fixture_metadata.policy_rules_exercised[]`.
- `expected_mechanical_decision` — the verdict a compliant tool
  must reach. Reviewers MUST NOT override this without updating the
  policy doc and the companion YAMLs in the same change.

Adding a new example is additive-minor. Removing or renaming a file
is breaking because the policy doc cites these files by name.
