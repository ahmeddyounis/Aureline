# Companion Notification Triage, Review Queues, and CI-Status Cards

- Packet: `companion-triage-surface:stable:0001`
- Label: `Companion Notification Triage, Review Queues, and CI-Status Cards`
- Sections: 3 | Notifications: 4 | Review queue: 3 | CI cards: 3
- Exact handoff for every item: yes
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)
- Degraded: none

## Sections

- **notification_triage**: `stable` / `general_availability` (matrix lane `companion_notification`)
- **review_queue**: `stable` / `general_availability` (matrix lane `companion_review`)
- **ci_status_cards**: `beta` / `staged_rollout` (matrix lane `companion_session_follow`)

## Notification triage

- `notif:build:0001` [build/high] unread — Build failed on the active workspace → `ci_pipeline` (exact)
- `notif:review:0002` [review/normal] triaged — Review requested on a pending change → `review_panel` (exact)
- `notif:agent:0003` [agent/normal] unread — Agent run finished and is awaiting review → `agent_session` (exact)
- `notif:incident:0004` [incident/critical] escalated_to_desktop — Incident raised from a crash trail → `incident_workspace` (exact)

## Review queue

- `review:0001` [agent_change] pending (approve_or_defer) — Agent change set staged for approval → `file_location` (exact)
- `review:0002` [diff_review] pending (approve_or_defer) — Diff awaiting review → `review_panel` (exact)
- `review:0003` [approval_request] deferred (escalate_only) — Approval request deferred to the desktop host → `review_panel` (exact)

## CI-status cards

- `ci:main:0001` main — `passed` (live, 0 failing) → `ci_pipeline` (exact)
- `ci:pr:0002` pull-request — `failed` (live, 2 failing) → `ci_pipeline` (exact)
- `ci:nightly:0003` nightly — `running` (cached, 0 failing) → `ci_pipeline` (exact)
