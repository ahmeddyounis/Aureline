# Work-Item Detail Headers, Transition Sheets, Comment Publish Review, and Offline Handoff Packets

- Packet: `work-item-mutation-review:stable:0001`
- Surface: `M5 work-item mutation review side-effect previews`
- Detail headers: 7
- Transition sheets: 5 (1 blocked)
- Comment publish reviews: 5 (1 deferred or local)
- Offline handoff packets: 3
- Conflict or reconcile rows: 1
- Proof freshness SLO: 168 hours (last refresh: 2026-06-12T22:35:00Z)

## Detail headers

- **AUR-241** (`issue_tracker boundary · Issue tracker`): Lifecycle state token remains provider/local/import exact. · Linked review workspace state is summarized by reference., owner `Reviewable assignee label`, sync `provider_authoritative · live_authoritative_fresh · published_observed_authoritative`, open `Open provider object through reviewed handoff`
- **AUR-242** (`issue_tracker boundary · Issue tracker`): Lifecycle state token remains provider/local/import exact. · Linked review workspace state is summarized by reference., owner `Reviewable assignee label`, sync `cached_stale · degraded_beyond_grace_local_continues · draft_only`, open `Open provider object through reviewed handoff`
- **AUR-243** (`issue_tracker boundary · Issue tracker`): Lifecycle state token remains provider/local/import exact. · Linked review workspace state is summarized by reference., owner `Reviewable assignee label`, sync `read_only · warm_within_grace · inspect_only`, open `Open provider object through reviewed handoff`
- **AUR-244** (`issue_tracker boundary · Issue tracker`): Lifecycle state token remains provider/local/import exact. · Linked review workspace state is summarized by reference., owner `Reviewable assignee label`, sync `policy_blocked · warm_within_grace · draft_only`, open `Open provider object through reviewed handoff`
- **TASK-17** (`issue_tracker boundary · Issue tracker`): Lifecycle state token remains provider/local/import exact. · Linked review workspace state is summarized by reference., owner `Reviewable assignee label`, sync `local_draft · local_draft_never_published · draft_only`, open `Open provider object through reviewed handoff`
- **AUR-245** (`issue_tracker boundary · Issue tracker`): Lifecycle state token remains provider/local/import exact. · Linked review workspace state is summarized by reference., owner `Reviewable assignee label`, sync `queued · local_draft_never_published · queued_for_publish_later`, open `Open provider object through reviewed handoff`
- **INC-246** (`issue_tracker boundary · Issue tracker`): Lifecycle state token remains provider/local/import exact. · Linked review workspace state is summarized by reference., owner `Reviewable assignee label`, sync `offline_captured · imported_snapshot_no_refresh_path · offline_captured_not_submitted`, open `Open provider object through reviewed handoff`

## Transition sheets

- **work_items:detail:provider-authoritative**: `Current lifecycle token: provider_triaged.` -> `Requested lifecycle token: provider_in_review.` via `publish_now`; fallback `preserve the local draft on failure`
- **work_items:detail:provider-authoritative**: `Current lifecycle token: provider_triaged.` -> `Requested lifecycle token: provider_in_review.` via `open_in_provider`; fallback `preserve the local draft on failure`
- **work_items:detail:local-draft**: `Current lifecycle token: provider_triaged.` -> `Requested lifecycle token: provider_in_review.` via `local_draft`; fallback `preserve the local draft on failure`
- **work_items:detail:queued**: `Current lifecycle token: provider_triaged.` -> `Requested lifecycle token: provider_in_review.` via `deferred_publish`; fallback `queue for publish later and preserve the local draft`
- **work_items:detail:policy-blocked**: `Current lifecycle token: provider_triaged.` -> `Requested lifecycle token: provider_in_review.` via `deferred_publish`; fallback `preserve the local draft on failure`

## Comment publish review

- **Create-comment publish-review row exposes provider mutation and notification fanout.**: `Provider-owned comment is observed authoritative and in sync.` / `publish now to the provider`; visibility `provider work-item timeline`; fallback `preserve the local draft on failure`
- **Open-in-provider comment publish-review row preserves the local draft and typed browser handoff continuity.**: `Queued comment awaits publish-later drain; provider acceptance is unverified.` / `route through browser handoff`; visibility `provider timeline via browser handoff`; fallback `preserve the local draft on failure`
- **Edit-comment publish-review row preserves local draft on failure.**: `Provider-owned comment is observed authoritative and in sync.` / `publish now to the provider`; visibility `provider work-item timeline`; fallback `preserve the local draft on failure`
- **Delete-comment publish-review row leaves local draft preserved on failure.**: `Provider-owned comment is observed authoritative and in sync.` / `publish now to the provider`; visibility `provider work-item timeline`; fallback `preserve the local draft on failure`
- **Status-transition-plus-comment publish-review row binds transition packet and comment record.**: `Provider-owned comment is observed authoritative and in sync.` / `publish now to the provider`; visibility `provider work-item timeline`; fallback `preserve the local draft on failure`

## Offline handoff packets

- **work_items:offline_handoff:provider-unreachable**: acceptance `not_submitted_local_capture_only`, drain `captured_pending_drain`, expiry `2026-05-19T09:10:00Z`, target `work_items:detail:cached-stale`
- **work_items:offline_handoff:browser-blocked**: acceptance `not_submitted_local_capture_only`, drain `captured_pending_export_user_initiated`, expiry `2026-05-19T09:10:00Z`, target `work_items:detail:policy-blocked`
- **work_items:offline_handoff:imported-evidence**: acceptance `imported_handoff_evidence_only_no_provider_path`, drain `exported_pending_external_apply`, expiry `2026-05-19T09:10:00Z`, target `work_items:detail:offline-captured`

## Conflict Or Reconcile

- **provider_reconcile.result.issue.84.drift**: local `local draft still preserved` vs provider `provider state drifted and remains authoritative until review`; next `compare_rebase_review`; compare `Compare local draft against provider state`
