# Work-item rows and provider chip groups

- Packet: `m5-work-item-row-provider-chip-controls:stable:0001`
- Surface: `M5 work-item rows and provider chip groups: canonical id, title, state, owner, priority/severity, linked-change count, keyboard-complete default actions, provider/project-or-space scope, tenant/org cue, and explicit read-only/comment-link/full-edit/offline-capture/policy-blocked write posture`
- Work-item rows: 6 (5 not provider-authoritative)
- Provider chip groups: 5 (3 not writable)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Work-item rows

- **PROJ-1421** (issue) — provider_owned [synced_with_provider] → `provider_authoritative`
- **LOCAL-0007** (task) — local_draft [local_only_draft] → `local_only_draft`
- **INC-3390** (incident) — provider_owned [queued_for_publish] → `publish_pending`
- **CHG-2048** (change_request) — mirrored_read_only [synced_with_provider] → `snapshot_only`
- **PROJ-1500** (issue) — policy_pinned [synced_with_provider] → `blocked_capability`
- **PROJ-1466** (task) — provider_owned [conflict_held] → `publish_pending`

## Provider chip groups

- **GitHub Issues** / acme-eng / platform board [mirrored_read_only] posture `read_only`
- **Jira** / PLAT project [provider_owned] posture `comment_link`
- **Linear** / Platform team space [provider_owned] posture `full_edit`
- **Local capture** / Unsynced drafts space [local_draft] posture `offline_capture`
- **ServiceNow** / Security incidents queue [policy_pinned] posture `policy_blocked`
