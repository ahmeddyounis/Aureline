# M5 Control-Grant and Presenter-Handoff-Sheet Registries

- Packet: `m5-control-grant-and-presenter-handoff-sheet-registries:stable:0001`
- Label: `M5 control-grant and presenter-handoff-sheet registries emitting one durable machine-readable control-grant sheet per sensitive terminal / debug session — one typed field per record section: the requester, the issuer, and the accepter identities, the granted scope and target context, the time-box and expiry, the revoke path, and the single-active-driver binding kept distinct from ordinary presence and follow state — each bound to one session / target scope, so a control grant never drops its session / target scope and presence never reads as terminal / debug control, with canonical / accessible / audit resolution-form coverage, and a machine-readable presenter-handoff sheet (the presenter / moderator token, its holder, and its handoff chain, re-raised as a fresh visible authority event when a request, a grant to a single driver, a deny, a revoke, an expiry, or a presenter handoff touches an already-active session) that names the requester, issuer, accepter, and scope, and whether write control is unavailable, requestable, granted to a single driver, or expired — so a presenter / moderator handoff never silently transfers shell / debugger control, no more than one driver ever holds mutating control on a sensitive surface, and no prior terminal / debug input replays on join or restore — rather than a green summary across shared terminal / debug, companion follow, control-grant prompt, paste / secret guard, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_terminal_debug_view**: `stable`
  - Owner: Shared-terminal-debug-view owner
  - Scope: The shared terminal / debug view resolves a sensitive session to one typed control-grant sheet — the requester, issuer, and accepter identities, the granted scope and target context, the time-box and expiry, the revoke path, and the single-active-driver binding — from the shared registry and proves the presenter-handoff sheet naming whether write control is unavailable, requestable, granted to a single driver, or expired; a control grant missing its session / target scope and a presenter handoff that would let a viewer inherit input authority from presence alone degrade honestly instead of leaving presence to read as terminal / debug control
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **collaboration_join_review_sheet**: `stable`
  - Owner: Collaboration-join-review-sheet owner
  - Scope: The join / follow review sheet reads the control-grant sheet and separately names each authority dimension — the requester, issuer, and accepter, the granted scope and target context, whether control is unavailable, requestable, granted to a single driver, or expired, and the revoke path — before any control is assumed; a session presenting itself as control-capable from presence alone and a second driver acquiring mutating control on an active sensitive surface are caught before a green summary can hide them, so a presenter / moderator handoff never silently transfers shell / debugger control and presence never implies control
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the control-grant sheet's session / target scope while keeping the requester, issuer, accepter, granted scope, time-box, and revoke path bound to the export, and reports the presenter-handoff chain and its authority state; a control grant that is a hand-copied per-entry assumption and a presenter handoff on an unclassified authority binding degrade honestly so the control-grant history stays visible and exportable as audit-safe metadata without raw command capture, and raw secrets, command text, or clipboard contents stay outside the export boundary
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **control_grant_prompt**: `stable`
  - Owner: Control-grant-prompt owner
  - Scope: The control-grant prompt resolves the control-grant sheet's authority state — view-first default, control requested, control granted to a single driver, denied, revoked, or expired — bound to the registry so the authority sources can no longer be flattened into one generic presence badge; an unstated session / target scope on a control grant is caught before it can let presence read as an implicit control grant
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **paste_secret_guard**: `stable`
  - Owner: Paste-secret-guard owner
  - Scope: The paste / secret guard renders the same resolved control-grant sheet and presenter-handoff truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the single-active-driver binding and the guard posture stay inspectable off-renderer so raw secrets, command text, variable bodies, or clipboard contents are never revealed without an explicit policy / consent posture and visible guardrail
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved control-grant sheet and presenter-handoff truth, so a dropped session / target scope, an unstated requester or issuer, presence masquerading as an implicit control grant, or a second driver acquiring mutating control without a fresh visible authority event is visible in evidence — a request event, a grant-to-single-driver event, a revoke event, or an expiry event — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
