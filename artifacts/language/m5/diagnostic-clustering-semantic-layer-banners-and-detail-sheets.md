# Diagnostic clustering and semantic-layer banners — human-readable rendering

Human-readable rendering of the stable diagnostic-cluster semantic-layer truth
packet (`artifacts/language/m5/diagnostic_cluster_semantic_layer_truth_packet.json`).
The packet binds diagnostic clustering, semantic-layer banners, freshness/scope
labels, and the cluster detail-sheet model for the compiler, linter,
language-server, framework, runtime, notebook, and policy cluster lanes across
the M5 notebook, framework, preview, and generated-code surfaces, reading the
closed provider, conflict, and diagnostic-source vocabulary frozen by the
provider/refactor matrix packet.

Promotion state: **stable** · rows: **9** · consumer projections: **10**.

## Cluster identity — converged sources, provenance, and detail route

| Surface | Lane | Sources | Provenance | Differentiation | Detail route |
| --- | --- | --- | --- | --- | --- |
| notebook | notebook | notebook_kernel | single_provider_cluster | single_source_not_applicable | open_cluster_detail_sheet |
| framework | framework | framework_schema, lsp | per_provider_preserved | differentiated_by_source | open_cluster_detail_sheet |
| preview | compiler | compiler_build | single_provider_cluster | single_source_not_applicable | open_cluster_detail_sheet |
| generated_code | language_server | lsp, generated_artifact_validation | per_provider_preserved | differentiated_by_source | open_provider_breakdown |
| notebook | linter | linter_formatter | single_provider_cluster | single_source_not_applicable | open_cluster_detail_sheet |
| framework | runtime | runtime_test_debug, policy_trust, lsp | per_provider_preserved | differentiated_by_source | open_provider_breakdown |
| preview | policy | policy_trust | single_provider_cluster | single_source_not_applicable | open_cluster_detail_sheet |
| generated_code | linter | linter_formatter | single_provider_cluster | single_source_not_applicable | open_cluster_detail_sheet |
| generated_code | framework | framework_schema, generated_artifact_validation | per_provider_preserved | differentiated_by_source | open_provider_breakdown |

Every multi-provider cluster preserves per-provider detail, timestamps/epochs,
suppression/baseline state, and related evidence, and the framework/runtime
cluster keeps runtime, policy, and static findings differentiated by source
rather than fusing them into one undifferentiated row.

## Semantic-layer banner — posture, freshness, and claimable scope

| Surface | Lane | Banner | Freshness | Scope |
| --- | --- | --- | --- | --- |
| notebook | notebook | runtime_only | warm | open_cells |
| framework | framework | semantic | live | loaded_slice |
| preview | compiler | semantic | live | whole_workspace |
| generated_code | language_server | graph_warm | warm | generated_excluded |
| notebook | linter | syntax_only | warm | active_file |
| framework | runtime | runtime_only | warm | loaded_slice |
| preview | policy | cached | cached | active_file |
| generated_code | linter | syntax_only | stale | single_artifact |
| generated_code | framework | partial | live | loaded_slice |

Only the live semantic compiler cluster (preview) claims a whole-workspace
scope; the cached, stale, and runtime-only clusters narrow their scope and never
claim the full semantic banner.

## Provider arbitration and fix offer

| Surface | Lane | Acting provider | Conflict | Loser visibility | Fix offer |
| --- | --- | --- | --- | --- | --- |
| notebook | notebook | notebook_adapter | single_provider_no_conflict | not_applicable_single_provider | no_fix_offered |
| framework | framework | framework_analyzer | arbitrated_winner_loser_preserved | losers_preserved_inspectable | non_mutating_fix |
| preview | compiler | lsp_provider | single_provider_no_conflict | not_applicable_single_provider | no_fix_offered |
| generated_code | language_server | generated_source_bridge | single_provider_no_conflict | not_applicable_single_provider | notebook_generated_fix |
| notebook | linter | lsp_provider | single_provider_no_conflict | not_applicable_single_provider | mutating_quick_fix |
| framework | runtime | framework_analyzer | unresolved_disagreement_surfaced | losers_preserved_inspectable | no_fix_offered |
| preview | policy | ai_overlay | policy_override_recorded | not_applicable_single_provider | ai_planned_fix |
| generated_code | linter | generated_source_bridge | single_provider_no_conflict | not_applicable_single_provider | organize_imports_fix |
| generated_code | framework | generated_source_bridge | arbitrated_winner_loser_preserved | losers_preserved_inspectable | schema_codegen_fix |

Every disagreement keeps the losing provider inspectable behind a real
detail-sheet route. Each offered fix names its acting provider and
freshness/scope posture; the mutating fixes — `notebook_generated_fix`,
`mutating_quick_fix`, `ai_planned_fix`, `organize_imports_fix`, and
`schema_codegen_fix` — bind a typed preview completeness and a rollback
checkpoint, preserving the launch-language refactor safety model on M5-only
artifacts and framework packs.

This artifact is regenerated by
`cargo run -p aureline-language --example dump_diagnostic_cluster_semantic_layer_truth_packet`.
