# Companion degraded-state continuity controls

- Packet: `m5-companion-degraded-state-continuity-controls:stable:0001`
- Surface: `M5 companion degraded-state continuity: cached, offline, auth-blocked, policy-blocked, loading, and deleted-object states with summary-first object continuity, derived trust and next-safe-action, safe triage verbs, and a desktop fallback before any broken or over-privileged tap`
- Degraded surfaces: 7 (6 not live, 4 needing a desktop fallback)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Degraded surfaces

- **Review requested on change-4201** (notification_row) — scope `repo_scoped`, state `live` → trust `live_trusted`, freshness `live`, next `proceed_in_companion`, handoff `review_panel`
- **Diff review for change-4202 (cached)** (mobile_review_card) — scope `repo_scoped`, state `cached` → trust `cached_reduced`, freshness `cached`, next `refresh_for_latest`, handoff `review_panel`
- **CI run pipeline-4203 (offline)** (ci_status_card) — scope `org_scoped`, state `offline` → trust `offline_stale`, freshness `offline_held`, next `retry_when_online`, handoff `ci_pipeline_run`
- **Follow session sess-4204 (reauth needed)** (session_follow_tile) — scope `workspace_scoped`, state `auth_blocked` → trust `blocked`, freshness `stale`, next `reauth_on_desktop`, handoff `agent_session`
- **Incident inc-4205 (companion publish blocked)** (incident_snapshot_card) — scope `org_scoped`, state `policy_blocked` → trust `blocked`, freshness `cached`, next `open_on_desktop_read_only`, handoff `incident_workspace`
- **Open failing test on desktop (loading)** (desktop_handoff_sheet) — scope `repo_scoped`, state `loading` → trust `loading`, freshness `unknown_freshness`, next `wait_for_load`, handoff `file_location`
- **Handoff target no longer exists** (desktop_handoff_sheet) — scope `account_global`, state `deleted_object` → trust `gone`, freshness `expired_snapshot`, next `view_cached_summary_only`, handoff `no_handoff`
