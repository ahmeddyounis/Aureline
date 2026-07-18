# M5 Retention-Review and Sealed-Archive-Manifest Registries

- Packet: `m5-retention-review-and-sealed-archive-manifest-registries:stable:0001`
- Label: `M5 retention-review and sealed-archive-manifest registries emitting one durable machine-readable retention review per session — one typed field per record section: the retention mode (live-only, metadata audit, replayable text / comment timeline, or elevated support / regulatory evidence), the disclosed retention envelope, the export / delete-right posture required, and the visible retention badge kept distinct from ordinary presence and follow state — each bound to one session / retention scope, so a review never drops its retention envelope / export / delete rights and a retention change never begins silently, with canonical / accessible / audit resolution-form coverage, and a machine-readable sealed-archive manifest (the content-addressed, policy-labeled archive identity and its audit-safe attribution, re-raised as a fresh visible consent event when a disclose, a consent-renewal, a guest-scope-widen, an export / delete-right change, or a sealed-archive event touches a session) that names the retention mode, retention envelope, export / delete rights, and whether the archive is live-only, metadata audit, replayable text / comment timeline, or elevated support / regulatory evidence — so a user can tell at join time and at retention-change time exactly what will be retained and what export / delete rights apply, no raw session body ever crosses into logs or exports, sealed archives carry content-addressed, policy-labeled manifests instead of generic "session recording saved" language, and retention-change or guest-scope-widen events stay attributable — rather than a green summary across shared terminal / debug, companion follow, control-grant prompt, paste / secret guard, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_terminal_debug_view**: `stable`
  - Owner: Shared-terminal-debug-view owner
  - Scope: The shared terminal / debug view resolves a session to one typed retention review — its retention mode (live-only, metadata audit, replayable text / comment timeline, or elevated support / regulatory evidence), the disclosed retention envelope, the export / delete-right posture, and the visible retention badge — from the shared registry and proves the sealed-archive manifest naming, by content-address and policy label, exactly what was retained; a review missing its retention envelope / export / delete rights and a manifest that would let a retention change or archive begin silently degrade honestly instead of reading as a generic session recording
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **collaboration_join_review_sheet**: `stable`
  - Owner: Collaboration-join-review-sheet owner
  - Scope: The join / follow review sheet reads the retention review and separately names each retention dimension — the retention mode, the disclosed retention envelope, whether the archive is live-only, metadata-audit, replayable text / comment timeline, or elevated support / regulatory evidence, the export / delete-right posture, and the visible badge — before any session join commits; a retention change or guest-scope widening attempting to begin silently and an archive created without an explicit policy / consent posture are caught before a green summary can hide them, so a user can tell at join time exactly what will be retained and what export / delete rights apply and presence never implies consent to retain
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the retention review's retention envelope / export / delete rights while keeping the retention mode, the guest-scope posture, and the sealed / export / delete outcome bound to the export, and reports the sealed-archive manifest's content-address and policy label; a review that is a hand-copied per-entry assumption and a manifest on an unclassified archive binding degrade honestly so retention-change and guest-scope-widen events stay attributable and exportable as audit-safe metadata without raw session capture, and raw session bodies, command text, variable bodies, or clipboard contents stay outside the export boundary
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **control_grant_prompt**: `stable`
  - Owner: Control-grant-prompt owner
  - Scope: The control-grant prompt resolves the retention review's posture — view-first default, disclose retention envelope, consent-renewal required, guest-scope-widen, or export / delete-right change — bound to the registry so the retention postures can no longer be flattened into one generic "session recording saved" dialog; an unstated retention envelope on a review is caught before it can let a retention change read as pre-approved
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **paste_secret_guard**: `stable`
  - Owner: Paste-secret-guard owner
  - Scope: The paste / secret guard renders the same resolved retention review and sealed-archive manifest truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the export / delete-right posture and the archive binding stay inspectable off-renderer so raw session bodies, command text, variable bodies, or clipboard contents are never retained or archived without an explicit policy / consent posture and visible badge
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved retention review and sealed-archive manifest truth, so a dropped retention envelope, an unstated retention mode, a sealed archive masquerading as a generic recording, or a retention-change or guest-scope-widen event dropping its attribution is visible in evidence — a disclose event, a consent-renewal event, a guest-scope-widen event, an export / delete-right event, or a sealed-archive event — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
