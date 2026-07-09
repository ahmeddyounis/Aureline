# Run comparison tables and compare guard banners

- Packet: `m5-run-comparison-table-compare-guard-banner-controls:stable:0001`
- Surface: `M5 run comparison tables and compare guard banners: baseline/candidate identity, confounder disclosure, and no-fair-delta claims when parity evidence is incomplete across claimed compare flows`
- Run comparison tables: 6 (5 not a fair baseline)
- Compare guard banners: 6 (5 do not permit a fair comparison)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Run comparison tables

- **Ranker NDCG@10: v7 vs v6** — axis `metric_delta`, comparability `comparable` → `fair_baseline`, baseline `run-notebook-1041` vs candidate `run-notebook-1042`, deep link `run_object`
- **Learning-rate sweep: lr=3e-4 vs lr=1e-4** — axis `param_diff`, comparability `comparable_with_caveats` → `caveated_baseline`, baseline `run-notebook-1039` vs candidate `run-notebook-1043`, deep link `notebook_location`
- **Accuracy: managed run vs local run** — axis `dataset_diff`, comparability `not_comparable` → `unfair_baseline`, baseline `run-notebook-1042` vs candidate `run-managed-2207`, deep link `dataset_catalog_anchor`
- **Throughput: torch 2.3 vs torch 2.1** — axis `env_diff`, comparability `confounded` → `unfair_baseline`, baseline `run-notebook-1042` vs candidate `run-notebook-1038`, deep link `run_object`
- **F1: imported run vs local run** — axis `code_revision_diff`, comparability `insufficient_overlap` → `unproven_baseline`, baseline `run-notebook-1042` vs candidate `run-imported-0031`, deep link `docs_anchor`
- **Checkpoint size: attached vs local** — axis `artifact_diff`, comparability `unknown_comparability` → `unproven_baseline`, baseline `run-notebook-1042` vs candidate `run-manual-attach-0009`, deep link `no_deep_link`

## Compare guard banners

- **Comparison permitted after dataset check** — reason `dataset_mismatch`, guard `comparison_permitted` → `comparable_permitted`, permits-fair `true`, deep link `run_object`
- **Comparison caveated: environment drift** — reason `environment_drift`, guard `comparison_caveated` → `partially_comparable`, permits-fair `false`, deep link `notebook_location`
- **Guard acknowledged: code revision gap** — reason `code_revision_gap`, guard `guard_acknowledged` → `partially_comparable`, permits-fair `false`, deep link `run_object`
- **Guard overridden: metric definition changed** — reason `metric_definition_change`, guard `guard_overridden_by_choice` → `overridden_comparison`, permits-fair `false`, deep link `docs_anchor`
- **Comparison blocked: sample size imbalance** — reason `sample_size_imbalance`, guard `comparison_blocked` → `not_comparable_blocked`, permits-fair `false`, deep link `dataset_catalog_anchor`
- **Guard unavailable: confounder present** — reason `confounder_present`, guard `guard_unavailable` → `guard_unavailable`, permits-fair `false`, deep link `no_deep_link`
