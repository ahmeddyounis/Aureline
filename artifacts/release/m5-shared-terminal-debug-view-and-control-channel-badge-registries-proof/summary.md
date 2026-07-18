# M5 Shared-Terminal-Debug-View and Control-Channel-Badge Registries

- Packet: `m5-shared-terminal-debug-view-and-control-channel-badge-registries:stable:0001`
- Label: `M5 shared-terminal-debug-view and control-channel-badge registries emitting one durable machine-readable shared-terminal-debug view stream per sensitive session — one typed field per record section: the stable session start / stop identity, the surface type and target context, the participant scope and observing roles, the command or frame markers, the read-only default a session begins in, and the control-channel state kept distinct from text / presence channels — each bound to one session / target scope, so a view stream never drops its session / target scope and presence never reads as terminal / debug control, with canonical / accessible / audit resolution-form coverage, and a machine-readable control-channel badge (read before a viewer assumes any authority, and re-raised as a fresh visible authority event when a control request, a grant to a single driver, an expiry, or a presence reconnect / cursor-follow / companion resume touches an already-active session) that names the surface type, the target context, and whether input authority is unavailable, requestable, granted, or expired — so a presence reconnect, cursor-follow change, or companion resume never silently upgrades view-only to control-capable, no more than one driver ever holds control on a sensitive surface, and no prior terminal / debug input replays on join or restore — rather than a green summary across shared terminal / debug, companion follow, control-grant prompt, paste / secret guard, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_terminal_debug_view**: `stable`
  - Owner: Shared-terminal-debug-view owner
  - Scope: The shared terminal / debug view resolves a sensitive session to one typed shared-terminal-debug view stream — its stable session start / stop identity, surface type and target context, participant scope and observing roles, command or frame markers, the read-only default it begins in, and the control-channel state kept distinct from text / presence channels — from the shared registry and proves the control-channel badge naming whether input authority is unavailable, requestable, granted, or expired; a view stream missing its session / target scope and a control-channel badge that would let a viewer inherit input authority from presence alone degrade honestly instead of leaving presence to read as terminal / debug control
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **collaboration_join_review_sheet**: `stable`
  - Owner: Collaboration-join-review-sheet owner
  - Scope: The join / follow review sheet reads the control-channel badge and separately names each authority dimension — the surface type and target context, who is observing, whether control is unavailable, requestable, granted to a single driver, or expired, and the read-only default — before any control is assumed; a session presenting itself as control-capable from presence alone and a second driver acquiring input on an active sensitive surface are caught before a green summary can hide them, so a presence reconnect, cursor-follow change, or companion resume never silently upgrades view-only to control-capable and presence never implies control
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the shared-terminal-debug view stream's session / target scope while keeping the surface type, target context, observing scope, and command or frame markers bound to the export, and reports the control-channel badge authority state; a view stream that is a hand-copied per-entry assumption and a control-channel badge on an unclassified authority binding degrade honestly so the session start / stop identity and the control-channel state are never dropped on export or companion handoff, and raw secrets, command text, or clipboard contents stay outside the export boundary
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **control_grant_prompt**: `stable`
  - Owner: Control-grant-prompt owner
  - Scope: The control-grant prompt resolves the control-channel badge's authority state — view-first default, control requested, control granted to a single driver, or expired — bound to the registry so the authority sources can no longer be flattened into one generic presence badge; an unstated session / target scope on a view stream is caught before it can let presence read as an implicit control grant
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **paste_secret_guard**: `stable`
  - Owner: Paste-secret-guard owner
  - Scope: The paste / secret guard renders the same resolved shared-terminal-debug view stream and control-channel badge truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the control-channel state and the guard posture stay inspectable off-renderer so raw secrets, command text, variable bodies, or clipboard contents are never revealed without an explicit policy / consent posture and visible guardrail
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved shared-terminal-debug view stream and control-channel badge truth, so a dropped session / target scope, an unstated surface type, presence masquerading as an implicit control grant, or a second driver acquiring input without a fresh visible authority event is visible in evidence — a control-request event, a grant-to-single-driver event, or an expiry event — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
