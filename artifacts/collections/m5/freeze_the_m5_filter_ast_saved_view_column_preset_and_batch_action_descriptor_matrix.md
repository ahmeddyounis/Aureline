# M5 Dense Collection Qualification Matrix

- Packet: `m5-collection-qualification-matrix:stable:0001`
- Label: `M5 Dense Collection Qualification Matrix`
- Rows: 9 (9 claimed, 1 downgraded)
- Surfaces: 9 / 9
- Evidence freshness SLO: 168 hours (last refresh: 2026-06-13T00:00:00Z)

## Rows

- **collection-row:pipeline-run-list:0001** (pipeline_run_list): claim `beta` -> effective `beta`
  - Pipeline run list with a typed clause filter, loaded-set selection, and local rerun/export
  - filter=`typed_clause_ast` scope=`loaded_set` counter=`exact_count` batch=`local_reversible_batch`
  - actions: rerun, export
- **collection-row:review-queue:0001** (review_queue): claim `beta` -> effective `beta`
  - Provider-backed review queue with all-matching query scope and provider-authoritative approval
  - filter=`typed_clause_ast` scope=`all_matching_query` counter=`provider_limited_count` batch=`provider_authoritative_batch`
  - actions: approve, suppress
- **collection-row:incident-list:0001** (incident_list): claim `beta` -> effective `beta`
  - Incident list with a typed clause filter, visible-range selection, and destructive gated delete
  - filter=`typed_clause_ast` scope=`visible_range` counter=`approximate_count` batch=`destructive_gated_batch`
  - actions: suppress, delete
- **collection-row:graph-list:0001** (graph_list): claim `preview` -> effective `preview`
  - Graph/reference list with an explicit custom selection and local copy/export
  - filter=`typed_clause_ast` scope=`explicit_custom_set` counter=`exact_count` batch=`local_reversible_batch`
  - actions: copy, export
- **collection-row:marketplace-results:0001** (marketplace_results): claim `beta` -> effective `beta`
  - Marketplace results with a provider-delegated query and mixed client/provider install/update
  - filter=`provider_delegated_query` scope=`loaded_set` counter=`provider_limited_count` batch=`mixed_client_provider_batch`
  - actions: install, update
- **collection-row:activity-rows:0001** (activity_rows): claim `stable` -> effective `stable`
  - Activity rows with a scoped free-text filter, streaming counts, and local export/copy
  - filter=`free_text_scoped` scope=`loaded_set` counter=`partial_streaming_count` batch=`local_reversible_batch`
  - actions: export, copy
- **collection-row:provider-admin-table:0001** (provider_admin_table): claim `beta` -> effective `beta`
  - Provider/admin table with a provider-delegated query and provider-authoritative update/delete
  - filter=`provider_delegated_query` scope=`all_matching_query` counter=`provider_limited_count` batch=`provider_authoritative_batch`
  - actions: update, delete
- **collection-row:query-backed-result-set:0001** (query_backed_result_set): claim `beta` -> effective `beta`
  - Query-backed result set with a saved query snapshot, all-matching scope, and export/share
  - filter=`saved_query_snapshot` scope=`all_matching_query` counter=`approximate_count` batch=`mixed_client_provider_batch`
  - actions: export, share
- **collection-row:support-export:0001** (support_export_projection): claim `beta` -> effective `held`
  - Support/export projection of a collection row whose batch-action scope class is not yet identified
  - filter=`saved_query_snapshot` scope=`explicit_custom_set` counter=`exact_count` batch=`unidentified`
  - actions: none
  - Degraded: Batch-action scope class not yet identified for this projected row; held below preview until a batch-action descriptor is published
