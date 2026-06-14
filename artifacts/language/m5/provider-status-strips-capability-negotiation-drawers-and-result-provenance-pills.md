# Provider-status surface objects — human-readable rendering

Human-readable rendering of the stable provider-status surface truth packet
(`artifacts/language/m5/provider_status_surface_truth_packet.json`). The
packet binds the provider-status strip, capability-negotiation drawer, and
result-provenance pill across the M5 framework, notebook, generated-source,
preview, docs-linked, and structured-artifact surfaces, reading the closed
provider vocabulary frozen by the provider/refactor matrix packet.

## Surfaces, providers, and where they run

| Surface | Provider family | Locality | Lifecycle | Capability | Detail route |
| --- | --- | --- | --- | --- | --- |
| framework_surface | framework_analyzer | workspace_local_process | ready_live | full_semantic_negotiated | open_negotiation_drawer |
| notebook_surface | notebook_adapter | notebook_kernel_session | degraded_partial | partial_semantic_negotiated | open_capability_inspector |
| generated_source_surface | generated_source_bridge | in_process_engine | ready_live | text_fallback_negotiated | open_scope_limit_detail |
| preview_surface | lsp_provider | local_host_subprocess | ready_live | full_semantic_negotiated | open_negotiation_drawer |
| docs_linked_surface | ai_overlay | remote_managed_service | restarting | partial_semantic_negotiated | open_provider_health_panel |
| structured_artifact_surface | lsp_provider | local_host_subprocess | ready_live | full_semantic_negotiated | open_capability_inspector |

## Drawers — disagreement, result, scope, freshness, recovery

| Surface | Conflict | Losing provider preserved | Selected result | Scope | Freshness | Recovery |
| --- | --- | --- | --- | --- | --- | --- |
| framework_surface | arbitrated_winner_loser_preserved | yes | arbitrated_winner_result | full_workspace_scope | fresh_live | retry_request |
| notebook_surface | single_provider_no_conflict | n/a | single_provider_result | open_cells_scope | cached_recent | restart_provider |
| generated_source_surface | policy_override_recorded | n/a | policy_override_result | sparse_index_scope | imported_snapshot | regenerate_from_source |
| preview_surface | single_provider_no_conflict | n/a | single_provider_result | full_workspace_scope | fresh_live | rerun_preview |
| docs_linked_surface | single_provider_no_conflict | n/a | fused_result | single_file_scope | stale_pending_refresh | refresh_result |
| structured_artifact_surface | unresolved_disagreement_surfaced | yes | unresolved_disagreement_result | workset_subset_scope | fresh_live | retry_request |

## Provenance pills — anchor, provenance, downgrade

| Surface | Anchor target | Result provenance | Preview completeness | Downgrade label |
| --- | --- | --- | --- | --- |
| framework_surface | framework_aware_result | live_semantic | n/a | full_to_partial_completeness |
| notebook_surface | completion_result | cached_semantic | n/a | semantic_to_text_fallback |
| generated_source_surface | definition_result | imported_scan | n/a | generated_edit_to_regenerate_first |
| preview_surface | rename_preview | live_semantic | complete | previewable_to_compare_only |
| docs_linked_surface | hover_doc_result | stale_pending_refresh | n/a | provider_unavailable_text_only |
| structured_artifact_surface | reference_result | live_semantic | n/a | full_to_partial_completeness |

The `preview_surface` pill anchors a `rename_preview`, so it binds a typed,
`complete` preview completeness — the rename-preview pill never bypasses the
launch-language refactor safety model.

## Consumer projections

The framework-pack panel, notebook surface, request runner, preview
surface, docs surface, generated-artifact surface, support export, release
proof index, Help/About proof card, and conformance dashboard each preserve
the closed vocabulary verbatim and export JSON without raw private material
or ambient authority.
