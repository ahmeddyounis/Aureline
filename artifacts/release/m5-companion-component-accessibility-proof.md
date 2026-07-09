# M5 Companion Component Accessibility & Auto-Narrowing

- Packet: `m5-companion-component-accessibility-parity:stable:0001`
- As of: `2026-07-09T00:00:00Z`
- Families: 6 certified across 6 / 6 frozen families
- Status: 2 green / 4 yellow / 0 red

## Rows

- **a11y:notification-row-freshness-stale** (notification_row) — family=notification_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=live_companion_safe effective_claim=stale_freshness_projection status=narrowed_disclosed
  - Auto-narrow: live_companion_safe → stale_freshness_projection (dimension=object_freshness, trigger=freshness_hidden) — Object freshness has gone stale and this notification must be refreshed — shown as a stale-freshness projection with its canonical object identity, client scope, and severity still preserved, never as a live notification
- **a11y:mobile-review-card-authority-limited** (mobile_review_card) — family=mobile_review_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=live_companion_safe effective_claim=limited_authority_projection status=narrowed_disclosed
  - Auto-narrow: live_companion_safe → limited_authority_projection (dimension=companion_authority, trigger=capability_boundary_unstated) — This review requires desktop authority and only a read-only companion view remains — shown as a limited-authority projection that names its review kind and capability boundary, never as a companion-completable review
- **a11y:ci-status-card** (ci_status_card) — family=ci_status_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=live_companion_safe effective_claim=live_companion_safe status=parity
- **a11y:session-follow-tile-tenant-narrowed** (session_follow_tile) — family=session_follow_tile keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=live_companion_safe effective_claim=narrowed_tenant_projection status=narrowed_disclosed
  - Auto-narrow: live_companion_safe → narrowed_tenant_projection (dimension=tenant_scope, trigger=client_scope_unstated) — The followed session's tenant scope has narrowed from what was granted and must be reconciled — shown as a narrowed-tenant projection that names its presenter and session identity, never as an in-scope live session
- **a11y:incident-snapshot-card** (incident_snapshot_card) — family=incident_snapshot_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=cached_continuity_safe effective_claim=cached_continuity_safe status=parity
- **a11y:desktop-handoff-sheet-handoff-revoked** (desktop_handoff_sheet) — family=desktop_handoff_sheet keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=live_companion_safe effective_claim=revoked_handoff_projection status=narrowed_disclosed
  - Auto-narrow: live_companion_safe → revoked_handoff_projection (dimension=handoff_validity, trigger=handoff_target_unresolved) — The desktop-handoff target is revoked and no longer resolves exactly — shown as a revoked-handoff projection that names its target object and identity, never as a sheet that will open the intended object on desktop
