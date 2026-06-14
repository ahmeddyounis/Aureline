# M5 Keyboard Assistive Selection Parity And Roving Focus

- Packet: `m5-assistive-selection-parity:stable:0001`
- Label: `M5 Keyboard Assistive Selection Parity And Roving Focus`
- Profiles: 6 (4 with offscreen selection)
- Surfaces: 6 / 6
- View kinds: 4 / 4
- Data modes: 3 / 5
- Focus models: 2 / 2
- Churn events: 4 / 4

## Profiles

- **profile:pipeline-run-list:0001** (pipeline_run_list / list / streaming): Pipeline run list keyboard selection with roving tabindex over streaming rows
  - focus: `roving_tabindex` (by_identity=true)
  - commands: select_current, extend_range, clear_selection, inspect_hidden_count, open_batch_review
  - churn: streaming_insert, background_refresh
  - offscreen_selected=0 hidden_count_exposed=true
- **profile:review-queue:0001** (review_queue / queue / filtered_sorted): Review queue keyboard selection with aria-activedescendant and hidden-selected exposure
  - focus: `aria_activedescendant` (by_identity=true)
  - commands: select_current, extend_range, clear_selection, inspect_hidden_count, open_batch_review
  - churn: sort_or_filter_change, background_refresh
  - offscreen_selected=3 hidden_count_exposed=true
- **profile:incident-list:0001** (incident_list / list / streaming): Incident list keyboard selection that holds focus while new incidents stream in
  - focus: `roving_tabindex` (by_identity=true)
  - commands: select_current, extend_range, clear_selection, inspect_hidden_count, open_batch_review
  - churn: streaming_insert, sort_or_filter_change
  - offscreen_selected=0 hidden_count_exposed=true
- **profile:graph-list:0001** (graph_list / tree / virtualized): Reference graph tree keyboard selection with focus re-anchor on virtualization recycle
  - focus: `aria_activedescendant` (by_identity=true)
  - commands: select_current, extend_range, clear_selection, inspect_hidden_count, open_batch_review
  - churn: virtualization_recycle, sort_or_filter_change
  - offscreen_selected=5 hidden_count_exposed=true
- **profile:marketplace-results:0001** (marketplace_results / table / virtualized): Marketplace results keyboard selection with a large offscreen-selected population
  - focus: `roving_tabindex` (by_identity=true)
  - commands: select_current, extend_range, clear_selection, inspect_hidden_count, open_batch_review
  - churn: virtualization_recycle, streaming_insert
  - offscreen_selected=188 hidden_count_exposed=true
- **profile:provider-admin-table:0001** (provider_admin_table / table / filtered_sorted): Provider/admin table keyboard selection with focus re-anchor after a filter drop
  - focus: `aria_activedescendant` (by_identity=true)
  - commands: select_current, extend_range, clear_selection, inspect_hidden_count, open_batch_review
  - churn: sort_or_filter_change, background_refresh
  - offscreen_selected=6 hidden_count_exposed=true
