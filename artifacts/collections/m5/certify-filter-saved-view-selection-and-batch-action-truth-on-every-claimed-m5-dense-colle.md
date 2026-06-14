# M5 Dense Collection Certification

- Packet: `m5-collection-certification:release:0001`
- Label: `M5 Dense Collection Certification`
- Rows: 9 (9 claimed, 7 certified, 1 narrowed, 1 blocked)
- Surfaces: 9 / 9
- Evidence freshness SLO: 168 hours (last refresh: 2026-06-13T00:00:00Z)

## Rows

- **certification-row:pipeline-run-list:0001** (pipeline_run_list): claim `beta` -> certified `beta` [certified]
  - Pipeline run list certified across filter, saved-view, count, selection, and rerun/export batch truth
  - proofs: filter_ast=current, saved_view=current, result_count=current, selection_scope=current, batch_action=current
- **certification-row:review-queue:0001** (review_queue): claim `beta` -> certified `beta` [certified]
  - Provider-backed review queue certified across all-matching count, selection, and approval batch truth
  - proofs: filter_ast=current, saved_view=current, result_count=current, selection_scope=current, batch_action=current
- **certification-row:incident-list:0001** (incident_list): claim `beta` -> certified `beta` [certified]
  - Incident list certified across filter, count, selection, and destructive gated delete truth
  - proofs: filter_ast=current, saved_view=current, result_count=current, selection_scope=current, batch_action=current
- **certification-row:graph-list:0001** (graph_list): claim `preview` -> certified `preview` [certified]
  - Graph/reference list certified across explicit custom selection and local copy/export truth
  - proofs: filter_ast=current, saved_view=current, result_count=current, selection_scope=current, batch_action=current
- **certification-row:marketplace-results:0001** (marketplace_results): claim `beta` -> certified `beta` [certified]
  - Marketplace results certified across provider-delegated query and mixed install/update truth
  - proofs: filter_ast=current, saved_view=current, result_count=current, selection_scope=current, batch_action=current
- **certification-row:activity-rows:0001** (activity_rows): claim `stable` -> certified `stable` [certified]
  - Activity rows certified across scoped free-text filter, streaming counts, and local export/copy truth
  - proofs: filter_ast=current, saved_view=current, result_count=current, selection_scope=current, batch_action=current
- **certification-row:query-backed-result-set:0001** (query_backed_result_set): claim `beta` -> certified `beta` [certified]
  - Query-backed result set certified across saved-query snapshot, all-matching scope, and export/share truth
  - proofs: filter_ast=current, saved_view=current, result_count=current, selection_scope=current, batch_action=current
- **certification-row:provider-admin-table:0001** (provider_admin_table): claim `beta` -> certified `held` [blocked]
  - Provider/admin table whose candidate build erased provider/policy narrowing disclosure
  - proofs: filter_ast=current, saved_view=current, result_count=current, selection_scope=current, batch_action=current
  - Regression: provider_policy_narrowing_erased
  - Narrowed: Candidate build hid provider/policy narrowing inside a generic filter chip; promotion blocked and the claim is held below beta
- **certification-row:support-export:0001** (support_export_projection): claim `beta` -> certified `held` [auto_narrowed]
  - Support/export projection whose batch-action review proof is not yet published
  - proofs: filter_ast=current, saved_view=current, result_count=current, selection_scope=current, batch_action=missing
  - Narrowed: Batch-action review proof missing for this projected row; auto-narrowed to held until a batch review sheet is published
