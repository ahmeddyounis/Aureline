# M5 Unsent-Local-Edit-Preservation and Recovery-Disposition Registries

- Packet: `m5-unsent-local-edit-preservation-and-recovery-disposition-registries:stable:0001`
- Label: `M5 unsent-local-edit-preservation and recovery-disposition registries emitting one durable machine-readable preservation record per collaboration downgrade — role loss, permission narrowing, explicit leave, host removal, or relay failure — one typed field per record section: the workspace-root and buffer / object identity of the unsent local shared text, the downgrade trigger, the preserved-state class it lands in (local-only, reconnect-ready, or reviewable patch packet), and the export posture — so a downgrade preserves local unsent work first rather than silently dropping or remotely resolving it, with canonical / accessible / audit resolution-form coverage, and a machine-readable recovery-disposition record per downgrade that declares the next action a user can take (continue-local, reopen-share, export-patch, or discard-with-review) with its actor / time provenance and its policy-labeled export lineage — so unsent local collaboration edits are never silently dropped on downgrade or disconnect, users can inspect and act on preserved local-only state or reviewable patch packets before rejoining, exporting, or discarding, a discard never applies without review, and exported patches, op-logs, snapshots, or archives never export without policy-labeled redaction and actor lineage — rather than a green summary across the shared editor replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_editor_replica_view**: `stable`
  - Owner: Shared-editor-replica-view owner
  - Scope: The shared editor replica view resolves each downgrade — role loss, permission narrowing, explicit leave, host removal, or relay failure — to one typed unsent-local-edit-preservation record naming the workspace-root and buffer / object identity of the unsent local shared text, the downgrade trigger, the preserved-state class it lands in (local-only, reconnect-ready, or reviewable patch packet), and its export posture, and to the recovery disposition offering continue-local, reopen-share, export-patch, or discard-with-review with actor / time provenance; the preserved work is materialized before the session narrows, and a record that cannot bind its preserved-work identity or that would let a downgrade read as clean while unsent local edits are dropped degrades honestly instead of discarding local canonical truth
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **presence_cursor_layer**: `stable`
  - Owner: Presence-cursor-layer owner
  - Scope: The presence / cursor layer resolves a relay-loss or leave-session downgrade to the preservation record that marks the local session reconnect-ready — carrying the provenance and freshness a consumer must see — rather than letting presence quietly vanish; a downgrade that would drop unsent local work while reporting presence as current and a stale reconnect-ready state shown as live degrade honestly instead of letting an awareness downgrade read as clean
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the preserved unsent-edit records and their recovery dispositions while keeping the preserved-work identity, recovery action, and export posture bound to the export-patch route; a disposition whose actor / time provenance is unstated and an exported patch, op-log, or archive that would export without policy-labeled redaction and actor lineage degrade honestly so the preserved-work identity, recovery action, and provenance are never dropped on export or companion handoff
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **comment_annotation_review_pin_layer**: `stable`
  - Owner: Comment-annotation-review-pin-layer owner
  - Scope: The comment / annotation / review-pin layer resolves a downgrade over server-ordered comments, annotations, and review pins to a preservation record that keeps unsent local pin and comment edits as a reviewable patch packet with an append-only, reviewable drift history; a disposition whose recovery action is unstated and a discard applied without review degrade honestly instead of silently dropping an unsent comment or pin edit
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **collaboration_degradation_banner**: `stable`
  - Owner: Collaboration-degradation-banner owner
  - Scope: The collaboration degradation banner resolves the higher-risk control-plane downgrade to a preservation record that names the distinct trigger — role loss, permission narrowing, explicit leave, host removal, or relay failure — and surfaces the recovery disposition (continue-local, reopen-share, export-patch, or discard-with-review) rather than a generic stale or broken badge; a downgrade whose preserved-state class is unstated and a distinct downgrade collapsed into a generic stale badge degrade honestly so preserved unsent work never disappears behind an undifferentiated banner
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same unsent-local-edit-preservation and recovery-disposition truth, so a downgrade whose preserved-state class is unstated, unsent local edits discarded without preservation, or a discard applied without review is visible in evidence — each preserved state named as local-only, reconnect-ready, or reviewable patch packet, and each action as continue-local, reopen-share, export-patch, or discard-with-review — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
