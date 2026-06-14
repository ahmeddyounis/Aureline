# M5 Selection Bars And Stale-Query-Snapshot Guards

- Packet: `m5-selection-bar-continuity:stable:0001`
- Label: `M5 Selection Bars And Stale-Query-Snapshot Guards`
- Bars: 6 (2 stale)
- Surfaces: 6 / 6
- View kinds: 4 / 4
- Data modes: 5 / 5

## Bars

- **bar:pipeline-run-list:0001** (pipeline_run_list / list / static_complete): Pipeline run list selection over a complete, fresh dataset
  - selected=3 visible=3 outside_filter=0 prior_snapshot=0 blocked=0
  - guard: change=`unchanged` outcome=`proceed_fresh`
- **bar:review-queue:0001** (review_queue / queue / filtered_sorted): Review queue selection with members outside the active filter
  - selected=12 visible=9 outside_filter=3 prior_snapshot=0 blocked=1
  - guard: change=`reordered_only` outcome=`proceed_fresh`
- **bar:incident-list:0001** (incident_list / list / streaming): Incident list selection that survives live streaming reorders
  - selected=2 visible=2 outside_filter=0 prior_snapshot=0 blocked=0
  - guard: change=`reordered_only` outcome=`proceed_fresh`
- **bar:graph-list:0001** (graph_list / tree / virtualized): Reference graph tree shift-range selection anchored by stable identity
  - selected=3 visible=3 outside_filter=0 prior_snapshot=0 blocked=0
  - range anchor `graph:node:a` -> `graph:node:c` (present=true)
  - guard: change=`unchanged` outcome=`proceed_fresh`
- **bar:marketplace-results:0001** (marketplace_results / table / paginated): Marketplace results selection from a prior snapshot that went stale
  - selected=240 visible=50 outside_filter=190 prior_snapshot=240 blocked=6
  - guard: change=`rows_added_or_removed` outcome=`require_reopen_review`
- **bar:provider-admin-table:0001** (provider_admin_table / table / virtualized): Provider/admin table selection downgraded after a provider epoch change
  - selected=18 visible=12 outside_filter=6 prior_snapshot=0 blocked=0
  - range anchor `admin:row:gone` -> `admin:row:1` (present=false)
  - guard: change=`provider_epoch_changed` outcome=`downgrade_to_visible_only`
