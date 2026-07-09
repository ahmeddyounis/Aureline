# CI-status cards and session-follow tiles

- Packet: `m5-ci-status-card-session-follow-tile-controls:stable:0001`
- Surface: `M5 CI-status cards and session-follow tiles: provider/source class, run/commit/session identity, freshness, failure counts, keyboard-complete follow/open-logs/handoff quick actions, companion-versus-desktop capability boundary, and an exact desktop-handoff target`
- CI-status cards: 6 (1 not a live result)
- Session-follow tiles: 6 (4 not joinable)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## CI-status cards

- **Release pipeline** (ci_run) — scope `repo_scoped`, source `hosted_provider`, status `passed`, freshness `live` → `green`, handoff `ci_pipeline_run`
- **Integration pipeline** (ci_run) — scope `repo_scoped`, source `self_hosted_runner`, status `failed`, freshness `live` → `red`, handoff `ci_pipeline_run`
- **Unit pipeline** (ci_run) — scope `workspace_scoped`, source `local_core`, status `running`, freshness `live` → `in_flight`, handoff `ci_pipeline_run`
- **Nightly pipeline** (ci_run) — scope `org_scoped`, source `aggregated_source`, status `queued`, freshness `cached` → `in_flight`, handoff `ci_pipeline_run`
- **Fuzz pipeline** (ci_run) — scope `repo_scoped`, source `mirrored_snapshot`, status `canceled`, freshness `stale` → `canceled`, handoff `ci_pipeline_run`
- **Deploy pipeline** (ci_run) — scope `account_global`, source `unknown_source`, status `stale`, freshness `unknown_freshness` → `stale_unknown`, handoff `no_handoff`

## Session-follow tiles

- **Live pairing session** (followed_session) — scope `workspace_scoped`, state `live_following`, freshness `live` → `live_joinable`, handoff `agent_session`
- **Paused pairing session** (followed_session) — scope `workspace_scoped`, state `paused_follow`, freshness `cached` → `paused_resumable`, handoff `agent_session`
- **Diverged pairing session** (followed_session) — scope `device_scoped`, state `diverged_from_host`, freshness `stale` → `stale_read_only`, handoff `agent_session`
- **Idle pairing session** (followed_session) — scope `workspace_scoped`, state `host_inactive`, freshness `offline_held` → `not_joinable`, handoff `agent_session`
- **Mirror pairing session** (followed_session) — scope `org_scoped`, state `read_only_mirror`, freshness `expired_snapshot` → `stale_read_only`, handoff `agent_session`
- **Ended pairing session** (followed_session) — scope `account_global`, state `follow_ended`, freshness `unknown_freshness` → `not_joinable`, handoff `no_handoff`
