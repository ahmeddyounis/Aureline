# M5 Change-Intent Lifecycle-State and Reconcile-Flow Registries

- Packet: `m5-change-intent-lifecycle-registries:stable:0001`
- Label: `M5 change-intent lifecycle-state and reconcile-flow registries emitting one reusable machine-readable lifecycle-state record per tracked change-intent — one typed field per record section: the provider ownership, the local-versus-provider commit state, the linked branch/worktree/review identity, the relation source, and the validation evidence — each bound to one commit state with its actor lineage, so a change-intent surface never keeps a stale branch/review/provider relation green after drift is detected and no local draft or queued publish reads as a provider-committed update, with canonical / accessible / audit resolution-form coverage, and a machine-readable reconcile-flow object (compare and reconcile when provider state diverges from the local handoff packet, the linked branch/worktree, or the hosted review target) that keeps each lifecycle state and relation source a visible, typed action distinguishing local draft, queued publish, provider linked, stale relation, broken relation, reconcile required, and superseded instead of one generic badge — so queued-publish and reconcile flows preserve the original local packet, linked evidence, and actor lineage for retry and support export — across work-item detail, review detail, start-work sheet, linked-change panel, provider handoff, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **work_item_detail**: `stable`
  - Owner: Work-item-detail owner
  - Scope: Work-item detail resolves a tracked change-intent to one typed lifecycle-state record — its provider ownership, local-versus-provider commit state (local draft, queued publish, provider linked, stale relation, broken relation, reconcile required, or superseded), linked branch/worktree/review identity, relation source, and validation evidence — from the shared registry; a stale or broken branch/review/provider relation never keeps a green badge after drift is detected, and a record that would let a local draft or queued publish read as a provider-committed update degrades honestly instead of implying the provider accepted an update it has not
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves the same lifecycle-state record from the tracked item and shows the linked review target, its relation source, and commit state bound together; a record letting a queued publish or local handoff packet read as provider-committed and a dropped local packet / linked evidence / actor lineage are caught before a green summary can hide them, so review detail renders the same lifecycle truth as work-item detail without contradiction
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the record's commit state while keeping the original local packet / linked evidence / actor lineage and the linked-by-provider / linked-locally / queued-for-publish attribution bound to the export; a record that is a hand-copied per-item assumption and a reconcile flow on an unclassified binding degrade honestly so queued-publish and reconcile flows preserve the original local packet, linked evidence, and actor lineage for retry and support export and never drop them
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **linked_change_panel**: `stable`
  - Owner: Linked-change-panel owner
  - Scope: The linked-change panel renders the same record's linked branch/worktree/review identity and relation source bound to its commit state — linked by provider, linked locally, suggested by Aureline, stale or broken, or queued for publish — from the registry so the relation-source classes can no longer be flattened into one generic badge, and a stale or broken relation stays visible and actionable instead of keeping a green badge after drift is detected; an unstated commit state on a record is caught before it can drift
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **start_work_sheet**: `stable`
  - Owner: Start-work-sheet owner
  - Scope: The start-work sheet renders the same resolved change-intent lifecycle-state record and reconcile-flow truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied state, so a change-intent that moves local draft -> queued publish -> provider linked -> stale/broken relation -> reconcile required -> superseded stays one typed record with its linked branch/worktree/review identity, relation source, and commit state; a stale or broken relation never keeps a green badge after drift is detected, and the reconcile-flow state stays inspectable off-renderer so a local handoff packet never reads as a provider-committed link
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved lifecycle-state record and reconcile-flow truth, so a dropped local packet or linked evidence, an unstated commit state, a local handoff packet masquerading as a provider-committed link, or a stale/broken relation still shown green after drift is visible in evidence — a relation-source change or a commit-state change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
