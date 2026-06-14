# M5 Scope Receipts And Saved-Query Deep-Link Snapshots

- Packet: `m5-scope-receipt:stable:0001`
- Label: `M5 Scope Receipts And Saved-Query Deep-Link Snapshots`
- Receipts: 8 across 7 / 5 surfaces
- Scope classes: 5 / 5
- Deep-link snapshots: 4 across 4 / 4 postures

## Scope receipts

- **receipt:pipeline-rerun:0001** (pipeline_run_list / list / rerun): Re-ran the 8 selected runs, not all 25 matching the failed filter.
  - scope=`selected_items` origin=`local_client` acted_on=8 omitted=0
  - selected=8 visible=10 loaded=10 matching=25 approx=false
- **receipt:review-update:0001** (review_queue / queue / update): Updated the 30 loaded reviewed items, not all 120 matching the queue.
  - scope=`loaded_rows` origin=`mixed_client_provider` acted_on=30 omitted=0
  - selected=0 visible=12 loaded=30 matching=120 approx=false
- **receipt:incident-suppress:0001** (incident_list / list / suppress): Suppressed the 15 visible incidents, not all 40 loaded or matching.
  - scope=`visible_rows` origin=`local_client` acted_on=15 omitted=0
  - selected=0 visible=15 loaded=40 matching=40 approx=false
- **receipt:marketplace-install:0001** (marketplace_results / table / install): Installed the provider-side selection of 6 extensions resolved by the marketplace.
  - scope=`provider_side_selection` origin=`mixed_client_provider` acted_on=6 omitted=0
  - selected=0 visible=20 loaded=20 matching=? approx=true
- **receipt:admin-delete:0001** (provider_admin_table / table / delete): Deleted 138 of all 140 matching records; 2 provider-locked records were omitted.
  - scope=`all_matching_query` origin=`provider_authoritative` acted_on=138 omitted=2
  - selected=0 visible=25 loaded=50 matching=140 approx=false
- **receipt:query-export:0001** (query_backed_result_set / table / export): Exported all ~1,240 rows matching the query snapshot, not just the 50 on screen.
  - scope=`all_matching_query` origin=`local_client` acted_on=1240 omitted=0
  - selected=0 visible=50 loaded=50 matching=? approx=true
- **receipt:activity-copy:0001** (activity_rows / list / copy): Copied the 8 visible activity rows, not all 60 loaded or matching.
  - scope=`visible_rows` origin=`local_client` acted_on=8 omitted=0
  - selected=0 visible=8 loaded=60 matching=60 approx=false
- **receipt:admin-update:0001** (provider_admin_table / table / update): Updated the 5 selected records, not all 140 matching the admin filter.
  - scope=`selected_items` origin=`provider_authoritative` acted_on=5 omitted=0
  - selected=5 visible=25 loaded=50 matching=140 approx=false

## Deep-link snapshots

- **snapshot:query-export:0001** (query_backed_result_set / captured `all_matching_query`): Shared export scope: captured ~1,240 rows; 1,237 still match the live query.
  - posture=`current_diverged_from_captured` reopened=true frozen_certainty=false
  - omitted `no_longer_matches_query` x3: 3 captured rows no longer match the live query and were dropped on reopen.
- **snapshot:marketplace-install:0001** (marketplace_results / captured `provider_side_selection`): Shared install scope: provider-side set may differ; 4 of 6 captured remain.
  - posture=`provider_results_may_differ` reopened=true frozen_certainty=false
  - omitted `provider_removed` x2: 2 captured extensions were removed by the provider since the link was shared.
- **snapshot:pipeline-rerun:0001** (pipeline_run_list / captured `selected_items`): Shared rerun scope: all 8 selected runs still match; verified on reopen.
  - posture=`captured_matches_current` reopened=true frozen_certainty=false
- **snapshot:incident-stale:0001** (incident_list / captured `all_matching_query`): Saved incident scope is stale; it will be re-resolved against the live query.
  - posture=`captured_snapshot_stale` reopened=false frozen_certainty=false
