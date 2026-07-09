# M5 Companion Component Surface Certification

- Packet: `m5-companion-component-certification:stable:0001`
- As of: `2026-07-09T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-companion-component-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Companion truth preserved on every surface: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:notification-inbox** — surface=notification_inbox claimed=live_companion_safe certified=live_companion_safe status=green narrowed_axes=0 companion_truth_preserved=true
- **cert:ci-status-dashboard** — surface=ci_status_dashboard claimed=live_companion_safe certified=live_companion_safe status=green narrowed_axes=0 companion_truth_preserved=true
- **cert:incident-awareness** — surface=incident_awareness claimed=cached_continuity_safe certified=cached_continuity_safe status=green narrowed_axes=0 companion_truth_preserved=true
- **cert:support-export** — surface=support_export claimed=cached_continuity_safe certified=cached_continuity_safe status=green narrowed_axes=0 companion_truth_preserved=true
- **cert:mobile-review-queue** — surface=mobile_review_queue claimed=live_companion_safe certified=limited_authority_projection status=yellow narrowed_axes=1 companion_truth_preserved=true
- **cert:session-follow** — surface=session_follow claimed=live_companion_safe certified=narrowed_tenant_projection status=yellow narrowed_axes=1 companion_truth_preserved=true
- **cert:desktop-handoff** — surface=desktop_handoff claimed=live_companion_safe certified=revoked_handoff_projection status=yellow narrowed_axes=1 companion_truth_preserved=true
- **cert:help-docs** — surface=help_docs claimed=live_companion_safe certified=stale_freshness_projection status=yellow narrowed_axes=1 companion_truth_preserved=true
