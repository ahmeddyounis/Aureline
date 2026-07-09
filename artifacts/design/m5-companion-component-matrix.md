# M5 Notification-Row, Mobile-Review-Card, CI-Status-Card, Session-Follow-Tile, Incident-Snapshot-Card, and Desktop-Handoff-Sheet Component Matrix

- Packet: `m5-companion-components:stable:0001`
- Label: `M5 notification-row, mobile-review-card, ci-status-card, session-follow-tile, incident-snapshot-card, and desktop-handoff-sheet component matrix`
- Component families: 6 (6 stable)
- Dispositions: review_only, comment_capable, desktop_required, cached, stale, policy_blocked, handoff_ready
- Freshness classes: live, cached, stale, offline_held, expired_snapshot, unknown_freshness
- Proof freshness SLO: 168 hours (last refresh: 2026-07-09T00:00:00Z)

## Component families

- **notification_row**: `stable`
  - Owner: Companion notification-row owner
  - Canonical schema: `schemas/ui/m5-companion-notification-row.schema.json`
  - Scope: One notification-row model naming exactly which object a tap opens (a notification event bound to a build, review, agent, incident, sync, or mention), its severity, its client scope, and its freshness, so a user never has to infer what a tap opens or how urgent it is before acting from a browser or mobile companion
  - Required labels: identity, state, keyboard_route, scope_and_freshness, severity_and_handoff_target
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **mobile_review_card**: `stable`
  - Owner: Companion mobile-review-card owner
  - Canonical schema: `schemas/ui/m5-mobile-review-card.schema.json`
  - Scope: One mobile-review-card model naming the review kind it carries (agent change, diff review, comment thread, approval request, policy gate, or merge readiness), its client scope, and whether it is review-only or comment-capable from the companion versus desktop-required, so a user never has to infer which actions are companion-safe before tapping
  - Required labels: identity, state, keyboard_route, scope_and_freshness, capability_boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **ci_status_card**: `stable`
  - Owner: Companion CI-status-card owner
  - Canonical schema: `schemas/ui/m5-ci-status-card.schema.json`
  - Scope: One ci-status-card model naming its pipeline status (passed, failed, running, queued, canceled, or stale), its repo/workspace scope, and its freshness (live, cached, or stale), so a stale pipeline status is never shown as live and a user always knows whether the status is current before acting
  - Required labels: identity, state, keyboard_route, scope_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **session_follow_tile**: `stable`
  - Owner: Companion session-follow-tile owner
  - Canonical schema: `schemas/ui/m5-session-follow-tile.schema.json`
  - Scope: One session-follow-tile model naming the followed session's state (live following, paused, diverged from host, host inactive, read-only mirror, or follow ended), its scope, and its freshness, so a diverged or stale followed session is never shown as live and the read/write boundary stays honest
  - Required labels: identity, state, keyboard_route, scope_and_freshness, capability_boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **incident_snapshot_card**: `stable`
  - Owner: Companion incident-snapshot-card owner
  - Canonical schema: `schemas/ui/m5-incident-snapshot-card.schema.json`
  - Scope: One incident-snapshot-card model naming the incident's severity (critical, high, moderate, low, informational, or unspecified), its scope, and its freshness, so a stale incident snapshot is never shown as live and a user always sees how severe an incident is before escalating or handing off to desktop
  - Required labels: identity, state, keyboard_route, scope_and_freshness, severity_and_handoff_target
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **desktop_handoff_sheet**: `stable`
  - Owner: Companion desktop-handoff-sheet owner
  - Canonical schema: `schemas/ui/m5-desktop-handoff-sheet.schema.json`
  - Scope: One desktop-handoff-sheet model naming the exact target it will open on desktop (a file location, a review panel, a CI pipeline run, an incident workspace, an agent session, or no handoff) and whether an active host is required, so a user always knows exactly what opens on desktop before a tap and a desktop-required action never reads as companion-safe
  - Required labels: identity, state, keyboard_route, capability_boundary, severity_and_handoff_target
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
