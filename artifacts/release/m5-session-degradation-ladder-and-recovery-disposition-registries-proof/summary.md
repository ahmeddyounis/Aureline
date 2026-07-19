# M5 Session-Degradation-Ladder and Recovery-Disposition Registries

- Packet: `m5-session-degradation-ladder-and-recovery-disposition-registries:stable:0001`
- Label: `M5 session-degradation-ladder and recovery-disposition registries emitting one durable append-only machine-readable degradation-ladder record per degraded shared session — a participant-lag, shared-degraded, relay-partition, awareness-degraded, or convergence-degraded transition — one typed field per record section: the degraded session state, the sticky banner it raises, the recent-activity rows it keeps, the degradation-order rung so local editing degrades last, the actor provenance of the transition, and the recovery paths still available — so a degradation keeps local editing first rather than freezing local work or letting remote authority silently resolve divergence, with canonical / accessible / audit resolution-form coverage, and a machine-readable recovery-disposition-descriptor record per degraded session that declares the recovery path a user can take (continue-local, retry-shared, retry-convergence, review-and-reconcile, or leave-open) with its actor / time provenance and its local-edit-first continuity lineage — so relay or participant failure never freezes local editing or silently fixes divergence by remote authority, users can tell whether a session lost awareness, convergence, or both and what recovery path remains, a recovery never acts without actor / time provenance, and no recovery discards unsent local edits — rather than a green summary across the shared editor replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_editor_replica_view**: `stable`
  - Owner: Shared-editor-replica-view owner
  - Scope: The shared editor replica view resolves each degradation of a shared session — a participant-lag, shared-degraded, relay-partition, awareness-degraded, or convergence-degraded transition — to one typed degradation-ladder record naming the degraded state, the sticky banner it raises, the recent-activity rows it keeps, the degradation-order rung so local editing degrades last, the actor provenance of the transition, and the recovery paths still available, and to the recovery-disposition descriptor offering continue-local, retry-shared, retry-convergence, review-and-reconcile, or leave-open with actor / time provenance; the degraded state and its still-open recovery paths are recorded before any shared or convergence rung is touched, and a record that would freeze local editing on relay or participant failure or let remote authority silently resolve divergence degrades honestly instead of dropping what still remains safely local
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **presence_cursor_layer**: `stable`
  - Owner: Presence-cursor-layer owner
  - Scope: The presence / cursor layer resolves an awareness-degraded transition to the degradation-ladder record that keeps the degraded state and recent-activity rows inspectable — carrying the provenance and freshness a consumer must see — rather than letting lost awareness vanish behind a generic stale badge; a record that would collapse a distinct awareness-degraded state into a generic badge and a stale banner shown as current degrade honestly instead of letting the degraded session read as clean
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the recorded degradation-ladder entries and their recovery-disposition descriptors while keeping the degraded state, recent-activity rows, awareness / convergence facts, and recovery path bound to the export route, so an export or support flow can explain whether a session lost awareness, convergence, or both and what recovery path remains, and preserve actor provenance; a descriptor whose actor / time provenance is unstated and a recovery that would resolve divergence by remote authority without review degrade honestly so the degraded state, recovery path, and provenance are never dropped on export or companion handoff
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **comment_annotation_review_pin_layer**: `stable`
  - Owner: Comment-annotation-review-pin-layer owner
  - Scope: The comment / annotation / review-pin layer resolves a convergence-degraded transition over server-ordered comments, annotations, and review pins to a degradation-ladder record that keeps the degraded state, recent-activity rows, and degradation-order rung as an append-only, inspectable ladder with a continue-local-or-reconcile descriptor; a descriptor whose recovery path is unstated and a recovery that would silently rebind or reconcile pins by remote authority without review degrade honestly instead of resolving divergence over comment or pin state on the user's behalf
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **collaboration_degradation_banner**: `stable`
  - Owner: Collaboration-degradation-banner owner
  - Scope: The collaboration degradation banner resolves a degraded-session condition to a degradation-ladder record that names the distinct participant-lag / shared-degraded / relay-partition / awareness-degraded / convergence-degraded state and surfaces the recovery-disposition descriptor (continue-local, retry-shared, retry-convergence, review-and-reconcile, or leave-open) as a sticky banner rather than a generic stale or broken badge; a degradation whose degraded state is unstated and a distinct awareness- or convergence-degraded state collapsed into a generic stale badge degrade honestly so a failing session never disappears behind an undifferentiated banner
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same session-degradation and recovery-disposition truth, so a degradation whose degraded state is unstated, a recovery that resolves divergence by remote authority without review, or a recovery that discards unsent local edits is visible in evidence — each state named as participant-lag, shared-degraded, relay-partition, awareness-degraded, or convergence-degraded, and each recovery path as continue-local, retry-shared, retry-convergence, review-and-reconcile, or leave-open — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
