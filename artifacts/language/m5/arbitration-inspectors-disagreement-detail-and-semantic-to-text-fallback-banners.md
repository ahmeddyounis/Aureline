# Semantic-result arbitration — human-readable rendering

Human-readable rendering of the stable semantic-result arbitration truth
packet (`artifacts/language/m5/semantic_result_arbitration_truth_packet.json`).
The packet binds the arbitration inspector, disagreement detail, and
semantic-to-text fallback banner for the definition, references, hierarchy,
and completion result lanes across the M5 search, docs, framework, notebook,
and generated-source surfaces, reading the closed provider vocabulary frozen
by the provider/refactor matrix packet and anchoring the objects certified by
the provider-status surface packet.

Promotion state: **stable** · rows: **20** · consumer projections: **10**.

## Arbitration inspector — winner, basis, and alternate visibility

| Surface | Lane | Acting provider | Basis | Alternates |
| --- | --- | --- | --- | --- |
| search | definition | semantic_graph_lane | single_provider_authoritative | not_applicable_single_provider |
| search | references | text_fallback | narrowed_no_semantic_winner | not_applicable_single_provider |
| search | hierarchy | semantic_graph_lane | only_admissible_provider | alternates_preserved_inspectable |
| search | completion | semantic_graph_lane | freshness_recency | not_applicable_single_provider |
| docs | definition | lsp_provider | single_provider_authoritative | not_applicable_single_provider |
| docs | references | lsp_provider | framework_overlay_precedence | alternates_preserved_inspectable |
| docs | hierarchy | lsp_provider | highest_semantic_authority | not_applicable_single_provider |
| docs | completion | text_fallback | narrowed_no_semantic_winner | not_applicable_single_provider |
| framework | definition | framework_analyzer | framework_overlay_precedence | alternates_preserved_inspectable |
| framework | references | framework_analyzer | highest_semantic_authority | not_applicable_single_provider |
| framework | hierarchy | framework_analyzer | freshness_recency | not_applicable_single_provider |
| framework | completion | framework_analyzer | highest_semantic_authority | not_applicable_single_provider |
| notebook | definition | notebook_adapter | freshness_recency | not_applicable_single_provider |
| notebook | references | semantic_graph_lane | only_admissible_provider | alternates_preserved_inspectable |
| notebook | hierarchy | notebook_adapter | single_provider_authoritative | not_applicable_single_provider |
| notebook | completion | notebook_adapter | single_provider_authoritative | not_applicable_single_provider |
| generated_source | definition | generated_source_bridge | single_provider_authoritative | not_applicable_single_provider |
| generated_source | references | generated_source_bridge | highest_semantic_authority | not_applicable_single_provider |
| generated_source | hierarchy | text_fallback | narrowed_no_semantic_winner | not_applicable_single_provider |
| generated_source | completion | generated_source_bridge | framework_overlay_precedence | alternates_preserved_inspectable |

## Fallback banner — tier, retained vs lost guarantee, and claim scope

| Surface | Lane | Tier | Banner | Retained | Lost | Claim scope |
| --- | --- | --- | --- | --- | --- | --- |
| search | references | text_lexical | semantic_to_text_fallback | lexical_match_only | lost_all_references_guarantee | active_file_results |
| search | hierarchy | heuristic_structural | semantic_to_heuristic_fallback | structural_match_only | lost_semantic_target_identity | generated_excluded_results |
| search | completion | cached_semantic | cached_semantic_reuse | file_local_semantic | lost_cross_file_semantic | loaded_slice_results |
| framework | completion | partial_semantic | semantic_to_file_local_fallback | file_local_semantic | lost_whole_workspace_scope | loaded_slice_results |
| generated_source | references | partial_semantic | semantic_to_file_local_fallback | file_local_semantic | lost_whole_workspace_scope | loaded_slice_results |
| generated_source | hierarchy | text_lexical | semantic_to_text_fallback | lexical_match_only | lost_all_references_guarantee | active_file_results |

Exact-semantic rows (the remainder) carry no banner, retain a full semantic
guarantee, lose nothing, and claim only `single_target` or — where the
provider proved complete, live coverage — `whole_workspace_all_results`.

## Disagreement detail — what the conflict changes and how it shows

The arbitrated and unresolved rows (search/hierarchy, docs/references,
framework/definition, notebook/references, generated_source/completion)
preserve their losing provider, bind a material disagreement impact
(`target_identity_changed` or `scope_coverage_changed`), and open an
`open_disagreement_detail` route behind an inline conflict panel or
side-panel inspector. No row collapses a disagreement into ranking-only
output, and no material conflict is fused silently into an exact answer.

## Refactor safety

The mutating follow-up rows (framework/completion,
generated_source/references) anchor `mutating_followup_preview` and bind a
typed `partial` preview completeness plus a rollback checkpoint, preserving
the launch-language refactor safety model on M5-only artifacts and framework
packs.

This artifact is regenerated by
`cargo run -p aureline-language --example dump_semantic_result_arbitration_truth_packet`.
