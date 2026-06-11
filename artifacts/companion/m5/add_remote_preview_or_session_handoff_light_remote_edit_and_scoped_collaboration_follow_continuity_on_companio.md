# Companion Remote-Preview, Session-Handoff, Light-Remote-Edit, and Collaboration-Follow Continuity

- Packet: `companion-continuity-surface:stable:0001`
- Label: `Companion Remote-Preview, Session-Handoff, Light-Remote-Edit, and Collaboration-Follow Continuity`
- Surfaces: 3 | Remote-preview: 3 | Light-remote-edit: 2 | Collaboration-follow: 3
- Exact handoff for every item: yes
- Stale state honestly labeled: yes
- Local work never stranded: yes
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)
- Degraded: none

## Surfaces

- **remote_preview_handoff**: `beta` / `staged_rollout` [read_only] (matrix lane `companion_session_follow`)
- **light_remote_edit**: `preview` / `early_access` [bounded_write_relayed_to_host] (matrix lane `companion_light_edit`)
- **collaboration_follow**: `stable` / `general_availability` [read_only] (matrix lane `companion_review`)

## Remote-preview-handoff

- `preview:editor:0001` [editor_preview] local_authoritative — Remote preview of the active editor on the host workspace (live) → `file_location` (exact)
- `preview:agent:0002` [agent_run_preview] handoff_staged — Remote preview of a running agent session staged for handoff (live) → `agent_session` (exact)
- `preview:terminal:0003` [terminal_preview] handoff_resumed — Resumed terminal preview from a cached handoff (cached) → `file_location` (exact)

## Light-remote-edit

- `edit:0001` [text_touch_up] awaiting_host_approval (bounded_write_relayed_to_host) — Bounded text touch-up relayed for host approval → `file_location` (exact)
- `edit:0002` [comment_reply] relayed_for_preview (bounded_write_relayed_to_host) — Comment reply relayed to the host for preview → `review_panel` (exact)

## Collaboration-follow

- `collab:0001` [driver/shared_session_scope] Following the driver within a shared session scope (live) → `review_panel` (exact)
- `collab:0002` [reviewer/shared_review_scope] Following a reviewer within a shared review scope (cached) → `review_panel` (exact)
- `collab:0003` [observer/shared_file_scope] Observer follow with a stale shared-file scope (stale) → `file_location` (exact)
