# M5 Experiment-Component Accessibility & Auto-Narrowing (M05-1018)

This lane is the accessibility / keyboard / screen-reader / CLI / export parity and honest
auto-narrowing capstone over the frozen M5 experiment-component matrix
(`freeze_the_m5_experiment_run_row_...`). Where the freeze matrix defines the reusable experiment
run row, dataset provenance card, artifact lineage panel, run comparison table, environment
fingerprint card, compare guard banner, sensitivity / sharing banner, and result summary card
primitives — and the 1013–1017 implementation / consumer lanes resolve their per-surface truth —
this lane certifies, per component family, that experiment claims stay **keyboard-complete,
assistive-tech-reachable, CLI/export-safe, and self-narrowing**.

## What it guarantees

- **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
  screen-reader-reachable, and CLI/headless-reachable path into the same run origin, code revision,
  environment fingerprint, dataset provenance, sensitivity state, comparability / confounder
  disclosure, and summary-versus-evidence-versus-raw export scope the rich component shows — never a
  hover-only chip. The hierarchy-heavy artifact lineage panel (nested producing-run / artifact /
  derived-artifact lineage) additionally binds its tree to a flat list / textual path.
- **Export parity.** The support / release / CLI export reconstructs each component's meaning from
  typed tokens and opaque refs **without a raw payload**, preserving stable run IDs, code revisions,
  provenance / sensitivity posture, comparability disclosure, export scope, and narrowing reasons —
  so support, docs, and release proof can reconstruct exactly what the user was actually shown
  without leaking blocked raw data.
- **Honest auto-narrowing.** When a dataset's sensitivity blocks preview, artifact lineage is stale
  or missing, an environment fingerprint is partial, comparison evidence is incomplete, a compare
  guard is blocked, or a dataset's provenance is severed, the component's result claim auto-narrows
  from `exact_comparable_result` / `reviewable_result` to a partial-fingerprint / incomparable-runs
  / guard-blocked / stale-lineage / unprovenanced-data / blocked-preview projection, discloses the
  narrowing with a precise trigger and binding dimension, and preserves the canonical run-identity /
  provenance / lineage. A partial-fingerprint / incomplete-comparison / stale-lineage /
  severed-provenance state can never keep an exact comparable-result claim, and an unproven
  comparison never implies an apples-to-apples fair baseline.
- **Cross-surface disclosure.** The same narrowed state surfaces in the notebook, experiment
  dashboard, comparison, data-catalog, lineage, review, CLI, support-export, and product surfaces so
  product, docs, and release publication stay aligned on downgrade behavior.

## Model

- **Result claim tiers** (strongest first): `exact_comparable_result`, `reviewable_result`,
  `partial_fingerprint_projection`, `incomparable_runs_projection`, `guard_blocked_projection`,
  `stale_lineage_projection`, `unprovenanced_data_projection`, `blocked_preview_projection`.
- **Claim dimensions** (1:1 with the eight families): `run_origin_traceability`,
  `dataset_provenance`, `artifact_lineage`, `comparability_evidence`, `environment_fingerprint`,
  `compare_guard_clearance`, `sensitivity_disclosure`, `export_scope_clarity`.
- **Condition states**: `live_exact_result` (baseline) plus the operational states
  `compare_guard_blocked` and `sensitivity_blocks_preview`, and the four "cannot-be-proven"
  incomplete-evidence narrowing axes `fingerprint_partial`, `comparability_incomplete`,
  `lineage_stale`, and `provenance_severed`.

Each condition state maps 1:1 to a permitted claim ceiling and names the on-topic frozen downgrade
trigger (`environment_fingerprint_unstated`, `comparability_overstated`, `cached_state_hidden`,
`dataset_provenance_severed`, `sensitivity_class_unstated`) so certified reasons stay
byte-identical to the freeze matrix. Only the four cannot-be-proven incomplete-evidence states can
never keep an exact comparable-result claim; a blocked compare guard and a sensitivity-blocked
preview are honest guard / privacy operations, not exactness overstatements.

## Artifacts

- Schema: `schemas/ui/m5-experiment-component-accessibility-fallback.schema.json`
- Support export (canonical): `artifacts/release/m5-experiment-component-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-experiment-component-accessibility-fallback/matrix.csv`
- Report: `artifacts/release/m5-experiment-component-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-experiment-component-accessibility-fallback/`

Regenerate the checked-in artifacts with:

```
GEN_EXPERIMENT_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-notebook generate_artifacts
```
