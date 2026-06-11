# Companion Session-Follow and Incident-Awareness Surfaces

- Packet: `companion-scope-surface:stable:0001`
- Label: `Companion Session-Follow and Incident-Awareness Surfaces`
- Surfaces: 3 | Session-follow: 3 | Incident-awareness: 3 | Light-edit: 2
- Exact handoff for every item: yes
- Stale state honestly labeled: yes
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)
- Degraded: none

## Surfaces

- **session_follow**: `beta` / `staged_rollout` [read_only] (matrix lane `companion_session_follow`)
- **incident_awareness**: `stable` / `general_availability` [read_only] (matrix lane `incident_workspace`)
- **bounded_light_edit**: `preview` / `early_access` [bounded_write_relayed_to_host] (matrix lane `companion_light_edit`)

## Session-follow

- `follow:editor:0001` [active_editor] following — Following the active editor on the host workspace (live) → `file_location` (exact)
- `follow:agent:0002` [agent_run] following — Following a running agent session (live) → `agent_session` (exact)
- `follow:terminal:0003` [terminal_stream] paused — Paused on a cached terminal stream (cached) → `file_location` (exact)

## Incident-awareness

- `incident:0001` [critical/attributed] open — Critical incident raised from a crash trail (live) → `incident_workspace` (exact)
- `incident:0002` [high/attributed] mitigating — High-severity incident under mitigation on the host (cached) → `incident_workspace` (exact)
- `incident:0003` [medium/partially_attributed] acknowledged — Acknowledged incident with stale awareness state (stale) → `incident_workspace` (exact)

## Bounded light-edit

- `edit:0001` [text_touch_up] awaiting_host_approval (bounded_write_relayed_to_host) — Bounded text touch-up relayed for host approval → `file_location` (exact)
- `edit:0002` [comment_reply] relayed_for_preview (bounded_write_relayed_to_host) — Comment reply relayed to the host for preview → `review_panel` (exact)
