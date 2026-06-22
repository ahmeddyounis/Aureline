# M5 Staged-Review Sheets Across Mutation Flows

- Packet: `m5-staged-review-sheets:stable:0001`
- Label: `M5 staged-review sheets — target scope, omitted defaults, side effects, and included/excluded/blocked/hidden counts across mutation flows`
- As of: `2026-06-21T00:00:00Z`
- Sheets: 7
- Effective: 4 certified, 1 narrowed, 1 review overlay, 0 unsafe, 1 labs

| Sheet | Flow | Lane | Scope | Origin | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- |
| sheet:provider-publish-later:0001 | provider_publish_later | provider | single_object | provider_commit | sheet_certified | sheet_certified |
| sheet:settings-bulk-apply:0001 | settings_bulk_apply | settings | multi_object_explicit | local_commit | sheet_certified | sheet_certified |
| sheet:package-lifecycle:0001 | package_lifecycle | package | multi_object_explicit | local_commit | sheet_certified | sheet_certified |
| sheet:admin-source-management:0001 | admin_source_management | admin | workspace_wide | local_commit | sheet_certified | sheet_certified |
| sheet:request-replay:0001 | request_replay_mutation | request | query_backed | remote_commit | sheet_certified | sheet_narrowed |
| sheet:import-export-publish:0001 | import_export_publish | import | query_backed | imported_review | sheet_review_overlay | sheet_review_overlay |
| sheet:experimental-quick-apply:0001 | settings_bulk_apply | settings | single_object | local_commit | sheet_labs_not_claimed | sheet_labs_not_claimed |

- sheet:request-replay:0001: Held at sheet_narrowed below the sheet_certified claim: the verification proof is stale; the scope stays reopenable until re-verified.
