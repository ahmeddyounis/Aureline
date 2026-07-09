# M5 experiment-component matrix contract

This document is the human-readable companion to the frozen M5 experiment-component matrix.
The authoritative gate is the Rust validator in
`crates/aureline-notebook/src/freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix`.
The checked-in support export under `artifacts/release/m5-experiment-component-proof/` is the
single source of truth; the schemas under `schemas/ui/` document the shape.

## Purpose

The matrix freezes the reusable experiment / reproducibility components so notebook-adjacent
run summaries, dataset provenance, lineage, and comparison surfaces stop drifting across
claimed M5 data workflows. It names each component family once and binds it to stable run
IDs, execution origin, comparability labels, sensitivity classes, and export-scope truth
before widening consumer coverage.

## Component families

- `experiment_run_row` — where a run came from (notebook cell, script task, scheduled task,
  manual attach, imported run, unknown origin) and where it stands (queued, running,
  succeeded, failed, canceled, stale).
- `dataset_provenance_card` — what data a run used (tracked dataset, local file, remote
  snapshot, synthetic data, redacted sample, unknown source) and how completely it is
  provenanced (complete, partial, missing, version pinned, version drifted, access
  restricted).
- `artifact_lineage_panel` — what a generated artifact is (model checkpoint, metrics table,
  plot / figure, exported report, log bundle, unknown artifact) and how completely its
  lineage resolves (complete, partial, broken, derived upstream known / unknown,
  regenerated).
- `run_comparison_table` — along which axis runs are compared (metric delta, param diff,
  dataset diff, env diff, code revision diff, artifact diff) and whether two runs are
  actually comparable (comparable, comparable with caveats, not comparable, confounded,
  insufficient overlap, unknown comparability).
- `environment_fingerprint_card` — which slice of the environment is captured (interpreter,
  kernel spec, packages, hardware accelerator, OS / platform, container image) and how
  completely (captured complete / partial / missing, pinned, drifted, unavailable).
- `compare_guard_banner` — why a comparison is guarded (dataset mismatch, environment drift,
  code revision gap, metric definition change, sample size imbalance, confounder present)
  and what the guard permits (permitted, caveated, blocked, acknowledged, overridden by
  choice, unavailable).
- `sensitivity_sharing_banner` — how sensitive a result or dataset is (public-safe, internal,
  confidential, regulated, production-like, unknown) and what a share includes (summary only,
  summary plus metadata, evidence included, raw payload included, redacted share, share
  blocked).
- `result_summary_card` — what a summary is showing (headline metric, metric table, narrative
  summary, evidence link, raw payload reference, no result) and what scope it exports
  (summary, metadata, evidence, raw, redacted, export withheld).

## One controlled disposition vocabulary

Every consumer binds one disposition vocabulary; no surface invents a parallel word for it:
`local_run`, `managed_run`, `imported_run`, `manual_attach`, `reproducible`,
`likely_reproducible`, `needs_rerun`, `context_incomplete`.

## Hard invariants

Every component row must keep all four of these `false`:

- `masks_provenance_or_sensitivity_state`
- `hides_run_origin_or_revision`
- `implies_apples_to_apples_without_parity`
- `invents_alternate_state_label`

## Regenerating the artifacts

The checked-in support export, matrix CSV, design report, and narrowed fixtures are minted
only from the seed builders through the headless emitter:

```sh
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- support-export > artifacts/release/m5-experiment-component-proof/support_export.json
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- csv > artifacts/release/m5-experiment-component-proof/matrix.csv
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- report > artifacts/design/m5-experiment-component-matrix.md
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- fixture-run-comparison-table-beta-narrowed > fixtures/ui/m5-experiment-components/run_comparison_table_beta_narrowed.json
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_experiment_component_matrix -- fixture-sensitivity-sharing-banner-preview-narrowed > fixtures/ui/m5-experiment-components/sensitivity_sharing_banner_preview_narrowed.json
```

The inline tests assert the checked-in artifact and fixtures match the seed builders, so any
drift fails `cargo test -p aureline-notebook`.
