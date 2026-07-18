# M5 Session-Policy-Manifest and Join-Review-Sheet Registries

- Packet: `m5-session-policy-manifest-and-join-review-sheet-registries:stable:0001`
- Label: `M5 session-policy-manifest and join-review-sheet registries emitting one durable machine-readable session-policy manifest per collaboration session — one typed field per record section: the stable session identity, the session type and tenant / guest policy, the participant list or scope class, the active roles and active badges, the retention envelope and export / delete posture, and the read-only default a session begins in — each bound to one session / tenant scope, so a session-policy manifest never drops its session / tenant scope and presence never reads as terminal / debug control, with canonical / accessible / audit resolution-form coverage, and a machine-readable join-review / consent envelope (surfaced before a participant joins, and re-raised as a fresh visible consent event when an external guest, scope widening, a retention-mode change, or a route-share visibility change affects an already-active session) that discloses who can see the session, what may be retained, and what authority is available — so recording, transcript retention, guest-scope widening, or route-visibility expansion never starts silently and no prior terminal / debug input replays on join or restore — rather than a green summary across desktop join, shared terminal / debug, companion follow, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_terminal_debug_view**: `stable`
  - Owner: Shared-terminal-debug-view owner
  - Scope: The shared terminal / debug view resolves a collaboration session to one typed session-policy manifest — its stable session identity, session type and tenant / guest policy, participant list or scope class, active roles and active badges, retention envelope and export / delete posture, and the read-only default it begins in — from the shared registry and proves the join-review disclosure gating any participant before join; a manifest missing its session / tenant scope and a join-review disclosure that would let a participant join without seeing retention or guest presence degrade honestly instead of leaving presence to read as terminal / debug control
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **collaboration_join_review_sheet**: `stable`
  - Owner: Collaboration-join-review-sheet owner
  - Scope: The join-review sheet resolves the disclosure a participant must pass before join and separately names each consent dimension — who can see the session, what may be retained, what authority is available, and the read-only default — before any control is granted; a session presenting itself as safe to join without disclosing guest presence and a retention mode changed on an active session without a fresh visible consent event are caught before a green summary can hide them, so recording, transcript retention, guest-scope widening, or route-visibility expansion never starts silently and presence never implies control
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the session-policy manifest's session / tenant scope while keeping the participant scope, active roles, retention envelope, and export / delete posture bound to the export, and reports the join-review consent state; a manifest that is a hand-copied per-entry assumption and a consent envelope on an unclassified change binding degrade honestly so the session identity and retention envelope are never dropped on export or companion handoff
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **control_grant_prompt**: `stable`
  - Owner: Control-grant-prompt owner
  - Scope: The control-grant prompt resolves the session-policy manifest's active roles and authority state — view-first default, control requested, control granted to a single driver, or expired — bound to the registry so the authority sources can no longer be flattened into one generic badge; an unstated session / tenant scope on a manifest is caught before it can let presence read as an implicit control grant
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **paste_secret_guard**: `stable`
  - Owner: Paste-secret-guard owner
  - Scope: The paste / secret guard renders the same resolved session-policy manifest and join-review disclosure truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the retention envelope and the consent / guard posture stay inspectable off-renderer so raw secrets, command text, or clipboard contents are never revealed without an explicit consent posture and visible guardrail
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved session-policy manifest and join-review disclosure truth, so a dropped session / tenant scope, an unstated session type, presence masquerading as an implicit control grant, or a retention mode widened without a fresh visible consent event is visible in evidence — a guest-scope change, a retention-mode change, or a route-visibility change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
