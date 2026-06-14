# M5 Result-Scope Counters And Hidden-Narrowing Chips

- Packet: `m5-result-scope-counter:stable:0001`
- Label: `M5 Result-Scope Counters And Hidden-Narrowing Chips`
- Bindings: 6 (2 narrowed)
- Surfaces: 6 / 6
- View kinds: 4 / 4

## Bindings

- **counter:pipeline-run-list:0001** (pipeline_run_list / list): Pipeline run list with a complete, exact, current dataset
  - posture=`client_complete` visible=40 loaded=184 matching=184 total=184
- **counter:review-queue:0001** (review_queue / queue): Review queue with policy and provider narrowing disclosed near the filters
  - posture=`client_complete` visible=30 loaded=152 matching=152 total=200
  - hidden by policy: 30 (30 items hidden by your review-access policy)
  - hidden by workset: 18 (18 items outside the active workset)
- **counter:incident-list:0001** (incident_list / list): Incident list with stale matching counts and partial loaded rows pending refresh
  - posture=`partial_data_pending` visible=25 loaded=96 matching=140 total=140
- **counter:graph-list:0001** (graph_list / tree): Reference graph tree with a complete, exact dataset
  - posture=`client_complete` visible=22 loaded=310 matching=310 total=310
- **counter:marketplace-results:0001** (marketplace_results / table): Marketplace results table paged behind the provider with approximate totals
  - posture=`provider_paginated` visible=24 loaded=50 matching=1200~ total=5000~
- **counter:provider-admin-table:0001** (provider_admin_table / table): Provider/admin table rendering a narrow client window with client-limited rows hidden
  - posture=`narrow_client_windowed` visible=20 loaded=80 matching=240 total=250
  - hidden by client: 10 (10 rows not loaded on this narrow client window)
