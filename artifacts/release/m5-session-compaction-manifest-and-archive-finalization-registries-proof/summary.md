# M5 Session-Compaction-Manifest and Archive-Finalization Registries

- Packet: `m5-session-compaction-manifest-and-archive-finalization-registries:stable:0001`
- Label: `M5 session-compaction-manifest and archive-finalization registries emitting one durable append-only machine-readable session-compaction-manifest record per compaction of the CRDT-backed session text — bounding op-log and tombstone growth after a buffer edit, generated-output refresh, or imported-snapshot transition — one typed field per record section: the snapshot ID, the retained-object references it keeps, the tombstone / op-log bounds it draws, the actor provenance of the compaction, and the export posture — so a compaction preserves inspectable snapshot / manifest lineage first rather than reclaiming state as hidden storage churn, with canonical / accessible / audit resolution-form coverage, and a machine-readable session-archive-finalization-descriptor record per compaction that declares the finalization decision a user can take (finalize-archive, export-snapshot, export-op-log, redact-and-finalize, or leave-open) with its actor / time provenance and its policy-labeled export lineage — so collaboration compaction is inspectable through snapshot / manifest lineage rather than hidden storage churn, session archives and optional op-log exports preserve actor provenance and the compaction / redaction / omission facts support or audit flows can explain, an archive never finalizes without recorded compaction lineage, and exported op-logs, snapshots, or archives never export without policy-labeled redaction and actor lineage — rather than a green summary across the shared editor replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_editor_replica_view**: `stable`
  - Owner: Shared-editor-replica-view owner
  - Scope: The shared editor replica view resolves each compaction of the CRDT-backed session text — bounding op-log and tombstone growth after a buffer edit, generated-output refresh, or imported-snapshot transition — to one typed session-compaction-manifest record naming the snapshot ID, the retained-object references it keeps, the tombstone / op-log bounds it draws, the actor provenance of the compaction, and its export posture, and to the session-archive-finalization descriptor offering finalize-archive, export-snapshot, export-op-log, redact-and-finalize, or leave-open with actor / time provenance; the compaction lineage is recorded before storage is reclaimed, and a manifest that cannot bind its snapshot identity or that would let compaction read as clean while state was reclaimed as hidden storage churn without inspectable snapshot / manifest lineage degrades honestly instead of silently discarding retained session state
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **presence_cursor_layer**: `stable`
  - Owner: Presence-cursor-layer owner
  - Scope: The presence / cursor layer resolves a compaction that bounds presence-adjacent op-log growth to the session-compaction-manifest record that keeps the snapshot ID and retained-object references inspectable — carrying the provenance and freshness a consumer must see — rather than letting reclaimed state vanish as silent storage churn; a compaction that would reclaim state while reporting the session as un-compacted and a stale snapshot shown as current degrade honestly instead of letting compaction read as clean
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the recorded session-compaction-manifest entries and their session-archive-finalization descriptors while keeping the snapshot identity, retained-object references, compaction / redaction / omission facts, and export posture bound to the export route, so an export or support flow can explain what was compacted, redacted, or omitted and preserve actor provenance; a descriptor whose actor / time provenance is unstated and an exported op-log, snapshot, or archive that would export without policy-labeled redaction and actor lineage degrade honestly so the snapshot identity, finalization decision, and provenance are never dropped on export or companion handoff
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **comment_annotation_review_pin_layer**: `stable`
  - Owner: Comment-annotation-review-pin-layer owner
  - Scope: The comment / annotation / review-pin layer resolves compaction over server-ordered comments, annotations, and review pins to a session-compaction-manifest record that keeps the snapshot ID, retained-object references, and tombstone / op-log bounds as an append-only, inspectable compaction lineage with a finalize-or-export descriptor; a descriptor whose finalization decision is unstated and an archive finalized without recorded compaction lineage degrade honestly instead of silently discarding retained comment or pin state
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **collaboration_degradation_banner**: `stable`
  - Owner: Collaboration-degradation-banner owner
  - Scope: The collaboration degradation banner resolves a compaction-bounded or archive-finalized condition to a session-compaction-manifest record that names the distinct compacted / retained / tombstoned state and surfaces the session-archive-finalization descriptor (finalize-archive, export-snapshot, export-op-log, redact-and-finalize, or leave-open) rather than a generic stale or broken badge; a compaction whose retained state is unstated and a distinct archive-finalized state collapsed into a generic stale badge degrade honestly so bounded session state never disappears behind an undifferentiated banner
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same session-compaction and archive-finalization truth, so a compaction whose retained state is unstated, an archive finalized without recorded compaction lineage, or a snapshot / op-log exported without policy-labeled redaction is visible in evidence — each state named as compacted, retained, or tombstoned, and each decision as finalize-archive, export-snapshot, export-op-log, redact-and-finalize, or leave-open — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
