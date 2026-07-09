# Notification rows and mobile review cards

- Packet: `m5-notification-row-mobile-review-card-controls:stable:0001`
- Surface: `M5 notification rows and mobile review cards: event/object identity, repo/workspace client scope, freshness, severity/importance, unread state, keyboard-complete quick triage verbs, companion-versus-desktop capability boundary, and an exact desktop-handoff target`
- Notification rows: 6 (5 not live)
- Mobile review cards: 6 (2 not companion-sufficient)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Notification rows

- **Release build failed** (ci_run) — scope `repo_scoped`, severity `critical`, freshness `live` → `live`, handoff `ci_pipeline_run`
- **Incident opened** (incident_record) — scope `workspace_scoped`, severity `high`, freshness `stale` → `stale`, handoff `incident_workspace`
- **Review requested** (review_item) — scope `repo_scoped`, severity `moderate`, freshness `cached` → `cached`, handoff `review_panel`
- **Agent run finished** (followed_session) — scope `device_scoped`, severity `low`, freshness `offline_held` → `stale`, handoff `agent_session`
- **Sync completed** (notification_event) — scope `org_scoped`, severity `informational`, freshness `unknown_freshness` → `unknown`, handoff `no_handoff`
- **You were mentioned** (notification_event) — scope `account_global`, severity `unspecified`, freshness `expired_snapshot` → `stale`, handoff `file_location`

## Mobile review cards

- **Agent change ready** (review_item) — scope `repo_scoped`, kind `agent_change`, disposition `comment_capable` → `comment_capable`, handoff `review_panel`
- **Diff to review** (review_item) — scope `repo_scoped`, kind `diff_review`, disposition `review_only` → `review_only`, handoff `review_panel`
- **Comment thread awaiting response** (review_item) — scope `workspace_scoped`, kind `comment_thread`, disposition `comment_capable` → `comment_capable`, handoff `review_panel`
- **Approval requested** (review_item) — scope `repo_scoped`, kind `approval_request`, disposition `desktop_required` → `desktop_required`, handoff `review_panel`
- **Policy gate awaiting acknowledgement** (review_item) — scope `org_scoped`, kind `policy_gate`, disposition `policy_blocked` → `policy_blocked`, handoff `review_panel`
- **Merge readiness summary** (review_item) — scope `repo_scoped`, kind `merge_readiness`, disposition `review_only` → `review_only`, handoff `review_panel`
