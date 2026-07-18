# M5 Comment-Annotation-Review-Pin Anchor-History and Rebind-Review Registries

- Packet: `m5-comment-annotation-review-pin-anchor-history-and-rebind-review-registries:stable:0001`
- Label: `M5 comment-annotation-review-pin anchor-history and rebind-review registries emitting one durable append-only machine-readable anchor-history record per comment / annotation / review-pin drift — from a buffer edit, generated-output refresh, or imported-snapshot transition — one typed field per record section: the comment / annotation / review-pin identity, the textual and semantic anchors, the revision-pair lineage across which the anchor moved, the drift / unresolved / rebound state it lands in, and the export posture — so a drift preserves append-only drift history first rather than silently jumping a pin to a new location, with canonical / accessible / audit resolution-form coverage, and a machine-readable rebind-review-disposition record per drift that declares the manual or assisted rebind decision a user can take (keep-textual, keep-semantic, manual-rebind, assisted-rebind, or leave-unresolved) with its actor / time provenance and its policy-labeled export lineage — so collaboration comments or review pins never silently jump to a new location without drift history and rebind state, export and support flows can show the current anchor state plus the full history of drift and rebound decisions, a rebind never applies without review, and exported op-logs, snapshots, or archives never export without policy-labeled redaction and actor lineage — rather than a green summary across the shared editor replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_editor_replica_view**: `stable`
  - Owner: Shared-editor-replica-view owner
  - Scope: The shared editor replica view resolves each anchor move — buffer edit, generated-output refresh, or imported-snapshot transition — to one typed anchor-history record naming the comment / annotation / review-pin identity, the textual and semantic anchors, the revision-pair lineage across which the anchor moved, the drift / unresolved / rebound state it lands in, and its export posture, and to the rebind-review disposition offering keep-textual, keep-semantic, manual-rebind, assisted-rebind, or leave-unresolved with actor / time provenance; the append-only drift history is recorded before the pin moves, and a record that cannot bind its anchor identity or that would let a drift read as clean while a comment or pin jumped without recorded drift history degrades honestly instead of silently rebinding the anchor
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **presence_cursor_layer**: `stable`
  - Owner: Presence-cursor-layer owner
  - Scope: The presence / cursor layer resolves an anchor drift near a live cursor or selection to the anchor-history record that marks the pin unresolved-pending-rebind — carrying the provenance and freshness a consumer must see — rather than letting a pin quietly snap to a new location; a drift that would move a pin while reporting its anchor as current and a stale unresolved state shown as rebound degrade honestly instead of letting an anchor drift read as clean
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the recorded anchor-history entries and their rebind-review dispositions while keeping the anchor identity, current anchor state, drift history, and export posture bound to the export route, so an export or support flow can show the current anchor state plus the full history of drift and rebound decisions; a disposition whose actor / time provenance is unstated and an exported op-log, snapshot, or archive that would export without policy-labeled redaction and actor lineage degrade honestly so the anchor identity, rebind decision, and provenance are never dropped on export or companion handoff
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **comment_annotation_review_pin_layer**: `stable`
  - Owner: Comment-annotation-review-pin-layer owner
  - Scope: The comment / annotation / review-pin layer resolves a drift over server-ordered comments, annotations, and review pins to an anchor-history record that keeps the textual anchor, semantic anchor, and revision-pair lineage as an append-only, reviewable drift history with a manual or assisted rebind review sheet; a disposition whose rebind decision is unstated and a rebind applied without review degrade honestly instead of silently jumping a comment or pin to a new location
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **collaboration_degradation_banner**: `stable`
  - Owner: Collaboration-degradation-banner owner
  - Scope: The collaboration degradation banner resolves an anchor-unresolved condition to an anchor-history record that names the distinct drift / unresolved / rebound state — from a buffer edit, generated-output refresh, or imported-snapshot transition — and surfaces the rebind-review disposition (keep-textual, keep-semantic, manual-rebind, assisted-rebind, or leave-unresolved) rather than a generic stale or broken badge; a drift whose drift state is unstated and a distinct anchor-unresolved state collapsed into a generic stale badge degrade honestly so a drifted anchor never disappears behind an undifferentiated banner
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same anchor-history and rebind-review truth, so a drift whose drift state is unstated, an anchor rebound without recorded drift history, or a rebind applied without review is visible in evidence — each drift state named as drift, unresolved, or rebound, and each decision as keep-textual, keep-semantic, manual-rebind, assisted-rebind, or leave-unresolved — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
