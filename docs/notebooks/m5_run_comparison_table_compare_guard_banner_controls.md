# M5 run comparison tables and compare guard banners

The run comparison table and the compare guard banner are two of the eight governed experiment
components frozen by the
[M5 experiment-component matrix](m5_experiment_component_matrix.md). This lane implements those
two families as two co-equal control vectors in one export-safe packet,
[`RunComparisonTableCompareGuardBannerControlsPacket`](../../crates/aureline-notebook/src/implement_run_comparison_tables_and_compare_guard_banners_with_baseline_candidate_identity_confounder_disclosure_and_no_fair_delta_claims_when_parity_evidence_is_incomplete_across_claimed_m5_compare_flows/mod.rs),
so a claimed M5 notebook, experiment-dashboard, comparison, lineage, share-review, or CLI surface
can project a comparison table and a guard banner that keep metric deltas **honest about code,
data, environment, and hardware comparability** — never implying a fair baseline when the parity
evidence is incomplete.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never asserted.

### `resolve_run_comparison`

Given a comparison table's comparability state, the resolver derives a **fairness class**:

- `comparable` → `fair_baseline`
- `comparable_with_caveats` → `caveated_baseline` (must carry an explicit caveat note), not a fair
  baseline
- `not_comparable` → `unfair_baseline` (must carry an explicit not-comparable note), not a fair
  baseline
- `confounded` → `unfair_baseline` (must carry an explicit confounder note), not a fair baseline
- `insufficient_overlap` → `unproven_baseline` (must carry an explicit insufficient-overlap note),
  not a fair baseline
- `unknown_comparability` → `unproven_baseline` (must carry an explicit unknown-comparability
  note), not a fair baseline

Every table always names its baseline and candidate run identities, its metric values, its delta,
its threshold state, its confidence note, its comparator type, and its explicit **code / data /
environment / hardware difference summaries**, so the differences always stay beside the delta and
a not-comparable, confounded, or insufficiently-overlapping comparison can never read as a fair
apples-to-apples baseline.

### `resolve_compare_guard`

Given a guard banner's guard state, the resolver derives a **guard comparability class**:

- `comparison_permitted` → `comparable_permitted` (permits a fair comparison)
- `comparison_caveated` / `guard_acknowledged` → `partially_comparable` (must carry an explicit
  partial-comparability note)
- `guard_overridden_by_choice` → `overridden_comparison` (must carry an explicit override warning)
- `comparison_blocked` → `not_comparable_blocked` (must carry an explicit blocked note)
- `guard_unavailable` → `guard_unavailable` (must carry an explicit unavailable note)

Only a `comparable_permitted` guard permits a fair comparison, so a blocked or overridden guard is
never silently bypassed and a comparison is never permitted apples-to-apples when the parity
evidence is incomplete.

## Baseline / candidate identity, difference factors, and guard truth

- **Baseline and candidate identity** — every comparison table names its baseline and candidate
  run ids and labels, so no side of a comparison is ever anonymous.
- **Delta beside the differences** — every comparison table names its metric values, delta,
  threshold state, confidence, comparator type, and its code / data / environment / hardware
  difference summaries, so the delta is never read without the factors that could confound it.
- **Open / export** — every comparison table offers the mandatory `open_baseline_run`,
  `open_current_run`, and `export_comparison` actions, plus `open_full_lineage`, `open_deep_link`,
  and `copy_comparison_id` as appropriate.
- **Comparability disclosure** — every guard banner names what is comparable, partially comparable,
  or not comparable, which lineage fields are missing, which environment / data / code factors
  changed, and what was redacted.
- **Open full lineage / review** — every guard banner offers the mandatory `open_full_lineage` and
  `review_comparability` actions, plus `view_changed_factors`, `acknowledge_guard`,
  `open_deep_link`, and `copy_guard_id` as appropriate.
- **Stable deep links** — every next step names a stable `run_object`, `notebook_location`,
  `dataset_catalog_anchor`, or `docs_anchor` deep link with a resolvable reference. A component
  that offers a deep-link action must name a resolvable kind, so a next step is never an ephemeral
  overlay or hidden route.

## Controlled trust labels

The four reproducibility trust labels — `reproducible`, `likely_reproducible`, `needs_rerun`, and
`context_incomplete` — remain controlled labels drawn from the one shared experiment disposition
vocabulary. Validation requires that all four appear across the comparison tables and guard banners
so they stay first-class and consistent across compare surfaces, exports, and support evidence.

## Hard invariants

Every component keeps five bools `false`, and validation flags any that is `true`:

- `masks_provenance_or_sensitivity_state` — provenance and sensitivity posture stay visible.
- `hides_baseline_or_candidate_identity` — both compared runs stay named.
- `hides_difference_factors_beside_delta` — the code / data / environment / hardware differences
  stay beside the delta.
- `implies_apples_to_apples_without_parity` — a comparison is never implied comparable without
  parity evidence.
- `invents_alternate_state_label` — no surface invents a second word for a governed comparability,
  fairness, guard-reason, or guard-state label.

Tables and banners reuse Aureline's existing comparability vocabulary instead of inventing
comparison-specific exceptions; cached, offline, and local-only state stays visible.

## Coverage

The checked-in support export exercises every fairness class, every comparison axis class, and
every comparability state across the six seeded comparison tables, and every guard comparability
class, every compare guard reason, and every compare guard state across the six seeded guard
banners.

## Source of truth and artifacts

- Boundary schema: [`schemas/ui/m5-run-comparison-table-compare-guard-banner-controls.schema.json`](../../schemas/ui/m5-run-comparison-table-compare-guard-banner-controls.schema.json)
- Support export: [`artifacts/release/m5-run-comparison-table-compare-guard-banner-proof/support_export.json`](../../artifacts/release/m5-run-comparison-table-compare-guard-banner-proof/support_export.json)
- Matrix CSV: [`artifacts/release/m5-run-comparison-table-compare-guard-banner-proof/matrix.csv`](../../artifacts/release/m5-run-comparison-table-compare-guard-banner-proof/matrix.csv)
- Design report: [`artifacts/design/m5-run-comparison-table-compare-guard-banner.md`](../../artifacts/design/m5-run-comparison-table-compare-guard-banner.md)
- Scenario fixtures: [`fixtures/ui/m5-run-comparison-table-compare-guard-banner-controls/`](../../fixtures/ui/m5-run-comparison-table-compare-guard-banner-controls/)

Regenerate every artifact and fixture from the single seed with the headless emitter:

```sh
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- support-export
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- csv
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- report
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- fixture-comparison-table-not-comparable
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- fixture-compare-guard-banner-blocked
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_run_comparison_table_compare_guard_banner_primitive -- validate
```
