# M5 Session-Restore-View and Restore-Grant-Posture Registries

- Packet: `m5-session-restore-view-and-restore-grant-posture-registries:stable:0001`
- Label: `M5 session-restore-view and restore-grant-posture registries emitting one durable machine-readable session-restore view per reconnect — one typed field per record section: the transcript class (replay-free render summary, metadata restore summary, text / comment timeline summary, or elevated support / regulatory evidence summary), the restore path, the target context, whether live control was re-requested, and the replay-free render summary kept distinct from ordinary presence and follow state — each bound to one session / restore scope, so a restore never drops its restore path / target context and a reconnect never replays prior input silently, with canonical / accessible / audit resolution-form coverage, and a machine-readable restore-grant posture (the view-only-by-default control posture and its audit-safe attribution, re-raised as a fresh visible authority event when a restore-disclosed-view-only, an observing-view-only, a control-re-request, a control-re-grant, a reopen-target-required, or a replay-blocked-no-rerun outcome touches a session) that names the transcript class, restore path, target context, and whether the restore is observing view-only, requesting control again, or needs to reopen the target from scratch — so a user can tell at restore time and at reconnect time exactly whether they are observing, requesting control again, or need to reopen the target, no prior input is ever replayed and no authority carries over into logs or exports, restored sessions carry replay-free, view-only postures instead of generic "session reconnected" language, and reconnect or reopen events stay attributable — rather than a green summary across shared terminal / debug, companion follow, control-grant prompt, paste / secret guard, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_terminal_debug_view**: `stable`
  - Owner: Shared-terminal-debug-view owner
  - Scope: The shared terminal / debug view resolves a reconnect to one typed session-restore view — its transcript class (replay-free render summary, metadata restore summary, text / comment timeline summary, or elevated support / regulatory evidence summary), the restore path, the target context, whether live control was re-requested, and the replay-free render summary — from the shared registry and proves the restore-grant posture naming, by transcript class and control outcome, exactly what was rejoined; a restore missing its restore path / target context and a posture that would replay prior input or carry authority forward on reconnect degrade honestly instead of reading as a control-capable rejoin
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **collaboration_join_review_sheet**: `stable`
  - Owner: Collaboration-join-review-sheet owner
  - Scope: The join / follow review sheet reads the session-restore view and separately names each restore dimension — the transcript class, the restore path, whether the render is a replay-free render summary, metadata restore summary, text / comment timeline summary, or elevated support / regulatory evidence summary, whether live control was re-requested, and the view-only default — before any rejoin commits; a reconnect replaying prior input and a restored session presenting itself as control-capable without a fresh grant are caught before a green summary can hide them, so a user can tell at restore time exactly whether they are observing, requesting control again, or need to reopen the target, and presence never implies control
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the session-restore view's restore path / target context while keeping the transcript class, whether control was re-requested, and the observe / re-request / reopen / replay-blocked outcome bound to the export, and reports the restore-grant posture's transcript class and control outcome; a restore that is a hand-copied per-entry assumption and a posture on an unclassified restore binding degrade honestly so reconnect and reopen events stay attributable and exportable as audit-safe metadata without raw input capture, and prior terminal / debug input, command text, variable bodies, or clipboard contents stay outside the export boundary and are never replayed
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **control_grant_prompt**: `stable`
  - Owner: Control-grant-prompt owner
  - Scope: The control-grant prompt resolves the session-restore view's posture — view-only default, observing view-only, control re-request pending, control re-granted, reopen target required, or replay blocked with no rerun — bound to the registry so the restore postures can no longer be flattened into one generic "session reconnected" dialog; an unstated restore path on a restore is caught before it can let a reconnect read as control-carrying
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **paste_secret_guard**: `stable`
  - Owner: Paste-secret-guard owner
  - Scope: The paste / secret guard renders the same resolved session-restore view and restore-grant posture truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the control-re-request posture and the restore binding stay inspectable off-renderer so prior terminal / debug input, command text, variable bodies, or clipboard contents are never replayed or reacquired without a fresh control grant and visible badge
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved session-restore view and restore-grant posture truth, so a dropped restore path, an unstated transcript class, a restored session masquerading as a control-capable rejoin, or a reconnect or reopen event dropping its attribution is visible in evidence — a restore-disclosed-view-only event, an observing-view-only event, a control-re-request event, a control-re-grant event, a reopen-target-required event, or a replay-blocked-no-rerun event — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
