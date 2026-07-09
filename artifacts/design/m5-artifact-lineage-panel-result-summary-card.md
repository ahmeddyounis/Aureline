# Artifact lineage panels and result summary cards

- Packet: `m5-artifact-lineage-panel-result-summary-card-controls:stable:0001`
- Surface: `M5 artifact lineage panels and result summary cards: producing-run identity, stale/diverged notes, include-raw toggles, and export-boundary truth across claimed experiment surfaces`
- Artifact lineage panels: 6 (4 not fully traced)
- Result summary cards: 6 (1 include a raw payload)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Artifact lineage panels

- **Ranker checkpoint v7** — kind `model_checkpoint`, lineage `lineage_complete` → `fully_traced`, producing run `run-notebook-1042`, deep link `run_object`
- **Eval metrics table** — kind `metrics_table`, lineage `regenerated` → `regenerated`, producing run `run-notebook-1042`, deep link `run_object`
- **Loss curve figure** — kind `plot_figure`, lineage `lineage_partial` → `partially_traced`, producing run `run-notebook-1042`, deep link `notebook_location`
- **Weekly results report** — kind `exported_report`, lineage `derived_upstream_known` → `partially_traced`, producing run `run-managed-2207`, deep link `run_object`
- **Training log bundle** — kind `log_bundle`, lineage `lineage_broken` → `untraced`, producing run `run-imported-0031`, deep link `docs_anchor`
- **Unlabeled attachment** — kind `unknown_artifact`, lineage `derived_upstream_unknown` → `untraced`, producing run `run-manual-attach-0009`, deep link `no_deep_link`

## Result summary cards

- **Ranker headline result** — content `headline_metric`, scope `summary_scope` → `metadata_safe`, include-raw `false`, deep link `run_object`
- **Eval metric table summary** — content `metric_table`, scope `metadata_scope` → `metadata_safe`, include-raw `false`, deep link `run_object`
- **Experiment narrative** — content `narrative_summary`, scope `evidence_scope` → `evidence_scoped`, include-raw `false`, deep link `notebook_location`
- **Redacted evidence bundle** — content `evidence_link`, scope `redacted_scope` → `redacted`, include-raw `false`, deep link `docs_anchor`
- **Raw payload export** — content `raw_payload_ref`, scope `raw_scope` → `raw_included`, include-raw `true`, deep link `docs_anchor`
- **No-result summary** — content `no_result`, scope `export_withheld` → `withheld`, include-raw `false`, deep link `no_deep_link`
