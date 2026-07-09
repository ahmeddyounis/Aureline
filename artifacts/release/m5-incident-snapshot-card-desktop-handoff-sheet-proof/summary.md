# Incident-snapshot cards and desktop-handoff sheets

- Packet: `m5-incident-snapshot-card-desktop-handoff-sheet-controls:stable:0001`
- Surface: `M5 incident-snapshot cards and desktop-handoff sheets: service/run identity, severity, latest status, freshness, bounded acknowledge/handoff actions, companion-versus-desktop capability boundary, exact desktop target, and auth/tenant reminder where relevant`
- Incident-snapshot cards: 6 (1 not a live incident)
- Desktop-handoff sheets: 6 (1 not openable)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Incident-snapshot cards

- **Checkout latency spike** (incident_record) — scope `org_scoped`, service `hosted_service`, severity `critical`, status `firing`, freshness `live` → `active_unacknowledged`, handoff `incident_workspace`
- **Queue backlog growing** (incident_record) — scope `workspace_scoped`, service `self_hosted_service`, severity `high`, status `acknowledged`, freshness `live` → `active_acknowledged`, handoff `incident_workspace`
- **Elevated error rate** (incident_record) — scope `repo_scoped`, service `local_core_service`, severity `moderate`, status `investigating`, freshness `cached` → `active_acknowledged`, handoff `incident_workspace`
- **Cache warm-up degraded** (incident_record) — scope `org_scoped`, service `aggregated_source`, severity `low`, status `mitigating`, freshness `live` → `mitigating`, handoff `incident_workspace`
- **Nightly job flake** (incident_record) — scope `repo_scoped`, service `mirrored_snapshot`, severity `informational`, status `resolved`, freshness `stale` → `resolved`, handoff `incident_workspace`
- **Unclassified signal** (incident_record) — scope `account_global`, service `unknown_source`, severity `unspecified`, status `stale`, freshness `unknown_freshness` → `stale_unknown`, handoff `no_handoff`

## Desktop-handoff sheets

- **Open failing test on desktop** (handoff_intent) — scope `repo_scoped`, target `file_location`, auth `same_auth_no_reminder`, freshness `live` → `opens_exact_location`
- **Open review on desktop** (handoff_intent) — scope `repo_scoped`, target `review_panel`, auth `reauth_required`, freshness `live` → `opens_exact_panel`
- **Open CI run on desktop** (handoff_intent) — scope `org_scoped`, target `ci_pipeline_run`, auth `tenant_switch_required`, freshness `cached` → `opens_exact_panel`
- **Open incident workspace on desktop** (handoff_intent) — scope `org_scoped`, target `incident_workspace`, auth `account_mismatch_warning`, freshness `live` → `opens_exact_workspace`
- **Open agent session on desktop** (handoff_intent) — scope `workspace_scoped`, target `agent_session`, auth `scope_elevation_required`, freshness `offline_held` → `opens_exact_workspace`
- **No desktop target available** (handoff_intent) — scope `account_global`, target `no_handoff`, auth `same_auth_no_reminder`, freshness `expired_snapshot` → `not_openable`
