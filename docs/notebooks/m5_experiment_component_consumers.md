# M5 experiment component consumers

Status: Stable · Schema `schemas/ui/m5-experiment-component-consumer.schema.json` · Record kind `add_shared_notebook_task_test_eval_review_support_and_export_consumers_so_experiment_components_keep_provenance_sensitivity_and_comparison_language_aligned_across_claimed_m5_profiles`

This is the **adoption lane** over the frozen M5 experiment-component matrix
(`docs/notebooks/m5_experiment_component_matrix.md`). The matrix freezes eight governed component
families and four sibling implement lanes narrow them into working primitives:

| Component family | Narrowed primitive | Canonical schema |
| --- | --- | --- |
| `experiment_run_row` | experiment run row / environment fingerprint card | `schemas/ui/m5-experiment-run-row-environment-fingerprint-controls.schema.json` |
| `environment_fingerprint_card` | experiment run row / environment fingerprint card | `schemas/ui/m5-experiment-run-row-environment-fingerprint-controls.schema.json` |
| `dataset_provenance_card` | dataset provenance card / sensitivity-sharing banner | `schemas/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls.schema.json` |
| `sensitivity_sharing_banner` | dataset provenance card / sensitivity-sharing banner | `schemas/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls.schema.json` |
| `artifact_lineage_panel` | artifact lineage panel / result summary card | `schemas/ui/m5-artifact-lineage-panel-result-summary-card-controls.schema.json` |
| `result_summary_card` | artifact lineage panel / result summary card | `schemas/ui/m5-artifact-lineage-panel-result-summary-card-controls.schema.json` |
| `run_comparison_table` | run comparison table / compare guard banner | `schemas/ui/m5-run-comparison-table-compare-guard-banner-controls.schema.json` |
| `compare_guard_banner` | run comparison table / compare guard banner | `schemas/ui/m5-run-comparison-table-compare-guard-banner-controls.schema.json` |

This lane proves those eight families are **reusable components** — not one notebook page plus a few
isolated data objects — by binding every claimed M5 experiment consumer to the same canonical
component schemas and the same descriptor vocabulary.

## Consumers

| Consumer | Token | Role |
| --- | --- | --- |
| Notebook Run History | `notebook_run_history` | lists runs with run-origin, revision, and fingerprint truth |
| Tasks / Tests / Evals | `task_test_eval_runs` | reads run, dataset, and comparison truth for task / test / eval runs |
| Review Evidence | `review_evidence` | surfaces provenance, lineage, and comparability without exposing raw data |
| Compare View | `compare_view` | shows deltas without implying an apples-to-apples fair baseline |
| Companion Summary | `companion_summary` | carries companion-safe, metadata-only summaries honestly |
| CLI / Headless Export | `cli_headless_export` | exports the same fingerprint and lineage truth headlessly |
| Support / Export Packet | `support_export` | the authoritative rendering; references the canonical schemas so its prose can never drift |

Every family is adopted by **at least two** distinct consumers, and the support / export packet
references the canonical schema for every family it adopts.

## Shared descriptor vocabulary

The acceptance criterion is one truth for **lineage / provenance, sensitivity state, comparability,
and export scope** across every experiment surface. Those four descriptors (`lineage_provenance`,
`sensitivity_state`, `comparability`, `export_scope`) are required on every binding, so users no
longer see one lineage claim in the notebook run history, another in a review pane, and a third in
an exported summary for the same run — and comparability / sensitivity / export-scope stay explicit
everywhere.

## Auto-narrowing

A consumer that cannot preserve full parity **auto-narrows** its claim language and always discloses
a self-contained banner naming the exact reason and the recovery action — never a generic
"degraded" note:

| Parity-health mode | Narrowing reason | Recovery action | Export caveat |
| --- | --- | --- | --- |
| `provenance_incomplete_narrowed` | `lineage_provenance_incomplete` | `open_producing_run_or_complete_lineage` | `lineage_provenance_incomplete` |
| `not_comparable_narrowed` | `comparability_unproven` | `review_comparability_before_trusting_delta` | `comparison_not_apples_to_apples` |
| `sensitivity_restricted_narrowed` | `sensitive_data_restricted` | `review_sensitivity_before_sharing` | `sensitive_data_redacted_not_raw` |
| `metadata_only_export_narrowed` | `export_metadata_only` | `request_full_evidence_export_if_permitted` | `export_metadata_only_not_raw` |

### An unproven comparison is never an apples-to-apples fair baseline

`comparability_unproven` reflects a comparison whose parity evidence is incomplete. The resolver
marks such a binding `reflects_unproven_comparability = true`, always narrows it, and always resolves
`asserts_apples_to_apples_parity = false`. Only a full-parity binding may assert an apples-to-apples
fair comparison. This is the acceptance criterion that a metric delta no longer implies a fair
baseline without parity evidence on any claimed M5 experiment consumer, and that raw production-like
data is never exposed by default.

## Resolver

`resolve_experiment_component_binding` takes one consumer's adoption of one component family, the
descriptor set it surfaces, the parity-health mode, and any export caveats, and produces one
`M5ExperimentComponentResolvedBinding`. It rejects an empty or incomplete descriptor set and any
forbidden binding material, keeps the descriptor vocabulary aligned at full parity, auto-narrows
under any weakened mode, and — when narrowed — emits a self-contained banner.

## Governance & proof

The checked support export, matrix CSV, and Markdown report live under
`artifacts/release/m5-experiment-component-consumer-proof/`, and the two narrowed fixtures
(compare view → Beta, review evidence → Preview) live under
`fixtures/ui/m5-experiment-component-consumers/`. All are minted only by the
`aureline_notebook_m5_experiment_component_consumers` headless emitter so the in-code matrix, the
artifact, the worked bindings, and the fixtures never drift. Raw secrets, endpoints, tokens, and
raw provider bodies never cross this boundary.
